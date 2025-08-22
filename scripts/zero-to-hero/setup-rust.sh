#!/bin/bash
# Rust Setup Script
# This script sets up Rust with the required version and components

set -e

RUST_VERSION="1.86"

echo "🦀 Setting up Rust $RUST_VERSION..."
echo "📦 Installing Rust toolchain and components..."

# Set default Rust version
rustup default $RUST_VERSION

# Add WASM target for the current platform
rustup target add wasm32-unknown-unknown --toolchain $RUST_VERSION

# Add rust source for the current platform  
rustup component add rust-src --toolchain $RUST_VERSION

echo "✅ Rust setup completed successfully!"
echo "📋 Installed components:"
echo "  - Rust toolchain: $RUST_VERSION"
echo "  - WASM target: wasm32-unknown-unknown"
echo "  - Rust source component"

# Verify installation
echo "🔍 Verifying installation..."
rustup show
