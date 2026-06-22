# soroban-batcher

Soroban utility contracts for batching multiple contract calls into a single transaction and for validating contract-call results.

## Overview

These are small, stateless Soroban (Rust) helper contracts used by the Aquarius AMM operations tooling to reduce transaction overhead. The **batcher** composes several sub-calls into one transaction (for example, applying many AMM plane updates at once); the **guard** calls a contract and asserts that the result matches an expected value. Neither contract holds storage or emits events — they are pure input/output utilities whose only effects come from the sub-calls they invoke.

## Contracts

The workspace (`Cargo.toml`) has two members:

- **batcher** (`batcher/`) — `batch(auth_users, batch, return_result)` executes a list of `(contract, fn_name, args)` calls sequentially, optionally collecting and returning their results.
- **guard** (`guard/`) — `assert_result(auth_users, contract, fn_name, args, expected_result)` calls a contract and recursively validates the returned value against `expected_result`. Supported value types include Bool, integers, Vec, Map, Bytes, String, Symbol, and Address. Error codes: 101 `ResultsMismatch`, 102 `UnsupportedType`, 103 `InvalidValue`, 104 `TypesMismatch`.

Each member has `src/lib.rs`, `src/test.rs`, and recorded `test_snapshots/`. Prebuilt artifacts (`soroban_batcher.wasm`, `soroban_guard.wasm`, and a liquidity-pool plane contract used in tests) are vendored in `contracts/`.

## Build

Toolchain is pinned by `rust-toolchain.toml` (Rust 1.87.0, target `wasm32v1-none`). Tasks are wrapped with `task` (see `Taskfile.yaml`):

```sh
task build      # soroban contract build + optimize both WASMs into contracts/
task test       # cargo test
task check      # cargo check --all
task all        # clean + fmt + build + test
```

The release profile is tuned aggressively for WASM size (`opt-level = "z"`, LTO, `strip = "symbols"`, single codegen unit, `panic = "abort"`); the batcher compiles to a few hundred bytes.

## Stack

- Rust (edition 2021), `soroban-sdk` 22.0.8, workspace version 1.1.0
- Build target: `wasm32v1-none`
