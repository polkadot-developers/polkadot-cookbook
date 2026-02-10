---
title: "Fork a Parachain"
description: "Verify the Fork a Parachain guide from docs.polkadot.com"
source_url: "https://docs.polkadot.com/parachains/testing/fork-a-parachain/"
last_tested: "2026-02-10"
---

# Fork a Parachain

[![Fork a Parachain](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-fork-a-parachain.yml/badge.svg)](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-fork-a-parachain.yml)

This project verifies the [Fork a Parachain](https://docs.polkadot.com/parachains/testing/fork-a-parachain/) guide from docs.polkadot.com.

## What This Tests

1. **Prerequisites** - Verifies Node.js and npx are available
2. **Chopsticks Installation** - Confirms Chopsticks CLI is accessible via npx
3. **Fork Polkadot** - Starts Chopsticks to fork Polkadot mainnet using a config file
4. **RPC Access** - Verifies the forked chain responds to JSON-RPC queries
5. **dev_newBlock** - Creates a new block and verifies block number increments
6. **dev_setStorage** - Modifies on-chain storage and verifies the change
7. **dev_timeTravel** - Sets a future timestamp and verifies it took effect

## Prerequisites

Before running tests, ensure you have:

- Node.js 22+

## Running Tests

```bash
# Install npm dependencies (uses lock file)
npm ci

# Run all verification tests
npm test
```

## Source

- [docs.polkadot.com guide](https://docs.polkadot.com/parachains/testing/fork-a-parachain/)
- [Chopsticks repository](https://github.com/AcalaNetwork/chopsticks)
