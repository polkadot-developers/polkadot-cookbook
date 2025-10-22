#!/bin/bash
# Build kitchensink-parachain and generate chain spec for a tutorial
# Usage: scripts/prepare-node-for-tutorial.sh <tutorial-slug>

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

export TUTORIAL_DIR
source "$REPO_ROOT/scripts/load-versions.sh"

echo "Preparing node for tutorial '$SLUG'"
echo "Using Rust ${RUST_VERSION}"

# Ensure rust toolchain and targets
pushd "$TUTORIAL_DIR/scripts" >/dev/null
bash ./setup-rust.sh
popd >/dev/null

# Build kitchensink-parachain runtime
pushd "$REPO_ROOT/kitchensink-parachain" >/dev/null
echo "Building kitchensink-parachain runtime..."
cargo build --release
popd >/dev/null

# Generate chain spec into kitchensink-parachain/chain_spec.json
pushd "$TUTORIAL_DIR/scripts" >/dev/null
bash ./generate-chain-spec.sh
popd >/dev/null

echo "Node preparation completed for '$SLUG'"


