---
title: "Uniswap V3 Periphery with Hardhat"
description: "Verify the Uniswap V3 Periphery with Hardhat guide from docs.polkadot.com"
source_url: "https://docs.polkadot.com/smart-contracts/cookbook/eth-dapps/uniswap-v3/periphery-v3/"
source_repo: "https://github.com/polkadot-developers/polkadot-docs/blob/master/smart-contracts/cookbook/eth-dapps/uniswap-v3/periphery-v3.md"
---

# Uniswap V3 Periphery with Hardhat

[![Uniswap V3 Periphery with Hardhat](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-uniswap-v3-periphery-hardhat.yml/badge.svg?event=push)](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-uniswap-v3-periphery-hardhat.yml)

This project verifies the Uniswap V3 Periphery with Hardhat guide from docs.polkadot.com.

The tests run against [revm-hardhat-examples](https://github.com/polkadot-developers/revm-hardhat-examples) at a pinned commit, compiling and testing the Uniswap V3 Periphery contracts (SwapRouter and NonfungiblePositionManager) on Hardhat's local network and deploying all four contracts via Hardhat Ignition to Polkadot Hub TestNet.

## What This Tests

1. **Environment Prerequisites** - Verifies Node.js v22+, npm, and git are available
2. **Clone Repository** - Clones `revm-hardhat-examples` at the pinned commit and verifies the expected project structure (SwapRouter, NonfungiblePositionManager, Ignition module)
3. **Install Dependencies** - Installs `uniswap-v3-periphery-hardhat` dependencies (resolves the local `@uniswap/v3-core` file reference automatically) and confirms Hardhat is available
4. **Configure Testnet Credentials** - Verifies `TESTNET_PRIVATE_KEY` is available (skipped when not set)
5. **Compile Contracts** - Runs `npx hardhat compile` and verifies the `SwapRouter.json` and `NonfungiblePositionManager.json` artifacts (ABI + bytecode)
6. **Run Hardhat Tests** - Runs the full Hardhat test suite (39 tests: 14 SwapRouter + 25 NFPM) on the local Hardhat network
7. **Deploy via Ignition** - Deploys UniswapV3Factory, WETH9, SwapRouter, and NonfungiblePositionManager using Hardhat Ignition to `polkadotTestnet` and verifies contract addresses are returned

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
