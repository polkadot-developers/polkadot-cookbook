---
title: "Transaction Example"
description: "Verification tests for the transaction-example recipe"
source_repo: "https://github.com/brunopgalvao/recipe-transaction-example"
last_tested: "2026-02-13"
---

# Transaction Example

[![Transaction Example](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/recipe-transaction-example.yml/badge.svg)](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/recipe-transaction-example.yml)

This folder contains verification tests for the [transaction-example](https://github.com/brunopgalvao/recipe-transaction-example) recipe.

## What This Test Verifies

1. **Prerequisites**: Node.js and git are available
2. **Clone**: The external recipe repo is cloned
3. **Install**: `npm ci` installs dependencies
4. **Test**: `npm test` passes all tests

## Running Tests

```bash
npm ci
npm test
```

## Source Repository

- Recipe: [brunopgalvao/recipe-transaction-example](https://github.com/brunopgalvao/recipe-transaction-example)
