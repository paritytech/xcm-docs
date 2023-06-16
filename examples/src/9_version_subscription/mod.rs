#[cfg(test)]
mod tests {
	use crate::simple_test_net::*;
	use frame_support::{assert_ok, pallet_prelude::Weight};
	use xcm::latest::prelude::*;
	use xcm_simulator::TestExt;

	/// Scenario:
	/// Parachain A wants to know which version of Xcm the relay chain uses.
	/// It sends the `SubscribeVersion` instruction to get the Xcm version of the relay chain
	/// and to receive updates if the version changes.
	/// When the parachain receives the version it unsubscribes from version updates.
	#[test]
	fn subscribe_and_unsubscribe_version() {
		MockNet::reset();

		let message_fee = relay_chain::estimate_message_fee(3);

		let query_id_set = 1234;
		let message = Xcm(vec![
			WithdrawAsset((Here, message_fee).into()),
			BuyExecution { fees: (Here, message_fee).into(), weight_limit: WeightLimit::Unlimited },
			SubscribeVersion { query_id: query_id_set, max_response_weight: Weight::from_all(0) },
		]);

		ParaA::execute_with(|| {
			assert_ok!(ParachainPalletXcm::send_xcm(Here, Parent, message.clone()));
		});

		ParaA::execute_with(|| {
			assert_eq!(
				parachain::MsgQueue::received_dmp(),
				vec![Xcm(vec![QueryResponse {
					query_id: query_id_set,
					response: Response::Version(3),
					max_weight: Weight::from_all(0),
					querier: None,
				}])],
			);

			let unsub_message = Xcm(vec![
				UnpaidExecution { weight_limit: WeightLimit::Unlimited, check_origin: None },
				UnsubscribeVersion,
			]);
			assert_ok!(ParachainPalletXcm::send_xcm(Here, Parent, unsub_message));
		});
	}
}
