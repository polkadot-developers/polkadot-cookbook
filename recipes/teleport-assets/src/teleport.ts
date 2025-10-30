/**
 * XCM Asset Teleport Example
 *
 * This example demonstrates how to teleport assets from Asset Hub to People Chain
 * using XCM v5 and the Polkadot API (PAPI).
 *
 * Based on: https://docs.polkadot.com/tutorials/interoperability/xcm-transfers/
 */

import {
  ahp,
  XcmV3Junction,
  XcmV3Junctions,
  XcmV3MultiassetFungibility,
  XcmV5AssetFilter,
  XcmV5Instruction,
  XcmV5Junction,
  XcmV5Junctions,
  XcmV5WildAsset,
  XcmVersionedXcm,
} from '@polkadot-api/descriptors';
import { createClient, Enum, FixedSizeBinary } from 'polkadot-api';
import { getWsProvider } from 'polkadot-api/ws-provider/node';
import { withPolkadotSdkCompat } from 'polkadot-api/polkadot-sdk-compat';
import { sr25519CreateDerive } from '@polkadot-labs/hdkd';
import {
  DEV_PHRASE,
  entropyToMiniSecret,
  mnemonicToEntropy,
} from '@polkadot-labs/hdkd-helpers';
import { getPolkadotSigner } from 'polkadot-api/signer';

/**
 * Teleport assets from Asset Hub to People Chain
 * @param endpoint - Asset Hub WebSocket endpoint (default: ws://localhost:8000 for Chopsticks XCM)
 */
export async function teleportAssets(endpoint: string = 'ws://localhost:8000') {
  console.log('🚀 Starting XCM Asset Teleport Example\n');

  // Step 1: Setup key derivation and signer
  console.log('📝 Setting up signer with Alice account...');
  const entropy = mnemonicToEntropy(DEV_PHRASE);
  const miniSecret = entropyToMiniSecret(entropy);
  const derive = sr25519CreateDerive(miniSecret);
  const keyPair = derive('//Alice');

  const polkadotSigner = getPolkadotSigner(
    keyPair.publicKey,
    'Sr25519',
    keyPair.sign
  );

  console.log(`   Alice address: ${keyPair.publicKey.toString()}\n`);

  // Step 2: Initialize client connection to Asset Hub
  console.log(`🔌 Connecting to Asset Hub at ${endpoint}...`);
  const client = createClient(
    withPolkadotSdkCompat(getWsProvider(endpoint))
  );

  const ahpApi = client.getTypedApi(ahp);
  console.log('   ✅ Connected to Asset Hub\n');

  // Step 3: Define constants
  const PEOPLE_PARA_ID = 1004;
  const DOT = {
    parents: 1,
    interior: XcmV3Junctions.Here(),
  };
  const DOT_UNITS = 10_000_000_000n; // 10 decimals for Westend

  console.log('⚙️  Configuration:');
  console.log(`   Destination: People Chain (ParaId ${PEOPLE_PARA_ID})`);
  console.log(`   Asset: WND (Westend native token)`);
  console.log(`   Amount: 10 WND\n`);

  // Step 4: Define assets and fees
  const dotToWithdraw = {
    id: DOT,
    fun: XcmV3MultiassetFungibility.Fungible(10n * DOT_UNITS),
  };

  const dotToPayFees = {
    id: DOT,
    fun: XcmV3MultiassetFungibility.Fungible(1n * DOT_UNITS),
  };

  // Step 5: Define destination
  const destination = {
    parents: 1,
    interior: XcmV3Junctions.X1(XcmV3Junction.Parachain(PEOPLE_PARA_ID)),
  };

  // Step 6: Define remote fees and assets
  const remoteFees = Enum(
    'Teleport',
    XcmV5AssetFilter.Definite([
      {
        id: DOT,
        fun: XcmV3MultiassetFungibility.Fungible(1n * DOT_UNITS),
      },
    ])
  );

  const assets = [
    Enum('Teleport', XcmV5AssetFilter.Wild(XcmV5WildAsset.AllCounted(1))),
  ];

  const beneficiary = FixedSizeBinary.fromBytes(keyPair.publicKey);

  // Step 7: Define remote XCM instructions
  const remoteXcm = [
    XcmV5Instruction.DepositAsset({
      assets: XcmV5AssetFilter.Wild(XcmV5WildAsset.AllCounted(1)),
      beneficiary: {
        parents: 0,
        interior: XcmV5Junctions.X1(
          XcmV5Junction.AccountId32({
            id: beneficiary,
            network: undefined,
          })
        ),
      },
    }),
  ];

  // Step 8: Construct XCM message
  console.log('📦 Constructing XCM message...');
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
      beneficiary: {
        parents: 0,
        interior: XcmV5Junctions.X1(
          XcmV5Junction.AccountId32({
            id: beneficiary,
            network: undefined,
          })
        ),
      },
    }),
  ]);
  console.log('   ✅ XCM message constructed\n');

  // Step 9: Query weight
  console.log('⚖️  Querying XCM weight...');
  const weightResult = await ahpApi.apis.XcmPaymentApi.query_xcm_weight(xcm);

  if (weightResult.success) {
    const weight = weightResult.value;
    console.log(`   Weight: ${stringify(weight)}\n`);

    // Step 10: Execute transaction
    console.log('📤 Executing XCM transfer...');
    const tx = ahpApi.tx.PolkadotXcm.execute({
      message: xcm,
      max_weight: weight,
    });

    const result = await tx.signAndSubmit(polkadotSigner);
    console.log('   ✅ Transaction submitted!');
    console.log(`   Result: ${stringify(result)}\n`);

    console.log('🎉 Teleport complete!');
    console.log('   Check People Chain for the transferred assets.');
  } else {
    console.error('❌ Failed to query XCM weight');
    console.error(weightResult);
  }

  // Cleanup
  client.destroy();
}

/**
 * Utility function to stringify objects with BigInt support
 */
function stringify(obj: any): string {
  return JSON.stringify(
    obj,
    (_, v) => (typeof v === 'bigint' ? v.toString() : v),
    2
  );
}

// Run if executed directly
if (import.meta.url === `file://${process.argv[1]}`) {
  teleportAssets().catch(console.error);
}
