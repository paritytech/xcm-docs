#[cfg(test)]
mod tests {
	use crate::simple_test_net::*;
	use bounded_collections::BoundedVec;
	use codec::Encode;
	use frame_support::{assert_ok, pallet_prelude::Weight};
	use xcm::latest::prelude::*;
	use xcm_simulator::TestExt;

	const AMOUNT: u128 = 1 * CENTS;
	/// Arbitrary query id
	const QUERY_ID: u64 = 1234;

	/// Scenario:
	/// A parachain wants to be notified that a transfer worked correctly.
	/// It sends a `ReportHolding` after the deposit to get notified on success.
	///
	/// Asserts that the balances are updated correctly and the expected XCM is sent.
	#[test]
	fn query_holding() {
		MockNet::reset();

		// Send a message which fully succeeds to the relay chain.
		// And then report the status of the holding register back to ParaA
		ParaA::execute_with(|| {
			let message = Xcm(vec![
				WithdrawAsset((Here, AMOUNT).into()),
				BuyExecution { fees: (Here, AMOUNT).into(), weight_limit: Unlimited },
				DepositAsset {
					assets: Definite((Here, AMOUNT - 5).into()),
					beneficiary: Parachain(2).into(),
				},
				ReportHolding {
					response_info: QueryResponseInfo {
						destination: Parachain(1).into(),
						query_id: QUERY_ID,
						max_weight: Weight::from_all(0),
					},
					assets: All.into(),
				},
			]);

			assert_ok!(ParachainPalletXcm::send_xcm(Here, Parent, message.clone(),));
		});

		// Check that transfer was executed
		Relay::execute_with(|| {
			// Withdraw executed
			assert_eq!(
				relay_chain::Balances::free_balance(parachain_sovereign_account_id(1)),
				INITIAL_BALANCE - AMOUNT
			);
			// Deposit executed
			assert_eq!(
				relay_chain::Balances::free_balance(parachain_sovereign_account_id(2)),
				INITIAL_BALANCE + AMOUNT - 5
			);
		});

		// Check that QueryResponse message was received
		ParaA::execute_with(|| {
			assert_eq!(
				parachain::MsgQueue::received_dmp(),
				vec![Xcm(vec![QueryResponse {
					query_id: QUERY_ID,
					response: Response::Assets((Parent, AMOUNT - (AMOUNT - 5)).into()),
					max_weight: Weight::from_all(0),
					querier: Some(Here.into()),
				}])],
			);
		});
	}

	/// Scenario:
	/// Parachain A wants to query the `PalletInfo` of the balances pallet in the relay chain.
	/// It sends a `QueryPallet` instruction to the relay chain.
	/// The relay chain responds with a `QueryResponse` instruction containing the `PalletInfo`.
	///
	/// Asserts that the relay chain has the balances pallet configured.
	#[test]
	fn query_pallet() {
		MockNet::reset();

		ParaA::execute_with(|| {
			let message = Xcm(vec![QueryPallet {
				module_name: "pallet_balances".into(),
				response_info: QueryResponseInfo {
					destination: Parachain(1).into(),
					query_id: QUERY_ID,
					max_weight: Weight::from_all(0),
				},
			}]);

			assert_ok!(ParachainPalletXcm::send_xcm(Here, Parent, message.clone(),));
			print_para_events();
		});

		ParaA::execute_with(|| {
			assert_eq!(
				parachain::MsgQueue::received_dmp(),
				vec![Xcm(vec![QueryResponse {
					query_id: QUERY_ID,
					response: Response::PalletsInfo(BoundedVec::truncate_from(vec![
						PalletInfo::new(1, "Balances".into(), "pallet_balances".into(), 4, 0, 0)
							.unwrap()
					])),
					max_weight: Weight::from_all(0),
					querier: Some(Here.into()),
				}])],
			);
		})
	}

	/// Scenario:
	/// Parachain A wants to know if the execution of their message on the relay chain succeeded without errors.
	/// They set the ErrorHandler to report the value of the error register.
	/// If the execution of the xcm instructions errors on the relay chain, the error is reported back to the Parachain.
	///
	/// The Relay chain errors on the Trap instruction (Trap always throws an error).
	#[test]
	fn report_error() {
		MockNet::reset();

		let message = Xcm(vec![
			// Set the Error Handler to report back status of Error register.
			SetErrorHandler(Xcm(vec![ReportError(QueryResponseInfo {
				destination: Parachain(1).into(),
				query_id: QUERY_ID,
				max_weight: Weight::from_all(0),
			})])),
			Trap(1u64),
		]);

		ParaA::execute_with(|| {
			assert_ok!(ParachainPalletXcm::send_xcm(Here, Parent, message.clone()));
		});

		ParaA::execute_with(|| {
			assert_eq!(
				parachain::MsgQueue::received_dmp(),
				vec![Xcm(vec![QueryResponse {
					query_id: QUERY_ID,
					response: Response::ExecutionResult(Some((1, XcmError::Trap(1)))),
					max_weight: Weight::from_all(0),
					querier: Some(Here.into()),
				}])],
			);
		});
	}

	/// Scenario:
	/// Parachain A wants to know if the execution of their `Transact` instruction on the relay chain succeeded without errors.
	/// They add the `ReportTransactStatus` instruction to the XCM to get the status of the transact status register reported back.
	#[test]
	fn report_transact_status() {
		MockNet::reset();

		// Runtime call dispatched by the Transact instruction
		let call = relay_chain::RuntimeCall::System(
			frame_system::Call::<relay_chain::Runtime>::remark_with_event {
				remark: "Hallo Relay!".as_bytes().to_vec(),
			},
		);

		let message = Xcm(vec![
			Transact {
				origin_kind: OriginKind::SovereignAccount,
				require_weight_at_most: Weight::from_parts(INITIAL_BALANCE as u64, 1024 * 1024),
				call: call.encode().into(),
			},
			ReportTransactStatus(QueryResponseInfo {
				destination: Parachain(1).into(),
				query_id: QUERY_ID,
				max_weight: Weight::from_all(0),
			}),
		]);

		ParaA::execute_with(|| {
			assert_ok!(ParachainPalletXcm::send_xcm(Here, Parent, message.clone(),));
		});

		ParaA::execute_with(|| {
			assert_eq!(
				parachain::MsgQueue::received_dmp(),
				vec![Xcm(vec![QueryResponse {
					query_id: QUERY_ID,
					response: Response::DispatchResult(MaybeErrorCode::Success),
					max_weight: Weight::from_all(0),
					querier: Some(Here.into()),
				}])],
			);
		});
	}
}
