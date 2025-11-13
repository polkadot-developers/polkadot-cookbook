#!/usr/bin/env bash
# Generate chain specification for local development

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

echo "🔧 Building runtime..."
cd "$PROJECT_ROOT"
cargo build --release

echo "📄 Generating chain specification..."

# Use chain-spec-builder to generate chain spec from the compiled runtime
chain-spec-builder --chain-spec-path "$PROJECT_ROOT/chain-spec.json" create \
  -t development \
  --relay-chain paseo \
  --para-id 1000 \
  --runtime ./target/release/wbuild/{{slug}}-runtime/{{slug_underscore}}_runtime.compact.compressed.wasm \
  named-preset development

echo "✅ Chain specification generated: chain-spec.json"
echo ""
echo "You can now:"
echo "  1. Generate TypeScript types: npm run generate:types"
echo "  2. Start the dev node: npm run start:node"
