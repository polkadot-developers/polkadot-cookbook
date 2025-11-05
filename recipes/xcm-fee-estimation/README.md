# XCM Fee Estimation

Estimate fees for cross-chain asset transfers using XCM (Cross-Consensus Messaging).

## Introduction

When sending cross-chain messages, it's crucial to estimate the fees for the operation to ensure that the transaction will be successful not only in the local chain but also in the destination chain and any intermediate chains.

This recipe demonstrates how to dry-run and estimate the fees for teleporting assets from the Paseo Asset Hub parachain to the Paseo Bridge Hub chain using Polkadot API.

## Fee Mechanism

There are three types of fees that can be charged when sending a cross-chain message:

- **Local execution fees**: Fees charged in the local chain for executing the message
- **Delivery fees**: Fees charged for delivering the message to the destination chain
- **Remote execution fees**: Fees charged in the destination chain for executing the message

```
┌──────────────┐                  ┌──────────────┐
│ Paseo Asset  │──Delivery Fees──>│ Paseo Bridge │
│     Hub      │                  │     Hub      │
└──────┬───────┘                  └──────┬───────┘
       │                                 │
  Local Execution              Remote Execution
      Fees                          Fees
```

The overall fees are: `local_execution_fees + delivery_fees + remote_execution_fees`

## Prerequisites

- Node.js 18+ installed
- Basic understanding of XCM
- Familiarity with TypeScript

## Installation

Install dependencies:

```bash
npm install
```

This will automatically generate the types for Paseo Asset Hub and Paseo Bridge Hub using Polkadot API.

## Usage

### Run with Local Forked Chains (Recommended)

Start Chopsticks to fork both chains locally:

```bash
npm run chopsticks
```

This will start:
- Paseo Asset Hub on port 8001
- Paseo Bridge Hub on port 8000

Then run the fee estimation:

```bash
npm run dev
```

### Run with Live Chains

Modify the endpoints in `src/index.ts` to point to live chain endpoints:

```typescript
const ASSET_HUB_ENDPOINT = 'wss://asset-hub-paseo-rpc.n.dwellir.com';
const BRIDGE_HUB_ENDPOINT = 'wss://bridge-hub-paseo.dotters.network';
```

Then run:

```bash
npm run dev
```

## How It Works

The script performs the following steps:

1. **Connect to chains**: Creates API clients for both Asset Hub and Bridge Hub
2. **Construct XCM message**: Builds an XCM V5 message that encodes the teleport operation
3. **Estimate local fees**: Calculates execution weight on Asset Hub using `XcmPaymentApi.query_xcm_weight()`
4. **Estimate delivery fees**: Dry-runs the XCM and extracts delivery fee requirements using `query_delivery_fees()`
5. **Estimate remote fees**: Connects to Bridge Hub and computes remote execution costs
6. **Aggregate fees**: Combines all fee components into a final estimate

## Expected Output

The script outputs a structured fee breakdown:

```
Fee Estimation for Teleporting Assets from Asset Hub to Bridge Hub:

Local Execution Fees (Asset Hub):
- Weight: 4,000,000,000 units
- Fee: 0.4 PAS

Delivery Fees:
- Fee: 0.1 PAS

Remote Execution Fees (Bridge Hub):
- Weight: 3,500,000,000 units
- Fee: 0.35 PAS

Total Estimated Fees: 0.85 PAS
```

## Configuration

### Chopsticks Configuration

The recipe includes two Chopsticks config files:

- `chopsticks-asset-hub.yml` - Paseo Asset Hub configuration
- `chopsticks-bridge-hub.yml` - Paseo Bridge Hub configuration

Both configs:
- Mock signature verification for testing
- Import storage with Alice's account funded
- Can be pinned to specific block numbers via environment variables

### Environment Variables (Optional)

You can pin the chains to specific block numbers:

```bash
export PASEO_ASSET_HUB_BLOCK_NUMBER=1234567
export PASEO_BRIDGE_HUB_BLOCK_NUMBER=7654321
npm run chopsticks
```

## Testing

Run the test suite:

```bash
npm test
```

Run tests in watch mode:

```bash
npm run test:watch
```

## Code Structure

```
├── src/
│   └── index.ts           # Main fee estimation logic
├── tests/
│   └── index.test.ts      # Test suite
├── chopsticks-asset-hub.yml   # Asset Hub Chopsticks config
├── chopsticks-bridge-hub.yml  # Bridge Hub Chopsticks config
└── package.json           # Project dependencies
```

## Key Functions

### `createTeleportXcmToBridgeHub()`

Constructs the XCM message with:
- Withdrawal instruction
- Fee payment setup
- Asset transfer instructions
- Destination chain specification

### `estimateXcmFeesFromAssetHubToBridgeHub()`

Calculates the three fee components:
- Queries local execution weight
- Extracts delivery fees from dry-run
- Queries remote execution fees from destination chain

### `main()`

Orchestrates the workflow:
- Connects to Asset Hub
- Creates XCM message
- Estimates all fees
- Generates transaction hex for submission

## Troubleshooting

### "Cannot connect to chain"

Ensure Chopsticks is running:
```bash
npm run chopsticks
```

### "Failed to generate descriptors"

Regenerate the API descriptors:
```bash
npm run generate
```

### "Insufficient balance"

The Chopsticks configs fund Alice's account with 10,000 PAS by default. Check the `import-storage` section in the config files.

## Learn More

- [XCM Documentation](https://wiki.polkadot.network/docs/learn-xcm)
- [Polkadot API Documentation](https://papi.how/)
- [Chopsticks Documentation](https://github.com/AcalaNetwork/chopsticks)

## Related Recipes

- `teleport-assets` - Basic asset teleportation
- `xcm-reserve-transfer` - Reserve-backed asset transfers
- `xcm-dry-run` - Dry-running XCM messages

## License

MIT OR Apache-2.0
