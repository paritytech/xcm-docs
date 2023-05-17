# Holding Register Modifiers
Most of the XCM instructions alter the Holding Register. We already have seen instructions that alter the Holding Register, like the `WithdrawAsset` or `DepositAsset` instructions. In this chapter we go over more instructions that alter the holding register, namely:

- BurnAsset
- ExchangeAsset

## BurnAsset
```rust,noplayground
BurnAsset(MultiAssets)
```
The `BurnAsset` instruction allows for the reduction of assets in the Holding Register by up to the specified assets. The execution of the instruction does not throw an error if the Holding Register does not contain the assets (to make this an error, use `ExpectAsset` prior).

### Example
For the full example, check [here](TODO).
The Scenario of the example is as follows:
Parachain A withdraws 10 units from its sovereign account on the relay chain and burns 4 of them.
The relay chain then reports back the status of the Holding Register to Parachain A. We expect the Holding Register to hold 6 units. 
Note: If we would have added more then 10 units worth of assets in the `BurnAsset` instruction, we would have burned all assets in the Holding Register and the execution would succeed.
```rust,noplayground
let message = Xcm(vec![
    WithdrawAsset((Here, 10 * CENTS).into()),
    BuyExecution { fees: (Here, CENTS).into(), weight_limit: WeightLimit::Unlimited },
    BurnAsset((Here, 4 * CENTS).into()),
    ReportHolding { 
        response_info: QueryResponseInfo { 
            destination: Parachain(1).into(), 
            query_id: QUERY_ID, 
            max_weight: Weight::from_parts(1_000_000_000, 64*64) },
        assets: All.into()
    }
]);
```

We expect the following response:
```rust,noplayground
Response::Assets((Parent, 6 * CENTS).into())
```


## ExchangeAsset
