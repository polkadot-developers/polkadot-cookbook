---
title: "Use Hardhat with Polkadot Hub (EVM)"
description: "Verify the Hardhat EVM dev environment setup guide from docs.polkadot.com"
source_url: "https://docs.polkadot.com/smart-contracts/dev-environments/hardhat/"
source_repo: "https://github.com/polkadot-developers/polkadot-docs/blob/master/smart-contracts/dev-environments/hardhat.md"
---

# Use Hardhat with Polkadot Hub (EVM)

[![Use Hardhat with Polkadot Hub (EVM)](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-hardhat-evm.yml/badge.svg?event=push)](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-hardhat-evm.yml)

This project verifies the [Use Hardhat with Polkadot Hub](https://docs.polkadot.com/smart-contracts/dev-environments/hardhat/) guide from docs.polkadot.com — specifically the **EVM track**, which configures standard Hardhat for Polkadot Hub's REVM-powered EVM environment.

The tests scaffold a fresh Hardhat project from scratch (mirroring the tutorial's `npx hardhat init` flow), inject the Polkadot Hub network configuration, compile a `Lock.sol` contract, and deploy to Polkadot Hub TestNet.

## What This Tests

1. **Environment Prerequisites** — Node.js v18+, npm, and git are available
2. **Initialize Hardhat Project** — Installs `hardhat@^2.27.0` and `@nomicfoundation/hardhat-toolbox`, verifies version ≥ 2.27.0
3. **Configure Polkadot Hub Network** — Writes `hardhat.config.ts` with the `polkadotTestnet` network block (RPC URL, chainId `420420417`, `vars.get("PRIVATE_KEY")`)
4. **Verify Testnet Credentials** — `PRIVATE_KEY` environment variable is set
5. **Compile Contracts** — Runs `npx hardhat compile` and verifies the `Lock.json` artifact (ABI + bytecode + function signatures)
6. **Deploy via Ignition** — Deploys `Lock` using Hardhat Ignition to `polkadotTestnet` and verifies a contract address is returned

## Prerequisites

- Node.js v18.x, v20.x, or v22.x (LTS)
- npm
- git
- A funded account on Polkadot Hub TestNet (get tokens from the [Polkadot Faucet](https://faucet.polkadot.io/))

> **Note**: Some Hardhat network helpers (`time.increase()`, `loadFixture`) are not fully compatible with Polkadot nodes. They work fine against Hardhat's in-process network but may behave differently when targeting `polkadotTestnet`.

## Running Tests Locally

```bash
# 1. Copy and populate the env file
cp .env.example .env
# Edit .env and set your PRIVATE_KEY (no 0x prefix)

# 2. Install wrapper dependencies
npm install

# 3. Run all verification tests
npm test
```

## Environment Variables

| Variable | Description |
|---|---|
| `PRIVATE_KEY` | Private key of a funded Polkadot Hub TestNet account (no `0x` prefix) |

In CI, Hardhat resolves `vars.get("PRIVATE_KEY")` via the `HARDHAT_VAR_PRIVATE_KEY` environment variable — no interactive `npx hardhat vars set` call is needed.

## Test Phases

### 1. Environment Prerequisites
Verifies Node.js v18+ (LTS), npm, and git are present.

### 2. Initialize Hardhat Project
Creates a project directory, runs `npm init -y`, and installs `hardhat@^2.27.0` and `@nomicfoundation/hardhat-toolbox`. Confirms the Hardhat binary reports version ≥ 2.27.0.

### 3. Configure Polkadot Hub Network
Writes the following files into the scaffolded project:
- `hardhat.config.ts` — includes the `polkadotTestnet` network block with RPC URL `https://services.polkadothub-rpc.com/testnet` and `chainId: 420420417`
- `tsconfig.json` — TypeScript compiler config required for Hardhat TS support
- `contracts/Lock.sol` — Hardhat's standard sample contract
- `ignition/modules/Lock.ts` — Hardhat Ignition deployment module

Verifies the config contains the correct RPC URL, chain ID, and `vars.get("PRIVATE_KEY")` reference.

### 4. Verify Testnet Credentials
Asserts `PRIVATE_KEY` is populated so subsequent steps fail with a clear message instead of a cryptic Hardhat error.

### 5. Compile Contracts
Runs `npx hardhat compile` and verifies:
- The `artifacts/` directory is created
- `artifacts/contracts/Lock.sol/Lock.json` exists
- The ABI is valid and non-empty
- The ABI contains both `withdraw()` and `unlockTime()` entries
- Bytecode is non-empty

### 6. Deploy via Ignition
Runs `npx hardhat ignition deploy ./ignition/modules/Lock.ts --network polkadotTestnet` and verifies the output contains a valid EVM contract address. Retries up to 3 times on transient network errors. Soft-fails on infrastructure issues (dry faucet, unreachable RPC) so that phases 1–5 are not blocked.

## Exact Replication Steps

```bash
# 1. Create and enter the project directory
mkdir hardhat-example
cd hardhat-example

# 2. Install Hardhat and toolbox
npm init -y
npm install --save-dev hardhat@^2.27.0 @nomicfoundation/hardhat-toolbox

# 3. Configure hardhat.config.ts with the polkadotTestnet network block
#    (see the tutorial for the full config)

# 4. Set your private key
npx hardhat vars set PRIVATE_KEY

# 5. Compile
npx hardhat compile

# 6. Deploy to Polkadot Hub TestNet
npx hardhat ignition deploy ignition/modules/Lock.ts --network polkadotTestnet
```

## Versions Tested

| Component | Version |
|---|---|
| Node.js | v18.x / v20.x / v22.x (LTS) |
| Hardhat | ^2.27.0 |
| Solidity | 0.8.28 |
| Polkadot Hub TestNet chain ID | 420420417 |

## Source

- [docs.polkadot.com guide](https://docs.polkadot.com/smart-contracts/dev-environments/hardhat/)
- [Polkadot Hub TestNet faucet](https://faucet.polkadot.io/)
