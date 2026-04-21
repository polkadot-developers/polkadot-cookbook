---
title: "Register a Local Asset on Polkadot Hub"
description: "Verify the Register a Local Asset guide from docs.polkadot.com"
source_url: "https://docs.polkadot.com/chain-interactions/token-operations/register-local-asset/"
source_github: "https://github.com/polkadot-developers/polkadot-docs/blob/master/chain-interactions/token-operations/register-local-asset.md"
---

# Register a Local Asset on Polkadot Hub

[![Register a Local Asset](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-register-local-asset.yml/badge.svg?event=push)](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-register-local-asset.yml)

This project verifies the [Register a Local Asset on Polkadot Hub](https://docs.polkadot.com/chain-interactions/token-operations/register-local-asset/) guide from docs.polkadot.com.

The guide itself is a Polkadot.js Apps UI walkthrough. This harness replicates the same on-chain effect programmatically via the `assets` pallet extrinsics that the UI builds under the hood.

## What This Tests

Uses a local Chopsticks fork of Polkadot Asset Hub (polkadot-asset-hub preset) and funds Alice via `dev_setStorage` so she can cover the 10 DOT + ~0.201 DOT metadata deposit required by the guide.

1. **assets pallet** — verify `assets.create`, `assets.setMetadata`, `assets.setTeam`, and `assets.asset`/`metadata` storage are all available on the forked runtime
2. **Pick a unique asset ID** — enumerate existing assets and choose an ID guaranteed not to collide
3. **`assets.create`** — sign and submit with Alice as creator/admin, assert the extrinsic is `InBlock` with no `ExtrinsicFailed` event, and verify `assets.asset(id).isSome` → `Live`
4. **`assets.setMetadata`** — sign and submit `setMetadata(id, name, symbol, decimals)` and verify `assets.metadata(id)` matches the submitted values
5. **`assets.setTeam`** — sign and submit `setTeam(id, admin, issuer, freezer)` and verify each role on the asset details

## Running Tests

```bash
# Install Node.js dependencies
npm install

# Run all tests (Chopsticks starts/stops automatically)
npm test
```
