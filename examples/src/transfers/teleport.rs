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

		let teleport_amount = 50 * CENTS;
		let message: Xcm<relay_chain::RuntimeCall> = Xcm(vec![
			WithdrawAsset((Here, teleport_amount).into()),
			InitiateTeleport {
				assets: AllCounted(1).into(),
				dest: Parachain(1).into(),
				xcm: Xcm(vec![DepositAsset {
					assets: AllCounted(1).into(),
					beneficiary: Junction::AccountId32 { network: None, id: ALICE.into() }.into(),
				}]),
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
				INITIAL_BALANCE - teleport_amount
			);
		});

		ParaA::execute_with(|| {
			let expected_message_received: Xcm<parachain::RuntimeCall> = Xcm(vec![
				ReceiveTeleportedAsset(vec![(Parent, teleport_amount).into()].into()),
				ClearOrigin,
				DepositAsset {
					assets: AllCounted(1).into(),
					beneficiary: Junction::AccountId32 { network: None, id: ALICE.into() }.into(),
				},
			]);

			assert_eq!(parachain::MsgQueue::received_dmp(), vec![expected_message_received]);

			assert_eq!(parachain::Assets::balance(0, &ALICE), INITIAL_BALANCE + teleport_amount);
		});
	}

	/// Scenario:
	/// ALICE teleports her nft from the relay chain to parachain A
	#[test]
	fn teleport_nft() {
		MockNet::reset();

		Relay::execute_with(|| {
			// Mint NFT for Alice on Relay chain
			assert_ok!(relay_chain::Uniques::force_create(
				relay_chain::RuntimeOrigin::root(),
				1,
				ALICE,
				true
			));
			assert_ok!(relay_chain::Uniques::mint(
				relay_chain::RuntimeOrigin::signed(ALICE),
				1,
				42,
				ALICE
			));

			assert_eq!(relay_chain::Uniques::owner(1, 42), Some(ALICE));
		});

		ParaA::execute_with(|| {
			// Create NFT collection representing the relay chain one
			assert_ok!(parachain::ForeignUniques::force_create(
				parachain::RuntimeOrigin::root(),
				1u32,
				ALICE,
				false
			));

			// Alice is Collection Owner.
			assert_eq!(parachain::ForeignUniques::collection_owner(1u32), Some(ALICE));
			// Alice does not own Collection Item 42 yet.
			assert_eq!(parachain::ForeignUniques::owner(1u32, 42u32.into()), None);
			assert_eq!(parachain::Balances::reserved_balance(&ALICE), 0);
		});

		let message: Xcm<relay_chain::RuntimeCall> = Xcm(vec![
			WithdrawAsset((GeneralIndex(1), 42u64).into()),
			InitiateTeleport {
				assets: AllCounted(1).into(),
				dest: Parachain(1).into(),
				xcm: Xcm(vec![DepositAsset {
					assets: AllCounted(1).into(),
					beneficiary: Junction::AccountId32 { id: ALICE.into(), network: None }.into(),
				}]),
			},
		]);

		Relay::execute_with(|| {
			assert_ok!(relay_chain::XcmPallet::execute(
				relay_chain::RuntimeOrigin::signed(ALICE),
				Box::new(xcm::VersionedXcm::V3(message.into())),
				(100_000_000_000, 100_000_000_000).into(),
			));
		});

		ParaA::execute_with(|| {
			assert_eq!(parachain::ForeignUniques::owner(1u32, 42u32.into()), Some(ALICE));
			assert_eq!(parachain::Balances::reserved_balance(&ALICE), 1000);
		});

		Relay::execute_with(|| {
			assert_eq!(relay_chain::Uniques::owner(1, 42), None);
		});
	}
}
