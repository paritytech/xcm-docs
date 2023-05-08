# Version Subscription
XCM is a versioned messaging format. One version may contain more or different instruction then another, so for parties to communicate via XCM it is important to know what version the other party is using. XCM enables a version subscription model, where parties can subscribe to each other for version information. XCM has two instructions to enable this:
- `SubscribeVersion`
- `UnsubscribeVersion`

The version subscription model can differ per XCVM implementation. The `xcm-executor` has a `SubscriptionService` [config item](../executor_config/README.md). When the `SubscribeVersion` instruction is send to a consensus system that implements the `xcm-executor` with the `xcm-pallet` as implementation for the `SubscriptionService`, the system will send back its currently `AdvertisedVersion` and will keep the subscribed location up to date when the version changes. The subscribed location can stop the subscription by sending the `UnsubscribeVersion` instruction.

```rust,noplayground
SubscribeVersion {
    #[codec(compact)]
    query_id: QueryId,
    max_response_weight: Weight,
}

UnsubscribeVersion
```

Check out the [example](TODO).


