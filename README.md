<div align="center">
<h1 align="center">Chocolate Contracts</h1>

This repository contains the [ink!](https://github.com/paritytech/ink) smart contracts for the [Chocolate](https://choc.network/) platform.

<br/>

ink! is an [eDSL](https://wiki.haskell.org/Embedded_domain_specific_language) to write smart contracts in Rust for blockchains built on the [Substrate](https://github.com/paritytech/substrate) framework. ink! contracts are compiled to [WebAssembly](https://github.com/WebAssembly).

</div>

## Updateing Rust Environenment

To work wiht this contract, we need to add some `Rust` source code to our Substrate development environment.

To update your development environment:

1. Open a terminal shell on your computer.
2. Update your Rust environment by running the following command:

    ```bash
    rustup component add rust-src
    ```

3. Verify that you have the WebAssembly target installed by running the following command:

   ```bash
   rustup target add wasm32-unknown-unknown --toolchain nightly
   ```

    If the target is installed and up-to-date, the command displays output similar to the following:

    ```bash
    info: component 'rust-std' for target 'wasm32-unknown-unknown' is up to date
    ```

## Install `cargo-contract` CLI Tool

`cargo-contract` is a command-line tool which you will use to build, deploy, and interact with your ink! contracts.

Note that in addition to Rust, installing `cargo-contract` requires a C++ compiler that supports C++17.

Modern releases of `gcc`, `clang`, as well as Visual Studio 2019+ should work.

1. Add the `rust-src` compiler component:

    ```bash
    rustup component add rust-src
    ```

2. Install the latest version of `cargo-contract`:

    ```bash
    cargo install --force --locked cargo-contract --version 2.0.0-rc
    ```

3. Verify the installation and explore the commands available by running the following command:

    ```bash
    cargo contract --help
    ```

## Install the Substrate Contract Node

You can download the `Chocolate Contract Node` from the [Chocolate Github repository](https://github.com/chocolatenetwork/contracts-node). The Chocolate Contract Node is a Substrate node with the `pallet-contracts` module enabled.

If you can't download the precompiled node, you can compile it locally with a command similar to the following:

```bash
cargo install contracts-node --git https://github.com/chocolatenetwork/contracts-node.git --force --locked
```

You can verify the installation buy running: `substrate-contracts-node --version`.

## Test the contract

At the bottom of the `lib.rs` source code file, there are simple test cases to verify the functionality of the contract. These are annotated using the `#[ink(test)]` macro. You can test whether this code is functioning as expected using the offchain test environment.

To test the contract:

1. Open a terminal shell on your computer, if needed.
2. Verify that you are in the `chocolate-contract` folder.
3. Use the `test` subcommand to execute the test for the `chocolate` contract by running the following command:

```bash
cargo contract test
```

The command should compile the contract and run the tests. If the tests pass, you will see the output similar to the following to indicate successful test completion:

```bash
running 2 tests
test chocolate::tests::it_works ... ok
test chocolate::tests::default_works ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

## Build the Contract

After testing the contract, you are ready to compile this project to WebAssembly.

To build the WebAssembly for this smart contract:

1. Open a terminal shell on your computer, if needed.
2. Verify that you are in the `chocolate-contract` project folder.
3. Compile the `chocolate` smart contract by running the following command:

    ```bash
    cargo contract build
    ```

    This command builds a WebAssembly binary for the `chocolate-contract` project, a metadata file that contains the contract Application Binary Interface (ABI), and a .contract file that you use to deploy the contract.

    For example, you should see output similar to the following:

    ```bash
    Original wasm size: 35.5K, Optimized: 11.9K

    The contract was built in DEBUG mode.

    Your contract artifacts are ready. You can find them in:
    /Users/dev-doc/chocolate-contract/target/ink

   - chocolate.contract (code + metadata)
   - chocolate.wasm (the contract's code)
   - metadata.json (the contract's metadata)
   ```

    The `.contract` file includes both the business logic and metadata. This is the file that tooling (e.g UIs) expect when you want to deploy your contract on-chain.

    The `metadata.json` file describes all the interfaces that you can use to interact with this contract. This file contains several important sections:

    - The `spec` section includes information about the functions—like constructors and messages—that can be called, the events that are emitted, and any documentation that can be displayed. This section also includes a selector field that contains a 4-byte hash of the function name and is used to route contract calls to the correct functions.
    - The `storage` section defines all the storage items managed by the contract and how to access them.
    - The `types` section provides the custom data types used by the contract.

## Start the Substrate Contracts Node

`TODO`

## Deploy the contract

`TODO`