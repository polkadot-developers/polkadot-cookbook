#!/bin/bash
# Rust Setup Script
# This script sets up Rust with the required version and components

set -e

RUST_VERSION="1.86"

echo "🦀 Setting up Rust 1.86..."
echo "📦 Installing Rust toolchain and components..."

# Set default Rust version
rustup default 1.86

# Add WASM target for the current platform
rustup target add wasm32-unknown-unknown --toolchain 1.86

# Add rust source for the current platform  
rustup component add rust-src --toolchain 1.86

echo "✅ Rust setup completed successfully!"
echo "📋 Installed components:"
echo "  - Rust toolchain: 1.86"
echo "  - WASM target: wasm32-unknown-unknown"
echo "  - Rust source component"

# Verify installation
echo "🔍 Verifying installation..."
rustup show
