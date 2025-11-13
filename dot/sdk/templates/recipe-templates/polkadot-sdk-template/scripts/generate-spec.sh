#!/usr/bin/env bash
# Generate chain specification for local development

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

echo "🔧 Building runtime..."
cd "$PROJECT_ROOT"
cargo build --release

echo "📄 Generating chain specification..."

# Use the compiled node binary to generate chain spec
./target/release/{{slug}}-node build-spec \
  --chain local \
  --disable-default-bootnode \
  > "$PROJECT_ROOT/chain-spec.json"

echo "✅ Chain specification generated: chain-spec.json"
echo ""
echo "You can now:"
echo "  1. Generate TypeScript types: npm run generate:types"
echo "  2. Start the dev node: npm run start:node"
