---
title: "Uniswap V2 Periphery with Hardhat"
description: "Verify the Uniswap V2 Periphery deployment guide using Hardhat from docs.polkadot.com"
source_url: "https://docs.polkadot.com/smart-contracts/cookbook/smart-contracts/deploy-uniswap-v2/uniswap-v2-periphery-hardhat/"
source_repo: "https://github.com/polkadot-developers/polkadot-docs/blob/master/smart-contracts/cookbook/smart-contracts/deploy-uniswap-v2/uniswap-v2-periphery-hardhat.md"
---

# Uniswap V2 Periphery with Hardhat

[![Uniswap V2 Periphery with Hardhat](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-uniswap-v2-periphery-hardhat.yml/badge.svg?event=push)](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-uniswap-v2-periphery-hardhat.yml)

This project verifies the Uniswap V2 Periphery with Hardhat guide from docs.polkadot.com.

The tests run against [revm-hardhat-examples](https://github.com/polkadot-developers/revm-hardhat-examples) at a pinned commit, compiling and testing the Uniswap V2 periphery contracts (Router01, Router02) on Hardhat's local network and deploying the Router via Hardhat Ignition to Polkadot Hub TestNet.

## What This Tests

1. **Environment Prerequisites** - Verifies Node.js v22+, npm, and git are available
2. **Clone Repository** - Clones `revm-hardhat-examples` at the pinned commit and verifies expected files (both core and periphery directories)
3. **Install Dependencies** - Installs `uniswap-v2-core-hardhat` dependencies first (local dependency), then periphery dependencies, and confirms Hardhat is available
4. **Configure Testnet Credentials** - Sets `TESTNET_URL` and `TESTNET_PRIVATE_KEY` as Hardhat configuration variables
5. **Compile Contracts** - Runs `npx hardhat compile` and verifies the `UniswapV2Router02.json` artifact (ABI + bytecode)
6. **Run Hardhat Tests** - Runs the full Hardhat/Mocha test suite (50 tests) on the local Hardhat network covering Router01 and Router02 functionality
7. **Deploy via Ignition** - Deploys WETH9, UniswapV2Factory, and UniswapV2Router02 using Hardhat Ignition to `polkadotTestnet` and verifies a contract address is returned

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
Clones `polkadot-developers/revm-hardhat-examples` and checks out the pinned commit SHA for reproducibility. Verifies that both `uniswap-v2-core-hardhat/` and `uniswap-v2-periphery-hardhat/` directories exist with the expected project structure (contracts, interfaces, libraries, test helpers, tests, Ignition module).

### 3. Install Dependencies
Installs dependencies in two steps:
1. `npm install` inside `uniswap-v2-core-hardhat/` — required because periphery references core as a local dependency (`@uniswap/v2-core: file:../uniswap-v2-core-hardhat`)
2. `npm install` inside `uniswap-v2-periphery-hardhat/`

Confirms the Hardhat binary is available.

### 4. Configure Testnet Credentials
Reads `TESTNET_URL` and `TESTNET_PRIVATE_KEY` from environment variables and stores them as Hardhat configuration variables.

### 5. Compile Contracts
Runs `npx hardhat compile` and verifies that:
- The `artifacts/` directory is created
- `artifacts/contracts/UniswapV2Router02.sol/UniswapV2Router02.json` exists
- The artifact contains a valid ABI and non-empty bytecode

### 6. Run Hardhat Tests
Runs `npx hardhat test` on the local Hardhat network. The Mocha suite covers:
- **UniswapV2Router01** (38 tests): addLiquidity, addLiquidityETH, removeLiquidity, removeLiquidityETH, removeLiquidityWithPermit, removeLiquidityETHWithPermit, swapExactTokensForTokens, swapTokensForExactTokens, swapExactETHForTokens, swapTokensForExactETH, swapExactTokensForETH, swapETHForExactTokens
- **UniswapV2Router02** (12 tests): quote, getAmountOut, getAmountIn, getAmountsOut, getAmountsIn, fee-on-transfer token support (removeLiquidity, swaps with DTT, WETH, ETH)

All 50 tests must pass.

### 7. Deploy via Ignition
Runs `npx hardhat ignition deploy ./ignition/modules/UniswapV2Router02.ts --network polkadotTestnet` which deploys WETH9, UniswapV2Factory, and UniswapV2Router02 and verifies the output contains a valid EVM contract address.

## Exact Replication Steps

```bash
# 1. Clone the example repo
git clone https://github.com/polkadot-developers/revm-hardhat-examples
cd revm-hardhat-examples

# 2. Install core dependencies (required by periphery)
cd uniswap-v2-core-hardhat && npm i && cd ..

# 3. Install periphery dependencies
cd uniswap-v2-periphery-hardhat && npm i

# 4. Set testnet credentials
npx hardhat vars set TESTNET_URL
npx hardhat vars set TESTNET_PRIVATE_KEY

# 5. Compile
npx hardhat compile

# 6. Run tests on local network
npx hardhat test

# 7. Deploy router to Polkadot TestNet
npx hardhat ignition deploy ./ignition/modules/UniswapV2Router02.ts --network polkadotTestnet
```

## Versions Tested

| Component | Version |
|---|---|
| Node.js | v22.13.1+ |
| Hardhat | ^2.22.16 |
| Solidity | 0.5.16 / 0.6.6 |

## Source

- [revm-hardhat-examples repository](https://github.com/polkadot-developers/revm-hardhat-examples)
