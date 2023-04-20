# First Look
In this section, we take you through a simple example of an XCM. In this example, we withdraw the native token from the account of Alice and deposit this token in the account of Bob. This message simulates a transfer between two accounts in the same consensus system (`ParaA`). Find here the [code example]().
## Message
```rust
 let message = Xcm(vec![
    WithdrawAsset((Here, amount).into()),
    BuyExecution{fees: (Here, amount).into(), weight_limit: WeightLimit::Unlimited},
    DepositAsset { 
        assets: All.into(), 
        beneficiary:  MultiLocation { 
            parents: 0,
            interior: Junction::AccountId32 { 
                network: None, 
                id: BOB.clone().into() 
            }.into(),
        }.into()
    }   
]);
```
The message consists of three instructions: WithdrawAsset, BuyExecution, and DepositAsset. In the following sections we will go over each of these instructions. 

### WithdrawAsset
```rust
WithdrawAsset((Here, amount).into())
```

The first instruction takes as an input the [MultiAsset]() that should be withdrawn. The MultiAsset describes the native parachain token with the `Here` keyword. The `amount` parameter is the number of tokens that are transferred. The withdrawal account depends on the Origin of the message. In this example the Origin of the message is Alice.
The WithdrawAsset instruction moves `amount` number of native tokens from Alice's account into the `Holding register`. 

### BuyExecution
```rust
BuyExecution{fees: (Here, amount).into(), weight_limit: WeightLimit::Unlimited}
```
To execute XCM instructions, weight (some kind of resources) has to be bought. The amount of weight depends on the number and type of instructions in the XCM. The `BuyExecution` instruction pays for the weight using the `fees`. The `fees` parameter describes the asset in the `Holding register` that should be used for paying for the weight. The `weight_limit` defines the maximum amount of fees that can be used for buying weight. There are special occasions where it is not necessary to buy weight. See [fees]() for more information about the fees in XCM.

### DepositAsset
```rust
DepositAsset { 
    assets: All.into(), 
    beneficiary:  MultiLocation { 
        parents: 0,
        interior: Junction::AccountId32 { 
            network: None, 
            id: BOB.clone().into() 
        }.into(),
    }.into()
}
```
The DepositAsset instruction is used to deposit funds from the holding register into the account of the `beneficiary`. We don’t actually know how much is remaining in the Holding Register after the BuyExecution instruction, but that doesn’t matter since we specify a wildcard for the asset(s) which should be deposited. In this case, the wildcard is `All`, meaning that all assets in the Holding register should be deposited. The `beneficiary` in this case is the account of Bob in the current consensus system. 

When the three instructions are combined, we withdraw `amount` native tokens from the account of Alice, pay for the execution of the instructions, and deposit the remaining tokens in the account of Bob. 


## What next?
Now that we have taken a first look at an XCM, we can dive deeper into all the XCM instructions. 
For an overview of the instructions check out the [xcm-format](https://github.com/paritytech/xcm-format#5-the-xcvm-instruction-set).
Or check out examples for each of the instruction in [A Journey through XCM]().
To get a better understanding about MultiLocations, MultiAssets, and other fundamental concepts in XCM, check out the [fundamentals chapter](fundamentals/README.md). 
