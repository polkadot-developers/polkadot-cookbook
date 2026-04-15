---
title: "Deploy an ERC-721 Using Hardhat"
description: "Verify the ERC-721 NFT deployment guide using Hardhat from docs.polkadot.com"
source_url: "https://docs.polkadot.com/smart-contracts/cookbook/smart-contracts/deploy-nft/nft-hardhat/"
source_repo: "https://github.com/polkadot-developers/polkadot-docs/blob/master/smart-contracts/cookbook/smart-contracts/deploy-nft/nft-hardhat.md"
pinned_commit: "6ac751e12c5901dd40f30b1a2610d83fca3c9575"
---

# Deploy an ERC-721 Using Hardhat

[![Deploy an ERC-721 Using Hardhat](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-nft-hardhat.yml/badge.svg?event=push)](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-nft-hardhat.yml)

This project verifies the [Deploy an ERC-721 Using Hardhat](https://docs.polkadot.com/smart-contracts/cookbook/smart-contracts/deploy-nft/nft-hardhat/) guide from docs.polkadot.com.

The tests follow the tutorial steps to create a Hardhat project from scratch, write an ERC-721 NFT contract using OpenZeppelin, compile it, and deploy to Polkadot Hub TestNet.

## What This Tests

1. **Environment Prerequisites** — Node.js v22+, npm are available
2. **Initialize Hardhat Project** — Runs `npx hardhat@^2.27.0 init` and installs OpenZeppelin contracts
3. **Write Contract and Config** — Creates `hardhat.config.ts`, `contracts/MyNFT.sol`, and `ignition/modules/MyNFT.ts` exactly as documented
4. **Compile Contracts** — Runs `npx hardhat compile` and verifies the `MyNFT.json` artifact (ABI + bytecode + function signatures)
5. **Deploy via Ignition** — Deploys `MyNFT` using Hardhat Ignition to `polkadotTestnet` and verifies a contract address is returned

## Prerequisites

- Node.js v22.13.1 or later
- npm
- A funded account on Polkadot Hub TestNet (get tokens from the [Polkadot Faucet](https://faucet.polkadot.io/))

## Running Tests Locally

```bash
# 1. Copy and populate the env file
cp .env.example .env
# Edit .env and set your PRIVATE_KEY

# 2. Install wrapper dependencies
npm install

# 3. Run all verification tests
npm test
```

## Environment Variables

| Variable | Description |
|---|---|
| `PRIVATE_KEY` | Private key of a funded account (no `0x` prefix) |

## Test Phases

### 1. Environment Prerequisites
Verifies Node.js v22+ and npm are present.

### 2. Initialize Hardhat Project
Creates a fresh Hardhat project using `npx hardhat@^2.27.0 init` and installs `@openzeppelin/contracts`.

### 3. Write Contract and Config
Writes the tutorial files:
- `hardhat.config.ts` — Polkadot TestNet network configuration with `@nomicfoundation/hardhat-toolbox-viem`
- `contracts/MyNFT.sol` — ERC-721 NFT contract using OpenZeppelin
- `ignition/modules/MyNFT.ts` — Hardhat Ignition deployment module

### 4. Compile Contracts
Runs `npx hardhat compile` and verifies:
- The `artifacts/` directory is created
- `artifacts/contracts/MyNFT.sol/MyNFT.json` exists
- The ABI contains `safeMint(address)` and inherits ERC-721 functions
- Bytecode is non-empty

### 5. Deploy via Ignition
Runs `npx hardhat ignition deploy ./ignition/modules/MyNFT.ts --network polkadotTestnet` and verifies the output contains a valid EVM contract address. Retries up to 3 times on transient network errors.

## Exact Replication Steps

```bash
# 1. Create and init project
mkdir hardhat-nft-deployment
cd hardhat-nft-deployment
npx hardhat@^2.27.0 init

# 2. Install OpenZeppelin
npm install @openzeppelin/contracts

# 3. Configure Hardhat (edit hardhat.config.ts)

# 4. Write contract (contracts/MyNFT.sol)

# 5. Set your private key
npx hardhat vars set PRIVATE_KEY

# 6. Compile
npx hardhat compile

# 7. Deploy to Polkadot TestNet
npx hardhat ignition deploy ignition/modules/MyNFT.ts --network polkadotTestnet
```

## Versions Tested

| Component | Version |
|---|---|
| Node.js | v22.13.1+ |
| Hardhat | ^2.27.0 |
| Solidity | 0.8.28 |
| OpenZeppelin Contracts | ^5.0.0 |

## Source

- [docs.polkadot.com guide](https://docs.polkadot.com/smart-contracts/cookbook/smart-contracts/deploy-nft/nft-hardhat/)
- [polkadot-docs source](https://github.com/polkadot-developers/polkadot-docs/blob/master/smart-contracts/cookbook/smart-contracts/deploy-nft/nft-hardhat.md)
