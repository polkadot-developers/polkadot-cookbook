#!/bin/bash
set -e

echo "🔧 Setting up Zombienet binaries for XCM testing..."
echo ""

# Detect platform
OS="$(uname -s)"
ARCH="$(uname -m)"

# Create binaries directory
BINARIES_DIR="./zombienet-binaries"
mkdir -p "$BINARIES_DIR"

echo "Platform detected: $OS $ARCH"
echo ""

# Function to check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Check for zombienet CLI
if ! command_exists zombienet; then
    echo "⚠️  Zombienet CLI not found!"
    echo "Install it with: npm install -g @zombienet/cli"
    echo ""
fi

# Step 1: Setup polkadot binary (relay chain)
echo "📦 Step 1/2: Setting up polkadot (relay chain)..."
if [[ "$OS" == "Linux" ]]; then
    echo "  Using zombienet setup for automatic download..."
    zombienet setup polkadot || {
        echo "  ⚠️  zombienet setup failed, you'll need to install polkadot manually"
        echo "  Download from: https://github.com/paritytech/polkadot-sdk/releases"
    }

    # Check if binary was downloaded
    if [ -f "$HOME/.local/share/zombienet/binaries/polkadot" ]; then
        echo "  ✅ polkadot binary downloaded to ~/.local/share/zombienet/binaries/"
        echo ""
        echo "  Add to PATH by running:"
        echo "  export PATH=\"\$HOME/.local/share/zombienet/binaries:\$PATH\""
        echo ""
    fi
else
    echo "  ℹ️  macOS detected: zombienet setup doesn't provide macOS binaries"
    echo ""
    echo "  Option 1 (Recommended): Install via Homebrew"
    echo "    brew install parity/polkadot/polkadot"
    echo ""
    echo "  Option 2: Build from source"
    echo "    git clone https://github.com/paritytech/polkadot-sdk.git"
    echo "    cd polkadot-sdk"
    echo "    cargo build --release -p polkadot"
    echo ""
    echo "  Option 3: Use Docker/Podman (see zombienet docs)"
    echo ""
fi

# Step 2: Setup polkadot-omni-node (for parachains)
echo "📦 Step 2/2: Setting up polkadot-omni-node (for both parachains)..."

if command_exists polkadot-omni-node; then
    echo "  ✅ polkadot-omni-node already installed"
    OMNI_VERSION=$(polkadot-omni-node --version 2>/dev/null || echo "unknown")
    echo "  Version: $OMNI_VERSION"
else
    echo "  Installing polkadot-omni-node via cargo..."
    echo "  This may take several minutes..."
    echo ""

    if command_exists cargo; then
        cargo install polkadot-omni-node && {
            echo "  ✅ polkadot-omni-node installed successfully"
        } || {
            echo "  ❌ Failed to install polkadot-omni-node"
            echo "  Please check your Rust installation and try again"
            exit 1
        }
    else
        echo "  ❌ Cargo not found! Please install Rust first:"
        echo "  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
        exit 1
    fi
fi

echo ""
echo "✅ Setup complete!"
echo ""
echo "Next steps:"
echo "  1. Ensure binaries are in your PATH"
if [[ "$OS" == "Linux" ]]; then
    echo "     export PATH=\"\$HOME/.local/share/zombienet/binaries:\$PATH\""
fi
echo "  2. Run: npm run zombienet:xcm"
echo ""
echo "Verify installation:"
echo "  polkadot --version"
echo "  polkadot-omni-node --version"
echo "  zombienet version"
