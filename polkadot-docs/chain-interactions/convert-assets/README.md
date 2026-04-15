---
title: "Convert Assets on Asset Hub"
description: "Verify the Convert Assets on Asset Hub guide from docs.polkadot.com"
source_url: "https://docs.polkadot.com/chain-interactions/token-operations/convert-assets/"
source_github: "https://github.com/polkadot-developers/polkadot-docs/blob/master/chain-interactions/token-operations/convert-assets.md"
---

# Convert Assets on Asset Hub

[![Convert Assets on Asset Hub](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-convert-assets.yml/badge.svg?event=push)](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-convert-assets.yml)

This project verifies the [Convert Assets on Asset Hub](https://docs.polkadot.com/chain-interactions/token-operations/convert-assets/) guide from docs.polkadot.com.

## What This Tests

Uses a local Chopsticks fork of Polkadot Asset Hub to exercise the Asset Conversion pallet operations described in the guide:

1. **Create a liquidity pool** — Call `assetConversion.createPool` to create a DOT/test-asset pool.
2. **Add liquidity** — Call `assetConversion.addLiquidity` to provide liquidity to the pool.
3. **Swap exact tokens for tokens** — Call `assetConversion.swapExactTokensForTokens` to exchange a precise input amount.
4. **Swap tokens for exact tokens** — Call `assetConversion.swapTokensForExactTokens` to acquire a precise output amount.
5. **Remove liquidity** — Call `assetConversion.removeLiquidity` to withdraw liquidity from the pool.

All operations use Alice's development keypair and run against a Chopsticks-forked Asset Hub instance.

## Running Tests

```bash
# Install Node.js dependencies
npm install

# Run all tests (Chopsticks starts/stops automatically)
npm test
```
