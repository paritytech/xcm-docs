# Overview

XCM allows for different consensus systems to communicate with each other.
This allows things like:
- Sending tokens from one chain to another
- Locking assets on one chain in order to gain some benefit on a smart contract on another chain
- Calling functions (extrinsics) on another chain

But that's just the beginning.
The true power of XCM comes from its composability.
Once you can communicate with other consensus systems, you can get creative and implement whatever use case you need.
This is especially true in the context of an ecosystem of highly specialized chains, like Polkadot.

Decentralized distributed systems are very complex, so when building interactions between them, it's easy to make mistakes.
Because of that, the end-user is not expected to write custom XCMs from scratch for all the interactions they want to achieve.
Instead, builders will use XCM to create enticing products that provide a good and safe user experience.
This is usually done by carefully thinking and testing the interaction, then packaging it into your system's runtime logic (via an extrinsic or smart contract for example), and exposing that functionality to users.

In this chapter, we will cover what XCM is, what it isn't, why it matters, and delve into the different components that make up the XCM ecosystem.

Let's begin.
