---
title: "Channels with System Parachains"
description: "Verify the channels with system parachains tutorial from docs.polkadot.com"
source_url: "https://docs.polkadot.com/parachains/interoperability/channels-with-system-parachains/"
source_github: "https://github.com/polkadot-developers/polkadot-docs/blob/master/parachains/interoperability/channels-with-system-parachains.md"
last_tested: "2026-02-19"
polkadot_sdk_version: "polkadot-stable2512-1"
parachain_template_version: "v0.0.5"
---

# Channels with System Parachains

[![Channels with System Parachains](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-channels-with-system-parachains.yml/badge.svg?event=push)](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-channels-with-system-parachains.yml)

This project verifies the [Channels with System Parachains](https://docs.polkadot.com/parachains/interoperability/channels-with-system-parachains/) tutorial from docs.polkadot.com.

## What This Tests

1. Prerequisites check (Rust, wasm target, required tools)
2. Clone the parachain template repository (v0.0.5)
3. Build the runtime WASM binary
4. Generate a chain spec for custom parachain (para ID 2000)
5. Download relay chain and system parachain binaries
6. Spawn a Zombienet network with 2 relay validators + Asset Hub (1000) + custom parachain (2000)
7. Open bidirectional HRMP channels between Asset Hub and the custom parachain
8. Verify both channel directions are established after a session boundary
9. Verify all chains continue producing blocks

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

- [docs.polkadot.com tutorial](https://docs.polkadot.com/parachains/interoperability/channels-with-system-parachains/)
