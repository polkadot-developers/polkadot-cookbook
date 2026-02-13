---
title: "Add Pallet Instances to Runtime"
description: "Verify the add pallet instances guide from docs.polkadot.com"
source_url: "https://docs.polkadot.com/parachains/customize-runtime/add-pallet-instances/"
source_github: "https://github.com/polkadot-developers/polkadot-docs/blob/master/parachains/customize-runtime/add-pallet-instances.md"
last_tested: "2026-02-13"
polkadot_sdk_version: "2512.1.0"
parachain_template_version: "v0.0.5"
---

# Add Pallet Instances to Runtime

[![Add Pallet Instances](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-add-pallet-instances.yml/badge.svg)](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-add-pallet-instances.yml)

This project verifies the [Add Pallet Instances](https://docs.polkadot.com/parachains/customize-runtime/add-pallet-instances/) guide from docs.polkadot.com.

## What This Tests

1. Prerequisites check (Rust, wasm target, required tools)
2. Clone the parachain template repository
3. Add `pallet-collective` to `runtime/Cargo.toml`
4. Add parameter_types for collective configuration
5. Create instance type aliases (`TechnicalCollective`, `CouncilCollective`)
6. Implement the `Config` trait for both instances in `runtime/src/configs/mod.rs`
7. Register both pallets (`TechnicalCommittee`, `Council`) in `runtime/src/lib.rs` using `Instance1`/`Instance2`
8. Build the modified runtime with `cargo build --release`
9. Generate chain specification
10. Start local node and verify both pallet instances are available

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

### 2. Clone and Modify Template
- Clones `polkadot-sdk-parachain-template` (v0.0.5)
- Adds `pallet-collective` dependency to Cargo.toml
- Adds parameter_types for collective pallets:
  - `MotionDuration`: 24 hours
  - `MaxProposals`: 100
  - `MaxMembers`: 100
  - `MaxProposalWeight`: 50% of max block weight
- Creates instance type aliases for both collectives
- Implements Config trait for `TechnicalCollective` (Instance1)
- Implements Config trait for `CouncilCollective` (Instance2)
- Registers `TechnicalCommittee` pallet in runtime macro (using `Instance1`)
- Registers `Council` pallet in runtime macro (using `Instance2`)

### 3. Build Verification
- Builds with `cargo build --release`
- Verifies WASM runtime is generated

### 4. Runtime Verification
- Generates chain specification
- Starts `polkadot-omni-node` in dev mode
- Verifies blocks are produced
- Verifies both collective instances are available in metadata

## Exact Replication Steps

To manually replicate this guide:

```bash
# 1. Clone the template (using v0.0.5 release)
git clone --branch v0.0.5 https://github.com/paritytech/polkadot-sdk-parachain-template.git
cd polkadot-sdk-parachain-template

# 2. Add pallet-collective to runtime/Cargo.toml features
# In [dependencies.polkadot-sdk] section, add to features array:
# "pallet-collective"

# 3. Add parameter_types in runtime/src/configs/mod.rs
# parameter_types! {
#     pub const MotionDuration: BlockNumber = 24 * HOURS;
#     pub const MaxProposals: u32 = 100;
#     pub const MaxMembers: u32 = 100;
#     pub MaxProposalWeight: Weight = Perbill::from_percent(50) * RuntimeBlockWeights::get().max_block;
# }

# 4. Create instance type aliases
# pub type TechnicalCollective = pallet_collective::Instance1;
# pub type CouncilCollective = pallet_collective::Instance2;

# 5. Implement Config trait for both instances
# impl pallet_collective::Config<TechnicalCollective> for Runtime { ... }
# impl pallet_collective::Config<CouncilCollective> for Runtime { ... }

# 6. Re-export the type aliases in runtime/src/lib.rs (after pub mod configs;)
# pub use configs::{TechnicalCollective, CouncilCollective};

# 7. Add instance imports and register both pallets in runtime/src/lib.rs
# use frame_support::instances::{Instance1, Instance2};
# #[runtime::pallet_index(16)]
# pub type TechnicalCommittee = pallet_collective<Instance1>;
# #[runtime::pallet_index(17)]
# pub type Council = pallet_collective<Instance2>;
#
# Note: The #[frame_support::runtime] macro requires Instance1/Instance2 identifiers
# directly in scope, not type aliases. The type aliases (TechnicalCollective,
# CouncilCollective) are used for the Config trait implementations.

# 8. Build
cargo build --release

# 9. Generate chain spec
chain-spec-builder create -t development \
  --relay-chain paseo \
  --para-id 1000 \
  --runtime ./target/release/wbuild/parachain-template-runtime/parachain_template_runtime.compact.compressed.wasm \
  named-preset development

# 10. Run node
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

- [docs.polkadot.com guide](https://docs.polkadot.com/parachains/customize-runtime/add-pallet-instances/)
- [polkadot-sdk-parachain-template](https://github.com/paritytech/polkadot-sdk-parachain-template)
