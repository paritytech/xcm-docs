#[cfg(test)]
mod tests {
	use crate::simple_test_net::*;
	use frame_support::{assert_ok, pallet_prelude::Weight};
	use xcm::latest::prelude::*;
	use xcm_simulator::TestExt;

	const QUERY_ID: u64 = 1234;

	/// Scenario:
	#[test]
	fn descend_origin() {
		MockNet::reset();
		ParaA::execute_with(|| {
			let message_fee = parachain::estimate_message_fee(6);
			let message = Xcm(vec![
				WithdrawAsset((Here, message_fee).into()),
				BuyExecution {
					fees: (Here, message_fee).into(),
					weight_limit: WeightLimit::Unlimited,
				},
				// Set the instructions that are executed when ExpectOrigin does not pass.
				// In this case, reporting back an error to the Parachain.
				SetErrorHandler(Xcm(vec![ReportError(QueryResponseInfo {
					destination: Parachain(1).into(),
					query_id: QUERY_ID,
					max_weight: Weight::from_all(0),
				})])),
				DescendOrigin((PalletInstance(1)).into()),
				// Checks if the XcmContext origin descended to `Parachain(1)/PalletInstance(1)`.
				ExpectOrigin(Some((Parachain(1), PalletInstance(1)).into())),
			]);
			assert_ok!(ParachainPalletXcm::send_xcm(Here, Parent, message.clone(),));
		});

		Relay::execute_with(|| {
			assert!(relay_successful_execution());
		});

		// Check that message queue is empty.
		// The ExpectOrigin instruction passed so we should not receive an error response.
		ParaA::execute_with(|| assert_eq!(parachain::MsgQueue::received_dmp(), vec![]));
	}
}
