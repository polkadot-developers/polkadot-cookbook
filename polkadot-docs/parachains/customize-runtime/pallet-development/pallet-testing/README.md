---
title: "Unit Test Pallets"
description: "Verify the unit test pallets guide from docs.polkadot.com"
source_url: "https://docs.polkadot.com/parachains/customize-runtime/pallet-development/pallet-testing/"
source_github: "https://github.com/polkadot-developers/polkadot-docs/blob/master/parachains/customize-runtime/pallet-development/pallet-testing.md"
docs_commit: "d4b41f851b16ac909a7422726a4fd47fea239ba3"
last_tested: "2026-02-13"
polkadot_sdk_version: "2512.1.0"
parachain_template_version: "v0.0.5"
---

# Unit Test Pallets

[![Unit Test Pallets](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-pallet-testing.yml/badge.svg?event=push)](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-pallet-testing.yml)

This project verifies the [Unit Test Pallets](https://docs.polkadot.com/parachains/customize-runtime/pallet-development/pallet-testing/) guide from docs.polkadot.com.

## What This Tests

1. Prerequisites check (Rust, wasm target, git)
2. Clone the parachain template repository (v0.0.5)
3. Create a `pallet-custom` with mock and tests modules
4. Write `src/lib.rs` with `#[cfg(test)] mod mock;` and `#[cfg(test)] mod tests;`
5. Create `src/mock.rs` with mock runtime (from mock-runtime guide)
6. Create `src/tests.rs` with 14 unit tests covering:
   - `set_counter_value` (works, requires root, respects max value)
   - `increment` (works, tracks interactions, overflow, max value)
   - `decrement` (works, underflow, tracks interactions)
   - Mixed operations and multi-user tracking
   - Genesis configuration
7. Run `cargo test --package pallet-custom` and verify all tests pass

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
- Writes pallet `Cargo.toml` with FRAME dependencies
- Adds pallet to workspace members and dependencies

### 3. Create Pallet with Mock and Tests Modules
- Writes `src/lib.rs` with both `mod mock;` and `mod tests;` declarations
- Creates `src/mock.rs` with mock runtime configuration
- Creates `src/tests.rs` with 14 unit tests

### 4. Verify Test Structure
- Verifies all test functions are present
- Checks proper imports (`assert_ok!`, `assert_noop!`)
- Validates testing patterns (event testing with `System::set_block_number(1)`)

### 5. Build and Run Unit Tests
- Builds the pallet with `cargo build --package pallet-custom`
- Runs all tests with `cargo test --package pallet-custom`
- Verifies all 14 tests pass

## Unit Tests Included

| Test Function | Description |
|--------------|-------------|
| `set_counter_value_works` | Root can set counter value and event emits |
| `set_counter_value_requires_root` | Non-root origins fail with BadOrigin |
| `set_counter_value_respects_max_value` | Values above max are rejected |
| `increment_works` | Increment increases counter and emits event |
| `increment_tracks_multiple_interactions` | User interactions are counted |
| `increment_fails_on_overflow` | Overflow protection works |
| `increment_respects_max_value` | Max value limit enforced |
| `decrement_works` | Decrement decreases counter and emits event |
| `decrement_fails_on_underflow` | Underflow protection works |
| `decrement_tracks_multiple_interactions` | User interactions tracked |
| `mixed_increment_and_decrement_works` | Mixed operations work correctly |
| `different_users_tracked_separately` | Per-user tracking is independent |
| `genesis_config_works` | Genesis configuration initializes state |

## Key Testing Patterns

### Event Testing
Events are not emitted on block 0 (genesis block), so tests set the block number first:
```rust
System::set_block_number(1);
// ... perform action ...
System::assert_last_event(Event::SomeEvent { ... }.into());
```

### Success Assertions
```rust
assert_ok!(CustomPallet::some_function(RuntimeOrigin::signed(1), params));
```

### Error Assertions
```rust
assert_noop!(
    CustomPallet::some_function(RuntimeOrigin::signed(1), bad_params),
    Error::<Test>::SomeError
);
```

### Genesis Helpers
The mock provides three genesis helper functions:
- `new_test_ext()` - Default empty state
- `new_test_ext_with_counter(value)` - Custom initial counter
- `new_test_ext_with_interactions(value, vec![(account, count)])` - Full custom state

## Versions Tested

| Component | Version |
|-----------|---------|
| Rust | See `rust-toolchain.toml` |
| Polkadot SDK | 2512.1.0 |
| Parachain Template | v0.0.5 |

## Source

- [docs.polkadot.com guide](https://docs.polkadot.com/parachains/customize-runtime/pallet-development/pallet-testing/)
- [polkadot-sdk-parachain-template](https://github.com/paritytech/polkadot-sdk-parachain-template)
