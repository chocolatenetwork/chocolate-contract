<div align="center">
<h1 align="center">Chocolate Contracts</h1>

This repository contains the [ink!](https://github.com/paritytech/ink) smart contracts for the [Chocolate](https://choc.network/) platform.

<br/>

ink! is an [eDSL](https://wiki.haskell.org/Embedded_domain_specific_language) to write smart contracts in Rust for blockchains built on the [Substrate](https://github.com/paritytech/substrate) framework. ink! contracts are compiled to [WebAssembly](https://github.com/WebAssembly).

</div>

## Updateing Rust Environenment

To work wiht this contract, we need to add some `Rust` source code to our Substrate development environment.

To update the development environment:

1. Open a terminal shell on your computer.
2. Update local Rust environment by running the following command:

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

`cargo-contract` is a command-line tool which you will use to build, deploy, and interact with the ink! contracts.

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

<!-------------------------------- TODO ---------------------------------->
<!-- These results bellow will be changed based on final test in Chocolate Contracts, after the contract is done -->
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
running 3 tests
test chocolate::tests::default_works ... ok
test chocolate::tests::it_works ... ok
test chocolate::tests::it_works_review ... ok

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

    The `.contract` file includes both the business logic and metadata. This is the file that tooling (e.g UIs) expect when you want to deploy the contract on-chain.

    The `metadata.json` file describes all the interfaces that you can use to interact with this contract. This file contains several important sections:

    - The `spec` section includes information about the functions—like constructors and messages—that can be called, the events that are emitted, and any documentation that can be displayed. This section also includes a selector field that contains a 4-byte hash of the function name and is used to route contract calls to the correct functions.
    - The `storage` section defines all the storage items managed by the contract and how to access them.
    - The `types` section provides the custom data types used by the contract.

## Start the Substrate Contracts Node

If you have successfully installed the [substrate-contracts-node](##install-the-substrate-contract-node), it's time to start the local node.

1. Start the contracts node in local development mode by running the following command:

    ```bash
    substrate-contracts-node --log info,runtime::contracts=debug 2>&1
    ```

    The extra logging is useful for development.

    You should see output in the terminal similar to the following:

    ```bash
    2023-01-30 23:08:49.835  INFO main sc_cli::runner: Substrate Contracts Node
    2023-01-30 23:08:49.836  INFO main sc_cli::runner: ✌️  version 0.23.0-87a3d76c880
    2023-01-30 23:08:49.836  INFO main sc_cli::runner: ❤️  by Parity Technologies <admin@parity.io>, 2021-2023
    2023-01-30 23:08:49.836  INFO main sc_cli::runner: 📋 Chain specification: Development
    2023-01-30 23:08:49.836  INFO main sc_cli::runner: 🏷  Node name: profuse-grandmother-6287
    2023-01-30 23:08:49.836  INFO main sc_cli::runner: 👤 Role: AUTHORITY
    2023-01-30 23:08:49.836  INFO main sc_cli::runner: 💾 Database: ParityDb at /tmp/substrateCu3FVo/chains/dev/paritydb/full
    2023-01-30 23:08:49.836  INFO main sc_cli::runner: ⛓  Native runtime: substrate-contracts-node-100 (substrate-contracts-node-1.tx1.au1)
    2023-01-30 23:08:54.570  INFO main sc_service::client::client: 🔨 Initializing Genesis block/state (state: 0x27d2…a1d8, header-hash: 0x6a05…1669)
    2023-01-30 23:08:54.573  INFO main sub-libp2p: 🏷  Local node identity is: 12D3KooWG4h1FpwAhybzyMxoEGQgY8SbrLb4F5FB6mCBZCY6u7W1
    2023-01-30 23:08:58.643  INFO main sc_service::builder: 📦 Highest known block at #0
    2023-01-30 23:08:58.643  INFO tokio-runtime-worker substrate_prometheus_endpoint: 〽️ Prometheus exporter started at 127.0.0.1:9615
    2023-01-30 23:08:58.644  INFO                 main sc_rpc_server: Running JSON-RPC HTTP server: addr=127.0.0.1:9933, allowed origins=None
    2023-01-30 23:08:58.644  INFO                 main sc_rpc_server: Running JSON-RPC WS server: addr=127.0.0.1:9944, allowed origins=None
    2023-01-30 23:09:03.645  INFO tokio-runtime-worker substrate: 💤 Idle (0 peers), best: #0 (0x6a05…1669), finalized #0 (0x6a05…1669), ⬇ 0 ⬆ 0
    2023-01-30 23:09:08.646  INFO tokio-runtime-worker substrate: 💤 Idle (0 peers), best: #0 (0x6a05…1669), finalized #0 (0x6a05…1669), ⬇ 0 ⬆ 0
    ```

    Note that no blocks will be produced unless we send an extrinsic to the node. This is because the `substrate-contracts-node` uses `Manual Seal` as its consensus engine.

## Deploy the contract

At this point, you have completed the following steps:

- Installed the packages for local development.
- Generated the WebAssembly binary for the `chocolate` smart contract.
- Started the local node in development mode.
  
The next step is to deploy the `chocolate` contract on the Substrate chain.

However, deploying a smart contract on Substrate is a little different than deploying on traditional smart contract platforms.

For most smart contract platforms, you must deploy a completely new blob of the smart contract source code each time you make a change.

For example, the standard ERC20 token has been deployed to Ethereum thousands of times.

Even if a change is minimal or only affects some initial configuration setting, each change requires a full redeployment of the code.

Each smart contract instance consumes blockchain resources equivalent to the full contract source code, even if no code was actually changed.

In Substrate, the contract deployment process is split into two steps:

- Upload the contract code to the blockchain.
- Create an instance of the contract.

With this pattern, you can store the code for a smart contract like the ERC20 standard on the blockchain once, then instantiate it any number of times.

You don't need to reload the same source code repeatedly, so the smart contract doesn't consume unnecessary resources on the blockchain.

### Uploading the ink! Contract Node

We will use the `cargo-contract` CLI tool to `upload` and `instantiate` the ***chocolate*** contract on a Substrate chain.

1. Start your node using `substrate-contracts-node --log info,runtime::contracts=debug 2>&1`
2. Go to the `chocolate-contract` project folder.
3. Build the contract using `cargo contract build`.
4. Upload and instantiate your contract using:

    ```bash
    cargo contract instantiate --constructor new --args "false" --suri //Alice --salt $(date +%s)
    ```

    Some notes about the command:

    - The `instantiate` command will do both the `upload` and `instantiate` steps for you.
    - We need to specify the contract constructor to use, which in this case is `new()`
    - We need to specify the argument to the constructor, which in this case is `false`
    - We need to specify the account uploading and instantiating the contract, which in this case is the default development account of `//Alice`
    - During development we may want to upload the instantiate the same contract multiple times, so we specify a `salt` using the current time. Note that this is optional.
  
    After running the command confirming that we're happy with the gas estimatation we should see something like this:

    ```bash
        Dry-running new (skip with --skip-dry-run)
            Success! Gas required estimated at Weight(ref_time: 328660939, proof_size: 0)
        Confirm transaction details: (skip with --skip-confirm)
        Constructor new
            Args false
        Gas limit Weight(ref_time: 328660939, proof_size: 0)
        Submit? (Y/n):
            Events
            Event Balances ➜ Withdraw
                who: 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
                amount: 98.986123μUNIT
            Event System ➜ NewAccount
                account: 5GRAVvuSXx8pCpRUDHzK6S1r2FjadahRQ6NEgAVooQ2bB8r5
            ... snip ...
            Event TransactionPayment ➜ TransactionFeePaid
                who: 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
                actual_fee: 98.986123μUNIT
                tip: 0UNIT
            Event System ➜ ExtrinsicSuccess
                dispatch_info: DispatchInfo { weight: Weight { ref_time: 2827629132, proof_size: 0 }, class: Normal, pays_fee: Yes }

        Contract 5GRAVvuSXx8pCpRUDHzK6S1r2FjadahRQ6NEgAVooQ2bB8r5
    ```

    We will need the Contract `address` to `call` the contract, so make sure you ***don't lose*** it.

### Calling the Deployed ink! Contract

We can not only `upload` and `instantiate` contracts using cargo-contract, we can also `call` them!

<!-------------------------------- TODO ---------------------------------->
<!-- These methods bellow will be changed to fit the functions in Chocolate Contracts, after the contract is done -->

### **get() Message**

When we initialized the contract we set the initial value of the `chocolate` to false. We can confirm this by calling the `get()` message.

Since we are only reading from the blockchain state (we're not writing any new data) we can use the `--dry-run` flag to avoid submitting an extrinsic.

```bash
cargo contract call --contract 5GRAVvuSXx8pCpRUDHzK6S1r2FjadahRQ6NEgAVooQ2bB8r5 --message get --suri //Alice --dry-run
```

Some notes about the command:

- The address of the contract we want to call had to be specified using the `--contract` flag
- This can be found in the output logs of the `cargo contract instantiate` command
- We need to specify the contract message to use, which in this case is `get()`
- We need to specify the account callling the contract, which in this case is the default development account of `//Alice`
- We specify `--dry-run` to avoid submitting an extrinsic on-chain

After running the command should see something like this:

```markdown
Result Success!
Reverted false
    Data Tuple(Tuple { ident: Some("Ok"), values: [Bool(false)] })
```

We're interested in the `value` here, which is `false` as expected.

### **flip() Message**

The `flip()` message changes the storage value from `false` to `true` and vice versa.

To call the `flip()` message we will need to submit an extrinsic on-chain because we are altering the state of the blockchain.

To do this we can use the following command:

```bash
cargo contract call --contract 5GQwxP5VTVHwJaRpoQsK5Fzs5cERYBzYhgik8SX7VAnvvbZS --message flip --suri //Alice
```

Notice that we changed the message to `flip` and removed the `--dry-run` flag.

After running we expect to see something like:

```markdown
Dry-running flip (skip with --skip-dry-run)
    Success! Gas required estimated at Weight(ref_time: 8013742080, proof_size: 262144)
Confirm transaction details: (skip with --skip-confirm)
     Message flip
        Args
   Gas limit Weight(ref_time: 8013742080, proof_size: 262144)
Submit? (Y/n):
      Events
       Event Balances ➜ Withdraw
         who: 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
         amount: 98.974156μUNIT
       Event Contracts ➜ Called
         caller: 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
         contract: 5GQwxP5VTVHwJaRpoQsK5Fzs5cERYBzYhgik8SX7VAnvvbZS
       Event TransactionPayment ➜ TransactionFeePaid
         who: 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
         actual_fee: 98.974156μUNIT
         tip: 0UNIT
       Event System ➜ ExtrinsicSuccess
         dispatch_info: DispatchInfo { weight: Weight { ref_time: 1410915697, proof_size: 13868 }, class: Normal, pays_fee: Yes }
```

If we call the get() message again we can see that the storage value was indeed flipped!

```markdown
Result Success!
Reverted false
    Data Tuple(Tuple { ident: Some("Ok"), values: [Bool(true)] })
```