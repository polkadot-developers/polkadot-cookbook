---
title: "Uniswap V3 Core with Hardhat"
description: "Verify the Uniswap V3 Core with Hardhat guide from docs.polkadot.com"
source_url: "https://docs.polkadot.com/smart-contracts/cookbook/eth-dapps/uniswap-v3/core-v3/"
source_repo: "https://github.com/polkadot-developers/polkadot-docs/blob/master/smart-contracts/cookbook/eth-dapps/uniswap-v3/core-v3.md"
---

# Uniswap V3 Core with Hardhat

[![Uniswap V3 Core with Hardhat](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-uniswap-v3-core-hardhat.yml/badge.svg?event=push)](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-uniswap-v3-core-hardhat.yml)

This project verifies the Uniswap V3 Core with Hardhat guide from docs.polkadot.com.

The tests run against [revm-hardhat-examples](https://github.com/polkadot-developers/revm-hardhat-examples) at a pinned commit, compiling and testing the Uniswap V3 Core contracts (UniswapV3Factory, UniswapV3Pool) on Hardhat's local network and deploying the Factory via Hardhat Ignition to Polkadot Hub TestNet.

## What This Tests

1. **Environment Prerequisites** - Verifies Node.js v22+, npm, and git are available
2. **Clone Repository** - Clones `revm-hardhat-examples` at the pinned commit and verifies the expected project structure (contracts, libraries, test helpers, Ignition module)
3. **Install Dependencies** - Installs `uniswap-v3-core-hardhat` dependencies and confirms Hardhat is available
4. **Configure Testnet Credentials** - Verifies `TESTNET_PRIVATE_KEY` is available (skipped when not set)
5. **Compile Contracts** - Runs `npx hardhat compile` and verifies the `UniswapV3Factory.json` artifact (ABI + bytecode)
6. **Run Hardhat Tests** - Runs the full Hardhat/Mocha test suite (187 tests) on the local Hardhat network covering Factory and Pool functionality
7. **Deploy Factory via Ignition** - Deploys UniswapV3Factory using Hardhat Ignition to `polkadotTestnet` and verifies a contract address is returned

## Prerequisites

- Node.js v22.13.1 or later
- npm
- git
- A funded account on Polkadot Hub TestNet (get tokens from the [Polkadot Faucet](https://faucet.polkadot.io/))

## Running Tests Locally

```bash
# 1. Export testnet private key (RPC URL is hardcoded in hardhat.config.ts)
export TESTNET_PRIVATE_KEY="<your-private-key>"

# 2. Install wrapper dependencies
npm install

# 3. Run all verification tests
npm test
```

## Environment Variables

| Variable | Description |
|---|---|
| `TESTNET_PRIVATE_KEY` | Private key of a funded account (no `0x` prefix) |

## Test Phases

### 1. Environment Prerequisites
Verifies that Node.js v22+, npm, and git are present on the system.

### 2. Clone Repository
Clones `polkadot-developers/revm-hardhat-examples` and checks out the pinned commit SHA for reproducibility. Verifies the `uniswap-v3-core-hardhat/` directory exists with the expected project structure (contracts, libraries, test helpers, Ignition module).

### 3. Install Dependencies
Installs dependencies inside `uniswap-v3-core-hardhat/` and confirms the Hardhat binary is available.

### 4. Configure Testnet Credentials
Verifies `TESTNET_PRIVATE_KEY` is available (skipped when not set). The RPC URL is hardcoded in `hardhat.config.ts`.

### 5. Compile Contracts
Runs `npx hardhat compile` and verifies that:
- The `artifacts/` directory is created
- `artifacts/contracts/UniswapV3Factory.sol/UniswapV3Factory.json` exists
- The artifact contains a valid ABI and non-empty bytecode

### 6. Run Hardhat Tests
Runs `npx hardhat test` on the local Hardhat network. The Mocha suite covers:
- **UniswapV3Factory** (21 tests): pool creation, fee tier management, ownership controls
- **UniswapV3Pool** (166 tests): concentrated liquidity positions, swaps across tick boundaries, fee accumulation, flash loans, oracle observations, and edge cases

All 187 tests must pass.

### 7. Deploy Factory via Ignition
Runs `npx hardhat ignition deploy ./ignition/modules/UniswapV3Factory.ts --network polkadotTestnet` which deploys UniswapV3Factory and verifies the output contains a valid EVM contract address.

## Exact Replication Steps

```bash
# 1. Clone the example repo
git clone https://github.com/polkadot-developers/revm-hardhat-examples
cd revm-hardhat-examples
git checkout 3ff28ae44c4ab041a96953f49d0e2dae0408f28f

# 2. Install dependencies
cd uniswap-v3-core-hardhat && npm install

# 3. Set testnet private key
npx hardhat vars set TESTNET_PRIVATE_KEY

# 4. Compile
npx hardhat compile

# 5. Run tests on local network
npx hardhat test --network localNode

# 6. Deploy factory to Polkadot TestNet
npx hardhat ignition deploy ./ignition/modules/UniswapV3Factory.ts --network polkadotTestnet
```

## Versions Tested

| Component | Version |
|---|---|
| Node.js | v22.13.1+ |
| Hardhat | ^2.22.16 |
| Solidity | 0.7.6 |

## Source

- [revm-hardhat-examples repository](https://github.com/polkadot-developers/revm-hardhat-examples)
