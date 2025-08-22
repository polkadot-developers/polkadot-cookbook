#!/bin/bash
# Parachain Node Startup Script
# This script starts the parachain node in development mode

set -e

CHAIN_SPEC="./chain_spec.json"

echo "🖥️ Starting parachain node..."
echo "📋 Configuration:"
echo "  - Chain spec: $CHAIN_SPEC"
echo "  - Mode: Development"
echo "  - RPC: All CORS enabled, unsafe methods allowed"

# Check if chain spec exists
if [ ! -f "$CHAIN_SPEC" ]; then
  echo "❌ Chain specification not found: $CHAIN_SPEC"
  echo "💡 Generate it first by running: ./scripts/zero-to-hero/generate-chain-spec.sh"
  exit 1
fi

echo "🚀 Starting polkadot-omni-node..."
echo "📡 RPC will be available at: http://localhost:9944"
echo "🔍 Press Ctrl+C to stop the node"
echo ""

# Start the node
polkadot-omni-node \
  --chain $CHAIN_SPEC \
  --dev \
  --rpc-cors all \
  --rpc-methods unsafe
