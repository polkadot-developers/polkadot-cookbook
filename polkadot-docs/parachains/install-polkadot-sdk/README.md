---
title: "Install Polkadot SDK"
description: "Verify the Polkadot SDK installation guide from docs.polkadot.com"
source_url: "https://docs.polkadot.com/parachains/install-polkadot-sdk/"
last_tested: "2026-02-13"
---

# Install Polkadot SDK

[![Install Polkadot SDK](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-install-polkadot-sdk.yml/badge.svg)](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-install-polkadot-sdk.yml)

This project verifies the [Install Polkadot SDK](https://docs.polkadot.com/parachains/install-polkadot-sdk/) guide from docs.polkadot.com.

## What This Tests

1. **Rust Installation** - Verifies rustc, cargo, and rustup are installed with stable toolchain
2. **Rust Configuration** - Confirms wasm32-unknown-unknown target and rust-src component are installed
3. **System Dependencies** - Checks for git, protobuf compiler, clang, and make
4. **Clone Polkadot SDK** - Clones the polkadot-sdk repository and verifies structure
5. **Verify Workspace** - Validates the cargo workspace and binary crates exist

## Prerequisites

Before running tests, ensure you have:

- Rust stable (version specified in `rust-toolchain.toml`)
- wasm32-unknown-unknown target
- rust-src component
- Node.js 22+
- System dependencies: git, protobuf-compiler (protoc), clang, make

## Running Tests

```bash
# Install npm dependencies (uses lock file)
npm ci

# Run all verification tests
npm test
```

## Test Phases

### 1. Rust Installation
Verifies rustc, cargo, and rustup are installed with stable as the default toolchain.

### 2. Rust Configuration
Confirms the wasm32-unknown-unknown target and rust-src component are installed.

### 3. System Dependencies
Checks for required system tools: git, protoc (protobuf compiler), clang, and make.

### 4. Clone Polkadot SDK
Clones the polkadot-sdk repository (shallow clone for speed) and verifies the expected directory structure exists (polkadot, substrate, cumulus, bridges).

### 5. Verify Workspace
Validates the cargo workspace metadata is readable and checks for polkadot CLI and substrate node binary crates.

## Exact Replication Steps

To manually replicate this guide:

```bash
# 1. Install Rust (if needed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# 2. Set stable as default
rustup default stable

# 3. Add wasm target
rustup target add wasm32-unknown-unknown

# 4. Add rust-src component
rustup component add rust-src

# 5. Install system dependencies (Ubuntu/Debian)
sudo apt-get update
sudo apt-get install -y git protobuf-compiler clang make

# 6. Clone polkadot-sdk
git clone https://github.com/paritytech/polkadot-sdk.git
cd polkadot-sdk

# 7. Verify workspace
cargo metadata --format-version 1 --no-deps | head -1
```

## Versions Tested

| Component | Version |
|-----------|---------|
| Rust | See `rust-toolchain.toml` |
| Polkadot SDK | Latest from main branch |

## Source

- [docs.polkadot.com guide](https://docs.polkadot.com/parachains/install-polkadot-sdk/)
- [polkadot-sdk repository](https://github.com/paritytech/polkadot-sdk)
