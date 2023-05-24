# Architecture

XCM is a [format](https://github.com/paritytech/xcm-format), which means anyone is free to create an implementation for it.
The first one is made in [Rust](https://www.rust-lang.org/), primarily for [Substrate](https://substrate.io/)-based chains in the [Polkadot](https://polkadot.network/) ecosystem.
We'll be looking at this first implementation to tinker with different types of messages in the next sections.
For now, we'll take a look at how it's structured.

All the code lives in the [Polkadot repo](https://github.com/paritytech/polkadot/tree/master/xcm).
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
Many of these building blocks will be explained in the [Config Deep Dive](../executor_config/index.md) chapter.
They cover common use-cases but are not meant to be exhaustive.
It's very easy to build your own building blocks for your specific configuration when needed, using these as examples.

## Pallet

The XCM pallet is a [FRAME](https://docs.substrate.io/quick-start/substrate-at-a-glance/) pallet that can be used to execute XCMs locally or send them to a different system.
It also has extrinsics for specific use cases such as teleporting assets or doing reserve asset transfers, which we'll talk about later.
It's the glue between XCM and FRAME, which is highly used in the Polkadot ecosystem.

## Simulator 

The simulator allows for testing XCMs fast, without needing to boot up several different nodes in a network, or test in production.
It's a very useful tool which we'll use throughout this document to build and test different XCMs.
