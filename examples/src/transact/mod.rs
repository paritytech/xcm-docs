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
		// set_balance requires root origin.
		let call = parachain::RuntimeCall::Balances(
			pallet_balances::Call::<parachain::Runtime>::set_balance {
				who: ALICE,
				new_free: 5 * CENTS,
				new_reserved: 0,
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

	#[test]
	fn transact_para_to_para() {
		MockNet::reset();

		// We use a simple call, just a transfer, could've been done with the TransferAsset instruction
		let call = parachain::RuntimeCall::Balances(
			pallet_balances::Call::<parachain::Runtime>::transfer { dest: BOB, value: 5 * CENTS },
		);

		// We calculate and estimate fees
		let message_fee = parachain::estimate_message_fee(3);
		let estimated_call_weight = Weight::from_parts(100_000_000, 100_000);
		let estimated_call_fee = parachain::estimate_fee(estimated_weight_call);

		// We withdraw enough assets for fee payment from ALICE's account on parachain A
		// and call `Transact` on parachain B
		let message = Xcm(vec![
			WithdrawAsset((Here, message_fee + estimated_call_fee).into()),
			BuyExecution { fees: (Here, message_fee).into(), weight_limit: WeightLimit::Unlimited },
			Transact {
				origin_kind: OriginKind::SovereignAccount, // We are calling transact from a sovereign account
				require_weight_at_most: estimated_call_weight,
				call: call.encode().into(),
			},
		]);

		// We want the origin to be ../Parachain(1)/ALICE
		let interior = AccountId32 { id: ALICE.into(), network: None };

		ParaA::execute_with(|| {
			assert_ok!(parachain::PolkadotXcm::send_xcm(
				interior,
				(Parent, Parachain(2)),
				message.clone()
			));
		});

		ParaB::execute_with(|| {
			// The sovereign account of ../Parachain(1)/ALICE pays 5 cents to BOB's local account
			assert_eq!(
				parachain::Balances::free_balance(parachain_account_sovereign_account_id(1, ALICE)),
				INITIAL_BALANCE - 5 * CENTS
			);
			assert_eq!(parachain::Balances::free_balance(BOB), INITIAL_BALANCE + 5 * CENTS);
		});
	}
}
