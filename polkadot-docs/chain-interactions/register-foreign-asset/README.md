---
title: "Register a Foreign Asset on Polkadot Hub"
description: "Verify the Register a Foreign Asset guide from docs.polkadot.com"
source_url: "https://docs.polkadot.com/chain-interactions/token-operations/register-foreign-asset/"
source_github: "https://github.com/polkadot-developers/polkadot-docs/blob/master/chain-interactions/token-operations/register-foreign-asset.md"
---

# Register a Foreign Asset on Polkadot Hub

[![Register a Foreign Asset](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-register-foreign-asset.yml/badge.svg?event=push)](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-register-foreign-asset.yml)

This project verifies the [Register a Foreign Asset on Polkadot Hub](https://docs.polkadot.com/chain-interactions/token-operations/register-foreign-asset/) guide from docs.polkadot.com.

## What This Tests

Uses a local Chopsticks XCM environment (Polkadot relay chain + Polkadot Hub + Astar parachain) to simulate the foreign asset registration process.

1. **Setup** — Start Chopsticks XCM fork of Polkadot + Asset Hub + Astar
2. **Polkadot Hub — foreignAssets pallet** — Verify the pallet is available and the `create` extrinsic can be constructed with the correct Multilocation ID
3. **Astar — XCM call construction** — Verify the xcmPallet `send` extrinsic can be constructed to initiate the cross-chain asset registration
4. **Foreign asset registration verification** — Submit the XCM transaction and verify the foreign asset appears in the foreignAssets pallet storage on Asset Hub

## Running Tests

```bash
# Install Node.js dependencies
npm install

# Run all tests (Chopsticks XCM starts/stops automatically)
npm test
```
