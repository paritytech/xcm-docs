# Transfers

The first feature you'll be interested in when dealing with XCM is being able to transfer assets between consensus systems.
In the [Quickstart](../../overview/README.md) section, we saw a simple XCM that when executed, would send assets between two accounts on the same consensus system.
This can, of course, be done locally as well. The beauty with XCM is we can send it to another system for them to execute.
The end result is what's called a "remote transfer", making a transfer between two accounts on another system.

Now that we've learnt the [fundamentals](../../fundamentals/README.md), let's go over those same instructions.

## WithdrawAsset

```rust,noplayground
enum Instruction {
  ...
  WithdrawAsset(MultiAssets),
  ...
}
```

This instruction is the most common way to get assets to the holding register of the XCVM.
The `MultiAssets` in the operand will be stored in the holding register to be later used for other instructions.
As we've seen, we can use the expression `(Here, amount).into()` to take a certain `amount` of the native token.

## BuyExecution

```rust,noplayground
enum Instruction {
  ...
  BuyExecution { fees: MultiAssets, weight_limit: WeightLimit },
  ...
}
```

Because XCM is designed to be agnostic to the underlying consensus system, it doesn't have fee payment baked in.
This instruction lets you pay for the execution of the XCM using the assets in the holding register.
Most XCMs are not allowed to be executed (blocked by the [barrier](TODO:link)) if they don't contain this instruction as one of the first ones to pay for all future ones.

## DepositAsset

```rust,noplayground
enum Instruction {
  ...
  DepositAsset { assets: MultiAssetFilter, beneficiary: MultiLocation },
  ...
}
```

This instruction will put assets from the holding register that match the [MultiAssetFilter](../../fundamentals/multiasset.md) into the `beneficiary`.

## Examples

```rust,noplayground
let message = Xcm(vec![
  WithdrawAsset(),
  BuyExecution {  },
  DepositAsset {  },
]);
```

## Transferring between systems

But what if you want to make a transfer from one system to another?
There are two ways of transfering assets between systems:
- Teleporting
- Reserve-backed transfers

We'll be discussing them in the following chapters.
