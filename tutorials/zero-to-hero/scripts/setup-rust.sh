#!/bin/bash
# Rust Setup Script
set -e

# Load versions from tutorial-local or root config
SCRIPT_DIR=$(cd "$(dirname "$0")" && pwd)
REPO_ROOT=$(cd "$SCRIPT_DIR"/../../.. && pwd)
TUTORIAL_DIR=$(cd "$SCRIPT_DIR"/.. && pwd)
source "$REPO_ROOT/scripts/load-versions.sh"

echo "🦀 Setting up Rust ${RUST_VERSION}..."
echo "📦 Installing Rust toolchain and components..."

rustup default "${RUST_VERSION}"
rustup target add wasm32-unknown-unknown --toolchain "${RUST_VERSION}"
rustup component add rust-src --toolchain "${RUST_VERSION}"

echo "✅ Rust setup completed successfully!"
echo "📋 Installed components:"
echo "  - Rust toolchain: ${RUST_VERSION}"
echo "  - WASM target: wasm32-unknown-unknown"
echo "  - Rust source component"

echo "🔍 Verifying installation..."
rustup show
