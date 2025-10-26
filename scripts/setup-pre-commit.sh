#!/bin/bash
# Setup pre-commit hooks for Polkadot Cookbook
set -e

echo "🔧 Setting up pre-commit hooks..."

# Check if pre-commit is installed
if ! command -v pre-commit &> /dev/null; then
    echo "📦 Installing pre-commit..."

    # Try to install with pip
    if command -v pip3 &> /dev/null; then
        pip3 install pre-commit
    elif command -v pip &> /dev/null; then
        pip install pre-commit
    else
        echo "❌ Error: pip is not installed"
        echo "Please install Python and pip first:"
        echo "  - macOS: brew install python"
        echo "  - Ubuntu: sudo apt install python3-pip"
        echo "  - Or visit: https://www.python.org/downloads/"
        exit 1
    fi
fi

# Install the git hook scripts
echo "🪝 Installing git hooks..."
pre-commit install
pre-commit install --hook-type commit-msg

echo ""
echo "✅ Pre-commit hooks installed successfully!"
echo ""
echo "The following checks will run before each commit:"
echo "  • cargo fmt - Format Rust code"
echo "  • cargo clippy - Lint Rust code"
echo "  • YAML/JSON/TOML syntax checks"
echo "  • Markdown linting"
echo "  • Conventional commit format (warning only)"
echo ""
echo "To run checks manually:"
echo "  pre-commit run --all-files"
echo ""
echo "To skip hooks (use sparingly):"
echo "  git commit --no-verify"
echo ""
