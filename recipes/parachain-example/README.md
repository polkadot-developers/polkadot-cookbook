---
title: Parachain Example
description: Replace with a short description.
pathway: pallets
---

# Parachain Example

> A complete parachain development environment with PAPI integration testing.

## Overview

This tutorial guides you through developing a custom Polkadot parachain using the Polkadot SDK. You'll build a fully functional parachain runtime, test it with the Polkadot API (PAPI), and learn how to iterate quickly on your custom pallets.

**What you'll build:**
- A complete parachain runtime with 12+ essential pallets
- Custom FRAME pallets integrated into the runtime
- TypeScript integration tests using PAPI
- Local development node using polkadot-omni-node

**Who this is for:**
- Developers building custom parachains
- Teams creating application-specific blockchains
- Anyone learning Polkadot SDK runtime development

## Prerequisites

Before starting, ensure you have:

- **Rust 1.91+** - The Polkadot SDK requires a recent Rust toolchain
- **Node.js 20+** - For PAPI tooling and TypeScript tests
- **polkadot-omni-node** - Install via: `cargo install polkadot-omni-node`
- **Basic knowledge** of Rust and blockchain concepts

Optional but recommended:
- **Zombienet** - For multi-chain testing: `npm install -g @zombienet/cli`

## What You'll Learn

By completing this tutorial, you'll understand:

1. **Parachain Runtime Architecture**
   - How parachains differ from standalone chains
   - Essential pallets for parachain functionality
   - Runtime configuration and customization

2. **Custom Pallet Development**
   - Creating FRAME pallets with storage, events, and extrinsics
   - Integrating custom pallets into a runtime
   - Testing pallets in a full runtime context

3. **PAPI Integration Testing**
   - Connecting to your local parachain with TypeScript
   - Submitting transactions and querying state
   - Writing comprehensive integration tests

4. **Development Workflow**
   - Building and deploying runtime WASMs
   - Running a development node
   - Iterating quickly on runtime changes

## Project Structure

```
parachain-example/
├── pallets/
│   └── template/          # Your custom pallet
│       ├── src/
│       │   ├── lib.rs     # Pallet logic
│       │   ├── tests.rs   # Unit tests (mock runtime)
│       │   └── benchmarking.rs
│       └── Cargo.toml
├── runtime/
│   ├── src/
│   │   └── lib.rs         # Runtime configuration (12+ pallets)
│   ├── Cargo.toml
│   └── build.rs           # WASM builder config
├── tests/
│   └── template-pallet.test.ts  # PAPI integration tests
├── scripts/
│   ├── generate-spec.sh   # Generate chain specification
│   └── start-dev-node.sh  # Start local development node
├── package.json           # PAPI dependencies and scripts
├── zombienet.toml         # Single parachain network config
├── zombienet-omni-node.toml  # Omni-node configuration
├── zombienet-xcm.toml     # Multi-parachain XCM testing
└── Cargo.toml             # Workspace configuration
```

## Quick Start

### 1. Build the Runtime

First, compile your runtime to WebAssembly:

```bash
npm run build:runtime
# or: cargo build --release
```

This produces the runtime WASM at:
```
target/release/wbuild/parachain-example-runtime/parachain-example_runtime.compact.compressed.wasm
```

**Understanding WASM:** The runtime executes as WebAssembly on the parachain. This allows forkless upgrades - you can update the runtime without restarting the node.

### 2. Generate Chain Specification

Create a chain specification from your runtime:

```bash
npm run generate:spec
```

This generates `chain-spec.json` which defines your chain's genesis state, including:
- Initial balances
- Sudo key (Alice in dev mode)
- Runtime WASM
- Parachain ID

### 3. Start the Development Node

Launch your parachain locally using polkadot-omni-node:

```bash
npm run start:node
```

**What's polkadot-omni-node?** It's a white-labeled binary that can run any parachain runtime without requiring custom node code. Perfect for rapid development.

The node will:
- Start in development mode (single node, instant finality)
- Expose RPC at `ws://localhost:9944`
- Use Alice as the sudo account
- Store chain data in a temporary directory

### 4. Generate TypeScript Types

In a new terminal, generate PAPI types from your running chain:

```bash
npm run generate:types
```

This creates TypeScript definitions from your runtime metadata, enabling fully-typed blockchain interactions.

### 5. Run Integration Tests

Execute the PAPI test suite:

```bash
npm test
```

Tests will connect to your local node and verify:
- Node connectivity and chain info
- Account balances (Alice's dev account)
- Custom pallet storage and extrinsics

## Understanding the Runtime

### Core Parachain Pallets

Your runtime includes these essential pallets:

**System & Consensus:**
- `frame-system` - Core blockchain functionality
- `cumulus-pallet-parachain-system` - Parachain consensus integration
- `pallet-aura` - Block authoring (Authority Round)
- `cumulus-pallet-aura-ext` - Parachain-specific Aura extensions

**Accounts & Tokens:**
- `pallet-balances` - Native token management
- `pallet-transaction-payment` - Fee calculation and payment

**Governance:**
- `pallet-sudo` - Superuser access (dev/testing only)

**Collator Management:**
- `pallet-session` - Session key management
- `pallet-authorship` - Block author tracking
- `pallet-collator-selection` - Collator election

**Utilities:**
- `pallet-timestamp` - On-chain time
- `parachain-info` - Parachain ID and configuration

**Your Custom Logic:**
- `pallet-template` - Your custom pallet (customize this!)

### Runtime Configuration

The runtime is configured in `runtime/src/lib.rs`:

```rust
construct_runtime!(
    pub enum Runtime {
        System: frame_system = 0,
        ParachainSystem: cumulus_pallet_parachain_system = 1,
        // ... more pallets
        TemplatePallet: pallet_template = 50,
    }
);
```

Each pallet has a unique index and configuration implementation. For example:

```rust
impl pallet_template::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = ();
}
```

## Developing Your Custom Pallet

### Pallet Structure

Your custom pallet is in `pallets/template/`:

```rust
// pallets/template/src/lib.rs

#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    // Storage
    #[pallet::storage]
    pub type Something<T> = StorageValue<_, u32>;

    // Events
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        SomethingStored { value: u32, who: T::AccountId },
    }

    // Errors
    #[pallet::error]
    pub enum Error<T> {
        NoneValue,
        StorageOverflow,
    }

    // Dispatchable functions
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        pub fn do_something(origin: OriginFor<T>, value: u32) -> DispatchResult {
            let who = ensure_signed(origin)?;
            Something::<T>::put(value);
            Self::deposit_event(Event::SomethingStored { value, who });
            Ok(())
        }
    }
}
```

### Development Iteration Loop

1. **Modify your pallet** in `pallets/template/src/lib.rs`
2. **Run unit tests:** `cargo test --package pallet-template`
3. **Rebuild runtime:** `npm run build:runtime`
4. **Restart node:** Kill and run `npm run start:node` (node auto-detects new WASM)
5. **Regenerate types:** `npm run generate:types`
6. **Run integration tests:** `npm test`

### Testing Strategy

**Unit Tests (Mock Runtime):**
Located in `pallets/template/src/tests.rs`, these use a lightweight mock runtime:

```rust
#[test]
fn it_works_for_default_value() {
    new_test_ext().execute_with(|| {
        assert_eq!(TemplateModule::something(), None);
    });
}
```

Run with: `cargo test --package pallet-template`

**Integration Tests (PAPI):**
Located in `tests/template-pallet.test.ts`, these test against a real node:

```typescript
it('should query template pallet storage', async () => {
    const something = await api.query.TemplatePallet.Something.getValue();
    console.log(`Template storage value: ${something}`);
});
```

Run with: `npm test`

## PAPI Testing Deep Dive

### Connecting to Your Chain

The PAPI client connects via WebSocket:

```typescript
import { createClient } from 'polkadot-api';
import { getWsProvider } from 'polkadot-api/ws-provider/node';
import { dot } from '@polkadot-api/descriptors';

const provider = getWsProvider('ws://localhost:9944');
const client = createClient(provider);
const api = client.getTypedApi(dot);
```

### Querying Storage

```typescript
// Query account balance
const aliceAddress = '5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY';
const accountInfo = await api.query.System.Account.getValue(aliceAddress);
console.log(`Balance: ${accountInfo.data.free}`);

// Query your custom pallet
const value = await api.query.TemplatePallet.Something.getValue();
```

### Submitting Transactions

```typescript
import { sr25519CreateDerive } from '@polkadot-labs/hdkd';
import { getPolkadotSigner } from '@polkadot-labs/hdkd-helpers';

// Create Alice's signer (dev account)
const derive = sr25519CreateDerive(/* seed */);
const signer = getPolkadotSigner(/* ... */);

// Submit extrinsic
const tx = api.tx.TemplatePallet.do_something({ value: 42 });
await tx.signAndSubmit(signer);
```

## Multi-Chain Testing with Zombienet

For advanced scenarios like XCM, use Zombienet to spawn a multi-chain network.

### Basic Network (Single Parachain)

```bash
zombienet spawn zombienet-omni-node.toml
# or: zombienet spawn zombienet.toml
```

This spawns:
- 2 relay chain validators (Alice, Bob)
- 1 parachain collator running your runtime

### Multi-Parachain Network (XCM Testing)

For XCM development and cross-chain messaging:

```bash
# One-time setup (installs required binaries)
npm run setup:zombienet

# Spawn the network
npm run zombienet:xcm
```

This spawns a **Paseo testnet** with:
- 2 relay chain validators (Alice, Bob)
- 2 parachains for cross-chain messaging (IDs 1000 and 2000)
- Both parachains running your runtime

Perfect for testing:
- Cross-chain asset transfers
- Remote execution between parachains
- XCM message patterns (teleport, reserve transfer, etc.)
- Multi-hop routing

**Setup Script Handles:**
- Linux: Auto-downloads `polkadot` binary via zombienet
- All platforms: Installs `polkadot-omni-node` via cargo
- Platform-specific guidance for manual steps

**Binaries Required:**
- `polkadot` - Relay chain (use with `--chain=paseo-local`)
- `polkadot-omni-node` - Both parachains

The setup script (`npm run setup:zombienet`) will guide you through the installation process for your platform.

### Accessing the Network

- **Relay chain RPC:** `ws://localhost:9944`
- **Parachain RPC:** Check Zombienet output for assigned port

Your integration tests can connect to either endpoint.

## Building for Production

### Optimized Build

```bash
cargo build --release --features on-chain-release-build
```

The `on-chain-release-build` feature enables:
- Metadata hash generation for `CheckMetadataHash` extension
- Additional optimizations for on-chain deployment

### Runtime Size

Monitor your runtime WASM size:

```bash
ls -lh target/release/wbuild/parachain-example-runtime/*.wasm
```

**Target:** < 1-2 MB compressed for most parachains

### Benchmarking

Generate accurate weights for your extrinsics:

```bash
cargo build --release --features runtime-benchmarks
```

Benchmark execution is typically done in the node context - see Polkadot SDK docs for details.

## Troubleshooting

### Node Won't Start

**Issue:** `polkadot-omni-node` command not found
**Fix:** Install via `cargo install polkadot-omni-node`

**Issue:** Port 9944 already in use
**Fix:** Kill existing node: `pkill -f polkadot-omni-node`

### PAPI Tests Failing

**Issue:** "Could not connect to chain"
**Fix:** Ensure node is running: `npm run start:node`

**Issue:** Type generation errors
**Fix:** Regenerate after runtime changes: `npm run generate:types`

### Runtime Build Errors

**Issue:** Dependency version mismatches
**Fix:** Ensure all Polkadot SDK crates use the same version (currently {{polkadot_sdk_version}})

**Issue:** Out of memory during build
**Fix:** Increase system memory or build with `cargo build --release -j 1`

## Next Steps

### Extend Your Parachain

1. **Add More Pallets**
   - Explore FRAME pallets: `pallet-assets`, `pallet-nfts`, `pallet-collective`
   - Integrate into your runtime's `Cargo.toml` and `construct_runtime!`

2. **Create Additional Custom Pallets**
   - Copy `pallets/template` as a starting point
   - Add to workspace members in root `Cargo.toml`

3. **Implement Business Logic**
   - Design your chain's core functionality
   - Consider storage layout, economics, and governance

### Production Deployment

1. **Obtain a Parachain Slot**
   - Participate in a parachain auction
   - Or use a parathread slot for on-demand execution

2. **Generate Production Chain Spec**
   - Configure genesis state for mainnet
   - Set correct validator/collator keys
   - Remove sudo pallet

3. **Deploy Collators**
   - Run collator nodes to produce blocks
   - Register parachain with relay chain

4. **Monitor and Upgrade**
   - Set up monitoring infrastructure
   - Plan forkless runtime upgrades via governance

## Resources

### Documentation

- [Polkadot SDK Docs](https://docs.polkadot.com/polkadot-sdk/)
- [FRAME Pallets](https://docs.polkadot.com/polkadot-protocol/glossary/frame/)
- [Cumulus (Parachain SDK)](https://docs.polkadot.com/polkadot-sdk/cumulus/)
- [PAPI Docs](https://papi.how/)

### Tutorials

- [Build a Blockchain](https://docs.polkadot.com/tutorials/polkadot-sdk/build-a-blockchain/)
- [Build a Parachain](https://docs.polkadot.com/tutorials/polkadot-sdk/build-a-parachain/)
- [Parachain Template](https://github.com/paritytech/polkadot-sdk-parachain-template)

### Community

- [Substrate Stack Exchange](https://substrate.stackexchange.com/)
- [Polkadot Forum](https://forum.polkadot.network/)
- [Substrate GitHub](https://github.com/paritytech/polkadot-sdk)

## License

MIT OR Apache-2.0
