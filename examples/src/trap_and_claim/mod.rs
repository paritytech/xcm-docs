#[cfg(test)]
mod tests {
	use crate::simple_test_net::*;
	use frame_support::{assert_ok, pallet_prelude::Weight};
	use xcm::latest::prelude::*;
	use xcm_simulator::TestExt;

	const QUERY_ID: u64 = 1234;

	/// Scenario:
	/// Parachain A withdraws funds from its sovereign account on the relay chain.
	/// The assets are trapped because an error is thrown and the execution is halted.
	/// Parachain A claims the trapped assets and receives a report of the holding register.
	/// It then deposits the assets in the account of ALICE.
	#[test]
	fn trap_and_claim_assets() {
		let message = Xcm(vec![
			WithdrawAsset((Here, 10 * CENTS).into()),
			BuyExecution { fees: (Here, CENTS).into(), weight_limit: WeightLimit::Unlimited },
			Trap(0), // <-- Errors
			DepositAsset {
				// <-- Not executed because of error.
				assets: All.into(),
				beneficiary: AccountId32 {
					network: Some(parachain::RelayNetwork::get()),
					id: ALICE.into(),
				}
				.into(),
			},
		]);
		ParaA::execute_with(|| {
			assert_ok!(ParachainPalletXcm::send_xcm(Here, Parent, message.clone()));
		});

		let claim_message = Xcm(vec![
			ClaimAsset { assets: (Here, 10 * CENTS).into(), ticket: Here.into() },
			ReportHolding {
				response_info: QueryResponseInfo {
					destination: Parachain(1).into(),
					query_id: QUERY_ID,
					max_weight: Weight::from_parts(1_000_000_000, 64 * 64),
				},
				assets: All.into(),
			},
			DepositAsset {
				assets: All.into(),
				beneficiary: AccountId32 {
					network: Some(parachain::RelayNetwork::get()),
					id: ALICE.into(),
				}
				.into(),
			},
		]);
		ParaA::execute_with(|| {
			assert_ok!(ParachainPalletXcm::send_xcm(Here, Parent, claim_message.clone()));
		});

		Relay::execute_with(|| {
			assert_eq!(RelaychainBalances::free_balance(ALICE), INITIAL_BALANCE + 10 * CENTS);
		});

		ParaA::execute_with(|| {
			assert_eq!(
				parachain::MsgQueue::received_dmp(),
				vec![Xcm(vec![QueryResponse {
					query_id: QUERY_ID,
					response: Response::Assets((Parent, 10 * CENTS).into()),
					max_weight: Weight::from_parts(1_000_000_000, 64 * 64),
					querier: Some(Here.into()),
				}])],
			)
		});
	}
}
