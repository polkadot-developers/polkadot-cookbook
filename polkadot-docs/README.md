# Polkadot Docs Verification

This folder contains verified, reproducible tests for guides from [docs.polkadot.com](https://docs.polkadot.com). Each guide is a self-contained project that can be cloned and run to verify the documentation works as described.

## Purpose

This repository serves as the **source of truth** that Polkadot documentation works. When tests pass here, we can confidently link to these verified guides from the official docs.

## Verified Guides

| Guide | Status | Source |
|-------|--------|--------|
| [Set Up Parachain Template](./parachains/set-up-parachain-template/) | [![Set Up Parachain Template](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-set-up-parachain-template.yml/badge.svg)](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-set-up-parachain-template.yml) | [docs.polkadot.com](https://docs.polkadot.com/parachains/launch-a-parachain/set-up-the-parachain-template/) |
| [Add Existing Pallets](./parachains/customize-runtime/add-existing-pallets/) | [![Add Existing Pallets](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-add-existing-pallets.yml/badge.svg)](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-add-existing-pallets.yml) | [docs.polkadot.com](https://docs.polkadot.com/parachains/customize-runtime/add-existing-pallets/) |
| [Add Pallet Instances](./parachains/customize-runtime/add-pallet-instances/) | [![Add Pallet Instances](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-add-pallet-instances.yml/badge.svg)](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-add-pallet-instances.yml) | [docs.polkadot.com](https://docs.polkadot.com/parachains/customize-runtime/add-pallet-instances/) |
| [Create a Custom Pallet](./parachains/customize-runtime/pallet-development/create-a-pallet/) | [![Create a Custom Pallet](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-create-a-pallet.yml/badge.svg)](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-create-a-pallet.yml) | [docs.polkadot.com](https://docs.polkadot.com/parachains/customize-runtime/pallet-development/create-a-pallet/) |
| [Mock Your Runtime](./parachains/customize-runtime/pallet-development/mock-runtime/) | [![Mock Your Runtime](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-mock-runtime.yml/badge.svg)](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-mock-runtime.yml) | [docs.polkadot.com](https://docs.polkadot.com/parachains/customize-runtime/pallet-development/mock-runtime/) |
| [Unit Test Pallets](./parachains/customize-runtime/pallet-development/pallet-testing/) | [![Unit Test Pallets](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-pallet-testing.yml/badge.svg)](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-pallet-testing.yml) | [docs.polkadot.com](https://docs.polkadot.com/parachains/customize-runtime/pallet-development/pallet-testing/) |

## How It Works

Each guide folder contains:

- `README.md` - Guide description and exact replication steps
- `rust-toolchain.toml` - Pinned Rust version
- `package.json` + `package-lock.json` - Pinned npm dependencies
- `tests/` - Verification tests that run with `dot test`

## Running Tests

```bash
# Navigate to a guide
cd polkadot-docs/parachains/set-up-parachain-template

# Install dependencies (uses lock files for reproducibility)
npm ci

# Run verification tests
dot test
```

## Reproducibility

All guides use locked dependencies to ensure reproducible builds:

- `cargo build --locked` - Uses exact crate versions from `Cargo.lock`
- `npm ci` - Uses exact npm versions from `package-lock.json`
- `rust-toolchain.toml` - Pins exact Rust version

If tests pass in CI, they should pass on your machine with the same versions.

## Contributing

When adding a new guide:

1. Create a folder mirroring the docs.polkadot.com URL structure
2. Copy `rust-toolchain.toml` from repo root
3. Add `package.json` with test dependencies
4. Write tests that verify each step of the guide
5. Commit lock file (`package-lock.json`)
