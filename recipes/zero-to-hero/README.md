---
title: Zero to Hero
description: Build and deploy your first parachain from scratch. A complete hands-on introduction to Polkadot SDK development.
difficulty: Beginner
content_type: tutorial
categories: Basics, Parachains, Getting Started
---

# Zero to Hero

Build and deploy your first parachain from scratch. This comprehensive tutorial takes you from zero knowledge to running a fully functional parachain on a local test network.

## Prerequisites

- Rust `1.86+` (check with `rustc --version`)
- Node.js `20+` (check with `node --version`)
- Basic understanding of blockchain concepts
- Familiarity with command-line interfaces

## Learning Objectives

By the end of this recipe, you will:
- Understand the basic structure of a Polkadot SDK parachain
- Know how to build and compile a parachain runtime
- Be able to launch a local relay chain and parachain network
- Successfully deploy and test your parachain

## Steps

### 1. Setup Environment

```bash
cd recipes/zero-to-hero
npm install
```

### 2. Build the Parachain

The parachain uses the kitchensink parachain template:

```bash
cd src
cargo build --release
```

This will compile your parachain runtime and node binary.

### 3. Run the Local Test Network

Use the provided scripts to launch a local Rococo relay chain and connect your parachain:

```bash
# From the zero-to-hero directory
just start-network
```

Or manually using the scripts in `scripts/`:

```bash
./scripts/start-relay.sh
./scripts/start-para.sh
```

### 4. Test Your Parachain

Run the end-to-end tests to verify your parachain is working correctly:

```bash
npm test
```

The tests will:
- Connect to your local parachain node
- Verify the runtime is operational
- Test basic functionality

## Testing

To run the full test suite:

```bash
cd recipes/zero-to-hero
npm test
```

Tests use the fast-skip pattern, so they'll gracefully skip if no node is running.

## Next Steps

Now that you have a working parachain, you can:
- Explore other recipes to add custom pallets
- Learn about XCM for cross-chain messaging
- Implement custom runtime logic
- Deploy to a public testnet

## Resources

- [Polkadot SDK Documentation](https://docs.substrate.io/)
- [Parachain Development Guide](https://wiki.polkadot.network/docs/build-pdk)
- [Zombienet Documentation](https://github.com/paritytech/zombienet)
