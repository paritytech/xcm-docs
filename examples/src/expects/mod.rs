#[cfg(test)]
mod tests {
	use crate::simple_test_net::*;
	use bounded_collections::BoundedVec;
	use codec::Encode;
	use frame_support::{assert_ok, pallet_prelude::Weight};
	use xcm::latest::prelude::*;
	use xcm_simulator::TestExt;

	const AMOUNT: u128 = 50 * CENTS;
	const QUERY_ID: u64 = 1234;

	/// Scenario:
	/// Parachain wants to execute specific instructions on the relay chain that use assets in the holding register.
	/// Before executing these instructions it want to check if the assets in the holding register are expected.
	/// If the assets are not expected, it wants to be notified with an `ExpectationFalse` error.
	/// It first sets an error handler that reports back an error using the `ReportError` instruction.
	/// And adds a `ExpectAsset` instruction just before executing the specific instructions.
	#[test]
	fn expect_asset() {
		MockNet::reset();

		let message_fee = relay_chain::estimate_message_fee(5);

		ParaA::execute_with(|| {
			let message = Xcm(vec![
				WithdrawAsset((Here, AMOUNT + message_fee).into()),
				BuyExecution {
					fees: (Here, message_fee).into(),
					weight_limit: WeightLimit::Unlimited,
				},
				// Set the instructions that are executed when ExpectAsset does not pass.
				// In this case, reporting back an error to the Parachain.
				SetErrorHandler(Xcm(vec![ReportError(QueryResponseInfo {
					destination: Parachain(1).into(),
					query_id: QUERY_ID,
					max_weight: Weight::from_all(0),
				})])),
				ExpectAsset((Here, AMOUNT + 10 * CENTS).into()),
				// Add Instructions that do something with assets in holding when ExpectAsset passes.
			]);
			assert_ok!(ParachainPalletXcm::send_xcm(Here, Parent, message.clone(),));
		});

		let instruction_index_that_errored = 3;

		// Check that QueryResponse message with ExpectationFalse error was received.
		ParaA::execute_with(|| {
			assert_eq!(
				parachain::MsgQueue::received_dmp(),
				vec![Xcm(vec![QueryResponse {
					query_id: QUERY_ID,
					response: Response::ExecutionResult(Some((
						instruction_index_that_errored,
						XcmError::ExpectationFalse
					))),
					max_weight: Weight::from_all(0),
					querier: Some(Here.into()),
				}])],
			);
		});
	}

	/// Scenario:
	/// Parachain wants to make sure that XcmContext contains the expected `origin` at a certain point during execution.
	/// It sets the `ExpectOrigin` instruction to check for the expected `origin`.
	/// If the origin is not as expected, the instruction errors, and the ErrorHandler reports back the corresponding error to the parachain.
	#[test]
	fn expect_origin() {
		MockNet::reset();

		let message_fee = relay_chain::estimate_message_fee(6);

		ParaA::execute_with(|| {
			let message = Xcm(vec![
				WithdrawAsset((Here, AMOUNT + message_fee).into()),
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
				ClearOrigin,
				// Checks if the XcmContext origin is `Parachain(1).
				ExpectOrigin(Some(Parachain(1).into())),
			]);
			assert_ok!(ParachainPalletXcm::send_xcm(Here, Parent, message.clone(),));
		});

		let instruction_index_that_errored = 4;

		// Check that QueryResponse message with ExpectationFalse error was received.
		ParaA::execute_with(|| {
			assert_eq!(
				parachain::MsgQueue::received_dmp(),
				vec![Xcm(vec![QueryResponse {
					query_id: QUERY_ID,
					response: Response::ExecutionResult(Some((
						instruction_index_that_errored,
						XcmError::ExpectationFalse
					))),
					max_weight: Weight::from_all(0),
					querier: None,
				}])],
			);
		});
	}

	/// Scenario:
	/// Parachain wants to make sure that the relay chain has configured a specific pallet with a specific version.
	/// It sets the `ExpectPallet` instruction to check for the expected pallet.
	/// If the pallet is not as expected, the instruction errors, and the ErrorHandler reports back the corresponding error to the parachain.
	#[test]
	fn expect_pallet() {
		MockNet::reset();

		let message_fee = relay_chain::estimate_message_fee(5);

		ParaA::execute_with(|| {
			let message = Xcm(vec![
				WithdrawAsset((Here, message_fee).into()),
				BuyExecution {
					fees: (Here, message_fee).into(),
					weight_limit: WeightLimit::Unlimited,
				},
				// Set the instructions that are executed when ExpectPallet does not pass.
				// In this case, reporting back an error to the Parachain.
				SetErrorHandler(Xcm(vec![ReportError(QueryResponseInfo {
					destination: Parachain(1).into(),
					query_id: QUERY_ID,
					max_weight: Weight::from_all(0),
				})])),
				// Configured pallet has different `crate_major` so `VersionIncompatible` error is thrown.
				ExpectPallet {
					index: 1,
					name: "Balances".into(),
					module_name: "pallet_balances".into(),
					crate_major: 3,
					min_crate_minor: 0,
				},
				// Could execute pallet specific instructions after the expect pallet succeeds.
			]);
			assert_ok!(ParachainPalletXcm::send_xcm(Here, Parent, message.clone(),));
		});

		// Check that QueryResponse message with `VersionIncompatible` error was received.
		// Can also be a different error based on the pallet mismatch (i.e. PalletNotFound, NameMismatch).
		ParaA::execute_with(|| {
			assert_eq!(
				parachain::MsgQueue::received_dmp(),
				vec![Xcm(vec![QueryResponse {
					query_id: QUERY_ID,
					response: Response::ExecutionResult(Some((3, XcmError::VersionIncompatible))),
					max_weight: Weight::from_all(0),
					querier: Some(Here.into()),
				}])],
			);
		});
	}

	/// Scenario:
	/// Parachain wants to make sure that the `ErrorHandler` that it sets is only executed when a specific error is thrown.
	/// It sets an `ExpectError` instruction in the `SetErrorHandler` to check for the specific error.
	/// If a different Error is thrown, the `ErrorHandler` execution is halted.
	///
	/// Asserts that the ExpectPallet instruction throws an `PalletNotFound` error instead of the expected `VersionIncompatible` error.
	#[test]
	fn expect_error() {
		MockNet::reset();

		let message_fee = relay_chain::estimate_message_fee(6);

		ParaA::execute_with(|| {
			let message = Xcm(vec![
				WithdrawAsset((Here, message_fee).into()),
				BuyExecution {
					fees: (Here, message_fee).into(),
					weight_limit: WeightLimit::Unlimited,
				},
				// ReportError is only executed if the thrown error is the `VersionIncompatible` error.
				SetErrorHandler(Xcm(vec![
					ExpectError(Some((1, XcmError::VersionIncompatible))),
					ReportError(QueryResponseInfo {
						destination: Parachain(1).into(),
						query_id: QUERY_ID,
						max_weight: Weight::from_all(0),
					}),
				])),
				// Pallet index is wrong, so throws `PalletNotFound` error.
				ExpectPallet {
					index: 100,
					name: "Balances".into(),
					module_name: "pallet_balances".into(),
					crate_major: 4,
					min_crate_minor: 0,
				},
			]);
			assert_ok!(ParachainPalletXcm::send_xcm(Here, Parent, message.clone(),));
		});

		// Does not receive a message as the incorrect error was thrown during execution.
		ParaA::execute_with(|| {
			assert_eq!(parachain::MsgQueue::received_dmp(), vec![]);
		});
	}

	/// Scenario:
	/// Parachain wants to make sure that the `Transact` instruction succeeded as it does not throw an XcmError.
	/// It sets an `ExpectTransactStatus` instruction to MaybeErrorCode::Success to check if the transact succeeded.
	/// If the status was not succesful, the `ExpectTransactStatus` errors,
	/// and the ErrorHandler will report the error back to the Parachain.
	///
	/// Assert that `set_balance` execution fails as it requires the origin to be root,
	/// and the origin_kind is `SovereignAccount`.
	#[test]
	fn expect_transact_status() {
		MockNet::reset();

		// Runtime call dispatched by the Transact instruction.
		// set_balance requires root origin.
		let call = relay_chain::RuntimeCall::Balances(pallet_balances::Call::<
			relay_chain::Runtime,
		>::set_balance {
			who: ALICE,
			new_free: 100,
			new_reserved: 0,
		});

		let message_fee = relay_chain::estimate_message_fee(6);
		let set_balance_weight_estimation = Weight::from_parts(1_000_000_000, 10_000);
		let set_balance_fee_estimation =
			relay_chain::estimate_fee_for_weight(set_balance_weight_estimation);
		let fees = message_fee + set_balance_fee_estimation;

		let message = Xcm(vec![
			WithdrawAsset((Here, fees).into()),
			BuyExecution { fees: (Here, fees).into(), weight_limit: WeightLimit::Unlimited },
			SetErrorHandler(Xcm(vec![ReportTransactStatus(QueryResponseInfo {
				destination: Parachain(1).into(),
				query_id: QUERY_ID,
				max_weight: Weight::from_all(0),
			})])),
			Transact {
				origin_kind: OriginKind::SovereignAccount,
				require_weight_at_most: set_balance_weight_estimation,
				call: call.encode().into(),
			},
			ExpectTransactStatus(MaybeErrorCode::Success),
		]);

		ParaA::execute_with(|| {
			assert_ok!(ParachainPalletXcm::send_xcm(Here, Parent, message.clone()));
		});

		// The execution of set_balance does not succeed, and error is reported back to the parachain.
		ParaA::execute_with(|| {
			assert_eq!(
				parachain::MsgQueue::received_dmp(),
				vec![Xcm(vec![QueryResponse {
					query_id: QUERY_ID,
					response: Response::DispatchResult(MaybeErrorCode::Error(
						// The 2 is the scale encoded Error from the balances pallet
						BoundedVec::truncate_from(vec![2])
					)),
					max_weight: Weight::from_all(0),
					querier: Some(Here.into()),
				}])],
			);
		});
	}
}
