#!/bin/bash
# Omni Node Installation Script
set -e

# Load versions
SCRIPT_DIR=$(cd "$(dirname "$0")" && pwd)
REPO_ROOT=$(cd "$SCRIPT_DIR"/../../.. && pwd)
TUTORIAL_DIR=$(cd "$SCRIPT_DIR"/.. && pwd)
source "$REPO_ROOT/scripts/load-versions.sh"

echo "🚀 Installing polkadot-omni-node ${OMNI_NODE_VERSION}..."

cargo install --locked polkadot-omni-node@"${OMNI_NODE_VERSION}"

echo "✅ Omni node installation completed!"
echo "📋 Installed version: ${OMNI_NODE_VERSION}"

echo "🔍 Verifying installation..."
polkadot-omni-node --version
