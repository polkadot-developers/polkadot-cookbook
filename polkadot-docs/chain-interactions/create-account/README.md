---
title: "Create an Account"
description: "Verify the Create an Account guide from docs.polkadot.com"
source_url: "https://docs.polkadot.com/chain-interactions/accounts/create-account/"
source_github: "https://github.com/polkadot-developers/polkadot-docs/blob/master/chain-interactions/accounts/create-account.md"
---

# Create an Account

[![Create an Account](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-create-account.yml/badge.svg?event=push)](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-create-account.yml)

This project verifies the [Create an Account](https://docs.polkadot.com/chain-interactions/accounts/create-account/) guide from docs.polkadot.com.

## What This Tests

1. **Crypto Initialization** — `cryptoWaitReady()` resolves successfully
2. **Mnemonic Generation** — `mnemonicGenerate(12)` returns a 12-word BIP39 phrase
3. **Keyring Creation** — Create keyring with `sr25519` type and `ss58Format: 0`
4. **Account from Mnemonic** — `addFromMnemonic()` returns a keypair with a valid SS58 address
5. **Address Determinism** — Same mnemonic produces the same address
6. **Multiple Accounts** — Different mnemonics produce different addresses

## Running Tests

```bash
npm ci
npm test
```
