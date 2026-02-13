---
title: "Set Up the Parachain Template"
description: "Verify the parachain template setup guide from docs.polkadot.com"
source_url: "https://docs.polkadot.com/parachains/launch-a-parachain/set-up-the-parachain-template/"
source_github: "https://github.com/polkadot-developers/polkadot-docs/blob/master/parachains/launch-a-parachain/set-up-the-parachain-template.md"
docs_commit: "d4b41f851b16ac909a7422726a4fd47fea239ba3"
last_tested: "2026-02-13"
polkadot_sdk_version: "polkadot-stable2512-1"
---

# Set Up the Parachain Template

[![Set Up Parachain Template](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-set-up-parachain-template.yml/badge.svg?event=push)](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-set-up-parachain-template.yml)

This project verifies the [Set Up the Parachain Template](https://docs.polkadot.com/parachains/launch-a-parachain/set-up-the-parachain-template/) guide from docs.polkadot.com.

## What This Tests

1. Prerequisites check (Rust, wasm target, required tools)
2. Clone the parachain template repository
3. Build the template with `cargo build --release --locked`
4. Generate chain specification
5. Start local node with `polkadot-omni-node`
6. Verify block production via PAPI

## Prerequisites

Before running tests, ensure you have:

- Rust (version specified in `rust-toolchain.toml`)
- wasm32-unknown-unknown target
- Node.js 22+
- `staging-chain-spec-builder` CLI tool
- `polkadot-omni-node` CLI tool

### Installing Required Tools

```bash
# Install the tools with pinned versions (see "Versions Tested" below)
cargo install --locked staging-chain-spec-builder@16.0.0
cargo install --locked polkadot-omni-node@0.13.0
```

## Running Tests

```bash
# Install npm dependencies (uses lock file)
npm ci

# Run all verification tests
npm test
```

## Test Phases

### 1. Environment Check (`environment.test.ts`)
Verifies all prerequisites are installed with correct versions.

### 2. Build Verification (`build.test.ts`)
- Clones `polkadot-sdk-parachain-template`
- Builds with `cargo build --release --locked`
- Verifies WASM runtime is generated

### 3. Runtime Verification (`runtime.test.ts`)
- Generates chain specification
- Starts `polkadot-omni-node` in dev mode
- Connects via PAPI and verifies blocks are produced
- Cleans up node process

## Exact Replication Steps

To manually replicate this guide:

```bash
# 1. Install Rust (if needed)
rustup show  # Will install version from rust-toolchain.toml

# 2. Add wasm target
rustup target add wasm32-unknown-unknown

# 3. Install required tools (pinned versions)
cargo install --locked staging-chain-spec-builder@16.0.0
cargo install --locked polkadot-omni-node@0.13.0

# 4. Clone the template
git clone https://github.com/paritytech/polkadot-sdk-parachain-template.git
cd polkadot-sdk-parachain-template

# 5. Build
cargo build --release --locked

# 6. Generate chain spec
chain-spec-builder create -t development \
  --relay-chain paseo \
  --para-id 1000 \
  --runtime ./target/release/wbuild/parachain-template-runtime/parachain_template_runtime.compact.compressed.wasm \
  named-preset development

# 7. Run node
polkadot-omni-node --chain ./chain_spec.json --dev
```

## Versions Tested

| Component | Version |
|-----------|---------|
| Rust | See `rust-toolchain.toml` |
| Polkadot SDK | polkadot-stable2512-1 |
| chain-spec-builder | 16.0.0 |
| polkadot-omni-node | 0.13.0 |

## Source

- [docs.polkadot.com guide](https://docs.polkadot.com/parachains/launch-a-parachain/set-up-the-parachain-template/)
- [polkadot-sdk-parachain-template](https://github.com/paritytech/polkadot-sdk-parachain-template)
