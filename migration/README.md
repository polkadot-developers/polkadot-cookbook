# Migration

This folder contains resources for testing Polkadot REVM and PVM Solidity smart contracts.

## Purpose

- Test smart contract migrations between different execution environments
- Validate Solidity contracts on Polkadot's REVM (Ethereum-compatible EVM)
- Test contracts compiled to PVM (PolkaVM) using Revive compiler
- Detect compatibility issues when polkadot-sdk revive is upgraded

## Test Status

| Test | Status | Description |
|------|--------|-------------|
| [Uniswap V2 Core](./revm/uniswap-v2-core/) | [![REVM Uniswap V2 Core](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/migration-revm-uniswap-v2-core.yml/badge.svg?event=push)](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/migration-revm-uniswap-v2-core.yml) | Uniswap V2 Core contracts on REVM |
| [Uniswap V2 Periphery](./revm/uniswap-v2-periphery/) | [![REVM Uniswap V2 Periphery](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/migration-revm-uniswap-v2-periphery.yml/badge.svg?event=push)](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/migration-revm-uniswap-v2-periphery.yml) | Uniswap V2 Periphery contracts on REVM |

## Structure

```
migration/
├── README.md
└── revm/
    ├── uniswap-v2-core/       # Uniswap V2 Core REVM testing
    └── uniswap-v2-periphery/  # Uniswap V2 Periphery REVM testing
```

## Getting Started

Each test folder contains its own README with setup instructions. Tests run automatically:
- On push/PR to the specific test folder
- Weekly on Sunday (to detect compatibility issues with new releases)
- Manually via workflow dispatch
