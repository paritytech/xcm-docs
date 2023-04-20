# Teleporting

Asset teleportation is one of the ways we'll see for sending assets from one chain to another.
It's the simpler method of the two as it has only two actors, the source and the destination.

The process follows these steps:
- The source gathers the assets to be teleported from the sending account and *takes them out of the circulating supply*, taking note of the total amount of assets that were taken out.
- The destination grabs the teleported assets and *puts them into their circulating supply*, depositing them to the beneficiary account on the destination.

We'll go over the basic XCM instructions that achieve this, then some additional ones, and then see some examples.

## InitiateTeleport

This instruction takes the teleporting 

In Rust, we'd call it like so:

```rust,noplayground
InitiateTeleport
```

```rust,noplayground
InitiateTeleport()
ReceiveTeleportedAsset()
DepositAsset()
```
