#!/usr/bin/env bash
set -euo pipefail

# Sync versions from polkadot-docs upstream variables.yml into cookbook versions.yml.
# Only updates if upstream is strictly newer — never downgrades.

UPSTREAM_URL="https://raw.githubusercontent.com/polkadot-developers/polkadot-docs/master/variables.yml"
LOCAL_FILE="versions.yml"
UPSTREAM_FILE="/tmp/upstream-variables.yml"

curl -sSf "$UPSTREAM_URL" -o "$UPSTREAM_FILE"

CHANGELOG=""
HAS_UPDATES=false

# Compare two version strings.  Returns 0 if $2 > $1 (upstream is newer).
# Handles semver (v0.0.5, 0.13.0) and polkadot-sdk tags (polkadot-stable2512-1).
is_upstream_newer() {
  local local_ver="$1"
  local upstream_ver="$2"

  # Strip leading 'v' for comparison
  local lv="${local_ver#v}"
  local uv="${upstream_ver#v}"

  # For polkadot-stable tags: extract YYMM-patch portion
  if [[ "$lv" == polkadot-stable* ]]; then
    lv="${lv#polkadot-stable}"
    uv="${uv#polkadot-stable}"
    # Convert e.g. 2512-1 to 2512.1 for sort -V
    lv="${lv//-/.}"
    uv="${uv//-/.}"
  fi

  if [ "$lv" = "$uv" ]; then
    return 1  # same version
  fi

  # sort -V puts the smaller version first
  local smaller
  smaller=$(printf '%s\n%s\n' "$lv" "$uv" | sort -V | head -n1)
  if [ "$smaller" = "$lv" ]; then
    return 0  # upstream is newer
  else
    return 1  # local is newer or equal
  fi
}

# Update a single version entry if upstream is newer.
# Arguments: description, local yq path, upstream yq path
check_and_update() {
  local desc="$1"
  local local_path="$2"
  local upstream_path="$3"

  local local_ver
  local upstream_ver

  local_ver=$(yq "$local_path" "$LOCAL_FILE" 2>/dev/null || echo "null")
  upstream_ver=$(yq "$upstream_path" "$UPSTREAM_FILE" 2>/dev/null || echo "null")

  if [ "$local_ver" = "null" ] || [ "$upstream_ver" = "null" ]; then
    echo "SKIP $desc: local=$local_ver upstream=$upstream_ver"
    return
  fi

  echo "CHECK $desc: local=$local_ver upstream=$upstream_ver"

  if is_upstream_newer "$local_ver" "$upstream_ver"; then
    echo "  -> Updating $desc: $local_ver -> $upstream_ver"
    yq -i "$local_path = \"$upstream_ver\"" "$LOCAL_FILE"
    CHANGELOG="${CHANGELOG}- **${desc}**: \`${local_ver}\` -> \`${upstream_ver}\`\n"
    HAS_UPDATES=true
  else
    echo "  -> Up to date (or ahead)"
  fi
}

# Define version mappings: cookbook path <-> upstream path
check_and_update \
  "Polkadot SDK" \
  ".polkadot_sdk.release_tag" \
  ".dependencies.repositories.polkadot_sdk.version"

check_and_update \
  "Parachain Template" \
  ".parachain_template.version" \
  ".dependencies.repositories.polkadot_sdk_parachain_template.version"

check_and_update \
  "Polkadot Omni Node" \
  ".parachain_template.crates.polkadot_omni_node.version" \
  ".dependencies.crates.polkadot_omni_node.version"

check_and_update \
  "Chain Spec Builder" \
  ".parachain_template.crates.chain_spec_builder.version" \
  ".dependencies.repositories.polkadot_sdk_parachain_template.subdependencies.chain_spec_builder_version"

check_and_update \
  "Zombienet" \
  ".zombienet.version" \
  ".dependencies.repositories.zombienet.version"

# Set outputs for the workflow
if [ "$HAS_UPDATES" = true ]; then
  echo "has_updates=true" >> "$GITHUB_OUTPUT"
  {
    echo "changelog<<EOF"
    echo -e "$CHANGELOG"
    echo "EOF"
  } >> "$GITHUB_OUTPUT"
else
  echo "has_updates=false" >> "$GITHUB_OUTPUT"
fi

echo ""
echo "Done. has_updates=$HAS_UPDATES"
