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

Uses a local Chopsticks fork of Polkadot Asset Hub (single-chain) to simulate the foreign asset registration process via `dev_setStorage` state injection — mirroring the on-chain outcome that an XCM call from the source parachain would produce.

1. **Setup** — Start Chopsticks fork of Polkadot Asset Hub; fund the Astar sovereign account via `dev_setStorage`
2. **foreignAssets pallet** — Verify the pallet and all required extrinsics (`create`, `setMetadata`, `mint`, `freeze`) are available; enumerate existing foreign assets on the forked chain
3. **Extrinsic construction** — Build a `foreignAssets.create` call for the Astar Multilocation (`{ parents: 1, interior: { X1: [{ Parachain: 2006 }] } }`) and verify its encoded call index against the live runtime metadata
4. **State injection** — Inject the foreign asset entry via `dev_setStorage`, advance the chain by one block, then query and verify the registered asset's status, admin, and min balance
5. **Post-registration verification** — Confirm the total foreign asset count increased and the registered Multilocation is queryable

## Running Tests

```bash
# Install Node.js dependencies
npm install

# Run all tests (Chopsticks XCM starts/stops automatically)
npm test
```
