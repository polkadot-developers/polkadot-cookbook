#!/bin/bash
# Start the parachain node in dev mode
# Usage: ./run-node.sh [template_dir] [chain_spec_path]

set -e

TEMPLATE_DIR="${1:-./parachain-template}"
CHAIN_SPEC="${2:-./chain_spec.json}"
PID_FILE="${TEMPLATE_DIR}/.node.pid"
LOG_FILE="${TEMPLATE_DIR}/.node.log"

# Function to generate chain spec
generate_chain_spec() {
    echo "=== Generating Chain Specification ==="

    WASM_PATH="${TEMPLATE_DIR}/target/release/wbuild/parachain-template-runtime/parachain_template_runtime.compact.compressed.wasm"

    if [ ! -f "$WASM_PATH" ]; then
        echo "ERROR: WASM runtime not found. Run clone-and-build.sh first."
        exit 1
    fi

    chain-spec-builder create -t development \
        --relay-chain paseo \
        --para-id 1000 \
        --runtime "$WASM_PATH" \
        named-preset development

    # Move to expected location
    mv chain_spec.json "$CHAIN_SPEC"

    echo "Chain spec generated: $CHAIN_SPEC"
}

# Function to start the node
start_node() {
    echo "=== Starting Parachain Node ==="

    if [ ! -f "$CHAIN_SPEC" ]; then
        generate_chain_spec
    fi

    # Start node in background
    polkadot-omni-node --chain "$CHAIN_SPEC" --dev > "$LOG_FILE" 2>&1 &
    NODE_PID=$!
    echo $NODE_PID > "$PID_FILE"

    echo "Node started with PID: $NODE_PID"
    echo "Logs: $LOG_FILE"

    # Wait for node to be ready (check RPC endpoint)
    echo "Waiting for node to be ready..."
    for i in {1..60}; do
        if curl -s -X POST -H "Content-Type: application/json" \
            --data '{"jsonrpc":"2.0","method":"system_health","params":[],"id":1}' \
            http://127.0.0.1:9944 > /dev/null 2>&1; then
            echo "Node is ready!"
            return 0
        fi
        sleep 1
    done

    echo "ERROR: Node failed to start within 60 seconds"
    stop_node
    exit 1
}

# Function to stop the node
stop_node() {
    echo "=== Stopping Parachain Node ==="

    if [ -f "$PID_FILE" ]; then
        NODE_PID=$(cat "$PID_FILE")
        if kill -0 "$NODE_PID" 2>/dev/null; then
            kill "$NODE_PID"
            echo "Node stopped (PID: $NODE_PID)"
        fi
        rm -f "$PID_FILE"
    fi
}

# Main
case "${1:-start}" in
    start)
        shift || true
        start_node
        ;;
    stop)
        stop_node
        ;;
    generate)
        generate_chain_spec
        ;;
    *)
        echo "Usage: $0 {start|stop|generate} [template_dir] [chain_spec_path]"
        exit 1
        ;;
esac
