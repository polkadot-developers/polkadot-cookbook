#!/bin/bash
# Setup environment for a given tutorial using its versions.yml
# Usage: scripts/setup-tutorial-env.sh <tutorial-slug>

set -euo pipefail

if [[ $# -lt 1 ]]; then
  echo "Usage: $0 <tutorial-slug>" >&2
  exit 2
fi

SLUG="$1"
REPO_ROOT=$(cd "$(dirname "$0")"/.. && pwd)
TUTORIAL_DIR="$REPO_ROOT/tutorials/$SLUG"

if [[ ! -d "$TUTORIAL_DIR" ]]; then
  echo "Tutorial directory not found: $TUTORIAL_DIR" >&2
  exit 3
fi

# Load versions with preference to tutorial-local versions.yml
export TUTORIAL_DIR
source "$REPO_ROOT/scripts/load-versions.sh"

echo "Setting up tutorial '$SLUG' with versions:" \
  "RUST=${RUST_VERSION}, OMNI=${OMNI_NODE_VERSION}, CSB=${CHAIN_SPEC_BUILDER_VERSION}"

# Run tutorial-specific installers
pushd "$TUTORIAL_DIR/scripts" >/dev/null

echo "Installing Rust toolchain..."
bash ./setup-rust.sh

echo "Installing chain-spec-builder..."
bash ./install-chain-spec-builder.sh

echo "Installing polkadot-omni-node..."
bash ./install-omni-node.sh

popd >/dev/null

echo "Environment setup completed for '$SLUG'"


