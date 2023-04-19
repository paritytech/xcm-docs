# The XCVM

At the core of XCM lies the XCVM (Cross-Consensus Virtual Machine).
A message in XCM (referred to as an XCM, cross-consensus message, or XCMs for more than one) is an XCVM program.
The XCVM is a register-based state machine that executes every program by processing its instructions one at a time.
During execution, state is tracked in domain-specific registers, and is constantly being used and updated.
Most of the XCM format comprises these registers and the instructions used to compose XCVM programs.

Like XCM, the XCVM is also a specification.
The first implementation is [xcm-executor](https://github.com/paritytech/polkadot/tree/master/xcm/xcm-executor), provided by Parity.
It's built to be highly configurable, with its building blocks available in [xcm-builder](https://github.com/paritytech/polkadot/tree/master/xcm/xcm-builder).
Configuring the executor is an important and extensive topic, one we will dive into further in [Executor Deep Dive](TODO:link).
It's entirely possible to create another implementation of the XCVM if desired.
