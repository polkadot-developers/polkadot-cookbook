---
title: "Mock Your Runtime"
description: "Verify the mock your runtime guide from docs.polkadot.com"
source_url: "https://docs.polkadot.com/parachains/customize-runtime/pallet-development/mock-runtime/"
source_github: "https://github.com/polkadot-developers/polkadot-docs/blob/master/parachains/customize-runtime/pallet-development/mock-runtime.md"
docs_commit: "d4b41f851b16ac909a7422726a4fd47fea239ba3"
last_tested: "2026-02-13"
polkadot_sdk_version: "2512.1.0"
parachain_template_version: "v0.0.5"
---

# Mock Your Runtime

[![Mock Your Runtime](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-mock-runtime.yml/badge.svg?event=push)](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-mock-runtime.yml)

This project verifies the [Mock Your Runtime](https://docs.polkadot.com/parachains/customize-runtime/pallet-development/mock-runtime/) guide from docs.polkadot.com.

## What This Tests

1. Prerequisites check (Rust, wasm target, git)
2. Clone the parachain template repository (v0.0.5)
3. Create a `pallet-custom` in the pallets directory (prerequisite from create-a-pallet guide)
4. Write the pallet's `Cargo.toml` with FRAME dependencies
5. Write `src/lib.rs` with the `#[cfg(test)] mod mock;` declaration
6. Create `src/mock.rs` with the mock runtime configuration:
   - `construct_runtime!` macro setup
   - `frame_system::Config` implementation for `Test`
   - `pallet_custom::Config` implementation for `Test`
   - `new_test_ext()` helper function for basic test setup
   - `new_test_ext_with_counter()` for custom initial counter values
   - `new_test_ext_with_interactions()` for custom genesis configurations
7. Build and verify the mock runtime compiles correctly

## Prerequisites

Before running tests, ensure you have:

- Rust (version specified in `rust-toolchain.toml`)
- wasm32-unknown-unknown target
- Node.js 22+
- Git

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

### 2. Clone Template and Create Pallet (Prerequisite)
- Clones `polkadot-sdk-parachain-template` (v0.0.5)
- Creates `pallets/pallet-custom` directory
- Writes pallet `Cargo.toml` with FRAME dependencies (sp-io and sp-runtime are accessed via `frame::deps`)
- Adds pallet to workspace members and dependencies

### 3. Create Mock Runtime Module
- Writes `src/lib.rs` with `#[cfg(test)] mod mock;` declaration
- Creates `src/mock.rs` with:
  - Mock runtime using `construct_runtime!`
  - `frame_system::Config` implementation using `#[derive_impl]`
  - `pallet_custom::Config` implementation
  - `new_test_ext()` helper for basic test environment
  - `new_test_ext_with_counter()` for custom initial values
  - `new_test_ext_with_interactions()` for custom genesis state

### 4. Build and Verify
- Builds the pallet with `cargo build --package pallet-custom`
- Compiles tests with `cargo test --package pallet-custom --lib --no-run`
- Verifies with `cargo check --package pallet-custom`

## Exact Replication Steps

To manually replicate this guide:

```bash
# 1. Clone the template (using v0.0.5 release)
git clone --branch v0.0.5 https://github.com/paritytech/polkadot-sdk-parachain-template.git
cd polkadot-sdk-parachain-template

# 2. Create the pallet directory (if following from create-a-pallet guide)
mkdir -p pallets/pallet-custom/src

# 3. Create pallets/pallet-custom/Cargo.toml with FRAME dependencies
# (sp-io and sp-runtime are accessed via frame::deps re-exports)

# 4. Create pallets/pallet-custom/src/lib.rs with mock module declaration
# Add: #[cfg(test)] mod mock;

# 5. Create pallets/pallet-custom/src/mock.rs with mock runtime:
# - construct_runtime! macro
# - frame_system::Config implementation
# - pallet_custom::Config implementation
# - new_test_ext() helper function

# 6. Verify compilation
cargo test --package pallet-custom --lib --no-run
```

## Key Concepts Verified

### Mock Runtime Structure
The mock runtime uses a minimal configuration with:
- `frame_system::mocking::MockBlock<Test>` as the block type
- `u64` as the `AccountId` type for simplified testing
- `IdentityLookup` for account lookup

### Genesis Configuration
The test verifies three helper functions for different test scenarios:
- `new_test_ext()` - Basic test with default genesis
- `new_test_ext_with_counter()` - Custom initial counter value
- `new_test_ext_with_interactions()` - Full custom genesis with user interactions

### Config Trait Implementation
Uses `#[derive_impl(frame_system::config_preludes::TestDefaultConfig)]` to inherit sensible defaults while only specifying the required types.

## Versions Tested

| Component | Version |
|-----------|---------|
| Rust | See `rust-toolchain.toml` |
| Polkadot SDK | 2512.1.0 |
| Parachain Template | v0.0.5 |

## Source

- [docs.polkadot.com guide](https://docs.polkadot.com/parachains/customize-runtime/pallet-development/mock-runtime/)
- [polkadot-sdk-parachain-template](https://github.com/paritytech/polkadot-sdk-parachain-template)
