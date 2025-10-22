#!/bin/bash
# Chain Spec Builder Installation Script
set -e

# Load versions
SCRIPT_DIR=$(cd "$(dirname "$0")" && pwd)
REPO_ROOT=$(cd "$SCRIPT_DIR"/../../.. && pwd)
TUTORIAL_DIR=$(cd "$SCRIPT_DIR"/.. && pwd)
source "$REPO_ROOT/scripts/load-versions.sh"

echo "🔧 Installing staging-chain-spec-builder ${CHAIN_SPEC_BUILDER_VERSION}..."

cargo install --locked staging-chain-spec-builder@"${CHAIN_SPEC_BUILDER_VERSION}"

echo "✅ Chain spec builder installation completed!"
echo "📋 Installed version: ${CHAIN_SPEC_BUILDER_VERSION}"

echo "🔍 Verifying installation..."
chain-spec-builder --version
