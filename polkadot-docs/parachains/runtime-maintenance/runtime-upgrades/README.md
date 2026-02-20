---
title: "Runtime Upgrades"
description: "Verify the runtime upgrades tutorial from docs.polkadot.com"
source_url: "https://docs.polkadot.com/parachains/runtime-maintenance/runtime-upgrades/"
source_github: "https://github.com/polkadot-developers/polkadot-docs/blob/master/parachains/runtime-maintenance/runtime-upgrades.md"
docs_commit: "6249ee3d8487c858826109648c24cd8e25c76d16"
polkadot_sdk_version: "polkadot-stable2512-1"
parachain_template_version: "v0.0.5"
---

# Runtime Upgrades

[![Runtime Upgrades](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-runtime-upgrades.yml/badge.svg?event=push)](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-runtime-upgrades.yml)

This project verifies the [Runtime Upgrades](https://docs.polkadot.com/parachains/runtime-maintenance/runtime-upgrades/) tutorial from docs.polkadot.com.

## What This Tests

1. Prerequisites check (Rust, wasm target, required tools)
2. Clone the parachain template repository (v0.0.5)
3. Create and integrate a custom pallet (prerequisite from Create a Custom Pallet guide)
4. Build the initial runtime (spec_version = 1)
5. Start the node and verify spec_version = 1
6. Add a new `reset_counter` dispatchable to the custom pallet
7. Bump `spec_version` from 1 to 2
8. Build the upgraded runtime
9. Submit the runtime upgrade via `system_setCode` RPC
10. Verify spec_version = 2 after the upgrade
11. Verify the chain continues producing blocks post-upgrade

## Prerequisites

Before running tests, ensure you have:

- Rust (version specified in `rust-toolchain.toml`)
- wasm32-unknown-unknown target
- Node.js 22+
- `staging-chain-spec-builder` CLI tool
- `polkadot-omni-node` CLI tool

### Installing Required Tools

```bash
# Install the tools with pinned versions (see "Versions Tested" below)
cargo install --locked staging-chain-spec-builder@16.0.0
cargo install --locked polkadot-omni-node@0.13.0
```

## Running Tests

```bash
# Install npm dependencies (uses lock file)
npm ci

# Run all verification tests
npm test
```

## Test Phases

### 1. Environment Check
Verifies all prerequisites are installed with correct versions.

### 2. Set Up Parachain with Custom Pallet (Prerequisite)
- Clones `polkadot-sdk-parachain-template` (v0.0.5)
- Creates `pallet-custom` with counter functionality
- Integrates pallet into the runtime

### 3. Initial Build (spec_version = 1)
- Builds with `cargo build --release`
- Generates chain specification
- Verifies WASM runtime is generated

### 4. Start Chain and Verify spec_version = 1
- Starts `polkadot-omni-node` in dev mode
- Verifies blocks are produced
- Confirms spec_version is 1

### 5. Add New Feature and Bump spec_version
- Adds `reset_counter` dispatchable to custom pallet
- Bumps `spec_version` from 1 to 2

### 6. Build the Upgraded Runtime
- Rebuilds with `cargo build --release`
- Verifies new WASM binary is generated

### 7. Submit Runtime Upgrade
- Submits the new WASM via `system_setCode` RPC
- Verifies spec_version changes to 2
- Confirms `lastRuntimeUpgrade` storage is populated

### 8. Post-Upgrade Verification
- Verifies the chain continues producing blocks
- Confirms RPC queries work correctly

## Exact Replication Steps

To manually replicate this guide:

```bash
# 1. Clone the template (using v0.0.5 release)
git clone --branch v0.0.5 https://github.com/paritytech/polkadot-sdk-parachain-template.git
cd polkadot-sdk-parachain-template

# 2. Complete the "Create a Custom Pallet" guide first
# (creates pallets/pallet-custom with counter functionality)

# 3. Build the initial runtime
cargo build --release

# 4. Generate chain spec
chain-spec-builder create -t development \
  --relay-chain paseo \
  --para-id 1000 \
  --runtime ./target/release/wbuild/parachain-template-runtime/parachain_template_runtime.compact.compressed.wasm \
  named-preset development

# 5. Start the node
polkadot-omni-node --chain ./chain_spec.json --dev

# 6. Add reset_counter function to custom pallet src/lib.rs
# (see tutorial for the function code)

# 7. Bump spec_version from 1 to 2 in runtime/src/lib.rs

# 8. Rebuild the runtime
cargo build --release

# 9. Submit the upgrade via Polkadot.js Apps or system_setCode RPC

# 10. Verify spec_version is now 2
```

## Versions Tested

| Component | Version |
|-----------|---------|
| Rust | See `rust-toolchain.toml` |
| Polkadot SDK | polkadot-stable2512-1 |
| Parachain Template | v0.0.5 |
| chain-spec-builder | 16.0.0 |
| polkadot-omni-node | 0.13.0 |

## Source

- [docs.polkadot.com tutorial](https://docs.polkadot.com/parachains/runtime-maintenance/runtime-upgrades/)
- [polkadot-sdk-parachain-template](https://github.com/paritytech/polkadot-sdk-parachain-template)
