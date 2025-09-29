#!/bin/bash
# Chain Spec Builder Installation Script (template)
set -e
: "${CHAIN_SPEC_BUILDER_VERSION:?CHAIN_SPEC_BUILDER_VERSION is required}"

echo "🔧 Installing staging-chain-spec-builder ${CHAIN_SPEC_BUILDER_VERSION}..."

cargo install --locked staging-chain-spec-builder@${CHAIN_SPEC_BUILDER_VERSION}

echo "✅ Chain spec builder installation completed!"
echo "📋 Installed version: ${CHAIN_SPEC_BUILDER_VERSION}"

echo "🔍 Verifying installation..."
chain-spec-builder --version
