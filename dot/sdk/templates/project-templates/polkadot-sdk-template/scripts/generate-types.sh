#!/usr/bin/env bash
# Generate TypeScript types from running node

set -e

echo "🔍 Checking if node is ready..."

# Check if node is accessible and ready
# Try to get the system_chain RPC call with a timeout
MAX_RETRIES=30
RETRY_COUNT=0

while [ $RETRY_COUNT -lt $MAX_RETRIES ]; do
    if curl -s --max-time 2 -H "Content-Type: application/json" \
        -d '{"id":1,"jsonrpc":"2.0","method":"system_chain","params":[]}' \
        http://localhost:9944 2>/dev/null | grep -q "result"; then
        echo "✅ Node is ready!"
        break
    fi

    RETRY_COUNT=$((RETRY_COUNT + 1))
    if [ $RETRY_COUNT -eq $MAX_RETRIES ]; then
        echo ""
        echo "⚠️  Warning: Node not ready at ws://localhost:9944"
        echo "   Please ensure the node is running and fully initialized:"
        echo "   npm run start:node"
        echo ""
        echo "   Wait for the node to show 'Idle' or block production messages"
        exit 1
    fi

    echo -n "."
    sleep 1
done

echo "📝 Generating TypeScript types..."
papi
