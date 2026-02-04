# Run a Parachain Network

This folder contains verification tests for the [Run a Parachain Network](https://docs.polkadot.com/parachains/testing/run-a-parachain-network/) guide from docs.polkadot.com.

## What This Test Verifies

1. **Prerequisites**: Rust, cargo, and Zombienet are available
2. **Clone Repository**: OpenZeppelin's polkadot-runtime-templates is cloned
3. **Build Parachain Binary**: `cargo build --release` succeeds
4. **Download Relay Chain**: Polkadot binary is downloaded via `zombienet setup`
5. **Spawn Network**: Zombienet spawns a network with relay chain and parachain
6. **Block Production**: Both relay chain and parachain produce blocks

## Running Tests

```bash
# Install dependencies
npm ci

# Run tests (requires Zombienet installed)
npm test
```

## Prerequisites

- Rust toolchain (see `rust-toolchain.toml`)
- [Zombienet](https://github.com/paritytech/zombienet) installed and in PATH
- Node.js 22+

## Test Duration

This test suite takes approximately 30-45 minutes due to:
- Cloning the repository (~1 min)
- Building the parachain binary (~15-30 min)
- Downloading relay chain binary (~2-5 min)
- Spawning network and verifying blocks (~2-5 min)

## Network Configuration

The test uses `configs/network.toml` which defines:
- A rococo-local relay chain with 2 validators (alice, bob)
- A parachain (id 1000) with 1 collator

## Source Documentation

- Guide: [docs.polkadot.com/parachains/testing/run-a-parachain-network](https://docs.polkadot.com/parachains/testing/run-a-parachain-network/)
- Zombienet: [github.com/paritytech/zombienet](https://github.com/paritytech/zombienet)
- OpenZeppelin Templates: [github.com/OpenZeppelin/polkadot-runtime-templates](https://github.com/OpenZeppelin/polkadot-runtime-templates)
