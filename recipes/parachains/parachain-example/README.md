---
title: "Parachain Example"
description: "Verification tests for the parachain-example recipe"
source_repo: "https://github.com/brunopgalvao/recipe-parachain-example"
last_tested: "2026-02-13"
---

# Parachain Example

[![Parachain Example](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/recipe-parachain-example.yml/badge.svg?event=push)](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/recipe-parachain-example.yml)

This folder contains verification tests for the [parachain-example](https://github.com/brunopgalvao/recipe-parachain-example) recipe.

## What This Test Verifies

1. **Prerequisites**: Rust, cargo, Node.js, and git are available
2. **Clone**: The external recipe repo is cloned
3. **Build**: `cargo build --release` builds the parachain binary
4. **Install**: `npm ci` installs Node.js dependencies
5. **Start Node**: The parachain node is started in development mode
6. **Test**: `npm test` passes all PAPI integration tests
7. **Cleanup**: The node process is stopped

## Running Tests

```bash
npm ci
npm test
```

## Test Duration

This test suite takes approximately 30-45 minutes due to the Rust compilation step.

## Source Repository

- Recipe: [brunopgalvao/recipe-parachain-example](https://github.com/brunopgalvao/recipe-parachain-example)
