#[cfg(test)]
mod tests {
	use crate::simple_test_net::*;
	use frame_support::assert_ok;
	use xcm::latest::prelude::*;
	use xcm_simulator::TestExt;

	/// Scenario:
	/// ALICE transfers her FDOT from parachain A to parachain B.
	#[test]
	fn reserve_backed_transfer_para_to_para() {
		MockNet::reset();

		let withdraw_amount = 50 * CENTS;

		let message: Xcm<parachain::RuntimeCall> = Xcm(vec![
			WithdrawAsset((Parent, withdraw_amount).into()),
			InitiateReserveWithdraw {
				assets: All.into(),
				reserve: Parent.into(),
				xcm: Xcm(vec![
					BuyExecution {
						fees: (Here, withdraw_amount).into(),
						weight_limit: WeightLimit::Unlimited,
					},
					DepositReserveAsset {
						assets: All.into(),
						dest: Parachain(2).into(),
						xcm: Xcm(vec![DepositAsset {
							assets: All.into(),
							beneficiary: Junction::AccountId32 { id: ALICE.into(), network: None }
								.into(),
						}]),
					},
				]),
			},
		]);

		ParaA::execute_with(|| {
			assert_ok!(parachain::PolkadotXcm::execute(
				parachain::RuntimeOrigin::signed(ALICE),
				Box::new(xcm::VersionedXcm::V3(message.into())),
				(100_000_000_000, 100_000_000_000).into(),
			));

			assert_eq!(parachain::Assets::balance(1, &ALICE), INITIAL_BALANCE - withdraw_amount);
		});

		Relay::execute_with(|| {
			assert_eq!(
				relay_chain::Balances::free_balance(&parachain_sovereign_account_id(2)),
				INITIAL_BALANCE + withdraw_amount
			);
		});

		ParaB::execute_with(|| {
			assert_eq!(parachain::Assets::balance(1, &ALICE), INITIAL_BALANCE + withdraw_amount);
		});
	}

	/// Scenario:
	/// ALICE transfers her FDOT from relay to parachain B.
	#[test]
	fn reserve_backed_transfer_relay_to_para() {
		MockNet::reset();

		let withdraw_amount = 50 * CENTS;

		let message: Xcm<parachain::RuntimeCall> = Xcm(vec![TransferReserveAsset {
			assets: (Here, withdraw_amount).into(),
			dest: Parachain(2).into(),
			xcm: Xcm(vec![
				BuyExecution {
					fees: (Here, withdraw_amount).into(),
					weight_limit: WeightLimit::Unlimited,
				},
				DepositAsset {
					assets: All.into(),
					beneficiary: Junction::AccountId32 { id: ALICE.into(), network: None }.into(),
				},
			]),
		}]);

		Relay::execute_with(|| {
			assert_ok!(relay_chain::XcmPallet::execute(
				relay_chain::RuntimeOrigin::signed(ALICE),
				Box::new(xcm::VersionedXcm::V3(message.into())),
				(100_000_000_000, 100_000_000_000).into(),
			));

			// ALICE's balance in the relay chain decreases
			assert_eq!(
				relay_chain::Balances::free_balance(&ALICE),
				INITIAL_BALANCE - withdraw_amount
			);

			// Parachain(2)'s sovereign account's balance increases
			assert_eq!(
				relay_chain::Balances::free_balance(&parachain_sovereign_account_id(2)),
				INITIAL_BALANCE + withdraw_amount
			);
		});

		ParaB::execute_with(|| {
			assert_eq!(parachain::Assets::balance(1, &ALICE), INITIAL_BALANCE + withdraw_amount);
		});
	}
}
