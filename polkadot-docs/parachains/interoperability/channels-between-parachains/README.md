---
title: "Channels Between Parachains"
description: "Verify the channels between parachains tutorial from docs.polkadot.com"
source_url: "https://docs.polkadot.com/parachains/interoperability/channels-between-parachains/"
source_github: "https://github.com/polkadot-developers/polkadot-docs/blob/master/parachains/interoperability/channels-between-parachains.md"
polkadot_sdk_version: "polkadot-stable2512-1"
parachain_template_version: "v0.0.5"
---

# Channels Between Parachains

[![Channels Between Parachains](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-channels-between-parachains.yml/badge.svg?event=push)](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-channels-between-parachains.yml)

This project verifies the [Channels Between Parachains](https://docs.polkadot.com/parachains/interoperability/channels-between-parachains/) tutorial from docs.polkadot.com.

## What This Tests

1. Prerequisites check (Rust, wasm target, required tools)
2. Clone the parachain template repository (v0.0.5)
3. Build the runtime WASM binary
4. Generate two chain specs (para ID 1000 and 1001)
5. Download relay chain binaries
6. Spawn a Zombienet network with 2 relay validators + 2 parachain collators
7. Fund sovereign accounts for both parachains on the relay chain
8. Open an HRMP channel from parachain 1000 to 1001 via XCM
9. Accept the HRMP channel from parachain 1001 via XCM
10. Verify the channel is established after a session boundary
11. Verify both parachains continue producing blocks

## Prerequisites

Before running tests, ensure you have:

- Rust (version specified in `rust-toolchain.toml`)
- wasm32-unknown-unknown target
- Node.js 22+
- `staging-chain-spec-builder` CLI tool
- `polkadot-omni-node` CLI tool
- Zombienet

## Running Tests

```bash
# Install npm dependencies (uses lock file)
npm ci

# Run all verification tests
npm test
```

## Versions Tested

| Component | Version |
|-----------|---------|
| Rust | See `rust-toolchain.toml` |
| Polkadot SDK | polkadot-stable2512-1 |
| Parachain Template | v0.0.5 |
| chain-spec-builder | 16.0.0 |
| polkadot-omni-node | 0.13.0 |

## Source

- [docs.polkadot.com tutorial](https://docs.polkadot.com/parachains/interoperability/channels-between-parachains/)
