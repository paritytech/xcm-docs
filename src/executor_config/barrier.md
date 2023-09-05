-TODO: All Barriers Explained
-TODO: Security Note about misconfigured barriers
-TODO: Example

# Barrier
Before any XCMs are executed in the XCM executor, they must pass the Barrier.
The Barrier type implements the `ShouldExecute`
[trait](https://paritytech.github.io/polkadot/doc/xcm_executor/traits/trait.ShouldExecute.html)
and can be viewed as the firewall for the xcm-executor. 
Each time the xcm-executor receives an XCM, it checks with the barrier to determine if the XCM should be executed.

We can also define multiple barriers for our Barrier type using a tuple.
During execution, each barrier is checked, and if any of them succeed, the XCM is executed.
The combination of barriers is especially important as it can either block or allow certain XCM
functionalities. We will show an example of how barriers can be combined in the [example section]().

## Implementations
The [xcm-builder
directory](https://github.com/paritytech/polkadot-sdk/blob/master/polkadot/xcm/xcm-builder/src/barriers.rs)
has many implementations for the Barrier type:
- `TakeWeightCredit`
- `AllowTopLevelPaidExecutionFrom`
- `AllowUnpaidExecutionFrom`
- `AllowExplicitUnpaidExecutionFrom`
- `IsChildSystemParachain`
- `AllowKnownQueryResponses`
- `AllowSubscriptionsFrom`
- `WithComputedOrigin`
- `TrailingSetTopicAsId`
- `RespectSuspension`
- `DenyThenTry`
- `DenyReserveTransferToRelayChain`


### `TakeWeightCredit`
The `TakeWeightCredit`` barrier checks if the calculated weight of the XCM does not exceed a set weight limit. 
This Barrier is particularly useful to allow XCM execution by local chain users via extrinsics.
For example, in the `execute` function in `pallet-xcm`, users can set a max_weight that
specifies the maximum weight that the message may consume. The `TakeWeightCredit` Barrier
blocks the message if the specified `max_weight` is to low.

### `AllowTopLevelPaidExecutionFrom`
The `AllowTopLevelPaidExecutionFrom<T>` barrier accepts the execution of an XCM if the origin
of the XCM is contained in `T` and the XCM pays for execution. 
To ensure that the XCM is paid for, it should start with an instruction (`ReceiveTeleportedAsset`,
`ReserveAssetDeposited`, `WithdrawAsset`, `ClaimAsset`) that places assets in the
Holding Register to pay for execution followed by the `BuyExecution` instruction. 

The most commenly used configuration of this barrier:
```rust
// Accept all XCMs that pay for execution.
AllowTopLevelPaidExecutionFrom<Everything>
```

### `AllowUnpaidExecutionFrom`
The `AllowUnpaidExecutionFrom<T>` barrier accepts the execution of an XCM if the origin
of the XCM is contained in `T`. This barrier allows for free execution for specific origins.
SECURITY NOTE: Configure this barrier only for completely trusted origins, from which no
permissionless messages can be sent. 

```rust
// Parent and its pluralities (i.e. governance bodies) get free execution.
// For example the Kusama relay chain gets free execution on AssetHub.
AllowUnpaidExecutionFrom<ParentOrParentsPlurality>
```

### `AllowExplicitUnpaidExecutionFrom`
The `AllowExplicitUnpaidExecutionFrom` barrier is almost identical to the
`AllowUnpaidExecutionFrom`. The only difference is that it checks if the XCM begins with the
`UnpaidExecution` instruction with a sufficient `weight_limit`. 
This barrier is preferred over the `AllowUnpaidExecutionFrom` as the origin has to explicitly
specify that it expects the execution to be free. This prevents accidental free execution.

```rust
// Parent and its pluralities (i.e. governance bodies) get free execution.
// For example the Kusama relay chain gets free execution on AssetHub.
AllowUnpaidExecutionFrom<ParentOrParentsPlurality>
```


### `IsChildSystemParachain`
### `AllowKnownQueryResponses`
### `AllowSubscriptionsFrom`
### `WithComputedOrigin`
### `TrailingSetTopicAsId`
### `RespectSuspension`
### `DenyThenTry`
### `DenyReserveTransferToRelayChain`

