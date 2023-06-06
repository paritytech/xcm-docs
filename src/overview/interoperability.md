# Introduction

XCM is a messaging format, a language, designed to enable seamless communication between different consensus systems. Examples of consensus systems are blockchains and smart contracts.
XCM is originally developed for the [Polkadot](https://polkadot.network/) ecosystem, but is designed to be general enough to provide a common language for cross-consensus communication that can be used anywhere.

XCM is a language in which interactions (programs) can be written.
It aims to provide better interoperability between consensus systems, both more features and a better user and developer experience.

Its goal is to let blockchain ecosystems thrive via specialization instead of generalization.
If there's no interoperability, a chain is forced to host all services and support all functionalities on its own.
With XCM, we are able to achieve an ecosystem-wide division of labour: a chain can specialize and focus on its own business logic, and leverage the benefits of depending on other specialized blockchain for services that it does not provide.

XCM has four high-level inherent design assumptions:
1. Asynchronous: XCMs in no way assume that the sender will be blocking on its completion
2. Absolute: XCMs are guaranteed to be delivered and interpreted accurately, in order and in a timely fashion. Once a message is sent, one can be sure it will be processed as it was intended to be.
3. Asymmetric: XCMs, by default, do not have results that let the sender know that the message was received - they follow the 'fire and forget' paradigm. Any results must be separately communicated to the sender with an additional message back to the origin.
4. Agnostic: XCM makes no assumptions about the nature of the consensus systems between which the messages are being passed. XCM as a message format should be usable in any system that derives finality through consensus.

XCM is a work-in-progress; the format is expected to change over time.
It has an RFC process to propose changes, which end up in newer versions, the current one being v3.
To keep up with the development of the format, or to propose changes, go to [the XCM format repository](https://github.com/paritytech/xcm-format).
