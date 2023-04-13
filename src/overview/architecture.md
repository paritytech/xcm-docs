# Architecture

XCM is a [specification](https://github.com/paritytech/xcm-format), which means anyone is free to create an implementation for it.
The first one is made in [Rust](https://www.rust-lang.org/), primarily for [substrate](https://substrate.io/)-based chains in the [Polkadot](https://polkadot.network/) ecosystem.
We'll be looking at this first implementation to tinker with different types of messages in the next sections.
For now, we'll take a look at how it's structured.

All the code lives in its own [folder](https://github.com/paritytech/polkadot/tree/master/xcm) in the Polkadot repository.
The main structure is as follows:
- XCM: Defines the fundamental constructs used in XCM and an enum with all the instructions available.
- Executor: Implements the XCVM, capable of executing XCMs. Highly configurable.
- Builder: Offers common configuration building blocks for the executor.
- Pallet: FRAME pallet that provides extrinsics with specific XCM programs.
- Simulator: Allows for testing of XCM programs.

## Executor

The XCM executor is responsible for interpreting and executing XCM messages.
It is the core engine that processes and handles XCM instructions, ensuring that they are carried out accurately and in the correct order.
The XCM executor follows the Cross-Consensus Virtual Machine (XCVM) specification and can be extended, customized, or even replaced with an alternative construct that adheres to the XCVM spec.

## Builder

The XCM executor is highly configurable.
XCM builder provides building blocks people can use to configure their executor according to their needs.

## Pallet

The XCM pallet is a FRAME pallet that can be used to execute XCMs (Cross-Consensus Messages) or send them.
It also has extrinsics for specific use cases such as teleporting assets or doing reserve asset transfers, which we'll talk about later.

## Simulator 

The simulator allows for testing XCM programs needing to do it in production.
It's a very useful tool which we'll use later to build and test different XCMs.
