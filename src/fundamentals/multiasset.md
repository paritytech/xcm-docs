# MultiAsset
When working in XCM it’s often needed to refer to an asset of some sort. This is because practically all public blockchains in existence rely on some native digital asset to provide the backbone for its internal economy and security mechanism. For example, the native asset for the Polkadot relay chain is DOT. 

Some blockchains manage multiple assets, e.g. Ethereum’s ERC-20 framework allows for many different assets to be managed on-chain. Some manage assets that are not fungible such as Ethereum’s ETH but rather are non-fungible — one-of-a-kind instances; Crypto-kitties was an early example of such non-fungible tokens or NFTs.

XCM is designed to be able to handle all such assets without breaking a sweat. For this purpose, there is the datatype `MultiAsset` together with its associated types `MultiAssets`, `WildMultiAsset`, and `MultiAssetFilter`.

## MultiAsset Breakdown
Let's take a look at the MultiAsset struct: 
```rust,noplayground
pub struct MultiAsset {
    pub id: AssetId,
    pub fun: Fungibility,
}
```

So two fields define our asset: id and fun, this is pretty indicative of how XCM approaches assets. Firstly, an overall asset identity must be provided. For fungible assets, this simply identifies the asset. For NFTs this identifies the overall asset “class” — different asset instances may be within this class.

```rust,noplayground
enum AssetId {
   Concrete(MultiLocation),
   Abstract([u8; 32]),
}
``` 

The asset identity is expressed in one of two ways; either Concrete or Abstract. Abstract is not really in use, but it allows asset IDs to be specified by name. This is convenient but relies on the receiver interpreting the name in the way that the sender expects which may not always be so easy. Concrete is in general usage and uses a `MultiLocation` to identify an asset unambiguously. For native assets (such as DOT), the asset tends to be identified as the chain which mints the asset (the Polkadot Relay Chain in this case, which would be the location `..` from one of its parachains). Assets that are primarily administered within a chain’s pallet may be identified by a location including their index within that pallet. For some examples of `MultiAsset`s see the example section below.

```rust,noplayground
enum Fungibility {
   // Fungible cannot be 0 
   Fungible(u128),
   NonFungible(AssetInstance),
}
```
Secondly, they must be either fungible or non-fungible. If they’re fungible, then there should be some associated non-zero amount. If they’re not fungible, then instead of an amount, there should be some indication of which [AssetInstance](https://paritytech.github.io/polkadot/doc/xcm/v3/enum.AssetInstance.html) they are. (This is commonly expressed with an index, but XCM also allows arrays.)


## How to use Multiple Assets Together?
There are multiple ways to group Assets. In this section, we go over these methods.

### MultiAssets
One way to group a set of `MultiAsset` items is the [MultiAssets](https://paritytech.github.io/polkadot/doc/xcm/v3/struct.MultiAssets.html) type. It is a `Vec` of `MultiAsset` items.

```rust,noplayground
struct MultiAssets(Vec<MultiAsset>);
```

This structure must uphold some rules:
- It may contain no items of duplicate asset class;
- All items must be ordered;
- The number of items should grow no larger than MAX_ITEMS_IN_MULTIASSETS (currently set to 20).



### WildMultiAsset
Then we have WildMultiAsset; this is a wildcard that can be used to match against one or more MultiAsset items. 
All the WildMultiAsset wildcards describe the assets in the [Holding register](../overview/architecture.md).

```rust,noplayground
pub enum WildMultiAsset {
    /// All assets in Holding.
    All,
    /// All assets in Holding of a given fungibility and ID.
    AllOf { id: AssetId, fun: WildFungibility },
    /// All assets in Holding, up to `u32` individual assets (different instances of non-fungibles
    /// are separate assets).
    AllCounted(#[codec(compact)] u32),
    /// All assets in Holding of a given fungibility and ID up to `count` individual assets
    /// (different instances of non-fungibles are separate assets).
    AllOfCounted {
        id: AssetId,
        fun: WildFungibility,
        #[codec(compact)]
        count: u32,
    },
}
```

### MultiAssetFilter
Finally, there is `MultiAssetFilter`. This is used most often and is just a combination of MultiAssets and WildMultiAsset allowing either a wildcard or a list of definite (i.e. not wildcard) assets to be specified.

```rust,noplayground
pub enum MultiAssetFilter {
    /// Specify the filter as being everything contained by the given `MultiAssets` inner.
    Definite(MultiAssets),
    /// Specify the filter as the given `WildMultiAsset` wildcard.
    Wild(WildMultiAsset),
}
```

## Examples
### MultiAsset
For more information about the MultiLocations used to define concrete assets, see [MultiLocation](multilocation/README.md) and [Junction](multilocation/junction.md).
```rust,noplayground
// Location Relay Chain
// 100 Native Asset (three ways)
MultiAsset {id: Concrete(MultiLocation {parents: 0, interior: Here}), fun: Fungible(100u128)};
MultiAsset {id: Here.into(), fun: 100.into()};
(Here, 100).into();

// 100 Parachain's Native Asset 
(X1(Parachain(1000)), 100).into();
// 100 Fungible assets in Parachain 1000 with id 1234 
(X2(Parachain(1000), GeneralIndex(1234)), 100).into();
// Non Fungible asset with asset class 1234 containing only one nft instance in Parachain 1000
(X2(Parachain(1000), GeneralIndex(1234)), Undefined).into();
// Non Fungible asset with asset class 1234 and AssetInstance 1 in Parachain 1000
(X2(Parachain(1000), GeneralIndex(1234)), Index(1)).into();
```

### MultiAssetFilter

```rust,noplayground
let a1: MultiAssets = MultiAssets::from(vec![MultiAsset {id: Here.into(), fun: 100.into()}]);
let b1: MultiAssets = (Here, 100).into();
assert_eq!(a1, b1);

let a2: MultiAssetFilter = a1.into();
let b2 = MultiAssetFilter::Definite((Here, 100).into());
assert_eq!(a2, b2);

let a3 = MultiAssetFilter::Wild(WildMultiAsset::All);
let b3: MultiAssetFilter = All.into();
assert_eq!(a3, b3);
```

