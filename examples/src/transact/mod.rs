#[cfg(test)]
mod tests {
	use crate::simple_test_net::*;
	use codec::Encode;
	use frame_support::{assert_ok, pallet_prelude::Weight};
	use xcm::latest::prelude::*;
	use xcm_simulator::TestExt;

	const AMOUNT: u128 = 1 * CENTS;

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
				new_free: 5 * AMOUNT,
			},
		);

		let message = Xcm(vec![
			WithdrawAsset((Here, AMOUNT).into()),
			BuyExecution { fees: (Here, AMOUNT).into(), weight_limit: WeightLimit::Unlimited },
			Transact {
				origin_kind: OriginKind::Superuser,
				require_weight_at_most: Weight::from_parts(1_000_000_000, 1024 * 1024),
				call: call.encode().into(),
			},
		]);

		Relay::execute_with(|| {
			assert_ok!(RelaychainPalletXcm::send_xcm(Here, Parachain(1), message.clone(),));
		});

		ParaA::execute_with(|| {
			assert_eq!(ParachainBalances::free_balance(ALICE), 5 * AMOUNT);
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

		let mint =
			relay_chain::RuntimeCall::Uniques(pallet_uniques::Call::<relay_chain::Runtime>::mint {
				collection: 1u32,
				item: 1u32,
				owner: ALICE,
			});

		let message = Xcm(vec![
			WithdrawAsset((Here, AMOUNT).into()),
			BuyExecution { fees: (Here, AMOUNT).into(), weight_limit: WeightLimit::Unlimited },
			Transact {
				origin_kind: OriginKind::SovereignAccount,
				require_weight_at_most: Weight::from_parts(INITIAL_BALANCE as u64, 1024 * 1024),
				call: create_collection.encode().into(),
			},
			Transact {
				origin_kind: OriginKind::SovereignAccount,
				require_weight_at_most: Weight::from_parts(INITIAL_BALANCE as u64, 1024 * 1024),
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
}
