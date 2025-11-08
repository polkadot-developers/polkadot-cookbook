#!/bin/bash
set -e

# Polkadot Cookbook CLI Installer
# https://github.com/polkadot-developers/polkadot-cookbook

# Colors for output
PINK='\033[38;2;230;0;122m'
CYAN='\033[0;36m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
RESET='\033[0m'

REPO="polkadot-developers/polkadot-cookbook"
BINARY_NAME="dot"
INSTALL_DIR="${INSTALL_DIR:-/usr/local/bin}"

# Print colored output
print_pink() {
    echo -e "${PINK}$1${RESET}"
}

print_cyan() {
    echo -e "${CYAN}$1${RESET}"
}

print_green() {
    echo -e "${GREEN}$1${RESET}"
}

print_yellow() {
    echo -e "${YELLOW}$1${RESET}"
}

print_red() {
    echo -e "${RED}$1${RESET}"
}

# Detect OS
detect_os() {
    case "$(uname -s)" in
        Linux*)     echo "linux";;
        Darwin*)    echo "macos";;
        MINGW*|MSYS*|CYGWIN*) echo "windows";;
        *)          echo "unknown";;
    esac
}

# Detect architecture
detect_arch() {
    case "$(uname -m)" in
        x86_64|amd64)   echo "amd64";;
        aarch64|arm64)  echo "arm64";;
        *)              echo "unknown";;
    esac
}

# Get the appropriate binary name for the platform
get_binary_name() {
    local os=$1
    local arch=$2

    case "$os" in
        linux)
            if [ "$arch" = "amd64" ]; then
                echo "dot-linux-amd64.tar.gz"
            elif [ "$arch" = "arm64" ]; then
                echo "dot-linux-arm64.tar.gz"
            fi
            ;;
        macos)
            if [ "$arch" = "amd64" ]; then
                echo "dot-macos-intel.tar.gz"
            elif [ "$arch" = "arm64" ]; then
                echo "dot-macos-apple-silicon.tar.gz"
            fi
            ;;
        windows)
            echo "dot-windows-amd64.exe.zip"
            ;;
    esac
}

# Main installation
main() {
    print_pink "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    print_pink "  Polkadot Cookbook CLI Installer"
    print_pink "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo ""

    # Detect platform
    OS=$(detect_os)
    ARCH=$(detect_arch)

    print_cyan "→ Detected OS: $OS"
    print_cyan "→ Detected Architecture: $ARCH"
    echo ""

    if [ "$OS" = "unknown" ] || [ "$ARCH" = "unknown" ]; then
        print_red "✗ Unsupported platform: $OS $ARCH"
        print_yellow "  Please build from source:"
        echo "  git clone https://github.com/$REPO.git"
        echo "  cd polkadot-cookbook"
        echo "  cargo build --release --bin dot"
        exit 1
    fi

    BINARY_ARCHIVE=$(get_binary_name "$OS" "$ARCH")
    if [ -z "$BINARY_ARCHIVE" ]; then
        print_red "✗ No binary available for: $OS $ARCH"
        exit 1
    fi

    # Get latest release version
    print_cyan "→ Fetching latest release..."
    LATEST_RELEASE=$(curl -s "https://api.github.com/repos/$REPO/releases/latest" | grep '"tag_name":' | sed -E 's/.*"v([^"]+)".*/\1/')

    if [ -z "$LATEST_RELEASE" ]; then
        print_red "✗ Could not determine latest release"
        exit 1
    fi

    print_green "✓ Latest version: v$LATEST_RELEASE"
    echo ""

    # Download URL
    DOWNLOAD_URL="https://github.com/$REPO/releases/download/v$LATEST_RELEASE/$BINARY_ARCHIVE"
    print_cyan "→ Downloading from: $DOWNLOAD_URL"

    # Create temp directory
    TMP_DIR=$(mktemp -d)
    cd "$TMP_DIR"

    # Download binary
    if ! curl -fsSL "$DOWNLOAD_URL" -o "$BINARY_ARCHIVE"; then
        print_red "✗ Failed to download binary"
        rm -rf "$TMP_DIR"
        exit 1
    fi

    print_green "✓ Downloaded successfully"
    echo ""

    # Extract binary
    print_cyan "→ Extracting binary..."
    if [ "$OS" = "windows" ]; then
        unzip -q "$BINARY_ARCHIVE"
    else
        tar xzf "$BINARY_ARCHIVE"
    fi

    # Determine install location
    if [ -w "$INSTALL_DIR" ]; then
        USE_SUDO=""
    else
        if command -v sudo >/dev/null 2>&1; then
            USE_SUDO="sudo"
            print_yellow "⚠ Need sudo to install to $INSTALL_DIR"
        else
            # Fallback to user directory
            INSTALL_DIR="$HOME/.local/bin"
            mkdir -p "$INSTALL_DIR"
            USE_SUDO=""
            print_yellow "⚠ Installing to $INSTALL_DIR instead"
        fi
    fi

    # Install binary
    print_cyan "→ Installing to $INSTALL_DIR..."
    $USE_SUDO mv "$BINARY_NAME" "$INSTALL_DIR/"
    $USE_SUDO chmod +x "$INSTALL_DIR/$BINARY_NAME"

    # Clean up
    cd - > /dev/null
    rm -rf "$TMP_DIR"

    print_green "✓ Binary installed successfully"
    echo ""

    # Check if install directory is in PATH
    if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
        print_yellow "⚠ Warning: $INSTALL_DIR is not in your PATH"
        echo ""
        print_cyan "  Add it to your PATH by adding this to your shell config:"
        echo "  export PATH=\"$INSTALL_DIR:\$PATH\""
        echo ""
    fi

    # Verify installation
    if command -v "$BINARY_NAME" >/dev/null 2>&1; then
        VERSION=$("$BINARY_NAME" --version 2>/dev/null || echo "unknown")
        print_green "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
        print_green "  ✓ Installation complete!"
        print_green "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
        echo ""
        print_cyan "  Installed: $VERSION"
        print_cyan "  Location: $INSTALL_DIR/$BINARY_NAME"
        echo ""
        print_pink "  Get started:"
        echo "    dot --help          # Show all commands"
        echo "    dot setup           # Setup development environment"
        echo "    dot doctor          # Check your environment"
        echo "    dot recipe create   # Create a new recipe"
        echo ""
    else
        print_yellow "⚠ Installation complete, but 'dot' not found in PATH"
        echo ""
        print_cyan "  Run with full path: $INSTALL_DIR/$BINARY_NAME"
        echo ""
    fi
}

main "$@"
