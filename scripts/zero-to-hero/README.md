# Parachain Development Scripts

This directory contains scripts to set up and run a local parachain development environment.

## Generated Configuration

These scripts were generated automatically with the following versions:
- **Rust**: `1.86`
- **Chain Spec Builder**: `10.0.0`
- **Omni Node**: `0.5.0`
- **Para ID**: `1000`
- **Relay Chain**: `paseo`

**Version Changes**:  polkadot_omni_node(0.4.0→0.5.0)

## Quick Start

1. **Setup Rust toolchain:**
   ```bash
   ./scripts/zero-to-hero/setup-rust.sh
   ```

2. **Install chain-spec-builder:**
   ```bash
   ./scripts/zero-to-hero/install-chain-spec-builder.sh
   ```

3. **Install omni-node:**
   ```bash
   ./scripts/zero-to-hero/install-omni-node.sh
   ```

4. **Build your parachain runtime** (if not already done):
   ```bash
   cargo build --release
   ```

5. **Generate chain specification:**
   ```bash
   ./scripts/zero-to-hero/generate-chain-spec.sh
   ```

6. **Start the parachain node:**
   ```bash
   ./scripts/zero-to-hero/start-node.sh
   ```

## Script Details

### `setup-rust.sh`
Sets up the Rust toolchain with the required version and adds WASM compilation support.

### `install-chain-spec-builder.sh`
Installs the staging-chain-spec-builder tool for generating chain specifications.

### `install-omni-node.sh`
Installs the polkadot-omni-node for running the parachain.

### `generate-chain-spec.sh`
Creates a development chain specification file (`chain_spec.json`) for your parachain.

### `start-node.sh`
Starts the parachain node in development mode with RPC endpoints enabled.

## RPC Endpoints

Once the node is running, you can access:
- **HTTP RPC**: `http://localhost:9944`
- **WebSocket RPC**: `ws://localhost:9944`

## Troubleshooting

- **Runtime not found**: Make sure to build your parachain with `cargo build --release` before generating the chain spec
- **Chain spec not found**: Run `./scripts/zero-to-hero/generate-chain-spec.sh` before starting the node
- **Port already in use**: Stop any existing nodes or change the port using `--port` and `--rpc-port` flags

## Repository Information

- **Source**: `https://github.com/polkadot-developers/polkadot-docs-tests/`
- **Branch**: `master`
- **Generated**: $(date -Iseconds)
- **Workflow**: https://github.com/polkadot-developers/polkadot-docs-tests/actions/runs/17175701069
- **Commit**: 4b873493cf8994781c1e09b08f4b108cbb7e407a
