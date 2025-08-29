#!/usr/bin/env bash

set -euo pipefail

# Tutorial-specific chain spec generator for "add-nft-pallet"
# - Uses kitchensink runtime WASM
# - Writes chain_spec.json into kitchensink-parachain/
# - Post-processes JSON to tag it for this tutorial

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel)"

PARA_ID="1000"
RELAY_CHAIN="rococo-local"
KITCHEN_DIR="$REPO_ROOT/kitchensink-parachain"
WASM_PATH="$KITCHEN_DIR/target/release/wbuild/parachain-template-runtime/parachain_template_runtime.wasm"
OUT_PATH="$KITCHEN_DIR/chain_spec.json"

echo "⛓️  [add-nft-pallet] Generating chain specification..."
echo "📋  Config: PARA_ID=$PARA_ID RELAY_CHAIN=$RELAY_CHAIN"
echo "📄  Runtime WASM: $WASM_PATH"
echo "📝  Output: $OUT_PATH"

if ! command -v chain-spec-builder >/dev/null 2>&1; then
  echo "❌ chain-spec-builder not found. Install with: just install-chain-spec" >&2
  exit 1
fi

if [ ! -f "$WASM_PATH" ]; then
  echo "❌ Runtime WASM not found: $WASM_PATH" >&2
  echo "💡 Run: cd $KITCHEN_DIR && cargo build --profile production" >&2
  exit 1
fi

chain-spec-builder create \
  --relay-chain "$RELAY_CHAIN" \
  --para-id "$PARA_ID" \
  --runtime "$WASM_PATH" \
  named-preset development >"$OUT_PATH"

if [ ! -f "$OUT_PATH" ]; then
  echo "❌ Chain specification generation failed" >&2
  exit 1
fi

if command -v jq >/dev/null 2>&1; then
  echo "🔧  Tagging chain spec for tutorial: add-nft-pallet"
  tmpfile="$(mktemp)"
  jq '.name = ((.name // "development") + " (NFT Tutorial)") | .tutorial = "add-nft-pallet"' "$OUT_PATH" > "$tmpfile"
  mv "$tmpfile" "$OUT_PATH"
fi

echo "✅ Chain specification generated successfully for add-nft-pallet"
echo "📄 Output file: $OUT_PATH"
echo "📊 File size: $(du -h "$OUT_PATH" | cut -f1)"


