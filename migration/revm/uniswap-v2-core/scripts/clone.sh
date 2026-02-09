#!/bin/bash
# Clone the Uniswap V2 Core repository for REVM testing
# Usage: ./clone.sh

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TARGET_DIR="$SCRIPT_DIR/../.test-workspace"

echo "=== Cloning Uniswap V2 Core for Polkadot ==="

# Clone if directory doesn't exist
if [ ! -d "$TARGET_DIR" ]; then
    git clone https://github.com/papermoonio/uniswap-v2-polkadot.git "$TARGET_DIR"
else
    echo "Directory exists, pulling latest..."
    cd "$TARGET_DIR"
    git pull
    cd -
fi

echo ""
echo "=== Installing Dependencies ==="

cd "$TARGET_DIR"
npm install

echo ""
echo "=== Setup Complete ==="
echo ""
echo "To run tests on REVM local network:"
echo "  cd $TARGET_DIR"
echo "  REVM=true npx hardhat test --network local"
