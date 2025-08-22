#!/bin/bash
# Omni Node Installation Script  
# This script installs the polkadot-omni-node

set -e

OMNI_NODE_VERSION="0.5.0"

echo "🚀 Installing polkadot-omni-node $OMNI_NODE_VERSION..."

# Install omni-node with locked dependencies
cargo install --locked polkadot-omni-node@$OMNI_NODE_VERSION

echo "✅ Omni node installation completed!"
echo "📋 Installed version: $OMNI_NODE_VERSION"

# Verify installation
echo "🔍 Verifying installation..."
polkadot-omni-node --version
