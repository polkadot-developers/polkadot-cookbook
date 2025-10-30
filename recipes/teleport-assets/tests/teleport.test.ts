import { describe, it, expect, beforeAll, afterAll } from 'vitest';
import { createClient, type PolkadotClient } from 'polkadot-api';
import { getWsProvider } from 'polkadot-api/ws-provider/node';
import { ahp } from '@polkadot-api/descriptors';
import {
  DEV_PHRASE,
  entropyToMiniSecret,
  mnemonicToEntropy,
} from '@polkadot-labs/hdkd-helpers';
import { sr25519CreateDerive } from '@polkadot-labs/hdkd';
import { getPolkadotSigner } from 'polkadot-api/signer';

/**
 * XCM Teleport Integration Tests
 *
 * These tests verify the full XCM teleport flow using Chopsticks XCM mode.
 *
 * IMPORTANT: Start Chopsticks before running these tests:
 *   npm run chopsticks
 *
 * This will start (with HRMP channels configured):
 *   - Asset Hub on ws://127.0.0.1:8000
 *   - People Chain on ws://127.0.0.1:8001
 *   - Polkadot Relay Chain on ws://127.0.0.1:8002
 */

describe('XCM Teleport Tests', () => {
  let assetHubClient: PolkadotClient | undefined;
  let peopleChainClient: PolkadotClient | undefined;

  beforeAll(async () => {
    console.log('🔌 Attempting to connect to Chopsticks...');
    console.log('   Make sure Chopsticks is running: npm run chopsticks');

    // Try to connect to Asset Hub (expects Chopsticks XCM running on port 8000)
    const assetHubProvider = getWsProvider('ws://127.0.0.1:8000');
    const peopleChainProvider = getWsProvider('ws://127.0.0.1:8001');

    assetHubClient = createClient(assetHubProvider);
    peopleChainClient = createClient(peopleChainProvider);

    // Test connection with timeout - this will throw if connection fails
    const api = assetHubClient.getTypedApi(ahp);
    const connectionTest = Promise.race([
      api.constants.System.Version(),
      new Promise((_, reject) =>
        setTimeout(() => reject(new Error('❌ Connection timeout - Chopsticks not responding. Start it with: npm run chopsticks')), 10000)
      ),
    ]);

    await connectionTest;

    console.log('✓ Connected to Chopsticks');
  }, 15000);

  afterAll(async () => {
    if (assetHubClient) {
      assetHubClient.destroy();
    }
    if (peopleChainClient) {
      peopleChainClient.destroy();
    }
  });

  it('should have valid PAPI descriptor for Asset Hub', () => {
    expect(ahp).toBeDefined();
    expect(typeof ahp).toBe('object');
  });

  it('should connect to Asset Hub and verify chain spec', async () => {
    const api = assetHubClient!.getTypedApi(ahp);
    const chainVersion = await api.constants.System.Version();

    expect(chainVersion).toBeDefined();
    expect(chainVersion.spec_name).toBeDefined();
    console.log(`✓ Connected to ${chainVersion.spec_name} v${chainVersion.spec_version}`);
  });

  it('should connect to People Chain and verify chain spec', async () => {
    const api = peopleChainClient!.getTypedApi(ahp);
    const chainVersion = await api.constants.System.Version();

    expect(chainVersion).toBeDefined();
    console.log(`✓ Connected to ${chainVersion.spec_name} v${chainVersion.spec_version}`);
  });

  it('should have Alice account funded on Asset Hub', async () => {
    const api = assetHubClient!.getTypedApi(ahp);
    const aliceAddress = '5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY';

    const accountInfo = await api.query.System.Account.getValue(aliceAddress);
    const balance = accountInfo.data.free;

    expect(balance).toBeGreaterThan(0n);
    console.log(`✓ Alice balance on Asset Hub: ${formatBalance(balance)} DOT`);
  });

  it('should successfully teleport 10 DOT from Asset Hub to People Chain', async () => {

    const assetHubApi = assetHubClient!.getTypedApi(ahp);
    const peopleChainApi = peopleChainClient!.getTypedApi(ahp);

    const aliceAddress = '5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY';

    console.log('\n📊 Initial balances:');

    // Get initial balances
    const initialAssetHubBalance = await getBalance(assetHubApi, aliceAddress);
    const initialPeopleChainBalance = await getBalance(peopleChainApi, aliceAddress);

    console.log(`   Asset Hub: ${formatBalance(initialAssetHubBalance)} DOT`);
    console.log(`   People Chain: ${formatBalance(initialPeopleChainBalance)} DOT`);

    // Execute the teleport
    console.log('\n🚀 Executing teleport...');
    await executeTestTeleport(assetHubClient!);

    // Wait for XCM message to be processed
    console.log('⏳ Waiting for XCM to process...');
    await new Promise(resolve => setTimeout(resolve, 12000));

    // Get final balances
    const finalAssetHubBalance = await getBalance(assetHubApi, aliceAddress);
    const finalPeopleChainBalance = await getBalance(peopleChainApi, aliceAddress);

    console.log('\n📊 Final balances:');
    console.log(`   Asset Hub: ${formatBalance(finalAssetHubBalance)} DOT`);
    console.log(`   People Chain: ${formatBalance(finalPeopleChainBalance)} DOT`);

    // Verify the transfer
    const assetHubDiff = initialAssetHubBalance - finalAssetHubBalance;
    const peopleChainDiff = finalPeopleChainBalance - initialPeopleChainBalance;

    console.log('\n📈 Balance changes:');
    console.log(`   Asset Hub: -${formatBalance(assetHubDiff)} DOT`);
    console.log(`   People Chain: +${formatBalance(peopleChainDiff)} DOT`);

    // Asset Hub balance should decrease (10 DOT + fees)
    expect(finalAssetHubBalance).toBeLessThan(initialAssetHubBalance);
    expect(assetHubDiff).toBeGreaterThanOrEqual(10_000_000_000n); // At least 10 DOT

    // People Chain balance should increase
    expect(finalPeopleChainBalance).toBeGreaterThan(initialPeopleChainBalance);
    expect(peopleChainDiff).toBeGreaterThan(0n);

    console.log('\n✅ Teleport successful!');
  }, 90000);
});

/**
 * Helper: Get account balance
 */
async function getBalance(api: any, address: string): Promise<bigint> {
  const accountInfo = await api.query.System.Account.getValue(address);
  return accountInfo.data.free;
}

/**
 * Helper: Format balance for display
 */
function formatBalance(balance: bigint): string {
  const DOT_UNITS = 10_000_000_000n;
  const whole = balance / DOT_UNITS;
  const fraction = balance % DOT_UNITS;
  return `${whole}.${fraction.toString().padStart(10, '0').slice(0, 4)}`;
}

/**
 * Execute teleport for testing
 */
async function executeTestTeleport(client: PolkadotClient) {
  const {
    XcmV3Junction,
    XcmV3Junctions,
    XcmV3MultiassetFungibility,
    XcmV5AssetFilter,
    XcmV5Instruction,
    XcmV5Junction,
    XcmV5Junctions,
    XcmV5WildAsset,
    XcmVersionedXcm,
  } = await import('@polkadot-api/descriptors');

  const { Enum, FixedSizeBinary } = await import('polkadot-api');

  // Setup signer
  const entropy = mnemonicToEntropy(DEV_PHRASE);
  const miniSecret = entropyToMiniSecret(entropy);
  const derive = sr25519CreateDerive(miniSecret);
  const keyPair = derive('//Alice');

  const polkadotSigner = getPolkadotSigner(
    keyPair.publicKey,
    'Sr25519',
    keyPair.sign
  );

  const ahpApi = client.getTypedApi(ahp);

  // Constants
  const PEOPLE_PARA_ID = 1004;
  const DOT = {
    parents: 1,
    interior: XcmV3Junctions.Here(),
  };
  const DOT_UNITS = 10_000_000_000n;

  // Define assets
  const dotToWithdraw = {
    id: DOT,
    fun: XcmV3MultiassetFungibility.Fungible(10n * DOT_UNITS),
  };

  const dotToPayFees = {
    id: DOT,
    fun: XcmV3MultiassetFungibility.Fungible(1n * DOT_UNITS),
  };

  const destination = {
    parents: 1,
    interior: XcmV3Junctions.X1(XcmV3Junction.Parachain(PEOPLE_PARA_ID)),
  };

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

  // Construct XCM
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

  // Query weight
  const weightResult = await ahpApi.apis.XcmPaymentApi.query_xcm_weight(xcm);

  if (!weightResult.success) {
    throw new Error('Failed to query XCM weight');
  }

  // Execute transaction
  const tx = ahpApi.tx.PolkadotXcm.execute({
    message: xcm,
    max_weight: weightResult.value,
  });

  await tx.signAndSubmit(polkadotSigner);
  console.log('   ✓ Transaction submitted');
}
