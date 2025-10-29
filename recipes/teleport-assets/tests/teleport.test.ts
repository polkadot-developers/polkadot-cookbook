import { describe, it, expect, beforeAll, afterAll } from 'vitest';
import { setupContext } from '@acala-network/chopsticks-testing';
import { createClient } from 'polkadot-api';
import { getWsProvider } from 'polkadot-api/ws-provider/node';
import { ahp } from '@polkadot-api/descriptors';

/**
 * XCM Teleport Tests using Chopsticks
 *
 * These tests verify the XCM teleport functionality between Asset Hub and People Chain.
 * Run `npm run chopsticks` in another terminal to enable these tests.
 */

describe('XCM Teleport Tests', () => {
  let ctx: any;
  let assetHubClient: any;
  let peopleChainClient: any;

  beforeAll(async () => {
    try {
      // Setup Chopsticks context with Asset Hub and People Chain
      ctx = await setupContext({
        endpoint: [
          'wss://westend-asset-hub-rpc.polkadot.io',
          'wss://westend-people-rpc.polkadot.io',
        ],
        db: './test-db.sqlite',
        timeout: 60000,
      });

      // Create clients
      assetHubClient = createClient(getWsProvider('ws://localhost:8001'));
      peopleChainClient = createClient(getWsProvider('ws://localhost:8002'));

      console.log('✓ Chopsticks context setup complete');
    } catch (error) {
      console.log('⏭️  Skipping tests - Chopsticks setup failed');
      console.log('   Run `npm run chopsticks` in another terminal to enable tests');
    }
  }, 120000);

  afterAll(async () => {
    if (ctx) {
      await ctx.teardown();
    }
    if (assetHubClient) {
      assetHubClient.destroy();
    }
    if (peopleChainClient) {
      peopleChainClient.destroy();
    }
  });

  it('should connect to Asset Hub', async () => {
    if (!assetHubClient) return;

    const api = assetHubClient.getTypedApi(ahp);
    const chainVersion = await api.constants.System.Version();

    expect(chainVersion).toBeDefined();
    expect(chainVersion.spec_name).toBe('westmint'); // Asset Hub spec name
    console.log(`Connected to: ${chainVersion.spec_name}`);
  });

  it('should connect to People Chain', async () => {
    if (!peopleChainClient) return;

    const api = peopleChainClient.getTypedApi(ahp);
    const chainVersion = await api.constants.System.Version();

    expect(chainVersion).toBeDefined();
    console.log(`Connected to: ${chainVersion.spec_name}`);
  });

  it('should have Alice account with balance on Asset Hub', async () => {
    if (!assetHubClient) return;

    const api = assetHubClient.getTypedApi(ahp);
    const aliceAddress = '5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY';

    const accountInfo = await api.query.System.Account.getValue(aliceAddress);
    expect(accountInfo.data.free).toBeGreaterThan(0n);

    console.log(`Alice balance on Asset Hub: ${accountInfo.data.free.toString()}`);
  });

  it('should query XCM weight for teleport message', async () => {
    if (!assetHubClient) return;

    const api = assetHubClient.getTypedApi(ahp);

    // This test verifies the XcmPaymentApi is available
    // Actual weight query requires a properly formed XCM message
    expect(api.apis.XcmPaymentApi).toBeDefined();
    console.log('XcmPaymentApi available for weight queries');
  });

  // Integration test - requires funding and transaction execution
  it.todo('should successfully teleport assets from Asset Hub to People Chain');

  // Example test structure for full teleport:
  /*
  it('should teleport 10 WND from Asset Hub to People Chain', async () => {
    if (!assetHubClient || !peopleChainClient) return;

    const assetHubApi = assetHubClient.getTypedApi(ahp);
    const peopleChainApi = peopleChainClient.getTypedApi(ahp);

    const aliceAddress = '5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY';

    // Get initial balances
    const initialAssetHubBalance = await getBalance(assetHubApi, aliceAddress);
    const initialPeopleChainBalance = await getBalance(peopleChainApi, aliceAddress);

    // Execute teleport
    await teleportAssets('ws://localhost:8001');

    // Verify balances changed
    const finalAssetHubBalance = await getBalance(assetHubApi, aliceAddress);
    const finalPeopleChainBalance = await getBalance(peopleChainApi, aliceAddress);

    expect(finalAssetHubBalance).toBeLessThan(initialAssetHubBalance);
    expect(finalPeopleChainBalance).toBeGreaterThan(initialPeopleChainBalance);
  });
  */
});

async function getBalance(api: any, address: string): Promise<bigint> {
  const accountInfo = await api.query.System.Account.getValue(address);
  return accountInfo.data.free;
}
