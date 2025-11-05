# Simple Counter

A simple counter smart contract in Solidity

## Overview

This recipe demonstrates Solidity smart contract development using:
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

- How to write Solidity contracts
- How to compile and deploy contracts
- How to write tests for your contracts
- How to interact with deployed contracts

## Next Steps

- Add more complex contract logic
- Implement upgradeable contracts
- Add events and indexed parameters
- Optimize gas usage

## Resources

- [Solidity Documentation](https://docs.soliditylang.org/)
- [Polkadot SDK Documentation](https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/index.html)
- [Ethers.js Documentation](https://docs.ethers.org/)
