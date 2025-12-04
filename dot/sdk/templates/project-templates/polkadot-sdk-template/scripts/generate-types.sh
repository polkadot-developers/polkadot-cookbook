#!/usr/bin/env bash
# Generate TypeScript types from running node

set -e

# Check if node is accessible
if ! curl -s --max-time 2 http://localhost:9944 > /dev/null 2>&1; then
    echo "⚠️  Warning: Node not accessible at ws://localhost:9944"
    echo "   Please start the node first: npm run start:node"
    exit 1
fi

# Generate types
papi
