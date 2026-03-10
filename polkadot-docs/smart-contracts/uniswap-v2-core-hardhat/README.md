---
title: "Uniswap V2 Core with Hardhat"
description: "Verify the Uniswap V2 Core deployment guide using Hardhat from docs.polkadot.com"
source_url: "https://docs.polkadot.com/smart-contracts/cookbook/smart-contracts/deploy-uniswap-v2/uniswap-v2-core-hardhat/"
source_repo: "https://github.com/polkadot-developers/polkadot-docs/blob/master/smart-contracts/cookbook/smart-contracts/deploy-uniswap-v2/uniswap-v2-core-hardhat.md"
---

# Uniswap V2 Core with Hardhat

[![Uniswap V2 Core with Hardhat](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-uniswap-v2-core-hardhat.yml/badge.svg?event=push)](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-uniswap-v2-core-hardhat.yml)

This project verifies the Uniswap V2 Core with Hardhat guide from docs.polkadot.com.

The tests run against [revm-hardhat-examples](https://github.com/polkadot-developers/revm-hardhat-examples) at a pinned commit, compiling and testing the Uniswap V2 core contracts (Factory, Pair, ERC20) on Hardhat's local network and deploying the Factory to Polkadot Hub TestNet.

## What This Tests

1. **Environment Prerequisites** - Verifies Node.js v22+, npm, and git are available
2. **Clone Repository** - Clones `revm-hardhat-examples` at the pinned commit and verifies expected files
3. **Install Dependencies** - Runs `npm install` and confirms Hardhat is available
4. **Configure Testnet Credentials** - Sets `TESTNET_URL` and `TESTNET_PRIVATE_KEY` as Hardhat configuration variables
5. **Compile Contracts** - Runs `npx hardhat compile` and verifies the `UniswapV2Factory.json` artifact (ABI + bytecode)
6. **Run Hardhat Tests** - Runs the full Hardhat/Mocha test suite (29 tests) on the local Hardhat network covering ERC20, Factory, and Pair functionality
7. **Deploy via Ignition** - Deploys `UniswapV2Factory` using Hardhat Ignition to `polkadotTestnet` and verifies a contract address is returned

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
Clones `polkadot-developers/revm-hardhat-examples` and checks out the pinned commit SHA for reproducibility. Verifies the expected project structure exists (contracts, interfaces, libraries, tests, Ignition module).

### 3. Install Dependencies
Runs `npm install` inside `uniswap-v2-core-hardhat/` and confirms the Hardhat binary is available.

### 4. Configure Testnet Credentials
Reads `TESTNET_URL` and `TESTNET_PRIVATE_KEY` from environment variables and stores them as Hardhat configuration variables.

### 5. Compile Contracts
Runs `npx hardhat compile` and verifies that:
- The `artifacts/` directory is created
- `artifacts/contracts/UniswapV2Factory.sol/UniswapV2Factory.json` exists
- The artifact contains a valid ABI and non-empty bytecode

### 6. Run Hardhat Tests
Runs `npx hardhat test` on the local Hardhat network. The Mocha suite covers:
- **UniswapV2ERC20** (7 tests): name/symbol/decimals, approve, transfer, transfer:fail, transferFrom, transferFrom:max, permit
- **UniswapV2Factory** (5 tests): feeTo/feeToSetter/allPairsLength, createPair, createPair:reverse, setFeeTo, setFeeToSetter
- **UniswapV2Pair** (17 tests): mint, getInputPrice (7 cases), optimistic (4 cases), swap:token0, swap:token1, burn, feeTo:off, feeTo:on

All 29 tests must pass.

### 7. Deploy via Ignition
Runs `npx hardhat ignition deploy ./ignition/modules/UniswapV2Factory.ts --network polkadotTestnet` and verifies the output contains a valid EVM contract address.

## Exact Replication Steps

```bash
# 1. Clone the example repo
git clone https://github.com/polkadot-developers/revm-hardhat-examples
cd revm-hardhat-examples/uniswap-v2-core-hardhat

# 2. Install dependencies
npm i

# 3. Set testnet credentials
npx hardhat vars set TESTNET_URL
npx hardhat vars set TESTNET_PRIVATE_KEY

# 4. Compile
npx hardhat compile

# 5. Run tests on local network
npx hardhat test

# 6. Deploy factory to Polkadot TestNet
npx hardhat ignition deploy ./ignition/modules/UniswapV2Factory.ts --network polkadotTestnet
```

## Versions Tested

| Component | Version |
|---|---|
| Node.js | v22.13.1+ |
| Hardhat | ^2.22.16 |
| Solidity | ^0.8.19 (compiled with 0.8.28) |

## Source

- [revm-hardhat-examples repository](https://github.com/polkadot-developers/revm-hardhat-examples)
