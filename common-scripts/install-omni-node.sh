#!/bin/bash
# Omni Node Installation Script (template)
set -e
: "${OMNI_NODE_VERSION:?OMNI_NODE_VERSION is required}"

echo "🚀 Installing polkadot-omni-node ${OMNI_NODE_VERSION}..."

cargo install --locked polkadot-omni-node@${OMNI_NODE_VERSION}

echo "✅ Omni node installation completed!"
echo "📋 Installed version: ${OMNI_NODE_VERSION}"

echo "🔍 Verifying installation..."
polkadot-omni-node --version
