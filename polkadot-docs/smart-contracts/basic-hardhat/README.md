---
title: "Deploy a Basic Contract with Hardhat"
description: "Verify the basic contract deployment guide using Hardhat from docs.polkadot.com"
source_url: "https://docs.polkadot.com/smart-contracts/cookbook/smart-contracts/deploy-basic/basic-hardhat/"
source_repo: "https://github.com/polkadot-developers/polkadot-docs/blob/master/smart-contracts/cookbook/smart-contracts/deploy-basic/basic-hardhat.md"
---

# Deploy a Basic Contract with Hardhat

[![Deploy a Basic Contract with Hardhat](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-basic-hardhat.yml/badge.svg?event=push)](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-basic-hardhat.yml)

This project verifies the [Deploy a Basic Contract with Hardhat](https://docs.polkadot.com/smart-contracts/cookbook/smart-contracts/deploy-basic/basic-hardhat/) guide from docs.polkadot.com.

The tests run against [revm-hardhat-examples](https://github.com/polkadot-developers/revm-hardhat-examples) at a pinned commit, deploying a `Storage.sol` contract on Polkadot Hub TestNet.

## What This Tests

1. **Environment Prerequisites** — Node.js v22+, npm, and git are available
2. **Clone Repository** — Clones `revm-hardhat-examples` at the pinned commit and verifies expected files
3. **Install Dependencies** — Runs `npm install` and confirms Hardhat is available
4. **Verify Testnet Credentials** — `PRIVATE_KEY` environment variable is set
5. **Compile Contracts** — Runs `npx hardhat compile` and verifies the `Storage.json` artifact (ABI + bytecode + function signatures)
6. **Deploy via Ignition** — Deploys `Storage` using Hardhat Ignition to `polkadotTestnet` and verifies a contract address is returned

## Prerequisites

- Node.js v22.13.1 or later
- npm
- git
- A funded account on Polkadot Hub TestNet (get tokens from the [Polkadot Faucet](https://faucet.polkadot.io/))

## Running Tests Locally

```bash
# 1. Copy and populate the env file
cp .env.example .env
# Edit .env and set your PRIVATE_KEY

# 2. Install wrapper dependencies
npm install

# 3. Run all verification tests
npm test
```

## Environment Variables

| Variable | Description |
|---|---|
| `PRIVATE_KEY` | Private key of a funded account (no `0x` prefix) |

## Test Phases

### 1. Environment Prerequisites
Verifies Node.js v22+, npm, and git are present.

### 2. Clone Repository
Clones `polkadot-developers/revm-hardhat-examples` at a pinned SHA. Verifies `contracts/Storage.sol`, `ignition/modules/Storage.ts`, `hardhat.config.ts`, and `package.json`.

### 3. Install Dependencies
Runs `npm install` inside `basic-hardhat/` and confirms the Hardhat binary is available.

### 4. Verify Testnet Credentials
Asserts `PRIVATE_KEY` is populated so subsequent steps fail with a clear message instead of a cryptic Hardhat error.

### 5. Compile Contracts
Runs `npx hardhat compile` and verifies:
- The `artifacts/` directory is created
- `artifacts/contracts/Storage.sol/Storage.json` exists
- The ABI is valid and non-empty
- The ABI contains both `store(uint256)` and `retrieve()` functions
- Bytecode is non-empty

### 6. Deploy via Ignition
Runs `npx hardhat ignition deploy ./ignition/modules/Storage.ts --network polkadotTestnet` and verifies the output contains a valid EVM contract address. Retries up to 3 times on transient network errors.

## Exact Replication Steps

```bash
# 1. Clone the example repo
git clone https://github.com/polkadot-developers/revm-hardhat-examples
cd revm-hardhat-examples/basic-hardhat

# 2. Install dependencies
npm install

# 3. Set your private key
npx hardhat vars set PRIVATE_KEY

# 4. Compile
npx hardhat compile

# 5. Deploy to Polkadot TestNet
npx hardhat ignition deploy ignition/modules/Storage.ts --network polkadotTestnet
```

## Versions Tested

| Component | Version |
|---|---|
| Node.js | v22.13.1+ |
| Hardhat | ^2.27.0 |
| Solidity | 0.8.28 |

## Source

- [docs.polkadot.com guide](https://docs.polkadot.com/smart-contracts/cookbook/smart-contracts/deploy-basic/basic-hardhat/)
- [revm-hardhat-examples repository](https://github.com/polkadot-developers/revm-hardhat-examples)
