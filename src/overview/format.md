# A Format, Not a Protocol

It's essential to understand that XCM is a format, not a protocol.
It describes how messages should be structured and contains instructions that convey on-chain actions that the message intends to perform.
However, XCM does not dictate how messages are delivered.
That responsibility falls on [transport layer protocols](../transport_protocols/index.md) such as XCMP (Cross Chain Message Passing) and VMP (Vertical Message Passing) in the Polkadot ecosystem, or any others to come, such as bridging protocols.

This separation of concerns is useful, since it allows us to think of the interactions we want to build between systems without having to think about how the messages involved are actually routed.

XCM is similar to how RESTful services use REST as an architectural style of development, where HTTP requests contain specific parameters to perform some action.
Similar to UDP, out of the box XCM is a "fire and forget" model, unless there is a separate XCM designed to be a response message which can be sent from the recipient to the sender. All error handling should also be done on the recipient side.

XCM is not designed in a way where every system supporting the format is expected to be able to interpret any possible XCM. Practically speaking, one can imagine that some messages will not have reasonable interpretations under some systems or will be intentionally unsupported.

Furthermore, it's essential to realize that XCMs by themselves are not considered on-chain transactions: XCM describes how to change the state of the target consensus system, but the message by itself does not perform state changes. In short, XCM is a declarative language; the actual interpretation and behaviour of each instruction in an XCM is defined by target's XCVM implementation.

XCM is a language in which rich interactions between systems can be written.
Both simple and more complex scenarios can be expressed, and developers are encouraged to design and implement diverse cross-consensus communication solutions.
