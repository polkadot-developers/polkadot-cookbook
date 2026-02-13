#!/usr/bin/env bash
set -euo pipefail

# Check if upstream polkadot-docs tutorials have changed since we last tested them.
# Compares the docs_commit in each README frontmatter against the latest commit
# for that file in polkadot-developers/polkadot-docs.

REPO="polkadot-developers/polkadot-docs"
CHANGED=""
COUNT=0

# Find all polkadot-docs READMEs with source_github and docs_commit
while IFS= read -r readme; do
  # Extract source_github URL from frontmatter
  SOURCE_GITHUB=$(sed -n 's/^source_github: "\(.*\)"/\1/p' "$readme")
  if [ -z "$SOURCE_GITHUB" ]; then
    continue
  fi

  # Extract docs_commit from frontmatter
  DOCS_COMMIT=$(sed -n 's/^docs_commit: "\(.*\)"/\1/p' "$readme")
  if [ -z "$DOCS_COMMIT" ]; then
    echo "SKIP $readme: no docs_commit field"
    continue
  fi

  # Extract file path from GitHub URL
  # e.g. https://github.com/polkadot-developers/polkadot-docs/blob/master/parachains/foo.md
  #   -> parachains/foo.md
  FILE_PATH=$(echo "$SOURCE_GITHUB" | sed 's|.*/blob/master/||')

  # Extract title from frontmatter
  TITLE=$(sed -n 's/^title: "\(.*\)"/\1/p' "$readme")

  # Query GitHub API for latest commit on this file
  LATEST_COMMIT=$(gh api "repos/$REPO/commits?path=$FILE_PATH&per_page=1&sha=master" --jq '.[0].sha' 2>/dev/null || echo "")

  if [ -z "$LATEST_COMMIT" ]; then
    echo "WARN $readme: could not fetch latest commit for $FILE_PATH"
    continue
  fi

  echo "CHECK $TITLE: known=$DOCS_COMMIT latest=$LATEST_COMMIT"

  if [ "$DOCS_COMMIT" != "$LATEST_COMMIT" ]; then
    echo "  -> CHANGED"
    COUNT=$((COUNT + 1))
    CHANGED="${CHANGED}- **${TITLE}** — [view diff](https://github.com/$REPO/compare/${DOCS_COMMIT}...${LATEST_COMMIT})\n  - Test: \`${readme}\`\n  - Doc: \`${FILE_PATH}\`\n"
  else
    echo "  -> Up to date"
  fi
done < <(find polkadot-docs -name README.md -not -path '*/node_modules/*')

# Set outputs
if [ "$COUNT" -gt 0 ]; then
  echo "has_drift=true" >> "$GITHUB_OUTPUT"
  echo "drift_count=$COUNT" >> "$GITHUB_OUTPUT"
  {
    echo "drift_details<<EOF"
    echo -e "$CHANGED"
    echo "EOF"
  } >> "$GITHUB_OUTPUT"
else
  echo "has_drift=false" >> "$GITHUB_OUTPUT"
  echo "drift_count=0" >> "$GITHUB_OUTPUT"
fi

echo ""
echo "Done. $COUNT tutorial(s) with upstream changes."
