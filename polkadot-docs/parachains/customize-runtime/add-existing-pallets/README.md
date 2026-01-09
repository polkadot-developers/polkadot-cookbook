---
title: "Add Existing Pallets to Runtime"
description: "Verify the add existing pallets guide from docs.polkadot.com"
source_url: "https://docs.polkadot.com/parachains/customize-runtime/add-existing-pallets/"
last_tested: "2025-01-09"
polkadot_sdk_version: "polkadot-v2503.0.1"
---

# Add Existing Pallets to Runtime

This project verifies the [Add Existing Pallets](https://docs.polkadot.com/parachains/customize-runtime/add-existing-pallets/) guide from docs.polkadot.com.

## What This Tests

1. Prerequisites check (Rust, wasm target, required tools)
2. Clone the parachain template repository
3. Add `pallet-utility` to `runtime/Cargo.toml`
4. Implement the `Config` trait in `runtime/src/configs/mod.rs`
5. Register the pallet in `runtime/src/lib.rs`
6. Build the modified runtime with `cargo build --release`
7. Generate chain specification
8. Start local node and verify pallet is available

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
cargo install --locked staging-chain-spec-builder@10.0.0
cargo install --locked polkadot-omni-node@0.5.0
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

### 2. Clone and Modify Template
- Clones `polkadot-sdk-parachain-template`
- Adds `pallet-utility` dependency to Cargo.toml
- Implements Config trait for pallet-utility
- Registers pallet in runtime macro

### 3. Build Verification
- Builds with `cargo build --release`
- Verifies WASM runtime is generated

### 4. Runtime Verification
- Generates chain specification
- Starts `polkadot-omni-node` in dev mode
- Verifies blocks are produced
- Verifies pallet-utility is available in metadata

## Exact Replication Steps

To manually replicate this guide:

```bash
# 1. Clone the template
git clone https://github.com/paritytech/polkadot-sdk-parachain-template.git
cd polkadot-sdk-parachain-template

# 2. Add pallet-utility to runtime/Cargo.toml features
# In [dependencies.polkadot-sdk] section, add to features array:
# "pallet-utility"

# 3. Implement Config trait in runtime/src/configs/mod.rs
# Add:
# impl pallet_utility::Config for Runtime {
#     type RuntimeEvent = RuntimeEvent;
#     type RuntimeCall = RuntimeCall;
#     type PalletsOrigin = OriginCaller;
#     type WeightInfo = pallet_utility::weights::SubstrateWeight<Runtime>;
# }

# 4. Register pallet in runtime/src/lib.rs
# In #[frame_support::runtime] block, add:
# #[runtime::pallet_index(X)]
# pub type Utility = pallet_utility::Pallet<Runtime>;

# 5. Build
cargo build --release

# 6. Generate chain spec
chain-spec-builder create -t development \
  --relay-chain paseo \
  --para-id 1000 \
  --runtime ./target/release/wbuild/parachain-template-runtime/parachain_template_runtime.compact.compressed.wasm \
  named-preset development

# 7. Run node
polkadot-omni-node --chain ./chain_spec.json --dev
```

## Versions Tested

| Component | Version |
|-----------|---------|
| Rust | See `rust-toolchain.toml` |
| Polkadot SDK | polkadot-v2503.0.1 |
| chain-spec-builder | 10.0.0 |
| polkadot-omni-node | 0.5.0 |

## Source

- [docs.polkadot.com guide](https://docs.polkadot.com/parachains/customize-runtime/add-existing-pallets/)
- [polkadot-sdk-parachain-template](https://github.com/paritytech/polkadot-sdk-parachain-template)
