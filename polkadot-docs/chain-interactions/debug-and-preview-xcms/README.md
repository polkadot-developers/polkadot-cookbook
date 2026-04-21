# Replay and Dry Run XCMs

[![Replay and Dry Run XCMs](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-debug-and-preview-xcms.yml/badge.svg?event=push)](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-debug-and-preview-xcms.yml)

Verification tests for the [Replay and Dry Run XCMs](https://docs.polkadot.com/chain-interactions/send-transactions/interoperability/debug-and-preview-xcms/) guide.

## What This Tests

This harness verifies that the XCM replay and dry-run workflow described in the guide works correctly:

1. **Connect to Chopsticks fork** — connects to a local Polkadot Hub fork via WebSocket
2. **Decode XCM call data** — uses PAPI `txFromCallData` to decode the XCM extrinsic from block 9079592
3. **Dry-run via DryRunApi** — calls `DryRunApi.dry_run_call` with a signed origin to simulate XCM execution
4. **DryRunApi availability** — confirms the runtime API is accessible via Polkadot.js
5. **Polkadot.js dry-run XCM** — performs `dryRunXcm` via Polkadot.js for cross-validation

One Chopsticks fork runs locally during the test suite:
- Polkadot Hub (mainnet fork) on `ws://localhost:8000`

## Running Locally

```bash
# Install dependencies
npm install

# Generate PAPI descriptors (requires Chopsticks to be running)
# Start Chopsticks first: npx @acala-network/chopsticks --config polkadot-hub.yml --port 8000
npx papi add polkadotHub -w ws://localhost:8000

# Run tests (Chopsticks fork is started automatically)
npm test
```

## Guide

Source: [docs.polkadot.com](https://docs.polkadot.com/chain-interactions/send-transactions/interoperability/debug-and-preview-xcms/)
