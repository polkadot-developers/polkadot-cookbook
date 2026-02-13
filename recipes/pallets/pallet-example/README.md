---
title: "Pallet Example"
description: "Verification tests for the pallet-example recipe"
source_repo: "https://github.com/brunopgalvao/recipe-pallet-example"
last_tested: "2025-02-13"
---

# Pallet Example

[![Pallet Example](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/recipe-pallet-example.yml/badge.svg)](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/recipe-pallet-example.yml)

This folder contains verification tests for the [pallet-example](https://github.com/brunopgalvao/recipe-pallet-example) recipe.

## What This Test Verifies

1. **Prerequisites**: Rust, cargo, and git are available
2. **Clone**: The external recipe repo is cloned
3. **Test**: `cargo test --all-features --locked` passes all tests

## Running Tests

```bash
npm ci
npm test
```

## Source Repository

- Recipe: [brunopgalvao/recipe-pallet-example](https://github.com/brunopgalvao/recipe-pallet-example)
