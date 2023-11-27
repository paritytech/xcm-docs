#[cfg(test)]
mod tests {
	use crate::simple_test_net::*;
	use codec::Encode;
	use frame_support::{assert_ok, pallet_prelude::Weight};
	use xcm::latest::prelude::*;
	use xcm_simulator::TestExt;

	/// Scenario:
	/// Relay chain sets the balance of Alice on Parachain(1).
	/// The relay chain is able to do this, because Parachain(1) trusts the relay chain to execute runtime calls as root.
	#[test]
	fn transact_set_balance() {
		MockNet::reset();
		// Runtime call dispatched by the Transact instruction.
		// force_set_balance requires root origin.
		let call = parachain::RuntimeCall::Balances(
			pallet_balances::Call::<parachain::Runtime>::force_set_balance {
				who: ALICE,
				new_free: 5 * CENTS,
			},
		);

		let message_fee = parachain::estimate_message_fee(3);
		let set_balance_weight_estimation = Weight::from_parts(1_000_000_000, 10_000);
		let set_balance_fee_estimation =
			parachain::estimate_fee_for_weight(set_balance_weight_estimation);
		let fees = message_fee + set_balance_fee_estimation;

		let message = Xcm(vec![
			WithdrawAsset((Parent, fees).into()),
			BuyExecution { fees: (Parent, fees).into(), weight_limit: WeightLimit::Unlimited },
			Transact {
				origin_kind: OriginKind::Superuser,
				require_weight_at_most: set_balance_weight_estimation,
				call: call.encode().into(),
			},
		]);

		Relay::execute_with(|| {
			assert_ok!(RelaychainPalletXcm::send_xcm(Here, Parachain(1), message.clone(),));
		});

		ParaA::execute_with(|| {
			assert_eq!(ParachainBalances::free_balance(ALICE), 5 * CENTS);
		})
	}

	/// Scenario:
	/// Parachain A sends two transact instructions to the relay chain.
	/// The first instruction creates a NFT collection with as admin Parachain A.
	/// The second instruction mints a NFT for the collection with as Owner ALICE.
	#[test]
	fn transact_mint_nft() {
		MockNet::reset();

		let create_collection = relay_chain::RuntimeCall::Uniques(pallet_uniques::Call::<
			relay_chain::Runtime,
		>::create {
			collection: 1u32,
			admin: parachain_sovereign_account_id(1),
		});

		let message_fee = relay_chain::estimate_message_fee(4);
		let create_collection_weight_estimation = Weight::from_parts(1_000_000_000, 10_000);
		let create_collection_fee_estimation =
			relay_chain::estimate_fee_for_weight(create_collection_weight_estimation);
		let mint_nft_weight_estimation = Weight::from_parts(1_000_000_000, 10_000);
		let mint_nft_fee_estimation =
			relay_chain::estimate_fee_for_weight(mint_nft_weight_estimation);
		let fees = message_fee + create_collection_fee_estimation + mint_nft_fee_estimation;

		let mint =
			relay_chain::RuntimeCall::Uniques(pallet_uniques::Call::<relay_chain::Runtime>::mint {
				collection: 1u32,
				item: 1u32,
				owner: ALICE,
			});

		let message = Xcm(vec![
			WithdrawAsset((Here, fees).into()),
			BuyExecution { fees: (Here, fees).into(), weight_limit: WeightLimit::Unlimited },
			Transact {
				origin_kind: OriginKind::SovereignAccount,
				require_weight_at_most: create_collection_weight_estimation,
				call: create_collection.encode().into(),
			},
			Transact {
				origin_kind: OriginKind::SovereignAccount,
				require_weight_at_most: mint_nft_weight_estimation,
				call: mint.encode().into(),
			},
		]);

		// Create collection with Alice as owner.
		ParaA::execute_with(|| {
			assert_ok!(ParachainPalletXcm::send_xcm(Here, Parent, message.clone()));
		});

		Relay::execute_with(|| {
			assert_eq!(
				relay_chain::Uniques::collection_owner(1u32),
				Some(parachain_sovereign_account_id(1))
			);
			assert_eq!(relay_chain::Uniques::owner(1u32, 1u32), Some(ALICE));
		});
	}
	
	/// Scenario: Alice, an account in ParaA, usually deals with assets in ParaB via
	/// her sovereign account. She loses access to her account, but, thankfully, set up
	/// her recovery in ParaA. With this, she manages to get her account back and keeps on
	/// dealing with assets as normal in ParaB.
	#[test]
	fn recovery_pallet_works_cross_chain() {
		MockNet::reset();

		// Recovery parameters
		let friends: Vec<sp_runtime::AccountId32> = vec![
			[1u8; 32].into(),
			[2u8; 32].into(),
			[3u8; 32].into(),
		];
		let threshold = 3;
		let delay_period = 5;
		let rescuer = sp_runtime::AccountId32::new([7u8; 32]);

		let beneficiary_in_para_b = sp_runtime::AccountId32::new([12u8; 32]);

		ParaB::execute_with(|| {
			dbg!(&parachain::Balances::free_balance(&sibling_account_sovereign_account_id(1, ALICE)));
			dbg!(&parachain::Balances::locks(&sibling_account_sovereign_account_id(1, ALICE)));
		});

		let alice_from_para_b: MultiLocation = (
			Parent,
			Parachain(1),
			AccountId32 {
				id: ALICE.clone().into(),
				network: Some(NetworkId::Kusama)
			}
		).into();

		ParaA::execute_with(|| {
			// Alice sets up recovery, in case something bad happens.
			assert_ok!(parachain::Recovery::create_recovery(
				parachain::RuntimeOrigin::signed(ALICE),
				friends.clone(),
				threshold,
				delay_period,
			));

			// She then goes on handling assets in ParaB
			let message = Xcm::<()>::builder_unsafe()
				.lock_asset((Here, 30).into(), Parent.into())
				.build();
			assert_ok!(parachain::PolkadotXcm::send(
				parachain::RuntimeOrigin::signed(ALICE),
				Box::new(xcm::VersionedMultiLocation::V3((Parent, Parachain(2)).into())),
				Box::new(xcm::VersionedXcm::V3(message.into())),
			));
		});

		// Assets get changed in ParaB
		ParaB::execute_with(|| {
			dbg!(&parachain::Balances::free_balance(&sibling_account_sovereign_account_id(1, ALICE)));
			dbg!(&parachain::Balances::locks(&sibling_account_sovereign_account_id(1, ALICE)));
		});

		ParaA::execute_with(|| {
			// Alice can unlock her funds on ParaB, but
			// no other account in ParaA can
			let message = Xcm::<()>::builder_unsafe()
				.request_unlock((Parachain(2), 20).into(), Parachain(2).into())
				.build();
			// TODO: This doesn't work, but the extrinsic still succeeds
			let _ = parachain::PolkadotXcm::send(
				parachain::RuntimeOrigin::signed(rescuer.clone()),
				Box::new(xcm::VersionedMultiLocation::V3(Parent.into())),
				Box::new(xcm::VersionedXcm::V3(message.into())),
			);
		});

		// No assets are unlocked
		ParaB::execute_with(|| {
			dbg!(&parachain::Balances::locks(&sibling_account_sovereign_account_id(1, ALICE)));
		});

		ParaA::execute_with(|| {
			// Some time has passed, and Alice lost her account's key!
			parachain::run_to_block(10);
			// She uses a new account, rescuer, to try to recover her funds
			assert_ok!(parachain::Recovery::initiate_recovery(
				parachain::RuntimeOrigin::signed(rescuer.clone()),
				ALICE,
			));
			// Her friends vouch for her new account
			assert_ok!(parachain::Recovery::vouch_recovery(parachain::RuntimeOrigin::signed(friends[0].clone()), ALICE, rescuer.clone()));
			assert_ok!(parachain::Recovery::vouch_recovery(parachain::RuntimeOrigin::signed(friends[1].clone()), ALICE, rescuer.clone()));
			assert_ok!(parachain::Recovery::vouch_recovery(parachain::RuntimeOrigin::signed(friends[2].clone()), ALICE, rescuer.clone()));
			parachain::run_to_block(10 + delay_period);
			assert_ok!(parachain::Recovery::claim_recovery(parachain::RuntimeOrigin::signed(rescuer.clone()), ALICE));
			dbg!(parachain::Balances::free_balance(&ALICE));
			dbg!(parachain::Balances::free_balance(&rescuer));

			// Now, she can unlock assets with the new account
			let message = Xcm::<()>::builder_unsafe()
				.request_unlock((Parachain(2), 20).into(), Parachain(2).into())
				.build();
			let call = Box::new(parachain::RuntimeCall::PolkadotXcm(pallet_xcm::Call::<parachain::Runtime>::send {
				dest: Box::new(xcm::VersionedMultiLocation::V3(Parent.into())),
				message: Box::new(xcm::VersionedXcm::V3(message.into())),
			}));
			assert_ok!(parachain::Recovery::as_recovered(
				parachain::RuntimeOrigin::signed(rescuer.clone()),
				ALICE.clone(),
				call
			));
		});

		// Assets were unlocked by the new account! No need to handle recovery on any other chain
		ParaB::execute_with(|| {
			dbg!(&parachain::Balances::locks(&sibling_account_sovereign_account_id(1, ALICE)));
		});
	}
}
