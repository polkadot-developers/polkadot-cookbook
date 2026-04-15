---
title: "Transfer Assets Between Parachains"
description: "Verify the Transfer Assets Between Parachains guide from docs.polkadot.com"
source_url: "https://docs.polkadot.com/chain-interactions/send-transactions/interoperability/transfer-assets-parachains/"
source_github: "https://github.com/polkadot-developers/polkadot-docs/blob/master/chain-interactions/send-transactions/interoperability/transfer-assets-parachains.md"
---

# Transfer Assets Between Parachains

[![Transfer Assets Between Parachains](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-transfer-assets-parachains.yml/badge.svg?event=push)](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-transfer-assets-parachains.yml)

This project verifies the [Transfer Assets Between Parachains](https://docs.polkadot.com/chain-interactions/send-transactions/interoperability/transfer-assets-parachains/) guide from docs.polkadot.com.

## What This Tests

Uses the [ParaSpell XCM SDK](https://paraspell.github.io/docs/sdk/getting-started.html) to transfer PAS from Paseo's Asset Hub to Paseo's People Chain:

1. **Build** an XCM transfer transaction.
2. **Dry run** the transfer to simulate success.
3. **Verify ED** on the destination account.
4. **Get transfer info** including XCM fee estimates.
5. **Sign and submit** the transfer (skipped unless `SENDER_MNEMONIC` is set).

## Running Tests

```bash
# Install dependencies
npm install

# Run all tests (submit step skipped without SENDER_MNEMONIC)
npm test

# Run with a funded Paseo account to exercise the full submission path
SENDER_MNEMONIC="your twelve word mnemonic here" npm test
```
