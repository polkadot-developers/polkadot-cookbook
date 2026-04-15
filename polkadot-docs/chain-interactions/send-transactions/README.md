---
title: "Send Transactions with SDKs"
description: "Verify the Send Transactions with SDKs guide from docs.polkadot.com"
source_url: "https://docs.polkadot.com/chain-interactions/send-transactions/with-sdks/"
source_github: "https://github.com/polkadot-developers/polkadot-docs/blob/master/chain-interactions/send-transactions/with-sdks.md"
---

# Send Transactions with SDKs

[![Send Transactions with SDKs](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-send-transactions.yml/badge.svg?event=push)](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-send-transactions.yml)

This project verifies the [Send Transactions with SDKs](https://docs.polkadot.com/chain-interactions/send-transactions/with-sdks/) guide from docs.polkadot.com.

## What This Tests

Each SDK connects to Paseo Asset Hub testnet, constructs a balance transfer transaction, signs it, and submits it to the network.

SDKs tested:
- **PAPI (Polkadot API)** — TypeScript
- **Polkadot.js API** — JavaScript
- **Dedot** — TypeScript
- **Python Substrate Interface** — Python
- **Subxt** — Rust

Connection tests run unconditionally. Send tests require the `SENDER_MNEMONIC` environment variable to be set (and optionally `DEST_ADDRESS`).

## Running Tests

```bash
# Install Node.js dependencies
npm ci

# Generate PAPI descriptors
npx papi add polkadotTestNet -w wss://asset-hub-paseo.dotters.network

# Install Python dependency
pip install substrate-interface

# Download Subxt metadata and build
cd tests/subxt-send-transactions
subxt metadata --url wss://asset-hub-paseo.dotters.network -o asset_hub_metadata.scale
cargo build
cd ../..

# Run all tests (connection-only if no SENDER_MNEMONIC)
npm test

# Run with send tests enabled
SENDER_MNEMONIC="your mnemonic phrase" DEST_ADDRESS="5Gxxx..." npm test
```
