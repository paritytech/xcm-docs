# The Promise of Interoperability

XCM is a messaging format, a language, designed to enable seamless communication between different consensus systems, for example blockchains and smart contracts.
XCM was originally developed for the [Polkadot](https://polkadot.network/) ecosystem, but was designed to provide a common language for cross-consensus communication that can be used anywhere.

XCM is a language in which interactions (programs) can be written.
It aims to provide better interoperability between consensus systems, both more features and a better user and developer experience.

XCM has four high-level core design principles which it stands to follow:
1. Asynchronous: XCM messages in no way assume that the sender will be blocking on its completion
2. Absolute: XCM messages are guaranteed to be delivered and interpreted accurately, in order and in a timely fashion. Once a message is sent, one can be sure it will be processed as it was intended to be.
3. Asymmetric: XCM messages, by default, do not have results that let the sender know that the message was received - they follow the 'fire and forget' paradigm. Any results must be separately communicated to the sender with an additional message back to the origin.
4. Agnostic: XCM makes no assumptions about the nature of the consensus systems between which the messages are being passed. XCM as a message format should be usable in any system that derives finality through consensus.

XCM is a work-in-progress, the format is expected to change over time.
It has an RFC process to propose changes, which end up in newer versions, the current one being v3.
To keep up with the development of the format, or to propose changes, go to [the XCM format repository](https://github.com/paritytech/xcm-format).
