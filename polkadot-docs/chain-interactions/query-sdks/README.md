---
title: "Query On-Chain State with SDKs"
description: "Verify the Query On-Chain State with SDKs guide from docs.polkadot.com"
source_url: "https://docs.polkadot.com/chain-interactions/query-data/query-sdks/"
source_github: "https://github.com/polkadot-developers/polkadot-docs/blob/master/chain-interactions/query-data/query-sdks.md"
---

# Query On-Chain State with SDKs

[![Query On-Chain State with SDKs](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-query-sdks.yml/badge.svg?event=push)](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-query-sdks.yml)

This project verifies the [Query On-Chain State with SDKs](https://docs.polkadot.com/chain-interactions/query-data/query-sdks/) guide from docs.polkadot.com.

## What This Tests

Each SDK performs two queries:

1. **Query Account Balance** — `System.Account` storage for balance fields (nonce, free, reserved, frozen)
2. **Query Asset Information** — `Assets.Metadata`, `Assets.Asset`, and `Assets.Account` for USDT (asset ID 1984)

SDKs tested:
- **PAPI (Polkadot API)** — TypeScript
- **Polkadot.js API** — JavaScript
- **Dedot** — TypeScript
- **Python Substrate Interface** — Python
- **Subxt** — Rust

All SDKs query the same accounts on Paseo Asset Hub testnet.

## Running Tests

```bash
# Install Node.js dependencies
npm ci

# Generate PAPI descriptors
npx papi add polkadotTestNet -w wss://asset-hub-paseo.dotters.network

# Install Python dependency
pip install substrate-interface

# Download Subxt metadata and build
cd tests/subxt-query-sdks
subxt metadata --url wss://asset-hub-paseo.dotters.network -o asset_hub_metadata.scale
cargo build
cd ../..

# Run all tests
npm test
```
