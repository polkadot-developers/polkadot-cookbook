---
title: "Contracts Precompile Example"
description: "Verification tests for the contracts-precompile-example recipe"
source_repo: "https://github.com/brunopgalvao/recipe-contracts-precompile-example"
---

# Contracts Precompile Example

[![Contracts Precompile Example](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/recipe-contracts-precompile-example.yml/badge.svg?event=push)](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/recipe-contracts-precompile-example.yml)

This folder contains verification tests for the [contracts-precompile-example](https://github.com/brunopgalvao/recipe-contracts-precompile-example) recipe.

## What This Test Verifies

1. **Prerequisites**: Node.js and git are available
2. **Clone**: The external recipe repo is cloned
3. **Install**: `npm ci` installs dependencies
4. **Compile**: `npx hardhat compile` compiles Solidity contracts
5. **Test**: `npx hardhat test --network localhost` passes all tests against Asset Hub via Zombienet + eth-rpc

## Environment

This recipe uses `setup-zombienet-eth-rpc` instead of `setup-revive-dev-node` because the contracts interact with precompiles (e.g., XCM) that are only registered in the Asset Hub runtime, not in the standalone dev node.

## Running Tests

```bash
npm ci
npm test
```

## Source Repository

- Recipe: [brunopgalvao/recipe-contracts-precompile-example](https://github.com/brunopgalvao/recipe-contracts-precompile-example)
