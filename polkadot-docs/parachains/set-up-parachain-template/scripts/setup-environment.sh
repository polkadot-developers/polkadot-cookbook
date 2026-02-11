#!/bin/bash
# Setup environment for parachain template guide
# This script checks prerequisites and installs required tools

set -e

echo "=== Checking Prerequisites ==="

# Check Rust
if ! command -v rustc &> /dev/null; then
    echo "ERROR: Rust is not installed"
    echo "Install from: https://rustup.rs"
    exit 1
fi

RUST_VERSION=$(rustc --version | cut -d' ' -f2)
echo "Rust version: $RUST_VERSION"

# Check wasm target
if ! rustup target list --installed | grep -q "wasm32-unknown-unknown"; then
    echo "Adding wasm32-unknown-unknown target..."
    rustup target add wasm32-unknown-unknown
fi
echo "wasm32-unknown-unknown target: installed"

# Check chain-spec-builder
if ! command -v chain-spec-builder &> /dev/null; then
    echo "Installing chain-spec-builder..."
    cargo install staging-chain-spec-builder@16.0.0 --locked
fi
CHAIN_SPEC_VERSION=$(chain-spec-builder --version 2>&1 | head -1)
echo "chain-spec-builder: $CHAIN_SPEC_VERSION"

# Check polkadot-omni-node
if ! command -v polkadot-omni-node &> /dev/null; then
    echo "Installing polkadot-omni-node..."
    cargo install polkadot-omni-node@0.13.0 --locked
fi
OMNI_NODE_VERSION=$(polkadot-omni-node --version 2>&1 | head -1)
echo "polkadot-omni-node: $OMNI_NODE_VERSION"

echo ""
echo "=== Environment Ready ==="
