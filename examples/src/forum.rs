#[cfg(test)]
mod tests {
	use crate::simple_test_net::*;
	use frame_support::assert_ok;
	use xcm::v3::prelude::*;
	use xcm_simulator::TestExt;

	const BOB: sp_runtime::AccountId32 = sp_runtime::AccountId32::new([2u8; 32]);
	const QUERY_ID: u64 = 1234;

	/// Scenario:
	/// ALICE sends message from parachain A to parachain B
	/// Goal is to move both A's native asset and B's native asset to BOB
	fn forum() {
		MockNet::reset();

		let amount_a = 50 * CENTS;
		let amount_b = 50 * CENTS;

		let message: Xcm<()> = Xcm(vec![
			SetErrorHandler(Xcm(vec![ReportError(QueryResponseInfo {
				destination: ParentThen(X1(Parachain(1))).into(),
				query_id: QUERY_ID,
				max_weight: 0.into(),
			})])),
			ReserveAssetDeposited((ParentThen(X1(Parachain(1))), amount_a).into()),
			WithdrawAsset((Here, amount_b).into()),
			ClearOrigin,
			BuyExecution { fees: (Here, amount_b).into(), weight_limit: WeightLimit::Unlimited },
			DepositAsset {
				assets: AllCounted(2).into(),
				beneficiary: Junction::AccountId32 { network: None, id: BOB.clone().into() }.into(),
			},
		]);

		let destination: MultiLocation = (Parent, Parachain(2)).into();
		let alice = Junction::AccountId32 { id: ALICE.into(), network: None };

		ParaA::execute_with(|| {
			assert_ok!(parachain::PolkadotXcm::send_xcm(alice, destination, message));

			// ALICE on ParaA does not give up any balance
			assert_eq!(parachain::Balances::free_balance(ALICE), INITIAL_BALANCE);
		});

		ParaA::execute_with(|| {
			dbg!(&parachain::MsgQueue::received_dmp());
		});

		ParaB::execute_with(|| {
			dbg!(parachain::Balances::free_balance(sibling_account_sovereign_account_id(1, ALICE)));

			// ALICE does give up balance of ParaB's native asset...
			assert_eq!(
				parachain::Balances::free_balance(sibling_account_sovereign_account_id(1, ALICE)),
				INITIAL_BALANCE - amount_b
			);
			// ...and gives it to BOB
			assert_eq!(parachain::Balances::free_balance(BOB), amount_b);

			// ParaA's native asset is minted and deposited to BOB's account
			assert_eq!(parachain::Assets::balance(1, &BOB), amount_a);
		});
	}
}
