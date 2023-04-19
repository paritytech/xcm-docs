# Junction(s)
In the section on [MultiLocations](README.md), we looked at the MultiLocation struct. We talked about the Multilocation being a way to describe moving from one place in the consensus hierarchy to another. The `parents` parameter expresses the number of steps up in the hierarchy. In this section, we dive further into the MultiLocation struct and explain how we can use the Junctions type to describe steps in the consensus hierarchy.
Take a look at the MultiLocation struct again: 

```rust,noplayground
pub struct MultiLocation {
    pub parents: u8,
    pub interior: Junctions,
}
```

The consensus hierarchy consists of 1-to-n relations. Each place in the consensus hierarchy has one parent, so there is only one way up the hierarchy. That is why we can use a `u8` to describe the number of `parents` we want to move up. But moving down is a bit more difficult, as one consensus system can encapsulate multiple other consensus systems(e.g. a relaychain can have multiple parachains). So to describe the correct steps down the hierarchy, we use the `Junctions` [type](https://paritytech.github.io/polkadot/doc/xcm/v3/enum.Junctions.html).


## Junctions Type
```rust,noplayground
pub enum Junctions {
    /// The interpreting consensus system.
    Here,
    /// A relative path comprising 1 junction.
    X1(Junction),
    ...
    /// A relative path comprising 8 junctions.
    X8(Junction, Junction, Junction, Junction, Junction, Junction, Junction, Junction),
}
```
The `Junctions` enum can represent zero to eight steps down the hierarchy. When the `Here` junction is used, it means that we do not have to take steps down the hierarchy. We can for example describe the current location with `{parents: 0, interior: Here}` or the Parent location with `{parents: 1, interior: Here}`. If we want to take steps down the hierarchy, we express each step as a Junction. 

## Junction Type
A [Junction](https://paritytech.github.io/polkadot/doc/xcm/v3/enum.Junction.html) describes a step down in the Hierarchy. The `Junction`s are defined as follows: 

```rust,noplayground
pub enum Junction {
    Parachain(u32),
    AccountId32 {
        network: Option<NetworkId>,
        id: [u8; 32],
    },
    AccountIndex64 {
        network: Option<NetworkId>,
        index: u64,
    },
    AccountKey20 {
        network: Option<NetworkId>,
        key: [u8; 20],
    },
    PalletInstance(u8),
    GeneralIndex(u128),
    GeneralKey {
        length: u8,
        data: [u8; 32],
    },
    OnlyChild,
    Plurality {
        id: BodyId,
        part: BodyPart,
    },
    GlobalConsensus(NetworkId),
}
```

#### Parachain
The `Parachain` junction is used to describe a parachain from the point of a relaychain. Each parachain has an Id, e.g. statemine in the Kusama network has Id 1000.

#### PalletInstance
The `PalletInstance` junction is used to describe a pallet in one of the parachains or relaychain. Each pallet has an Id that can be used for the `PalletInstance` 

#### AccountId32 and AccountKey20
Each of these junctions can be used to describe an account located in the current consensus system. The `AccountId32` is used to describe substrate-based accounts, while the `AccountKey20` is mainly used to describe Ethereum or Bitcoin-based accounts or smart contracts. Both junctions express an account based on the context they are used in. If the current location is the Relaychain, then the junctions describe an account in the relaychain. The same is true for each parachain location.

#### GeneralIndex and GeneralKey
Non-descript indices and keys within the current context location. The usage will vary widely owing to its generality. An example use case for the `GeneralIndex` is to describe an Asset within an Assets Parachain.

NOTE: Try to avoid using this and instead use a more specific item.

#### AccountIndex64
The `AccountIndex64` can be used to describe an account index for the Indices Pallet. 

#### OnlyChild
The `OnlyChild` junction can be used to describe the child of a location if there exists a 1-to-1 relation between the parent and child in the consensus hierarchy. The `OnlyChild` junction is currently not used except as a fallback when deriving context.

#### Plurality 
The `Plurality` junction is used to describe a pluralistic body existing within the current consensus location.
Typical to be used to represent a governance origin of a chain, but could in principle be used to represent
things such as multisigs also. See the [BodyId documentation](https://paritytech.github.io/polkadot/doc/xcm/v3/enum.BodyId.html) for a better understanding of the bodies that the `Plurality` junction can represent. 

#### GlobalConsensus
A global network (e.g. Polkadot or Kusama) is capable of externalizing its own consensus. This is not generally meaningful outside of the universal level. An example would be describing the Kusama relaychain from the perspective of the Polkadot relaychain as `{parents: 1, interior: GlobalConsensus(Kusama)}`. An example use case could be routing XCM messages between global consensus networks using bridges. 

## Multiple ways to create a MultiLocation
```rust,noplayground
// Current Location
MultiLocation {parents: 0, interior: Here};
MultiLocation::new(0, Here);
MultiLocation::here();
MultiLocation::default();
Here.into();

// Parent Location
MultiLocation {parents: 1, interior: Here};
MultiLocation::parent();
Parent.into();

// Conversion
MultiLocation { parents: 2, interior: X2(Parachain(1), GeneralIndex(1))};
(Parent, Parent, Parachain(1), GeneralIndex(1)).into();
```# MultiLocation
The [MultiLocation](https://paritytech.github.io/polkadot/doc/xcm/v3/struct.MultiLocation.html) type identifies any single location that exists within the world of consensus. It is quite an abstract idea and can represent all manner of things that exist within consensus, from a scalable multi-shard blockchain such as Polkadot down to a lowly ERC-20 asset account on a parachain. MultiLocations are used to identify places to send XCM messages, places that can receive assets, and then can even help describe the type of an asset itself, as we will see in [MultiAsset](../multiasset.md).

### Location is relative
MultiLocation always expresses a location relative to the current location. You can think of it a bit like a file system path but where there is no way of directly expressing the “root” of the file system tree. This is for a simple reason: In the world of Polkadot, blockchains can be merged into, and split from other blockchains. A blockchain can begin life very much alone, and eventually be elevated to become a parachain within a larger consensus. If it did that, then the meaning of “root” would change overnight and this could spell chaos for XCM messages and anything else using MultiLocation. To keep things simple, we exclude this possibility altogether.

### Hierarchical structure
Locations in XCM are hierarchical; some places in consensus are wholly encapsulated within other places in consensus. A parachain of Polkadot exists wholly within the overall Polkadot consensus and we call it an interior location. Or a pallet exists wholly within a parachain or relaychain. Putting it more strictly, we can say that whenever there is a consensus system any change in which implies a change in another consensus system, then the former system is interior to the latter.

### So what is a MultiLocation: Simple example
A quick summary of the previous points:
- A MultiLocation identifies any single location that exists within the world of consensus.
- A MultiLocation is always relative to the current location.
- MultiLocations in XCM are hierarchical. 

Now take a look at the MultiLocation struct: 
```rust,noplayground
pub struct MultiLocation {
    pub parents: u8,
    pub interior: Junctions,
}
```
As we have already discussed, locations in XCM are hierarchical. The following image shows an example of such a Hierarchy.

![Simple Example](./../images/MultiLocation_simple_example.png)

Relaychain A completely encapsulates Parachain A and B (indicated by the arrows) and parachain A encapsulates an account `0x00...`. So RelayA is higher in the hierarchy than ParaA and ParaB and can be described as the `parent` of these parachains. So the `parents: u8` in the MultiLocation struct describes the number of steps in the hierarchy we want to move up. The `interior: Junctions` express the steps in the hierarchy we want to move down. The `Junctions` type will be further discussed in the next chapter about [Junctions](junction.md), but for now, it's just a way to express a way down the hierarchy. As all MultiLocations are relative to the current location, Parachain B relative to Parachain A is one step up and one step down in the hierarchy. 

To get a better understanding of this concept, we show some simple MultiLocations in the code example below. The first two examples are relative to RelayA and the second set of examples is relative to ParaB. In the `Location` comments, we expressed the locations in text. The `..` express a step up in the hierarchical structure (the “parent” or the encapsulating consensus system). The `..` are followed by some number of [Junctions](junction.md), all separated by `/`.

```rust,noplayground
// From: RelayA
// To: ParaB
// Location: /Parachain(2000)
MultiLocation {parents: 0, interior: X1(Parachain(2000))};
// To: Account in ParaA
// Location: /Parachain(1000)/AccountId32(0x00..)
MultiLocation {
    parents: 0, 
    interior: X2(
        Parachain(1000), 
        AccountId32{network: None, id: [0u8; 32]}
    )
};

// From: ParaB
// To: RelayA
// Location: ../here
MultiLocation {parents: 1, interior: Here};
// To: Account in ParaA
// Location: ../Parachain(1000)/AccountId32(0x00..)
MultiLocation {
    parents: 1, 
    interior: X2(
        Parachain(1000), 
        AccountId32{network: None, id: [0u8; 32]}
    )
};
```

## What's next:
- More information about [junctions](junction.md)
- More MultiLocation [examples](example.md)
- Expressing assets using Multilocations: [MultiAsset][../multiasset.md]


