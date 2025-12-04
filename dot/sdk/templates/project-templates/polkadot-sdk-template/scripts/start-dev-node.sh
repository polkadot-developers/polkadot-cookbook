#!/usr/bin/env bash
# Start local development node using polkadot-omni-node

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

echo "🚀 Starting {{title}} development node..."
echo ""

# Start polkadot-omni-node with the dev chain spec
cd "$PROJECT_ROOT"
polkadot-omni-node \
  --chain dev_chain_spec.json \
  --dev \
  --rpc-cors all \
  --rpc-methods unsafe \
  --rpc-port 9944 \
  --tmp

echo ""
echo "🎉 Node stopped"
