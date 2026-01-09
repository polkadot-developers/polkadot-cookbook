#!/bin/bash
# Clone and build the parachain template
# Usage: ./clone-and-build.sh [target_dir]

set -e

TARGET_DIR="${1:-./parachain-template}"

echo "=== Cloning Parachain Template ==="

# Clone if directory doesn't exist
if [ ! -d "$TARGET_DIR" ]; then
    git clone https://github.com/paritytech/polkadot-sdk-parachain-template.git "$TARGET_DIR"
else
    echo "Directory exists, pulling latest..."
    cd "$TARGET_DIR"
    git pull
    cd -
fi

echo ""
echo "=== Building Parachain Template ==="

cd "$TARGET_DIR"

# Build with locked dependencies
cargo build --release --locked

echo ""
echo "=== Verifying Build Output ==="

WASM_PATH="./target/release/wbuild/parachain-template-runtime/parachain_template_runtime.compact.compressed.wasm"

if [ -f "$WASM_PATH" ]; then
    WASM_SIZE=$(ls -lh "$WASM_PATH" | awk '{print $5}')
    echo "WASM runtime built successfully: $WASM_SIZE"
else
    echo "ERROR: WASM runtime not found at $WASM_PATH"
    exit 1
fi

echo ""
echo "=== Build Complete ==="
