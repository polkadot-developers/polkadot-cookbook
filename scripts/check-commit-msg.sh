#!/bin/bash
# Check commit message format (warning only, non-blocking)

COMMIT_MSG_FILE=$1
COMMIT_MSG=$(cat "$COMMIT_MSG_FILE")

# Valid conventional commit types
VALID_TYPES="feat|fix|docs|test|refactor|chore|ci|build"

# Pattern for conventional commits: type(optional-scope): description
PATTERN="^($VALID_TYPES)(\(.+\))?: .{1,}"

# Color codes
YELLOW='\033[1;33m'
GREEN='\033[0;32m'
NC='\033[0m' # No Color

# Check if it's a merge commit or revert
if echo "$COMMIT_MSG" | grep -qE "^Merge|^Revert"; then
    exit 0
fi

# Check conventional commit format
if ! echo "$COMMIT_MSG" | grep -qE "$PATTERN"; then
    echo ""
    echo -e "${YELLOW}⚠️  WARNING: Commit message doesn't follow conventional commit format${NC}"
    echo ""
    echo "Your message:"
    echo "  $COMMIT_MSG"
    echo ""
    echo "Recommended format:"
    echo "  type(scope): description"
    echo ""
    echo "Valid types:"
    echo "  feat     - New feature"
    echo "  fix      - Bug fix"
    echo "  docs     - Documentation changes"
    echo "  test     - Test changes"
    echo "  refactor - Code refactoring"
    echo "  chore    - Maintenance tasks"
    echo "  ci       - CI/CD changes"
    echo "  build    - Build system changes"
    echo ""
    echo "Examples:"
    echo -e "  ${GREEN}feat(recipe): add zero-to-hero recipe${NC}"
    echo -e "  ${GREEN}fix(cli): correct validation error${NC}"
    echo -e "  ${GREEN}docs: update CONTRIBUTING.md${NC}"
    echo ""
    echo "This is just a warning - commit will proceed anyway."
    echo ""
else
    echo -e "${GREEN}✓${NC} Commit message follows conventional format"
fi

# Always exit 0 (non-blocking)
exit 0
