---
title: "Cross-Chain Transaction Example"
description: "Verification tests for the cross-chain-transaction-example recipe"
source_repo: "https://github.com/brunopgalvao/recipe-xcm-example"
last_tested: "2026-02-13"
---

# Cross-Chain Transaction Example

[![Cross-Chain Transaction Example](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/recipe-xcm-example.yml/badge.svg)](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/recipe-xcm-example.yml)

This folder contains verification tests for the [recipe-xcm-example](https://github.com/brunopgalvao/recipe-xcm-example) recipe.

## What This Test Verifies

1. **Prerequisites**: Node.js and git are available
2. **Clone**: The external recipe repo is cloned
3. **Install**: `npm ci` installs dependencies
4. **Chopsticks**: Starts Chopsticks for local multi-chain testing
5. **Test**: `npm test` passes all XCM tests
6. **Cleanup**: Chopsticks processes are stopped

## Running Tests

```bash
npm ci
npm test
```

## Source Repository

- Recipe: [brunopgalvao/recipe-xcm-example](https://github.com/brunopgalvao/recipe-xcm-example)
