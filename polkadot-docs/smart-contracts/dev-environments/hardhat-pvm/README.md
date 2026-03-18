---
title: "Use Hardhat with Polkadot Hub (PVM)"
description: "Verify the Hardhat PVM dev environment setup guide from docs.polkadot.com"
source_url: "https://docs.polkadot.com/smart-contracts/dev-environments/hardhat/#pvm"
source_repo: "https://github.com/polkadot-developers/polkadot-docs/blob/master/smart-contracts/dev-environments/hardhat.md"
---

# Use Hardhat with Polkadot Hub (PVM)

[![Use Hardhat with Polkadot Hub (PVM)](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-hardhat-pvm.yml/badge.svg?event=push)](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-hardhat-pvm.yml)

This project verifies the [Use Hardhat with Polkadot Hub](https://docs.polkadot.com/smart-contracts/dev-environments/hardhat/#pvm) guide from docs.polkadot.com — specifically the **PVM track**, which uses `@parity/hardhat-polkadot` and `@parity/resolc` to compile Solidity contracts to PolkaVM bytecode.

The tests scaffold a fresh PVM-enabled Hardhat project via `npx hardhat-polkadot init`, compile a `MyToken.sol` contract using the resolc compiler, and deploy to Polkadot Hub TestNet.

## What This Tests

1. **Environment Prerequisites** — Node.js v22.5+, npm v10.9.0+, and git are available
2. **Initialize PVM Project** — Installs `@parity/hardhat-polkadot@0.2.7` and `@parity/resolc@1.0.0`, writes the project files (`hardhat.config.ts`, `contracts/MyToken.sol`, `ignition/modules/MyToken.js`)
3. **Verify Project Structure** — Confirms all project files exist; validates config references the PVM plugin, `polkadotTestnet` network, chainId `420420417`, and resolc version
4. **Compile Contracts** — Runs `npx hardhat compile` using the resolc compiler (PVM bytecode), verifies artifacts with valid ABI and bytecode
5. **Verify Testnet Credentials** — `PRIVATE_KEY` environment variable is set
6. **Deploy via Ignition** — Deploys `MyToken` using Hardhat Ignition to `polkadotTestnet` and verifies a contract address is returned

## Prerequisites

- Node.js v22.5+ and npm v10.9.0+ (stricter than standard Hardhat — required by the `@parity/hardhat-polkadot` plugin)
- git
- A funded account on Polkadot Hub TestNet (get tokens from the [Polkadot Faucet](https://faucet.polkadot.io/))

> **Note**: `@nomicfoundation/hardhat-toolbox/network-helpers` is not fully compatible with PVM — `time.increase()` and `loadFixture` may not work as expected.

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
Verifies Node.js v22.5+, npm v10.9.0+, and git are present.

### 2. Initialize PVM Project
Creates a project directory, runs `npm init -y`, and installs `@parity/hardhat-polkadot@0.2.7` and `@parity/resolc@1.0.0`. Writes the project files that `npx hardhat-polkadot init` would generate (the init command is interactive and cannot run in CI), then `npm install` for remaining dependencies.

### 3. Verify Project Structure
Confirms the project contains:
- `hardhat.config.ts` — references `@parity/hardhat-polkadot` plugin, `polkadotTestnet` network, chainId `420420417`, and resolc version
- `contracts/MyToken.sol` — sample ERC-20 token contract
- `ignition/modules/MyToken.js` — Hardhat Ignition deployment module

### 4. Compile Contracts
Runs `npx hardhat compile` using the resolc compiler to produce PVM bytecode. Verifies:
- The `artifacts/` directory is created
- Artifact JSON contains a valid, non-empty ABI
- Artifact contains non-empty bytecode

### 5. Verify Testnet Credentials
Asserts `PRIVATE_KEY` is populated so subsequent steps fail with a clear message.

### 6. Deploy via Ignition
Runs `npx hardhat ignition deploy --module-path ./ignition/modules/MyToken.js --network polkadotTestnet` and verifies the output contains a valid EVM contract address. Retries up to 3 times on transient network errors. Soft-fails on infrastructure issues.

## Exact Replication Steps

```bash
# 1. Create and enter the project directory
mkdir hardhat-pvm-example
cd hardhat-pvm-example

# 2. Initialize and install PVM tooling
npm init -y
npm install --save-dev @parity/hardhat-polkadot@0.2.7
npm install --save-dev @parity/resolc@1.0.0

# 3. Scaffold the project
npx hardhat-polkadot init
echo '/ignition/deployments/' >> .gitignore
npm install

# 4. Compile with resolc (PVM bytecode)
npx hardhat compile

# 5. Set your private key
npx hardhat vars set PRIVATE_KEY

# 6. Deploy to Polkadot Hub TestNet
npx hardhat ignition deploy --module-path ./ignition/modules/MyToken.js --network polkadotTestnet
```

## Known Limitations

- **macOS binary permissions**: You may need to run `chmod +x` on resolc binaries, and `xattr -d com.apple.quarantine /path/to/binary` to remove macOS quarantine flags.
- **Local node deployment hangs**: Set `ignition.requiredConfirmations: 1` in `hardhat.config.ts` to prevent deployment hangs on local nodes.
- **Network helpers**: `time.increase()` and `loadFixture` from `@nomicfoundation/hardhat-toolbox/network-helpers` are not fully compatible with Polkadot nodes.

## Versions Tested

| Component | Version |
|---|---|
| Node.js | v22.5+ |
| npm | v10.9.0+ |
| @parity/hardhat-polkadot | 0.2.7 |
| @parity/resolc | 1.0.0 |
| Solidity | 0.8.28 |
| Polkadot Hub TestNet chain ID | 420420417 |

## Source

- [docs.polkadot.com guide (PVM section)](https://docs.polkadot.com/smart-contracts/dev-environments/hardhat/#pvm)
- [Polkadot Hub TestNet faucet](https://faucet.polkadot.io/)
