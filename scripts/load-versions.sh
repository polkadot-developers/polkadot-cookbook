#!/bin/bash
# Load versions for tools from either a tutorial's versions.yml or the repo root versions.yml
# Exports: RUST_VERSION, OMNI_NODE_VERSION, CHAIN_SPEC_BUILDER_VERSION

set -e

# Detect repo root from this script's location
REPO_ROOT=$(cd "$(dirname "${BASH_SOURCE[0]}")"/.. && pwd)

# Inputs (optional):
# - TUTORIAL_DIR: absolute path to tutorial directory (containing versions.yml)
# - TUTORIAL_KEY: key in root versions.yml for per-tutorial overrides (e.g., zero_to_hero)

_parse_flat_yaml() {
  # Very simple YAML parser for flat key: value pairs (no nesting)
  # Args: file_path, key
  local file_path="$1"; shift
  local key="$1"; shift
  awk -v k="$key" '
    BEGIN { FS=":" }
    /^[[:space:]]*#/ { next }                # skip comments
    NF==0 { next }                           # skip empty
    {
      # capture lines like: key: value or key: "value"
      gsub(/^[[:space:]]+|[[:space:]]+$/, "", $1)
      if ($1 == k) {
        sub(/^[^:]*:[[:space:]]*/, "", $0)
        # trim quotes and whitespace
        gsub(/^[\"\'\s]+|[\"\'\s]+$/, "", $0)
        print $0
        exit 0
      }
    }
  ' "$file_path"
}

_extract_block_value() {
  # Extract a value for a subkey inside a top-level block in root versions.yml
  # Args: file_path, block_key (e.g., zero_to_hero), sub_key (e.g., rust)
  local file_path="$1"; shift
  local block_key="$1"; shift
  local sub_key="$1"; shift
  awk -v block="$block_key" -v subk="$sub_key" '
    /^[[:space:]]*#/ { next }
    /^\s*$/ { next }
    {
      # Enter block when line is exactly: block:
      if ($0 ~ "^" block ":[[:space:]]*$") { in_block=1; next }
      # Leave block when a new top-level key is encountered
      if ($0 ~ /^[^[:space:]].*:[[:space:]]*$/ && $0 !~ "^" block ":[[:space:]]*$") { in_block=0 }
      if (in_block==1) {
        # lines inside block: two or more spaces then key: value
        if ($0 ~ /^[[:space:]]{2,}[^[:space:]]+:[[:space:]]*.+$/) {
          line=$0
          sub(/^\s+/, "", line)
          split(line, parts, ":")
          key=parts[1]
          gsub(/^\s+|\s+$/, "", key)
          if (key == subk) {
            val=line
            sub(/^[^:]*:[[:space:]]*/, "", val)
            gsub(/^[\"\'\s]+|[\"\'\s]+$/, "", val)
            print val
            exit 0
          }
        }
      }
    }
  ' "$file_path"
}

load_versions() {
  local tutorial_dir="${TUTORIAL_DIR:-}"
  local tutorial_key="${TUTORIAL_KEY:-}"

  if [[ -n "$tutorial_dir" && -f "$tutorial_dir/versions.yml" ]]; then
    # Prefer tutorial-local versions.yml (flat keys)
    local rust=$(_parse_flat_yaml "$tutorial_dir/versions.yml" rust)
    local omni=$(_parse_flat_yaml "$tutorial_dir/versions.yml" polkadot_omni_node)
    local csb=$(_parse_flat_yaml "$tutorial_dir/versions.yml" chain_spec_builder)
    if [[ -n "$rust" ]]; then export RUST_VERSION="$rust"; fi
    if [[ -n "$omni" ]]; then export OMNI_NODE_VERSION="$omni"; fi
    if [[ -n "$csb" ]]; then export CHAIN_SPEC_BUILDER_VERSION="$csb"; fi
    return 0
  fi

  # Fallback to root versions.yml
  local root_file="$REPO_ROOT/versions.yml"
  if [[ ! -f "$root_file" ]]; then
    echo "versions.yml not found at $REPO_ROOT" >&2
    return 1
  fi

  # Defaults under versions:
  local def_rust=$(_extract_block_value "$root_file" versions rust)
  local def_omni=$(_extract_block_value "$root_file" versions polkadot_omni_node)
  local def_csb=$(_extract_block_value "$root_file" versions chain_spec_builder)

  export RUST_VERSION="$def_rust"
  export OMNI_NODE_VERSION="$def_omni"
  export CHAIN_SPEC_BUILDER_VERSION="$def_csb"

  # Optional overrides by tutorial key
  if [[ -z "$tutorial_key" && -n "$tutorial_dir" ]]; then
    # Derive key from dir name: replace dashes with underscores
    local slug
    slug=$(basename "$tutorial_dir")
    tutorial_key=${slug//-/_}
  fi

  if [[ -n "$tutorial_key" ]]; then
    local ov_rust=$(_extract_block_value "$root_file" "$tutorial_key" rust)
    local ov_omni=$(_extract_block_value "$root_file" "$tutorial_key" polkadot_omni_node)
    local ov_csb=$(_extract_block_value "$root_file" "$tutorial_key" chain_spec_builder)
    if [[ -n "$ov_rust" ]]; then export RUST_VERSION="$ov_rust"; fi
    if [[ -n "$ov_omni" ]]; then export OMNI_NODE_VERSION="$ov_omni"; fi
    if [[ -n "$ov_csb" ]]; then export CHAIN_SPEC_BUILDER_VERSION="$ov_csb"; fi
  fi
}

# If the script is sourced, export variables; if executed, print them JSON-ish
if [[ "${BASH_SOURCE[0]}" == "$0" ]]; then
  load_versions
  echo "RUST_VERSION=$RUST_VERSION"
  echo "OMNI_NODE_VERSION=$OMNI_NODE_VERSION"
  echo "CHAIN_SPEC_BUILDER_VERSION=$CHAIN_SPEC_BUILDER_VERSION"
else
  load_versions
fi


