---
title: "Query Account Information"
description: "Verify the Query Account Information guide from docs.polkadot.com"
source_url: "https://docs.polkadot.com/chain-interactions/accounts/query-accounts/"
source_github: "https://github.com/polkadot-developers/polkadot-docs/blob/master/chain-interactions/accounts/query-accounts.md"
---

# Query Account Information

[![Query Account Information](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-query-accounts.yml/badge.svg?event=push)](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-query-accounts.yml)

This project verifies the [Query Account Information](https://docs.polkadot.com/chain-interactions/accounts/query-accounts/) guide from docs.polkadot.com.

## What This Tests

1. **PAPI (Polkadot API)** — Connect via WebSocket, query `system.account`, verify balance fields
2. **Polkadot.js API** — Connect via WebSocket, query `system.account`, verify balance fields
3. **Dedot** — Connect via WebSocket, query `system.account`, verify balance fields
4. **Python Substrate Interface** — Connect via WebSocket, query `System.Account`, verify balance fields
5. **Subxt (Rust)** — Connect via WebSocket, query `system.account`, verify balance fields

All SDKs query the same account on Asset Hub Paseo testnet and verify the response contains the expected fields (nonce, consumers, providers, sufficients, free, reserved, frozen).

## Running Tests

```bash
# Install Node.js dependencies
npm ci

# Generate PAPI descriptors
npx papi add polkadotTestNet -w wss://asset-hub-paseo.dotters.network

# Install Python dependency
pip install substrate-interface

# Download Subxt metadata and build
cd tests/subxt-query-account
subxt metadata --url wss://asset-hub-paseo.dotters.network -o polkadot_testnet_metadata.scale
cargo build
cd ../..

# Run all tests
npm test
```
