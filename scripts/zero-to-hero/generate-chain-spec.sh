#!/bin/bash
# Chain Specification Generation Script
# This script generates a development chain specification for the parachain

set -e

PARA_ID="1000"
RELAY_CHAIN="paseo"
RUNTIME_PATH="./target/release/wbuild/parachain-template-runtime/parachain_template_runtime.compact.compressed.wasm"

echo "⛓️ Generating chain specification..."
echo "📋 Configuration:"
echo "  - Para ID: $PARA_ID"
echo "  - Relay Chain: $RELAY_CHAIN"
echo "  - Runtime: $RUNTIME_PATH"

# Check if runtime exists
if [ ! -f "$RUNTIME_PATH" ]; then
  echo "❌ Runtime WASM file not found: $RUNTIME_PATH"
  echo "💡 Make sure you have built the parachain runtime first"
  echo "💡 Try running: cargo build --release"
  exit 1
fi

# Generate chain specification
chain-spec-builder create \
  -t development \
  --relay-chain $RELAY_CHAIN \
  --para-id $PARA_ID \
  --runtime $RUNTIME_PATH \
  named-preset development

# Verify chain spec was created
if [ ! -f "chain_spec.json" ]; then
  echo "❌ Chain specification generation failed"
  exit 1
fi

echo "✅ Chain specification generated successfully!"
echo "📄 Output file: chain_spec.json"
echo "📊 File size: $(du -h chain_spec.json | cut -f1)"

# Validate JSON
if command -v jq >/dev/null 2>&1; then
  echo "🔍 Validating JSON structure..."
  if jq empty chain_spec.json; then
    echo "✅ Chain specification is valid JSON"
    
    # Extract key information
    CHAIN_NAME=$(jq -r '.name // "unknown"' chain_spec.json)
    echo "📋 Chain Name: $CHAIN_NAME"
  else
    echo "⚠️ Chain specification may have JSON formatting issues"
  fi
fi
