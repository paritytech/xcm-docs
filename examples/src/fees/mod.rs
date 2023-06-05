#[cfg(test)]
mod tests {
	use crate::simple_test_net::{*, parachain::estimate_message_fee};
	use frame_support::assert_ok;
	use xcm::latest::prelude::*;
	use xcm_simulator::TestExt;

	/// Scenario:
	/// Relay chain sends a XCM to Parachain A. 
    /// Enough execution is bought for the full message.
	/// However, the message errors at the Trap(1) instruction (simulates error in other instructions).
    /// The last three instructions are not executed 
    /// and the weight surplus of these instructions is refunded to the relay chain account.
	#[test]
	fn refund_surplus() {
		MockNet::reset();
		let message_fee = parachain::estimate_message_fee(9);
		let message = Xcm(vec![
			WithdrawAsset((Parent, message_fee).into()),
			BuyExecution {
				fees: (Parent, message_fee).into(),
				weight_limit: WeightLimit::Unlimited,
			},
			SetErrorHandler(Xcm(vec![
				RefundSurplus,
				DepositAsset {
					assets: All.into(),
					beneficiary: AccountId32 {
						network: Some(ByGenesis([0; 32])),
						id: relay_sovereign_account_id().into(),
					}
					.into(),
				},
			])),
			Trap(1),
            ClearOrigin,
            ClearOrigin,
            ClearOrigin,
		]);

		Relay::execute_with(|| {
			assert_ok!(RelaychainPalletXcm::send_xcm(Here, Parachain(1), message.clone(),));
		});

		ParaA::execute_with(|| {
			assert_eq!(
				ParachainAssets::balance(0, relay_sovereign_account_id()),
				INITIAL_BALANCE - message_fee + estimate_message_fee(3)
			);
		})
	}
}
