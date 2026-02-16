---
title: "Create a Custom Pallet"
description: "Verify the create a custom pallet guide from docs.polkadot.com"
source_url: "https://docs.polkadot.com/parachains/customize-runtime/pallet-development/create-a-pallet/"
source_github: "https://github.com/polkadot-developers/polkadot-docs/blob/master/parachains/customize-runtime/pallet-development/create-a-pallet.md"
docs_commit: "140e229805b6210db10e679c9543e53858a96d43"
last_tested: "2026-02-15"
polkadot_sdk_version: "2512.1.0"
parachain_template_version: "v0.0.5"
---

# Create a Custom Pallet

[![Create a Custom Pallet](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-create-a-pallet.yml/badge.svg?event=push)](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-create-a-pallet.yml)

This project verifies the [Create a Custom Pallet](https://docs.polkadot.com/parachains/customize-runtime/pallet-development/create-a-pallet/) guide from docs.polkadot.com.

## What This Tests

1. Prerequisites check (Rust, wasm target, required tools)
2. Clone the parachain template repository (v0.0.5)
3. Create a new `pallet-custom` in the pallets directory
4. Write the pallet's `Cargo.toml` with FRAME dependencies
5. Write the complete pallet implementation in `src/lib.rs`
6. Build the pallet with `cargo build --package pallet-custom`
7. Add the pallet to `runtime/Cargo.toml` dependencies and std features
8. Implement `pallet_custom::Config` for Runtime
9. Register `CustomPallet` in the runtime construct
10. Build the modified runtime with `cargo build --release`
11. Generate chain specification
12. Start local node and verify the pallet is available

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

### 2. Clone Template and Create Pallet
- Clones `polkadot-sdk-parachain-template` (v0.0.5)
- Creates `pallets/pallet-custom` directory
- Writes pallet `Cargo.toml` with FRAME dependencies
- Writes complete pallet implementation with:
  - `Config` trait with `RuntimeEvent` and `CounterMaxValue`
  - Events: `CounterValueSet`, `CounterIncremented`, `CounterDecremented`
  - Errors: `NoneValue`, `Overflow`, `Underflow`, `CounterMaxValueExceeded`
  - Storage: `CounterValue` and `UserInteractions` map
  - Genesis configuration
  - Dispatchable functions: `set_counter_value`, `increment`, `decrement`
- Builds the pallet with `cargo build --package pallet-custom`

### 3. Integrate Pallet into Runtime
- Adds `pallet-custom` to runtime dependencies
- Adds `pallet-custom/std` to runtime features
- Implements `pallet_custom::Config for Runtime`
- Registers `CustomPallet` in runtime (index 51)

### 4. Build Verification
- Builds with `cargo build --release`
- Verifies WASM runtime is generated

### 5. Runtime Verification
- Generates chain specification
- Starts `polkadot-omni-node` in dev mode
- Verifies blocks are produced
- Verifies CustomPallet is available

## Exact Replication Steps

To manually replicate this guide:

```bash
# 1. Clone the template (using v0.0.5 release)
git clone --branch v0.0.5 https://github.com/paritytech/polkadot-sdk-parachain-template.git
cd polkadot-sdk-parachain-template

# 2. Create the pallet directory
mkdir -p pallets/pallet-custom/src

# 3. Create pallets/pallet-custom/Cargo.toml with FRAME dependencies
# (see guide for complete Cargo.toml)

# 4. Create pallets/pallet-custom/src/lib.rs with pallet implementation
# (see guide for complete implementation)

# 5. Build the pallet
cargo build --package pallet-custom

# 6. Add to runtime/Cargo.toml [dependencies]:
# pallet-custom = { path = "../pallets/pallet-custom", default-features = false }

# 7. Add to runtime/Cargo.toml [features] std array:
# "pallet-custom/std",

# 8. Add Config implementation in runtime/src/configs/mod.rs:
# impl pallet_custom::Config for Runtime {
#     type RuntimeEvent = RuntimeEvent;
#     type CounterMaxValue = ConstU32<1000>;
# }

# 9. Register pallet in runtime/src/lib.rs:
# #[runtime::pallet_index(51)]
# pub type CustomPallet = pallet_custom;

# 10. Build the runtime
cargo build --release

# 11. Generate chain spec
chain-spec-builder create -t development \
  --relay-chain paseo \
  --para-id 1000 \
  --runtime ./target/release/wbuild/parachain-template-runtime/parachain_template_runtime.compact.compressed.wasm \
  named-preset development

# 12. Run node
polkadot-omni-node --chain ./chain_spec.json --dev
```

## Versions Tested

| Component | Version |
|-----------|---------|
| Rust | See `rust-toolchain.toml` |
| Polkadot SDK | 2512.1.0 |
| Parachain Template | v0.0.5 |
| chain-spec-builder | 16.0.0 |
| polkadot-omni-node | 0.13.0 |

## Source

- [docs.polkadot.com guide](https://docs.polkadot.com/parachains/customize-runtime/pallet-development/create-a-pallet/)
- [polkadot-sdk-parachain-template](https://github.com/paritytech/polkadot-sdk-parachain-template)
