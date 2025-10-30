# Simple Counter

A simple counter smart contract using pallet-revive

## Overview

This recipe demonstrates Solidity smart contract development using:
- **pallet-revive** - EVM-compatible smart contracts on Polkadot
- **Hardhat** - Development environment for Solidity
- **Ethers.js** - Library for interacting with contracts

## Prerequisites

- Node.js 20+
- Basic understanding of Solidity
- Familiarity with Ethereum development tools

## Setup

Install dependencies:

```bash
npm install
```

## Compile Contracts

```bash
npm run compile
```

This compiles your Solidity contracts and generates TypeScript types.

## Running Tests

```bash
npm test
```

## Deploy

Deploy to a local node:

```bash
npm run deploy:local
```

Deploy to a testnet:

```bash
npm run deploy:testnet
```

## Contract Structure

The recipe includes:

- `contracts/` - Solidity smart contracts
- `scripts/deploy.ts` - Deployment scripts
- `test/` - Contract tests

## What You'll Learn

- How to write Solidity contracts for pallet-revive
- How to compile and deploy contracts
- How to write tests for your contracts
- How to interact with deployed contracts

## Next Steps

- Add more complex contract logic
- Implement upgradeable contracts
- Add events and indexed parameters
- Optimize gas usage

## Resources

- [pallet-revive Documentation](https://paritytech.github.io/polkadot-sdk/master/pallet_revive/index.html)
- [Solidity Documentation](https://docs.soliditylang.org/)
- [Hardhat Documentation](https://hardhat.org/docs)
- [Ethers.js Documentation](https://docs.ethers.org/)
