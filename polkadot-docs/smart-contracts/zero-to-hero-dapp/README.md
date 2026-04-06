---
source_url: https://docs.polkadot.com/smart-contracts/cookbook/dapps/zero-to-hero/
source_repo: https://github.com/polkadot-developers/revm-hardhat-examples
pinned_commit: "a871364c8f4da052855b5c8ee4ed6b89fd182cb1"
---

# Zero to Hero Smart Contract DApp

[![Zero to Hero DApp](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-zero-to-hero-dapp.yml/badge.svg?event=push)](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-zero-to-hero-dapp.yml)

## What This Tests

Verifies every reproducible step of the **[Zero to Hero Smart Contract DApp](https://docs.polkadot.com/smart-contracts/cookbook/dapps/zero-to-hero/)** guide by cloning the upstream example repository and running through the complete workflow:

1. **Environment Prerequisites** — Node.js v22+, npm, git
2. **Clone Repository** — clone `revm-hardhat-examples` at a pinned commit and verify directory structure
3. **Install & Compile Smart Contract** — install dependencies, compile `Storage.sol`, validate ABI
4. **Install & Build DApp** — install Next.js dependencies, verify Storage ABI, build the frontend

## Prerequisites

| Tool | Version |
|------|---------|
| Node.js | v22+ |
| npm | latest |
| git | latest |

## Running Tests Locally

```bash
cd polkadot-docs/smart-contracts/zero-to-hero-dapp
npm ci
npm test
```

## Test Phases

### Phase 1 — Environment Prerequisites
Ensures Node.js v22+, npm, and git are available.

### Phase 2 — Clone Repository
Clones `polkadot-developers/revm-hardhat-examples` at the pinned commit and verifies the `zero-to-hero-dapp/` directory structure: `storage-contract/` with `hardhat.config.ts`, `Storage.sol`, and Ignition module; `dapp/` with the Next.js project.

### Phase 3 — Install & Compile Smart Contract
Runs `npm ci` inside `storage-contract/`, verifies Hardhat is available, compiles `Storage.sol`, and validates the artifact: ABI must expose `setNumber`, `getNumber`, and `NumberStored` event; bytecode must be non-empty.

### Phase 4 — Install & Build DApp
Runs `npm ci` inside `dapp/`, verifies `viem` and `next` are installed, checks the `Storage.json` ABI file exists in `dapp/abis/`, and runs `npm run build` to produce the Next.js production build.

## Exact Replication Steps

The test follows the guide's flow:
1. Clone the repository and navigate to `zero-to-hero-dapp/`
2. Install smart contract dependencies (`cd storage-contract && npm ci`)
3. Compile contracts (`npx hardhat compile`)
4. Install dapp dependencies (`cd dapp && npm ci`)
5. Build the frontend (`npm run build`)

## Versions Tested

| Dependency | Version |
|-----------|---------|
| Hardhat | 3.x (from upstream `package.json`) |
| Solidity | 0.8.28 |
| Next.js | 16.x (from upstream `package.json`) |
| Viem | ^2.38.5 |

## Source

- **Tutorial**: [docs.polkadot.com/smart-contracts/cookbook/dapps/zero-to-hero/](https://docs.polkadot.com/smart-contracts/cookbook/dapps/zero-to-hero/)
- **Example code**: [github.com/polkadot-developers/revm-hardhat-examples/tree/main/zero-to-hero-dapp](https://github.com/polkadot-developers/revm-hardhat-examples/tree/main/zero-to-hero-dapp)
