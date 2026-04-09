---
title: "Use Foundry with Polkadot Hub"
description: "Verify the Use Foundry with Polkadot Hub guide from docs.polkadot.com"
source_url: "https://docs.polkadot.com/smart-contracts/dev-environments/foundry/"
source_repo: "https://github.com/polkadot-developers/polkadot-docs/blob/master/smart-contracts/dev-environments/foundry.md"
---

# Use Foundry with Polkadot Hub

[![Use Foundry with Polkadot Hub](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-foundry.yml/badge.svg?event=push)](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-foundry.yml)

This project verifies the [Use Foundry with Polkadot Hub](https://docs.polkadot.com/smart-contracts/dev-environments/foundry/) guide from docs.polkadot.com.

The tests scaffold a fresh Foundry project from scratch (mirroring the tutorial's `forge init` flow), compile the default `Counter.sol`, run unit tests against a local Anvil instance, configure the project for Polkadot Hub TestNet, and attempt a live deployment.

## What This Tests

1. **Environment Prerequisites** — `forge`, `cast`, `anvil`, and `git` are available
2. **Initialize Foundry Project** — Runs `forge init my-foundry-project`, verifies `src/`, `script/`, `test/`, `lib/`, and `foundry.toml` are created
3. **Compile Contracts** — Runs `forge build` and verifies the `Counter.json` artifact (ABI + bytecode + function signatures)
4. **Configure for Polkadot Hub** — Writes `foundry.toml` with the `[etherscan]` block for `polkadot-testnet` (Blockscout verifier), adds the deployment script `Counter.s.sol`
5. **Run Unit Tests** — Runs `forge test`, `forge test -vvv`, and `forge test --match-test test_Increment` against a local Anvil instance (no network needed)
6. **Deploy to TestNet** — Deploys `Counter` using `forge create` to `polkadot-testnet` (soft-fails when `PRIVATE_KEY` is absent or network is unreachable)

## Prerequisites

- Foundry nightly (`foundryup --version nightly`)
- Git
- A funded account on Polkadot Hub TestNet for deployment (get tokens from the [Polkadot Faucet](https://faucet.polkadot.io/))

> **Note**: Foundry's nightly build is required for Polkadot Hub support (`--chain polkadot-testnet`). The stable release does not include the Polkadot chain definitions.

## Running Tests Locally

```bash
# 1. Install Foundry nightly (if not already installed)
curl -L https://foundry.paradigm.xyz | bash
foundryup --version nightly

# 2. Install wrapper dependencies
cd polkadot-docs/smart-contracts/dev-environments/foundry
npm install

# 3. (Optional) Set PRIVATE_KEY to run the deployment phase
export PRIVATE_KEY=your_private_key_here

# 4. Run all verification tests
npm test
```

## Environment Variables

| Variable | Description |
|---|---|
| `PRIVATE_KEY` | Private key of a funded Polkadot Hub TestNet account (with `0x` prefix) |

Phase 6 (deployment) is skipped when `PRIVATE_KEY` is not set. Phases 1–5 run without any network access or credentials.

## Test Phases

### 1. Environment Prerequisites
Verifies `forge`, `cast`, `anvil`, and `git` are available on `PATH`.

### 2. Initialize Foundry Project
Runs `forge init my-foundry-project` in a `.test-workspace/` directory. Verifies the project structure: `src/`, `script/`, `test/`, `lib/`, `foundry.toml`, and `src/Counter.sol`.

### 3. Compile Contracts
Runs `forge build` and verifies:
- `out/` directory is created
- `out/Counter.sol/Counter.json` exists
- ABI is valid and non-empty
- ABI contains both `setNumber()` and `increment()` functions
- Bytecode is non-empty

### 4. Configure for Polkadot Hub
Writes `foundry.toml` with the Polkadot Hub TestNet `[etherscan]` configuration (Blockscout verifier URL, `polkadot-testnet` chain, `solc_version = "0.8.28"`). Adds `script/Counter.s.sol` deployment script. Recompiles to verify no regressions.

### 5. Run Forge Unit Tests
Runs the default Counter tests against a local Anvil instance:
- `forge test` — all tests pass
- `forge test -vvv` — verbose output
- `forge test --match-test test_Increment` — targeted test execution

### 6. Deploy to Polkadot Hub TestNet
Runs `forge create src/Counter.sol:Counter --chain polkadot-testnet --broadcast` and verifies the output contains a valid EVM contract address. Retries up to 3 times on transient network errors. Soft-fails on infrastructure issues (dry faucet, unreachable RPC) so phases 1–5 are not blocked.

## Versions Tested

| Component | Version |
|---|---|
| Foundry | nightly (1.6.0-nightly at time of writing) |
| forge-std | v1.15.0 |
| Solidity | 0.8.28 (configured) / 0.8.30 (auto-selected by forge) |
| Polkadot Hub TestNet chain ID | 420420417 |

## Source

- [docs.polkadot.com guide](https://docs.polkadot.com/smart-contracts/dev-environments/foundry/)
- [Polkadot Hub TestNet faucet](https://faucet.polkadot.io/)
