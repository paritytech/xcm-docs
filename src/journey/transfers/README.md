# Transfers

The first feature you'll be interested in when dealing with XCM is being able to transfer assets between consensus systems.
In the [quickstart](../../quickstart/index.md) chapter, we saw a simple XCM that when executed, would send assets between two accounts on the same consensus system.
Now that we've learnt the [fundamentals](../../fundamentals/index.md), let's go over those same instructions once again.

## WithdrawAsset

```rust,noplayground
WithdrawAsset(MultiAssets),
```

This instruction is the most common way to get assets to the holding register of the XCVM.
The `MultiAssets` in the operand will be stored in the holding register to be later used for other instructions.
As we've seen, we can use the expression `(Here, amount).into()` to take a certain `amount` of the native token.

## BuyExecution

```rust,noplayground
BuyExecution { fees: MultiAssets, weight_limit: WeightLimit },
```

Because XCM is designed to be agnostic to the underlying consensus system, it doesn't have fee payment baked in.
This instruction lets you pay for the execution of the XCM using the assets in the holding register.
Most XCMs are not allowed to be executed (blocked by the [barrier](../../executor_config/index.md#barrier)) if they don't contain this instruction as one of the first ones to pay for all future ones.

## DepositAsset

```rust,noplayground
DepositAsset { assets: MultiAssetFilter, beneficiary: MultiLocation },
```

This instruction will put assets from the holding register that match the [MultiAssetFilter](../../fundamentals/multiasset.md#multiassetfilter) into the `beneficiary`.
Note that `beneficiary` must be a location where the local consensus system can actually deposit assets to, e.g. it doesn't make sense to deposit assets to `../AccountId32(0x0)`.

## Example

```rust,noplayground
let message = Xcm(vec![
  WithdrawAsset((Here, amount).into()),
  BuyExecution { fees: (Here, amount).into(), weight_limit: Unlimited },
  DepositAsset { 
    assets: All.into(),
    beneficiary: AccountId32 { id: ALICE.into(), network: None }.into()
  },
]);
```

As we've seen, the above message results in withdrawing assets from the origin of the message, paying for execution and depositing the rest to another account on the same system.
The full example can be seen in [the repo](https://github.com/paritytech/xcm-docs/tree/main/examples).

## Transferring between systems

But what if you want to make a transfer from one system to another?
There are two ways of doing this:
- Asset teleportation
- Reserve-backed transfers

We'll be discussing both in the following chapters.
