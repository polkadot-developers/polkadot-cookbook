# Uniswap V2 Periphery - REVM Testing

[![REVM Uniswap V2 Periphery](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/migration-revm-uniswap-v2-periphery.yml/badge.svg)](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/migration-revm-uniswap-v2-periphery.yml)

This project tests Uniswap V2 Periphery smart contracts on Polkadot's REVM (Ethereum-compatible EVM).

## What This Tests

- UniswapV2Router01 and UniswapV2Router02 contracts
- UniswapV2Migrator contract
- Example contracts (FlashSwap, Oracle, SlidingWindowOracle, SwapToPrice, ComputeLiquidityValue)

## Prerequisites

- Node.js 22+

## Running Tests

```bash
npm ci
npm test
```

## Source

- [v2-periphery-polkadot](https://github.com/papermoonio/v2-periphery-polkadot)
- [Original Uniswap V2 Periphery](https://github.com/Uniswap/v2-periphery)
