# xcm-examples
This repository contains the xcm examples for the xcm docs. 
The examples are set up using the [XCM-simulator](https://github.com/paritytech/polkadot/tree/master/xcm/xcm-simulator).
The testnet can be found in `examples/src/simple_test_net`.

#### How to run
To run the examples, do the following:
1. Clone the repository:
`git clone https://github.com/paritytech/xcm-docs.git`

2. cd to the examples folder:
`cd examples/`

3. Run all the tests: 
`cargo test`
or a single test:
`cargo test -p xcm-examples trap_and_claim_assets -- --nocapture`

#### events printing
You can print out the events on a parachain or the relay chain using the `print_para_events` or `print_relay_events` functions. The functions are used in a parachain or relay chain `TestExternalities`:

```rust
ParaA::execute_with(|| {
    print_para_events();
});
```

#### Tests
- `first_look`
- `transfers/teleport_fungible`
- `transfers/reserve_backed_transfer_para_to_para`
- `transfers/reserve_backed_transfer_relay_to_para`
- `transfers/reserve_backed_transfer_para_to_relay`
- `transact/transact_set_balance`
- `transact/transact_mint_nft`
- `origins/descend_origin`
- `holding_modifiers/burn_assets`
- `holding_modifiers/exchange_asset_maximal_true`
- `holding_modifiers/exchange_asset_maximal_false`
- `trap_and_claim/trap_and_claim_assets`
- `expects/expect_asset`
- `expects/expect_origin`
- `expects/expect_pallet`
- `expects/expect_error`
- `expects/expect_transact_status`
- `queries/query_holding`
- `queries/query_pallet`
- `queries/report_error`
- `queries/report_transact_status`
- `version_subscription/subscribe_and_unsubscribe_version`
- `locks/remote_locking_on_relay`
- `locks/locking_overlap`
