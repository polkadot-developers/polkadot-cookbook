---
title: "ERC-20 with Hardhat"
description: "Verify the ERC-20 deployment guide using Hardhat from docs.polkadot.com"
source_url: "https://docs.polkadot.com/smart-contracts/cookbook/smart-contracts/deploy-erc20/erc20-hardhat/"
source_github: "https://github.com/polkadot-developers/polkadot-docs/blob/master/smart-contracts/cookbook/smart-contracts/deploy-erc20/erc20-hardhat.md"
---

# ERC-20 with Hardhat

[![ERC-20 with Hardhat](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-erc20-hardhat.yml/badge.svg?event=push)](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-erc20-hardhat.yml)

This project verifies the [ERC-20 with Hardhat](https://docs.polkadot.com/smart-contracts/cookbook/smart-contracts/deploy-erc20/erc20-hardhat/) guide from docs.polkadot.com.

The tests run against [revm-hardhat-examples](https://github.com/polkadot-developers/revm-hardhat-examples) at a pinned commit, deploying and testing a standard OpenZeppelin ERC-20 token on Polkadot Hub TestNet.

## What This Tests

1. **Environment Prerequisites** - Verifies Node.js v22+, npm, and git are available
2. **Clone Repository** - Clones `revm-hardhat-examples` at the pinned commit and verifies expected files
3. **Install Dependencies** - Runs `npm install` and confirms Hardhat is available
4. **Configure Testnet Credentials** - Sets `TESTNET_URL` and `TESTNET_PRIVATE_KEY` as Hardhat configuration variables
5. **Compile Contracts** - Runs `npx hardhat compile` and verifies the `MyToken.json` artifact (ABI + bytecode)
6. **Run Hardhat Tests** - Runs the full Hardhat/Mocha test suite against `polkadotTestnet` and confirms all 6 tests pass
7. **Deploy via Ignition** - Deploys `MyToken` using Hardhat Ignition to `polkadotTestnet` and verifies a contract address is returned

## Prerequisites

- Node.js v22.13.1 or later
- npm
- git
- A funded account on Polkadot Hub TestNet (get tokens from the [Polkadot Faucet](https://faucet.polkadot.io/))

## Running Tests Locally

```bash
# 1. Export testnet credentials
export TESTNET_URL="<your-rpc-endpoint>"
export TESTNET_PRIVATE_KEY="<your-private-key>"

# 2. Install wrapper dependencies
npm install

# 3. Run all verification tests
npm test
```

## Environment Variables

| Variable | Description |
|---|---|
| `TESTNET_URL` | RPC endpoint for Polkadot Hub TestNet |
| `TESTNET_PRIVATE_KEY` | Private key of a funded account (no `0x` prefix) |

## Test Phases

### 1. Environment Prerequisites
Verifies that Node.js v22+, npm, and git are present on the system.

### 2. Clone Repository
Clones `polkadot-developers/revm-hardhat-examples` and checks out the pinned commit SHA for reproducibility. Verifies the expected project structure exists.

### 3. Install Dependencies
Runs `npm install` inside `erc20-hardhat/` and confirms the Hardhat binary is available.

### 4. Configure Testnet Credentials
Reads `TESTNET_URL` and `TESTNET_PRIVATE_KEY` from environment variables and stores them as Hardhat configuration variables using `npx hardhat vars set`.

### 5. Compile Contracts
Runs `npx hardhat compile` and verifies that:
- The `artifacts/` directory is created
- `artifacts/contracts/MyToken.sol/MyToken.json` exists
- The artifact contains a valid ABI and non-empty bytecode

### 6. Run Hardhat Tests
Runs `npx hardhat test --network polkadotTestnet`. The Mocha suite covers:
- Token name and symbol
- Owner assignment
- Zero initial supply
- Minting by owner
- Total supply increase on mint
- Balance tracking after multiple mints

All 6 tests must pass.

### 7. Deploy via Ignition
Runs `npx hardhat ignition deploy ./ignition/modules/MyToken.ts --network polkadotTestnet --reset` and verifies the output contains a valid EVM contract address.

## Exact Replication Steps

```bash
# 1. Clone the example repo
git clone https://github.com/polkadot-developers/revm-hardhat-examples
cd revm-hardhat-examples/erc20-hardhat

# 2. Install dependencies
npm i

# 3. Set testnet credentials
npx hardhat vars set TESTNET_URL
npx hardhat vars set TESTNET_PRIVATE_KEY

# 4. Compile
npx hardhat compile

# 5. Run tests against Polkadot TestNet
npx hardhat test --network polkadotTestnet

# 6. Deploy
npx hardhat ignition deploy ./ignition/modules/MyToken.ts --network polkadotTestnet
```

## Versions Tested

| Component | Version |
|---|---|
| Node.js | v22.13.1+ |
| Hardhat | ^2.22.16 |
| Solidity | 0.8.28 |
| OpenZeppelin Contracts | ^5.4.0 |

## Source

- [docs.polkadot.com guide](https://docs.polkadot.com/smart-contracts/cookbook/smart-contracts/deploy-erc20/erc20-hardhat/)
- [revm-hardhat-examples repository](https://github.com/polkadot-developers/revm-hardhat-examples)
