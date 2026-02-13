---
title: "Contracts Example"
description: "Verification tests for the contracts-example recipe"
source_repo: "https://github.com/brunopgalvao/recipe-contracts-example"
last_tested: "2025-02-13"
---

# Contracts Example

[![Contracts Example](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/recipe-contracts-example.yml/badge.svg)](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/recipe-contracts-example.yml)

This folder contains verification tests for the [contracts-example](https://github.com/brunopgalvao/recipe-contracts-example) recipe.

## What This Test Verifies

1. **Prerequisites**: Node.js and git are available
2. **Clone**: The external recipe repo is cloned
3. **Install**: `npm ci` installs dependencies
4. **Compile**: `npx hardhat compile` compiles Solidity contracts
5. **Test**: `npx hardhat test` passes all tests

## Running Tests

```bash
npm ci
npm test
```

## Source Repository

- Recipe: [brunopgalvao/recipe-contracts-example](https://github.com/brunopgalvao/recipe-contracts-example)
