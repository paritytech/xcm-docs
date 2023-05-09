# Weight and fees

The resources available to a blockchain are limited, so it's important to manage how operations on-chain use them.
Not managing how resources are used can open an attack vector, known as DoS (Denial of Service), where an attacker floods the chain with operations in order to get it to stop producing blocks.
In order to manage how resources are used and to protect against DoS attacks, XCM uses a concept of *weight*.
This concept, that aims to quantify usage of blockchain resources, comes from the [Substrate](https://substrate.io/) world.

Weight is two-dimensional, it tracks both time (execution time) and space (state accesses).
Weight determines how much fees need to be paid in order to perform some operation.
The good thing about having weight as an intermediate concept is that the logic for turning it into fees is configurable.

Some systems have the concept of *gas metering*, which is calculated during execution and only measures execution time.
Weight, however, is static, defined beforehand, which lets XCM execution lighter by not including gas metering.

The main two stages of XCM where fees could need to be paid are *sending* the message and actually *executing* it.
The fees for sending need to be paid on the local system, usually by the origin of the message.
The fees for execution, however, need to be paid in the destination system, via the `BuyExecution` instruction.
We'll talk more about `BuyExecution` in the [fee handling chapter](TODO:add_link).

XCM is agnostic, which means it doesn't assume fees need to be paid.
It's entirely possible to not pay for the effects of an XCM on the destination system.
Even in systems where fees have to be paid, special cases of free execution can be made.
There are security measures systems can put in place (see [barrier](TODO:add_link)) to not execute XCMs that do not pay for their fees.

## Executor config

The executor has a `Weigher` [configuration item](TODO:add_link) that specifies the weight of each instruction.
It then weighs the whole message by adding the weight of each instruction.
There are two basic approaches for weighing, either you set a common weight for all instructions, or you benchmark each instruction and assign them their custom values.

Another configuration item, `Trader`, turns weight into assets.
There are two basic approaches, one is to just set a value in assets to each unit of weight, the other is to reuse some existing transaction payment method for XCM weight.

## XCM pallet

FRAME pallets, like the XCM pallet, specify weights for each extrinsic they expose.
That means that when interacting with pallets that deal with XCM, there will be an additional fee at the beginning for calling the extrinsic locally.
