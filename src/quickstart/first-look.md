## First Look
In this section, we take you through a simple example of an XCM. In this example, we withdraw the native token from the account of Alice and deposit this token in the account of Bob. This message simulates a transfer between two accounts in the same consensus system (`SimpleParachain`). Find here the [code example]().

### Message
The message consists of three instructions: WithdrawAsset, BuyExecution, and DepositAsset. 

##### WithdrawAsset
```rust
WithdrawAsset((Here, amount).into())
```

The first instruction takes as an input the [MultiAsset]() that should be withdrawn. The MultiAsset describes the native parachain token with the `Here` keyword. The `amount` parameter is the number of tokens that are transferred. The withdrawal account depends on the Origin of the message. In this example the Origin of the message is Alice.
The WithdrawAsset instruction moves `amount` number of native tokens from Alice's account into the `Holding register`. 

##### BuyExecution
```rust
BuyExecution{fees: (Here, amount).into(), weight_limit: WeightLimit::Unlimited}
```
To execute XCM instructions, execution time (weight) has to be bought. The amount of execution time depends on the number and type of instructions in the XCM. The `BuyExecution` instruction pays for the weight using the `fees`. The `fees` parameter describes the asset in the `Holding register` that should be used for paying for the weight. The `weight_limit` defines the maximum amount of fees that can be used for buying execution time. See [fees]() for more information about the fee payments.

##### DepositAsset
```rust
DepositAsset { 
    assets: All.into(), 
    beneficiary:  MultiLocation { 
        parents: 0,
        interior: X1(Junction::AccountId32 { 
            network: Some(NetworkId::Kusama), 
            id: BOB.clone().into() 
        }),
    }.into()
}
```
The DepositAsset instruction is used to deposit funds from the holding register into the account of the `beneficiary`. We don’t actually know how much is remaining in the Holding Register after the BuyExecution instruction, but that doesn’t matter since we specify a wildcard for the asset(s) which should be deposited. In this case, the wildcard is `All`, meaning that all assets in the Holding register should be deposited. The `beneficiary` in this case is the account of Bob in the current consensus system. 

When the three instructions are combined, we withdraw `amount` native tokens from the account of Alice, pay for the execution of the instructions, and deposit the remaining tokens in the account of Bob. 