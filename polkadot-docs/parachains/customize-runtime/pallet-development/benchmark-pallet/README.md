---
title: "Benchmark Pallets"
description: "Verify the benchmark pallets guide from docs.polkadot.com"
source_url: "https://docs.polkadot.com/parachains/customize-runtime/pallet-development/benchmark-pallet/"
last_tested: "2025-01-28"
polkadot_sdk_version: "2503.0.1"
parachain_template_version: "v0.0.4"
---

# Benchmark Pallets

[![Benchmark Pallets](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-benchmark-pallet.yml/badge.svg)](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-benchmark-pallet.yml)

This project verifies the [Benchmark Pallets](https://docs.polkadot.com/parachains/customize-runtime/pallet-development/benchmark-pallet/) guide from docs.polkadot.com.

## What This Tests

1. Prerequisites check (Rust, wasm target, git)
2. Clone the parachain template repository (v0.0.4)
3. Create a `pallet-custom` with benchmarking support:
   - `Cargo.toml` with `runtime-benchmarks` feature
   - `lib.rs` with `WeightInfo` trait and benchmarking module declaration
   - `weights.rs` with `WeightInfo` trait and placeholder implementations
   - `benchmarking.rs` with benchmark functions for all dispatchables
   - `mock.rs` configured with `type WeightInfo = ()`
4. Verify structure:
   - WeightInfo trait has methods for all dispatchables
   - Benchmarks exist for set_counter_value, increment, decrement
   - Pallet calls use `T::WeightInfo::*` for weights
5. Build and test:
   - Build pallet normally
   - Run unit tests
   - Build with `--features runtime-benchmarks`
   - Run benchmark tests

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

### 2. Clone Template and Create Pallet
- Clones `polkadot-sdk-parachain-template` (v0.0.4)
- Creates `pallets/pallet-custom` directory
- Writes pallet `Cargo.toml` with `runtime-benchmarks` feature
- Adds pallet to workspace

### 3. Create Pallet with Benchmarking Support
- Writes `src/lib.rs` with:
  - `#[cfg(feature = "runtime-benchmarks")] mod benchmarking;`
  - `pub mod weights;`
  - `type WeightInfo: WeightInfo;` in Config trait
  - `#[pallet::weight(T::WeightInfo::*())]` annotations
- Creates `src/weights.rs` with:
  - `pub trait WeightInfo` with weight methods
  - `SubstrateWeight<T>` implementation
  - `()` implementation for testing
- Creates `src/benchmarking.rs` with:
  - `#[benchmarks]` module
  - `#[benchmark]` functions for each dispatchable
  - `impl_benchmark_test_suite!` macro

### 4. Verify Benchmarking Structure
- Checks WeightInfo trait has all methods
- Verifies benchmark functions exist
- Confirms pallet uses T::WeightInfo

### 5. Build and Test Benchmarks
- `cargo build --package pallet-custom`
- `cargo test --package pallet-custom --lib`
- `cargo build --package pallet-custom --features runtime-benchmarks`
- `cargo test --package pallet-custom --features runtime-benchmarks`

## Key Concepts Verified

### WeightInfo Trait
```rust
pub trait WeightInfo {
    fn set_counter_value() -> Weight;
    fn increment() -> Weight;
    fn decrement() -> Weight;
}
```

### Benchmark Function Pattern
```rust
#[benchmark]
fn increment() {
    let caller: T::AccountId = whitelisted_caller();
    let amount: u32 = 50;

    #[extrinsic_call]
    _(RawOrigin::Signed(caller.clone()), amount);

    assert_eq!(CounterValue::<T>::get(), amount);
}
```

### Using Weights in Dispatchables
```rust
#[pallet::weight(T::WeightInfo::increment())]
pub fn increment(origin: OriginFor<T>, amount: u32) -> DispatchResult {
    // ...
}
```

## Versions Tested

| Component | Version |
|-----------|---------|
| Rust | See `rust-toolchain.toml` |
| Polkadot SDK | 2503.0.1 |
| Parachain Template | v0.0.4 |

## Source

- [docs.polkadot.com guide](https://docs.polkadot.com/parachains/customize-runtime/pallet-development/benchmark-pallet/)
- [polkadot-sdk-parachain-template](https://github.com/paritytech/polkadot-sdk-parachain-template)
