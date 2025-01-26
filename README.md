<h1 align="center">
  <code>adapter-program</code>
</h1>
<p align="center">
  <img width="400" alt="adapter-program" src="https://github.com/user-attachments/assets/c462d64f-2c92-41d7-b574-1967df69c40e" />
</p>
<p align="center">
  Interact with <code>pinocchio</code> libraries from a program written with <code>solana-program</code>.
</p>

## Overview

The `adapter-program` provides an example of how a program written with `solana-program` can take advantage of optimized `pinocchio` crates to perform cross-program invications (CPIs).

One of the main differences when using `pinocchio` is the `AccountInfo` type. In `pinocchio`, the `AccountInfo` is a zero-copy type and it has a layout that matches directly the runtime input layout. As a consequence, it is not directly interchangeable with the `AccountInfo` from `solana-program`. 

The [`pinocchio-adapter`](https://github.com/anza-xyz/pinocchio/tree/febo/adapter/sdk/adapter) library provides an "adapter" type, allowing a lightweight creation of a `pinocchio`'s `AccountInfo` from a `solana-program`'s `AccountInfo`. This enables using `pinocchio` libraries directly from a program written with `solana-program`.

## But why?

`pinocchio` crates, in particular the CPI clients, are written to minimizing memory allocations and copies &mdash; in most cases, they are `no_std` crates without any allocation. This makes them consume significantly less compute units and, therefore, they are more efficient to be used in an on-chain program.

## Benchmark

To illustrate the difference in CU consumption, `adapter-program` includes 2 instructions: one using the `solana-program` types to perform a CPI into System program to create a new account; and a second one doing the same thing, but using `pinocchio`'s System program CPI client.

* `CreateAccountV1`: use `solana-program` types.
* `CreateAccountV2`: use `pinocchio` types.

| Instruction       | CUs  | Difference |
|-------------------|------|------------|
| create_account_v1 | 2866 | +921       |
| create_account_v2 | 1945 |            |

## Running the benchmark

To run the benchmarks, you will need to first build the program. From the root of the repository, run:

```bash
cargo build-sbf
```

After this, you are ready to run individual benchmarks by using:

```bash
cargo bench --bench <BENCH_NAME>
```

Currently there is one available bench, `create_account`.

The results of an execution are stored under `target/benches` and they are compared to the previous one (if there is one), with delta differences shown after a `+` and `-` symbol.
