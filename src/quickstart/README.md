# Quickstart

The XCM code can be found in [polkadot repository](https://github.com/paritytech/polkadot/tree/master/xcm).

### Rust & Cargo
A pre-requisite for using XCM is to have a stable Rust version and Cargo installed. Here's an [installation guide](https://docs.substrate.io/install/) on how to install rust and cargo.

### Running the Examples

All examples in the documentation are located in the [examples repository](). Follow the following steps to run the `first-look` example. First clone the repository:

```shell
git clone git@github.com:vstam1/xcm-examples.git
cd xcm-examples
```

To run the first-look example, run the following line:

```shell
cargo test -p xcm-examples parachain_a_simple_transfer -- --nocapture
```

It should show you the following output: 

```shell
running 1 test
test first_look::tests::parachain_a_simple_transfer ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.01s
```

