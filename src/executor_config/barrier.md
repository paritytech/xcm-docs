# Barrier
Before any XCMs are executed in the XCM executor, they must pass through the Barrier. The Barrier type implements the [`ShouldExecute`] trait and serves as the firewall for the xcm-executor. Each time the xcm-executor receives an XCM, it consults the Barrier to determine if the XCM should be executed.

Multiple barriers can be defined for the Barrier type using a tuple. During execution, each barrier is checked, and if any succeed, the XCM is executed. The combination of barriers is crucial as it can either block or allow specific XCM functionalities. An example of how barriers can be combined will be provided in the [example section](#example).

## Security
Barriers are vital for the security of the xcm-executor, acting as its firewall. Their main role is to decide whether a given XCM should be executed. Barriers can operate in various ways, such as checking if certain origins are authorized to execute an XCM, verifying if the XCM contains specific instructions, or a combination of both. Incorrectly configuring a barrier can lead to vulnerabilities, such as allowing attackers to flood the system with XCMs, potentially causing a Denial of Service.

## Implementations
The xcm-builder [`barriers`] file contains numerous implementations for the Barrier type. For more details, check out the linked documentation for each of these implementations.

- [`TakeWeightCredit`]
- [`AllowTopLevelPaidExecutionFrom`]
- [`AllowUnpaidExecutionFrom`]
- [`AllowExplicitUnpaidExecutionFrom`]
- [`AllowKnownQueryResponses`]
- [`AllowSubscriptionsFrom`]
- [`WithComputedOrigin`]
- [`TrailingSetTopicAsId`]
- [`RespectSuspension`]
- [`DenyThenTry`]
- [`DenyReserveTransferToRelayChain`]


### Example
The example below illustrates a comprehensive Barrier configuration, demonstrating how multiple barriers can be consecutively used or nested to fine-tune the execution of XCMs.

```rust
// Sets the message ID to `t` using a `SetTopic(t)` in the last position if present.
TrailingSetTopicAsId< 
	DenyThenTry<
        // Deny filter: 
        // Deny Reserve based transfers to the Relay chain. Allow everything else.
		DenyReserveTransferToRelayChain,
        // Allow filter:
		(
            // Allow local users to buy weight credit.
			TakeWeightCredit,
            // Evaluate XCMs with the inner barrier types using the newly computed origin.
            // XCMs without origin-altering instruction will be evaluated with the original origin.
			WithComputedOrigin<
				(
                    // If the message is one that immediately attemps to pay for execution, then
					// allow it.
					AllowTopLevelPaidExecutionFrom<Everything>,
                    // Parent and its pluralities (i.e. governance bodies) get free execution.
					AllowExplicitUnpaidExecutionFrom<ParentOrParentsPlurality>,
					// Subscriptions for version tracking are OK.
					AllowSubscriptionsFrom<Everything>,
				),
				UniversalLocation,
				ConstU32<8>,
			>,
		),
	>,
>;
```

[`ShouldExecute`]: https://paritytech.github.io/polkadot/doc/xcm_executor/traits/trait.ShouldExecute.html
[`barriers`]: https://github.com/paritytech/polkadot-sdk/blob/master/polkadot/xcm/xcm-builder/src/barriers.rs
[`WithComputedOrigin`]:https://paritytech.github.io/polkadot/doc/xcm_builder/struct.WithComputedOrigin.html
[`TakeWeightCredit`]:https://paritytech.github.io/polkadot/doc/xcm_builder/struct.TakeWeightCredit.html
[`AllowTopLevelPaidExecutionFrom`]:https://paritytech.github.io/polkadot/doc/xcm_builder/struct.AllowTopLevelPaidExecutionFrom.html
[`AllowUnpaidExecutionFrom`]:https://paritytech.github.io/polkadot/doc/xcm_builder/struct.AllowUnpaidExecutionFrom.html
[`AllowExplicitUnpaidExecutionFrom`]:https://paritytech.github.io/polkadot/doc/xcm_builder/struct.AllowExplicitUnpaidExecutionFrom.html
[`AllowKnownQueryResponses`]:https://paritytech.github.io/polkadot/doc/xcm_builder/struct.AllowKnownQueryResponses.html
[`AllowSubscriptionsFrom`]:https://paritytech.github.io/polkadot/doc/xcm_builder/struct.AllowSubscriptionsFrom.html
[`TrailingSetTopicAsId`]:https://paritytech.github.io/polkadot/doc/xcm_builder/struct.TrailingSetTopicAsId.html
[`RespectSuspension`]:https://paritytech.github.io/polkadot/doc/xcm_builder/struct.RespectSuspension.html
[`DenyThenTry`]:https://paritytech.github.io/polkadot/doc/xcm_builder/struct.DenyThenTry.html
[`DenyReserveTransferToRelayChain`]:https://paritytech.github.io/polkadot/doc/xcm_builder/struct.DenyReserveTransferToRelayChain.html




