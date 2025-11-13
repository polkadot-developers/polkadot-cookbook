#!/usr/bin/env bash
# Start local development node using polkadot-omni-node

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

echo "🚀 Starting {{title}} development node..."
echo ""

# Check if chain-spec.json exists
if [ ! -f "$PROJECT_ROOT/chain-spec.json" ]; then
  echo "⚠️  Chain specification not found. Generating..."
  "$SCRIPT_DIR/generate-spec.sh"
fi

# Start polkadot-omni-node with the generated chain spec
cd "$PROJECT_ROOT"
polkadot-omni-node \
  --dev \
  --rpc-cors all \
  --rpc-methods unsafe \
  --rpc-port 9944 \
  --runtime ./target/release/wbuild/{{slug}}-runtime/{{slug}}_runtime.compact.compressed.wasm \
  --tmp

echo ""
echo "🎉 Node stopped"
