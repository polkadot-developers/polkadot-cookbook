# Teleport Assets with XCM

Teleport assets from Asset Hub to People Chain using XCM v5 and Polkadot API (PAPI).

## Overview

This recipe demonstrates how to:
- Use **Polkadot API (PAPI)** for type-safe XCM operations
- Construct XCM v5 messages with `InitiateTransfer` instruction
- Teleport assets between parachains (Asset Hub → People Chain)
- Test XCM transfers with **Chopsticks** multi-chain environment

This recipe follows best practices using the modern Polkadot API (PAPI) for XCM operations.

## Prerequisites

- Node.js 20+
- Basic understanding of XCM (Cross-Consensus Messaging)
- Familiarity with Polkadot parachains (Asset Hub, People Chain)

## What is Asset Teleportation?

**Teleportation** is an XCM transfer mechanism where:
1. Assets are burned on the source chain (Asset Hub)
2. Equivalent assets are minted on the destination chain (People Chain)
3. Both chains trust each other to handle the burn/mint correctly

This is ideal for system parachains that share trust assumptions.

## Setup

Install dependencies:

```bash
npm install
```

## Running the Example

### Option 1: Run the Script Directly

```bash
npm run teleport
```

This executes the teleport against a local Chopsticks instance (requires Chopsticks running).

### Option 2: Start Chopsticks and Test

1. **Terminal 1** - Start Chopsticks XCM:
```bash
npm run chopsticks
```

This starts a multi-chain environment with HRMP channels configured:
- **Asset Hub** on port 8000 (ws://127.0.0.1:8000)
- **People Chain** on port 8001 (ws://127.0.0.1:8001)
- **Westend Relay Chain** on port 8002 (ws://127.0.0.1:8002)

2. **Terminal 2** - Run tests:
```bash
npm test
```

Or run the teleport script:
```bash
npm run teleport
```

**Note**: The `chopsticks xcm` command automatically:
- Forks live chains locally for testing
- Configures HRMP channels between parachains
- Enables cross-chain message passing
- Provides deterministic block production

## How It Works

### 1. Setup Signer

```typescript
const entropy = mnemonicToEntropy(DEV_PHRASE);
const miniSecret = entropyToMiniSecret(entropy);
const derive = sr25519CreateDerive(miniSecret);
const keyPair = derive('//Alice');

const polkadotSigner = getPolkadotSigner(
  keyPair.publicKey,
  'Sr25519',
  keyPair.sign
);
```

### 2. Connect to Asset Hub

```typescript
const client = createClient(
  withPolkadotSdkCompat(getWsProvider('ws://localhost:8000'))
);
const ahpApi = client.getTypedApi(ahp);
```

### 3. Define Assets and Destination

```typescript
const DOT_UNITS = 10_000_000_000n; // 10 decimals for Westend

const dotToWithdraw = {
  id: { parents: 1, interior: XcmV3Junctions.Here() },
  fun: XcmV3MultiassetFungibility.Fungible(10n * DOT_UNITS),
};

const destination = {
  parents: 1,
  interior: XcmV3Junctions.X1(XcmV3Junction.Parachain(1004)), // People Chain
};
```

### 4. Construct XCM Message

```typescript
const xcm = XcmVersionedXcm.V5([
  XcmV5Instruction.WithdrawAsset([dotToWithdraw]),
  XcmV5Instruction.PayFees({ asset: dotToPayFees }),
  XcmV5Instruction.InitiateTransfer({
    destination,
    remote_fees: remoteFees,
    preserve_origin: false,
    assets,
    remote_xcm: remoteXcm,
  }),
  XcmV5Instruction.RefundSurplus(),
  XcmV5Instruction.DepositAsset({
    assets: XcmV5AssetFilter.Wild(XcmV5WildAsset.AllCounted(1)),
    beneficiary,
  }),
]);
```

### 5. Query Weight and Execute

```typescript
const weightResult = await ahpApi.apis.XcmPaymentApi.query_xcm_weight(xcm);

if (weightResult.success) {
  const tx = ahpApi.tx.PolkadotXcm.execute({
    message: xcm,
    max_weight: weightResult.value,
  });

  const result = await tx.signAndSubmit(polkadotSigner);
  console.log('✅ Transaction submitted:', result);
}
```

## XCM Instructions Explained

1. **WithdrawAsset** - Withdraws 10 WND from Alice's account on Asset Hub
2. **PayFees** - Allocates 1 WND for execution fees
3. **InitiateTransfer** - Starts the cross-chain transfer with:
   - Destination: People Chain (ParaId 1004)
   - Remote fees: 1 WND for fees on People Chain
   - Assets: Remaining WND to teleport
   - Remote XCM: Instructions to execute on People Chain
4. **RefundSurplus** - Returns unused fees
5. **DepositAsset** - Deposits refunded assets back to Alice

## Testing with Chopsticks

Chopsticks provides a local multi-chain environment:

```yaml
# chopsticks.yml
relaychain:
  endpoint: wss://westend-rpc.polkadot.io
  port: 8000

parachains:
  - endpoint: wss://westend-asset-hub-rpc.polkadot.io
    paraId: 1000
    port: 8001

  - endpoint: wss://westend-people-rpc.polkadot.io
    paraId: 1004
    port: 8002
```

Benefits:
- ✅ Fork live networks locally
- ✅ Fast block times for testing
- ✅ No testnet tokens needed
- ✅ Deterministic test environment

## What You'll Learn

- How to use Polkadot API (PAPI) for XCM operations
- How to construct XCM v5 messages with type safety
- How to teleport assets between parachains
- How to query XCM execution weight
- How to test XCM with Chopsticks

## Common Issues

### "Failed to connect"
Make sure Chopsticks is running (`npm run chopsticks`)

### "Insufficient balance"
The dev account (Alice) is pre-funded in Chopsticks, but balance checks may vary

### "XCM execution failed"
Check that:
- Both chains support teleportation between them
- Asset amounts cover execution fees
- Weight limits are sufficient

## Next Steps

- Modify the amount and destination
- Try different XCM instructions (Reserve Transfer, Limited Reserve Transfer)
- Add error handling for failed XCM execution
- Test with different parachain pairs

## Resources

- [Polkadot API (PAPI) Documentation](https://papi.how/)
- [XCM Documentation](https://wiki.polkadot.network/docs/learn-xcm)
- [Chopsticks Documentation](https://github.com/AcalaNetwork/chopsticks)
- [XCM Format Reference](https://github.com/paritytech/xcm-format)
- [Polkadot SDK Documentation](https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/index.html)

## License

MIT OR Apache-2.0
