#!/bin/bash
# Chain Spec Builder Installation Script
# This script installs the staging-chain-spec-builder tool

set -e

CHAIN_SPEC_VERSION="10.0.0"

echo "🔧 Installing staging-chain-spec-builder 10.0.0..."

# Install chain-spec-builder with locked dependencies
cargo install --locked staging-chain-spec-builder@10.0.0

echo "✅ Chain spec builder installation completed!"
echo "📋 Installed version: 10.0.0"

# Verify installation
echo "🔍 Verifying installation..."
chain-spec-builder --version
