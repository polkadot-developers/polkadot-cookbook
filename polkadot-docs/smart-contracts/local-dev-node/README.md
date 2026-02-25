---
title: "Local Development Node"
description: "Verify the Local Development Node setup guide from docs.polkadot.com"
source_url: "https://docs.polkadot.com/smart-contracts/dev-environments/local-dev-node/"
source_repo: "https://github.com/polkadot-developers/polkadot-docs/blob/master/smart-contracts/dev-environments/local-dev-node.md"
---

# Local Development Node

[![Local Development Node](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-local-dev-node.yml/badge.svg?event=push)](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-local-dev-node.yml)

This project verifies the [Local Development Node](https://docs.polkadot.com/smart-contracts/dev-environments/local-dev-node/) guide from docs.polkadot.com.

The tests clone [polkadot-sdk](https://github.com/paritytech/polkadot-sdk) at the release tag pinned in `versions.yml`, build the `revive-dev-node` and `eth-rpc` binaries, run both processes, and confirm that the Ethereum JSON-RPC endpoint at `http://localhost:8545` responds correctly.

## What This Tests

1. **Prerequisites** — Rust, Cargo, and git are available
2. **Clone polkadot-sdk** — Shallow-clones the repo at the pinned release tag and verifies both target crates exist in the workspace
3. **Build revive-dev-node** — `cargo build -p revive-dev-node --bin revive-dev-node --release`
4. **Build eth-rpc** — `cargo build -p pallet-revive-eth-rpc --bin eth-rpc --release`
5. **Run revive-dev-node** — Starts `revive-dev-node --dev` and waits for the Substrate RPC to be available on port 9944
6. **Run eth-rpc adapter** — Starts `eth-rpc --dev` (connects to the dev node at `ws://127.0.0.1:9944`) and waits for port 8545
7. **Verify ETH-RPC endpoint** — Confirms `eth_chainId`, `eth_blockNumber`, `net_version`, and `eth_gasPrice` return valid responses

## Prerequisites

- Rust (stable toolchain)
- Cargo
- git
- System build dependencies: `protobuf-compiler`, `clang`, `libclang-dev`, `make`

On Ubuntu/Debian:
```bash
sudo apt-get install -y protobuf-compiler clang libclang-dev make
```

## Running Tests Locally

```bash
# 1. Install wrapper dependencies
npm install

# 2. Run all verification tests
npm test
```

The first run clones polkadot-sdk and compiles the binaries (~30 min). Subsequent runs reuse the `.test-workspace/` directory and skip the build step automatically.

## Environment Variables

| Variable | Description |
|---|---|
| `POLKADOT_SDK_TAG` | polkadot-sdk release tag to clone (default: value from `versions.yml`) |

## Test Phases

### 1. Prerequisites
Checks that `rustc`, `cargo`, and `git` are present.

### 2. Clone polkadot-sdk
Shallow-clones `paritytech/polkadot-sdk` at the pinned release tag. Verifies both `revive-dev-node` and `pallet-revive-eth-rpc` appear in `cargo metadata`.

### 3. Build revive-dev-node
Runs `cargo build -p revive-dev-node --bin revive-dev-node --release`. Skipped if the binary already exists (cache hit).

### 4. Build eth-rpc
Runs `cargo build -p pallet-revive-eth-rpc --bin eth-rpc --release`. Skipped if the binary already exists (cache hit).

### 5. Run revive-dev-node
Spawns `revive-dev-node --dev` in the background and polls `system_health` on port 9944 until the node is ready. Verifies block production.

### 6. Run eth-rpc adapter
Spawns `eth-rpc --dev` in the background. The adapter auto-connects to `ws://127.0.0.1:9944` and polls `eth_chainId` on port 8545 until ready.

### 7. Verify ETH-RPC endpoint
Sends four Ethereum JSON-RPC calls to `http://localhost:8545`:
- `eth_chainId` — must return a hex-encoded chain ID
- `eth_blockNumber` — must return a hex-encoded block number
- `net_version` — must return a non-empty string
- `eth_gasPrice` — must return a hex-encoded gas price

## Exact Replication Steps

```bash
# 1. Clone polkadot-sdk
git clone https://github.com/paritytech/polkadot-sdk.git
cd polkadot-sdk

# 2. Build revive-dev-node
cargo build -p revive-dev-node --bin revive-dev-node --release

# 3. Build eth-rpc adapter
cargo build -p pallet-revive-eth-rpc --bin eth-rpc --release

# 4. Terminal 1 — start the dev node
./target/release/revive-dev-node --dev

# 5. Terminal 2 — start the ETH-RPC adapter
./target/release/eth-rpc --dev

# 6. Verify the endpoint (any terminal)
curl -s -X POST http://localhost:8545 \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"eth_chainId","params":[],"id":1}'
```

## Versions Tested

| Component | Version |
|---|---|
| polkadot-sdk | `polkadot_sdk.release_tag` from `versions.yml` |
| Node.js | v22.13.1+ |
| Rust | stable |

## Source

- [docs.polkadot.com guide](https://docs.polkadot.com/smart-contracts/dev-environments/local-dev-node/)
- [polkadot-sdk repository](https://github.com/paritytech/polkadot-sdk)
