# Uniswap V2 Core - REVM Testing

[![REVM Uniswap V2 Core](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/migration-revm-uniswap-v2-core.yml/badge.svg)](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/migration-revm-uniswap-v2-core.yml)

This project tests Uniswap V2 Core smart contracts on Polkadot's REVM (Ethereum-compatible EVM).

## What This Tests

- Uniswap V2 ERC20 token functionality
- Uniswap V2 Factory contract
- Uniswap V2 Pair contract (liquidity pools)

## Prerequisites

- Node.js 18+

## Setup

```bash
# Clone the Uniswap V2 repository
npm run clone

# Or manually
./scripts/clone.sh
```

## Running Tests

```bash
# Run tests on REVM local network
npm test
```

## Source

- [uniswap-v2-polkadot](https://github.com/papermoonio/uniswap-v2-polkadot)
- [Original Uniswap V2 Core](https://github.com/Uniswap/v2-core)
