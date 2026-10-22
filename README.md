# soroban-batcher

Soroban utility contract for batching multiple contract calls into a single transaction.

## Overview

The **batcher** is a small, stateless Soroban (Rust) helper contract used by the Aquarius AMM operations tooling to reduce transaction overhead. It composes several sub-calls into one transaction (for example, applying many AMM plane updates at once). It holds no storage and emits no events — it is a pure input/output utility whose only effects come from the sub-calls it invokes.

## Contracts

The workspace (`Cargo.toml`) has one member:

- **batcher** (`batcher/`) — `batch(auth_users, batch, return_result)` executes a list of `(contract, fn_name, args)` calls sequentially, optionally collecting and returning their results.

The crate has `src/lib.rs` and `src/test.rs`. The only vendored WASM is a liquidity-pool plane contract in `contracts/`, used as a test fixture.

## Build

Toolchain is pinned by `rust-toolchain.toml` (Rust 1.96.0, target `wasm32v1-none`). Tasks are wrapped with `task` (see `Taskfile.yaml`):

```sh
task build      # stellar contract build --optimize into target/wasm32v1-none/release/
task test       # cargo test
task check      # cargo check --all
task all        # clean + fmt + build + test
```

The release profile is tuned aggressively for WASM size (`opt-level = "z"`, LTO, `strip = "symbols"`, single codegen unit, `panic = "abort"`); the batcher compiles to about 1 KB.

Pushing a `v*` tag builds and publishes the release WASM via the release workflow (`.github/workflows/release.yml`).

## Stack

- Rust (edition 2021), `soroban-sdk` 29.0.1, workspace version 2.2.0
- Build target: `wasm32v1-none`
