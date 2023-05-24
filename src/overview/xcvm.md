# The XCVM

At the core of XCM lies the XCVM (Cross-Consensus Virtual Machine).
A message in XCM (referred to as an XCM, cross-consensus message, or XCMs for more than one) is an XCVM program.
The XCVM is a register-based state machine that executes every program by processing its instructions one at a time.
During execution, state is tracked in domain-specific registers, and is constantly being used and updated.
Most of the XCM format comprises these registers and the instructions used to compose XCVM programs.

Like XCM, the XCVM is also a specification.
The implementation that will be used in this documentation is the [xcm-executor](https://github.com/paritytech/polkadot/tree/master/xcm/xcm-executor), built in Rust, provided by Parity.
It's built to be highly configurable, with its building blocks available in [xcm-builder](https://github.com/paritytech/polkadot/tree/master/xcm/xcm-builder).
Configuring the executor is an important and extensive topic, one we will dive into further in the [Config Deep Dive](../executor_config/index.md) chapter.

Anyone is free to make an implementation of the XCVM.
As long as they follow the standard, they'll be able to send XCMs to systems using other implementations.
Implementations in different programming languages will need to be used to bring XCM to other ecosystems.

Typically, an XCM takes the following path through the XCVM:
- Instructions within an XCM are read one-by-one by the XCVM. An XCM may contain one or more instructions.
- The instruction is executed. This means that the current values of the XCVM registers, the instruction type, and the instruction operands are all used to execute some operation, which might result in some registers changing their value, or in an error being thrown, which would halt execution.
- Each subsequent instruction within the XCM is read until the end of the message has been reached.

The XCVM register you will hear most about is the `holding` register.
An XCVM program that handles assets (which means most of them) will be putting them in and taking them out of this register.
Instructions we'll see later like `DepositAsset`, `WithdrawAsset` and many more, make use of this register.
You can see all registers in the [All XCVM Registers](TODO:link) section.
