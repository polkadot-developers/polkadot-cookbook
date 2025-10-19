#!/bin/bash
# Rust Setup Script (template)
set -e
: "${RUST_VERSION:?RUST_VERSION is required}"

echo "🦀 Setting up Rust ${RUST_VERSION}..."
echo "📦 Installing Rust toolchain and components..."

rustup default ${RUST_VERSION}

rustup target add wasm32-unknown-unknown --toolchain ${RUST_VERSION}

rustup component add rust-src --toolchain ${RUST_VERSION}

echo "✅ Rust setup completed successfully!"
echo "📋 Installed components:"
echo "  - Rust toolchain: ${RUST_VERSION}"
echo "  - WASM target: wasm32-unknown-unknown"
echo "  - Rust source component"

echo "🔍 Verifying installation..."
rustup show
