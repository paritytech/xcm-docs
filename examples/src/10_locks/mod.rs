#[cfg(test)]
mod tests {
	use crate::simple_test_net::*;
	use frame_support::assert_ok;
	use pallet_balances::{BalanceLock, Reasons};
	use xcm::latest::prelude::*;
	use xcm_simulator::TestExt;

	/// Scenario:
	/// ALICE from parachain A locks 5 cents of relay chain native assets of its Sovereign account on the relay chain and assigns Parachain B as unlocker.
	/// Parachain A then asks Parachain B to unlock the funds partly. Parachain B responds by sending an UnlockAssets instruction to the relay chain.
	#[ignore] // TODO: Fix issue upstream
	#[test]
	fn remote_locking_on_relay() {
		MockNet::reset();

		let fee = relay_chain::estimate_message_fee(4); // Accounts for the `DescendOrigin` instruction added by `send_xcm`

		ParaA::execute_with(|| {
			let message = Xcm(vec![
				WithdrawAsset((Here, fee).into()),
				BuyExecution { fees: (Here, fee).into(), weight_limit: WeightLimit::Unlimited },
				LockAsset { asset: (Here, 5 * CENTS).into(), unlocker: (Parachain(2)).into() },
			]);
			let interior = AccountId32 { id: ALICE.into(), network: None };
			assert_ok!(ParachainPalletXcm::send_xcm(interior, Parent, message.clone()));
		});

		Relay::execute_with(|| {
			assert_eq!(
				relay_chain::Balances::locks(&parachain_account_sovereign_account_id(1, ALICE)),
				vec![BalanceLock { id: *b"py/xcmlk", amount: 5 * CENTS, reasons: Reasons::All }]
			);
		});

		ParaB::execute_with(|| {
			assert_eq!(
				parachain::MsgQueue::received_dmp(),
				vec![Xcm(vec![NoteUnlockable {
					owner: (Parent, Parachain(1), AccountId32 { id: ALICE.into(), network: None })
						.into(),
					asset: (Parent, 5 * CENTS).into()
				}])]
			);
		});

		ParaA::execute_with(|| {
			let message = Xcm(vec![
				WithdrawAsset((Parent, fee).into()),
				BuyExecution { fees: (Parent, fee).into(), weight_limit: WeightLimit::Unlimited },
				RequestUnlock { asset: (Parent, 3 * CENTS).into(), locker: Parent.into() },
			]);
			let interior = AccountId32 { id: ALICE.into(), network: None };
			assert_ok!(ParachainPalletXcm::send_xcm(
				interior,
				(Parent, Parachain(2)),
				message.clone()
			));
		});

		Relay::execute_with(|| {
			assert_eq!(
				relay_chain::Balances::locks(&parachain_account_sovereign_account_id(1, ALICE)),
				vec![BalanceLock { id: *b"py/xcmlk", amount: 2 * CENTS, reasons: Reasons::All }]
			);
		});
	}

	/// Scenario:
	/// Parachain A sets two locks with Parachain B and Parachain C as unlockers on the relay chain.
	/// Parachain A then requests Parachain B to partly unlock.
	/// Note: The locks overlap.
	/// Steps:
	/// 1) Set locks on the relay chain.
	/// Unlockers: B, C; Funds registered in pallet-xcm: 10, 5.
	/// Lock set in pallet-balances: 10.
	/// 2) Parachain B and C receive `NoteUnlockable` instruction.
	/// 3) Parachain A sends an `RequestUnlock` instruction to Parachain B for 8 Cents.
	/// 4) Parachain B Unlocks a part of the funds by sending a `UnlockAsset` instruction to the relay chain.
	/// Unlockers: B, C; Funds registered in pallet-xcm: 2, 5.
	/// Lock set in pallet-balances: 5.
	///
	#[ignore] // TODO: Fix issue upstream
	#[test]
	fn locking_overlap() {
		MockNet::reset();

		let fee = relay_chain::estimate_message_fee(4);

		// 1)
		ParaA::execute_with(|| {
			let message = Xcm(vec![
				WithdrawAsset((Here, fee).into()),
				BuyExecution { fees: (Here, fee).into(), weight_limit: WeightLimit::Unlimited },
				LockAsset { asset: (Here, 10 * CENTS).into(), unlocker: (Parachain(2)).into() },
				LockAsset { asset: (Here, 5 * CENTS).into(), unlocker: (Parachain(3)).into() },
			]);
			let interior = AccountId32 { id: ALICE.into(), network: None };
			assert_ok!(ParachainPalletXcm::send_xcm(interior, Parent, message.clone()));
		});

		Relay::execute_with(|| {
			assert_eq!(
				relay_chain::Balances::locks(&parachain_sovereign_account_id(1)),
				vec![BalanceLock { id: *b"py/xcmlk", amount: 10 * CENTS, reasons: Reasons::All }]
			);
		});

		// 2)
		ParaB::execute_with(|| {
			assert_eq!(
				parachain::MsgQueue::received_dmp(),
				vec![Xcm(vec![NoteUnlockable {
					owner: (Parent, Parachain(1), AccountId32 { id: ALICE.into(), network: None })
						.into(),
					asset: (Parent, 10 * CENTS).into()
				}])]
			);
		});

		ParaC::execute_with(|| {
			assert_eq!(
				parachain::MsgQueue::received_dmp(),
				vec![Xcm(vec![NoteUnlockable {
					owner: (Parent, Parachain(1), AccountId32 { id: ALICE.into(), network: None })
						.into(),
					asset: (Parent, 5 * CENTS).into()
				}])]
			);
		});

		let new_fee = parachain::estimate_message_fee(3);

		// 3)
		ParaA::execute_with(|| {
			let message = Xcm(vec![
				WithdrawAsset((Parent, new_fee).into()),
				BuyExecution {
					fees: (Parent, new_fee).into(),
					weight_limit: WeightLimit::Unlimited,
				},
				RequestUnlock { asset: (Parent, 8 * CENTS).into(), locker: Parent.into() },
			]);
			let interior = AccountId32 { id: ALICE.into(), network: None };
			assert_ok!(ParachainPalletXcm::send_xcm(
				interior,
				(Parent, Parachain(2)),
				message.clone()
			));
		});

		// 4)
		Relay::execute_with(|| {
			assert_eq!(
				relay_chain::Balances::locks(&parachain_sovereign_account_id(1)),
				vec![BalanceLock { id: *b"py/xcmlk", amount: 5 * CENTS, reasons: Reasons::All }]
			);
		});
	}
}
