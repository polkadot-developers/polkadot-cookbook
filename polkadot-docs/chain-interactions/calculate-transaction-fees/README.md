---
title: "Calculate Transaction Fees"
description: "Verify the Calculate Transaction Fees guide from docs.polkadot.com"
source_url: "https://docs.polkadot.com/chain-interactions/send-transactions/calculate-transaction-fees/"
source_github: "https://github.com/polkadot-developers/polkadot-docs/blob/master/chain-interactions/send-transactions/calculate-transaction-fees.md"
---

# Calculate Transaction Fees

[![Calculate Transaction Fees](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-calculate-transaction-fees.yml/badge.svg?event=push)](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-calculate-transaction-fees.yml)

This project verifies the [Calculate Transaction Fees](https://docs.polkadot.com/chain-interactions/send-transactions/calculate-transaction-fees/) guide from docs.polkadot.com.

## What This Tests

1. **PAPI (Polkadot API)** — Connect via WebSocket, create a `transfer_keep_alive` transaction, estimate fees using `getEstimatedFees()`
2. **Polkadot.js API** — Connect via WebSocket, create a `transferKeepAlive` transaction, estimate fees using `paymentInfo()`

Both SDKs connect to Asset Hub Paseo testnet and verify that estimated fees are returned as valid positive values.

## Running Tests

```bash
# Install Node.js dependencies
npm install

# Generate PAPI descriptors
npx papi add polkadotTestNet -w wss://asset-hub-paseo.dotters.network

# Run all tests
npm test
```
