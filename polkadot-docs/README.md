# Polkadot Docs Verification

This folder contains verified, reproducible tests for guides from [docs.polkadot.com](https://docs.polkadot.com). Each guide is a self-contained project that can be cloned and run to verify the documentation works as described.

## Purpose

This repository serves as the **source of truth** that Polkadot documentation works. When tests pass here, we can confidently link to these verified guides from the official docs.

## Verified Guides

### Parachains

| Guide | Status | Source |
|-------|--------|--------|
| [Install Polkadot SDK](./parachains/install-polkadot-sdk/) | [![Install Polkadot SDK](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-install-polkadot-sdk.yml/badge.svg?event=push)](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-install-polkadot-sdk.yml) | [docs.polkadot.com](https://docs.polkadot.com/parachains/install-polkadot-sdk/) |
| [Set Up Parachain Template](./parachains/set-up-parachain-template/) | [![Set Up Parachain Template](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-set-up-parachain-template.yml/badge.svg?event=push)](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-set-up-parachain-template.yml) | [docs.polkadot.com](https://docs.polkadot.com/parachains/launch-a-parachain/set-up-the-parachain-template/) |
| [Add Existing Pallets](./parachains/customize-runtime/add-existing-pallets/) | [![Add Existing Pallets](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-add-existing-pallets.yml/badge.svg?event=push)](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-add-existing-pallets.yml) | [docs.polkadot.com](https://docs.polkadot.com/parachains/customize-runtime/add-existing-pallets/) |
| [Add Pallet Instances](./parachains/customize-runtime/add-pallet-instances/) | [![Add Pallet Instances](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-add-pallet-instances.yml/badge.svg?event=push)](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-add-pallet-instances.yml) | [docs.polkadot.com](https://docs.polkadot.com/parachains/customize-runtime/add-pallet-instances/) |
| [Create a Custom Pallet](./parachains/customize-runtime/pallet-development/create-a-pallet/) | [![Create a Custom Pallet](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-create-a-pallet.yml/badge.svg?event=push)](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-create-a-pallet.yml) | [docs.polkadot.com](https://docs.polkadot.com/parachains/customize-runtime/pallet-development/create-a-pallet/) |
| [Mock Your Runtime](./parachains/customize-runtime/pallet-development/mock-runtime/) | [![Mock Your Runtime](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-mock-runtime.yml/badge.svg?event=push)](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-mock-runtime.yml) | [docs.polkadot.com](https://docs.polkadot.com/parachains/customize-runtime/pallet-development/mock-runtime/) |
| [Unit Test Pallets](./parachains/customize-runtime/pallet-development/pallet-testing/) | [![Unit Test Pallets](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-pallet-testing.yml/badge.svg?event=push)](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-pallet-testing.yml) | [docs.polkadot.com](https://docs.polkadot.com/parachains/customize-runtime/pallet-development/pallet-testing/) |
| [Benchmark Pallets](./parachains/customize-runtime/pallet-development/benchmark-pallet/) | [![Benchmark Pallets](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-benchmark-pallet.yml/badge.svg?event=push)](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-benchmark-pallet.yml) | [docs.polkadot.com](https://docs.polkadot.com/parachains/customize-runtime/pallet-development/benchmark-pallet/) |
| [Channels Between Parachains](./parachains/interoperability/channels-between-parachains/) | [![Channels Between Parachains](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-channels-between-parachains.yml/badge.svg?event=push)](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-channels-between-parachains.yml) | [docs.polkadot.com](https://docs.polkadot.com/parachains/interoperability/channels-between-parachains/) |
| [Channels with System Parachains](./parachains/interoperability/channels-with-system-parachains/) | [![Channels with System Parachains](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-channels-with-system-parachains.yml/badge.svg?event=push)](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-channels-with-system-parachains.yml) | [docs.polkadot.com](https://docs.polkadot.com/parachains/interoperability/channels-with-system-parachains/) |
| [Runtime Upgrades](./parachains/runtime-maintenance/runtime-upgrades/) | [![Runtime Upgrades](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-runtime-upgrades.yml/badge.svg?event=push)](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-runtime-upgrades.yml) | [docs.polkadot.com](https://docs.polkadot.com/parachains/runtime-maintenance/runtime-upgrades/) |

### Networks

| Guide | Status | Source |
|-------|--------|--------|
| [Run a Parachain Network](./networks/run-a-parachain-network/) | [![Run a Parachain Network](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-run-a-parachain-network.yml/badge.svg?event=push)](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-run-a-parachain-network.yml) | [docs.polkadot.com](https://docs.polkadot.com/parachains/testing/run-a-parachain-network/) |
| [Fork a Parachain](./parachains/testing/fork-a-parachain/) | [![Fork a Parachain](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-fork-a-parachain.yml/badge.svg?event=push)](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-fork-a-parachain.yml) | [docs.polkadot.com](https://docs.polkadot.com/parachains/testing/fork-a-parachain/) |

### Chain Interactions

| Guide | Status | Source |
|-------|--------|--------|
| [Create an Account](./chain-interactions/create-account/) | [![Create an Account](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-create-account.yml/badge.svg?event=push)](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-create-account.yml) | [docs.polkadot.com](https://docs.polkadot.com/chain-interactions/accounts/create-account/) |
| [Query Account Information](./chain-interactions/query-accounts/) | [![Query Account Information](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-query-accounts.yml/badge.svg?event=push)](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-query-accounts.yml) | [docs.polkadot.com](https://docs.polkadot.com/chain-interactions/accounts/query-accounts/) |
| [Query On-Chain State with SDKs](./chain-interactions/query-sdks/) | [![Query On-Chain State with SDKs](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-query-sdks.yml/badge.svg?event=push)](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-query-sdks.yml) | [docs.polkadot.com](https://docs.polkadot.com/chain-interactions/query-data/query-sdks/) |
| [Query On-Chain State with Sidecar REST API](./chain-interactions/query-rest/) | [![Query On-Chain State with Sidecar REST API](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-query-rest.yml/badge.svg?event=push)](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-query-rest.yml) | [docs.polkadot.com](https://docs.polkadot.com/chain-interactions/query-data/query-rest/) |
| [Runtime API Calls](./chain-interactions/runtime-api-calls/) | [![Runtime API Calls](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-runtime-api-calls.yml/badge.svg?event=push)](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-runtime-api-calls.yml) | [docs.polkadot.com](https://docs.polkadot.com/chain-interactions/query-data/runtime-api-calls/) |
| [Calculate Transaction Fees](./chain-interactions/calculate-transaction-fees/) | [![Calculate Transaction Fees](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-calculate-transaction-fees.yml/badge.svg?event=push)](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-calculate-transaction-fees.yml) | [docs.polkadot.com](https://docs.polkadot.com/chain-interactions/send-transactions/calculate-transaction-fees/) |

### Smart Contracts

| Guide | Status | Source |
|-------|--------|--------|
| [Local Development Node](./smart-contracts/local-dev-node/) | [![Local Development Node](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-local-dev-node.yml/badge.svg?event=push)](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-local-dev-node.yml) | [docs.polkadot.com](https://docs.polkadot.com/smart-contracts/dev-environments/local-dev-node/) |
| [Basic Contract with Hardhat](./smart-contracts/basic-hardhat/) | [![Basic Contract with Hardhat](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-basic-hardhat.yml/badge.svg?event=push)](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-basic-hardhat.yml) | [docs.polkadot.com](https://docs.polkadot.com/smart-contracts/cookbook/smart-contracts/deploy-basic/basic-hardhat/) |
| [ERC-20 with Hardhat](./smart-contracts/erc20-hardhat/) | [![ERC-20 with Hardhat](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-erc20-hardhat.yml/badge.svg?event=push)](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-erc20-hardhat.yml) | [docs.polkadot.com](https://docs.polkadot.com/smart-contracts/cookbook/smart-contracts/deploy-erc20/erc20-hardhat/) |

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
