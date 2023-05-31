#[cfg(test)]
mod tests {
	use crate::simple_test_net::{parachain::RelayNativeAsset, *};
	use frame_support::{assert_ok, pallet_prelude::Weight};
	use xcm::latest::prelude::*;
	use xcm_simulator::TestExt;

	const QUERY_ID: u64 = 1234;

	/// Scenario:
	/// Parachain A withdraws funds from its sovereign account on the relay chain and burns part of them.
	/// The relay chain then reports back the status of the Holding Register to Parachain A.
	#[test]
	fn burn_assets() {
		let message = Xcm(vec![
			UnpaidExecution { weight_limit: WeightLimit::Unlimited, check_origin: None },
			WithdrawAsset((Here, 10 * CENTS).into()),
			BurnAsset((Here, 4 * CENTS).into()),
			ReportHolding {
				response_info: QueryResponseInfo {
					destination: Parachain(1).into(),
					query_id: QUERY_ID,
					max_weight: Weight::from_parts(1_000_000_000, 64 * 64),
				},
				assets: All.into(),
			},
		]);
		ParaA::execute_with(|| {
			assert_ok!(ParachainPalletXcm::send_xcm(Here, Parent, message.clone()));
		});

		ParaA::execute_with(|| {
			assert_eq!(
				parachain::MsgQueue::received_dmp(),
				vec![Xcm(vec![QueryResponse {
					query_id: QUERY_ID,
					response: Response::Assets((Parent, 6 * CENTS).into()),
					max_weight: Weight::from_parts(1_000_000_000, 64 * 64),
					querier: Some(Here.into()),
				}])],
			)
		});
	}

	/// Scenario:
	/// The relay chain sends an XCM to Parachain A that:
	/// 1) Withdraws some native assets
	/// 2) Exchanges these assets for relay chain derivative tokens, with maximal set to true.
	/// 3) Deposit all the assets that are in the Holding in the account of Alice.
	///
	/// NOTE: The implementation of the AssetExchanger is simple
	/// and in this case swaps all the assets in the exchange for the assets in `give`.
	/// Depending on the implementation of AssetExchanger, the test results could differ.
	#[test]
	fn exchange_asset_maximal_true() {
		// Exchange contains 10 CENTS worth of parachain A's derivative of the relay token
		let assets_in_exchange = vec![(Parent, 10 * CENTS).into()];
		parachain::set_exchange_assets(assets_in_exchange);

		let message = Xcm(vec![
			UnpaidExecution { weight_limit: WeightLimit::Unlimited, check_origin: None },
			WithdrawAsset((Here, 10 * CENTS).into()),
			// Maximal field set to true.
			ExchangeAsset {
				give: Definite((Here, 5 * CENTS).into()),
				want: (Parent, 5 * CENTS).into(),
				maximal: true,
			},
			DepositAsset {
				assets: AllCounted(2).into(),
				beneficiary: AccountId32 {
					network: Some(parachain::RelayNetwork::get()),
					id: ALICE.into(),
				}
				.into(),
			},
		]);

		Relay::execute_with(|| {
			assert_ok!(RelaychainPalletXcm::send_xcm(Here, Parachain(1), message.clone()));
		});

		ParaA::execute_with(|| {
			assert_eq!(parachain::exchange_assets(), vec![(Here, 5 * CENTS).into()].into());
			assert_eq!(ParachainAssets::balance(0, &ALICE), INITIAL_BALANCE + 10 * CENTS);
			assert_eq!(ParachainBalances::free_balance(ALICE), INITIAL_BALANCE + 5 * CENTS);
		})
	}

	/// Scenario:
	/// The relay chain sends an XCM to Parachain A that:
	/// 1) Withdraws some native assets
	/// 2) Exchanges these assets for relay chain derivative tokens, with maximal set to false.
	/// 3) Deposit all the assets that are in the Holding in the account of Alice.
	///
	/// NOTE: The implementation of the AssetExchanger is simple
	/// and in this case swaps all the assets in the exchange for the assets in `give`.
	/// Depending on the implementation of AssetExchanger, the test results could differ.
	#[test]
	fn exchange_asset_maximal_false() {
		// Exchange contains 10 CENTS worth of parachain A's derivative of the relay token
		let assets_in_exchange = vec![(Parent, 10 * CENTS).into()];
		parachain::set_exchange_assets(assets_in_exchange);

		let message = Xcm(vec![
			UnpaidExecution { weight_limit: WeightLimit::Unlimited, check_origin: None },
			WithdrawAsset((Here, 10 * CENTS).into()),
			// Maximal field set to false.
			ExchangeAsset {
				give: Definite((Here, 5 * CENTS).into()),
				want: (Parent, 5 * CENTS).into(),
				maximal: false,
			},
			DepositAsset {
				assets: AllCounted(2).into(),
				beneficiary: AccountId32 {
					network: Some(parachain::RelayNetwork::get()),
					id: ALICE.into(),
				}
				.into(),
			},
		]);

		Relay::execute_with(|| {
			assert_ok!(RelaychainPalletXcm::send_xcm(Here, Parachain(1), message.clone()));
		});

		ParaA::execute_with(|| {
			assert_eq!(
				parachain::exchange_assets(),
				vec![(Parent, 5 * CENTS).into(), (Here, 5 * CENTS).into()].into()
			);
			assert_eq!(ParachainAssets::balance(0, &ALICE), INITIAL_BALANCE + 5 * CENTS);
			assert_eq!(ParachainBalances::free_balance(ALICE), INITIAL_BALANCE + 5 * CENTS);
		})
	}
}
