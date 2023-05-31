#[cfg(test)]
mod tests {
	use crate::simple_test_net::*;
	use frame_support::assert_ok;
	use xcm::latest::prelude::*;
	use xcm_simulator::TestExt;

	/// Scenario:
	/// ALICE teleports her native assets from the relay chain to parachain A.
	#[test]
	fn teleport_fungible() {
		MockNet::reset();

		let withdraw_amount = 50 * CENTS;

		let fee_in_source = relay_chain::estimate_message_fee(3);
		let fee_in_destination = parachain::estimate_message_fee(4);

		let message: Xcm<relay_chain::RuntimeCall> = Xcm(vec![
			WithdrawAsset((Here, withdraw_amount).into()),
			BuyExecution {
				fees: (Here, fee_in_source).into(),
				weight_limit: WeightLimit::Unlimited,
			},
			InitiateTeleport {
				assets: All.into(),
				dest: Parachain(1).into(),
				xcm: Xcm(vec![
					BuyExecution {
						fees: (Parent, fee_in_destination).into(),
						weight_limit: WeightLimit::Unlimited,
					},
					DepositAsset {
						assets: All.into(),
						beneficiary: Junction::AccountId32 { network: None, id: ALICE.into() }
							.into(),
					},
				]),
			},
		]);

		Relay::execute_with(|| {
			assert_ok!(relay_chain::XcmPallet::execute(
				relay_chain::RuntimeOrigin::signed(ALICE),
				Box::new(xcm::VersionedXcm::V3(message.into())),
				(100_000_000_000, 100_000_000_000).into()
			));

			assert_eq!(
				relay_chain::Balances::free_balance(ALICE),
				INITIAL_BALANCE - withdraw_amount
			);
		});

		ParaA::execute_with(|| {
			let expected_message_received: Xcm<parachain::RuntimeCall> = Xcm(vec![
				ReceiveTeleportedAsset(
					vec![(Parent, withdraw_amount - fee_in_source).into()].into(),
				),
				ClearOrigin,
				BuyExecution {
					fees: (Parent, fee_in_destination).into(),
					weight_limit: WeightLimit::Unlimited,
				},
				DepositAsset {
					assets: All.into(),
					beneficiary: Junction::AccountId32 { network: None, id: ALICE.into() }.into(),
				},
			]);

			assert_eq!(parachain::MsgQueue::received_dmp(), vec![expected_message_received]);

			assert_eq!(
				parachain::Assets::balance(0, &ALICE),
				INITIAL_BALANCE + (withdraw_amount - fee_in_source - fee_in_destination)
			);
		});
	}
}
