---
title: "Pay Transaction Fees with Different Tokens"
description: "Verify the Pay Fees with Different Tokens guide from docs.polkadot.com"
source_url: "https://docs.polkadot.com/chain-interactions/send-transactions/pay-fees-with-different-tokens/"
source_github: "https://github.com/polkadot-developers/polkadot-docs/blob/master/chain-interactions/send-transactions/pay-fees-with-different-tokens.md"
---

# Pay Transaction Fees with Different Tokens

[![Pay Fees with Different Tokens](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-pay-fees-different-tokens.yml/badge.svg?event=push)](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-pay-fees-different-tokens.yml)

This project verifies the [Pay Fees with Different Tokens](https://docs.polkadot.com/chain-interactions/send-transactions/pay-fees-with-different-tokens/) guide from docs.polkadot.com.

## What This Tests

Uses a local Chopsticks fork of Polkadot Hub to send a DOT transfer while paying fees in USDT.

1. **PAPI (Polkadot API)** — Connect to Chopsticks fork, sign with Alice via hdkd, submit `transfer_keep_alive` with USDT fee asset
2. **Polkadot.js API** — Connect to Chopsticks fork, sign with Alice keyring, submit `transferKeepAlive` with USDT `assetId`
3. **Subxt (Rust)** — Connect to Chopsticks fork, sign with Alice dev keypair, submit transfer with `tip_of(0, asset_location)`

## Running Tests

```bash
# Install Node.js dependencies
npm install

# Generate PAPI descriptors (requires Chopsticks running)
npx @acala-network/chopsticks -c polkadot-asset-hub &
sleep 10
npx papi add assetHub -n polkadot_asset_hub

# Download Subxt metadata and build
cd tests/subxt-pay-fees
subxt metadata --url ws://localhost:8000 -o metadata/asset_hub.scale
cargo build
cd ../..

# Run all tests (Chopsticks starts/stops automatically)
npm test
```
