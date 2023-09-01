#[cfg(test)]
mod tests {
	// use crate::test_net::kusama_test_net::*;
	use crate::simple_test_net::*;
	use frame_support::assert_ok;
	pub use xcm::latest::prelude::*;
	use xcm_simulator::TestExt;

	#[test]
	fn para_a_simple_transfer() {
		MockNet::reset();

		ParaA::execute_with(|| {
			// Amount to transfer.
			let amount: u128 = 10 * CENTS;
			// Check that the balance of Alice is equal to the `INITIAL_BALANCE`.
			assert_eq!(ParachainBalances::free_balance(&ALICE), INITIAL_BALANCE);

			let fee = parachain::estimate_message_fee(3);

			// The XCM used to transfer funds from Alice to Bob.
			let message = Xcm(vec![
				WithdrawAsset(vec![(Here, amount).into(), (Parent, fee).into()].into()),
				BuyExecution { fees: (Parent, fee).into(), weight_limit: WeightLimit::Unlimited },
				DepositAsset {
					assets: All.into(),
					beneficiary: MultiLocation {
						parents: 0,
						interior: Junction::AccountId32 { network: None, id: BOB.clone().into() }
							.into(),
					}
					.into(),
				},
			]);

			// Execution of the XCM Instructions in the local consensus system.
			// The RuntimeOrigin is Alice, so Alice's account will be used for the WithdrawAsset.
			assert_ok!(ParachainPalletXcm::execute(
				parachain::RuntimeOrigin::signed(ALICE),
				Box::new(xcm::VersionedXcm::from(message.clone())),
				(100_000_000_000, 100_000_000_000).into()
			));

			// Check if the funds are subtracted from the account of Alice and added to the account of Bob.
			assert_eq!(ParachainBalances::free_balance(ALICE), INITIAL_BALANCE - amount);
			assert_eq!(parachain::Assets::balance(0, ALICE), INITIAL_BALANCE - fee);
			assert_eq!(ParachainBalances::free_balance(BOB), INITIAL_BALANCE + amount);
		});
	}
}
