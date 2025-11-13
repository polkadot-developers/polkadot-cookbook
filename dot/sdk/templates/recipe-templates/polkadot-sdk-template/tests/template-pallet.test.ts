import { describe, it, expect, beforeAll, afterAll } from 'vitest';
import { createClient } from 'polkadot-api';
import { getWsProvider } from 'polkadot-api/ws-provider/node';
import { dot } from '@polkadot-api/descriptors';

/**
 * Template Pallet Integration Tests using Polkadot API (PAPI)
 *
 * These tests demonstrate how to interact with your custom pallet using PAPI.
 *
 * Prerequisites:
 * 1. Build the runtime: npm run build:runtime
 * 2. Generate chain spec: npm run generate:spec
 * 3. Start the dev node: npm run start:node
 * 4. Run these tests: npm test
 *
 * Note: Tests will be skipped if the node is not running on ws://localhost:9944
 */

describe('Template Pallet Tests with PAPI', () => {
  const CHAIN_ENDPOINT = process.env.CHAIN_ENDPOINT || 'ws://localhost:9944';
  let client: ReturnType<typeof createClient> | null = null;
  let api: any = null;

  beforeAll(async () => {
    try {
      const provider = getWsProvider(CHAIN_ENDPOINT);
      client = createClient(provider);
      api = client.getTypedApi(dot);

      // Wait for connection
      await new Promise((resolve) => setTimeout(resolve, 1000));
    } catch (error) {
      console.warn('Could not connect to chain. Tests will be skipped.');
      console.warn('Make sure to run: npm run start:node');
    }
  });

  afterAll(() => {
    if (client) {
      client.destroy();
    }
  });

  it('should connect to the local dev node', async () => {
    if (!api) {
      console.log('⏭️  Skipping test - node not running');
      return;
    }

    // Query chain information
    const chainInfo = await api.constants.System.Version();
    expect(chainInfo.spec_name).toBeDefined();
    console.log(`✅ Connected to chain: ${chainInfo.spec_name}`);
  });

  it('should get current block number', async () => {
    if (!api) {
      console.log('⏭️  Skipping test - node not running');
      return;
    }

    const blockNumber = await api.query.System.Number.getValue();
    expect(blockNumber).toBeGreaterThanOrEqual(0);
    console.log(`✅ Current block number: ${blockNumber}`);
  });

  it('should query account balance for Alice', async () => {
    if (!api) {
      console.log('⏭️  Skipping test - node not running');
      return;
    }

    // Alice's account in dev mode
    const aliceAddress = '5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY';

    const accountInfo = await api.query.System.Account.getValue(aliceAddress);
    expect(accountInfo.data.free).toBeGreaterThan(0n);
    console.log(`✅ Alice's balance: ${accountInfo.data.free.toString()}`);
  });

  it('should query template pallet storage', async () => {
    if (!api) {
      console.log('⏭️  Skipping test - node not running');
      return;
    }

    // Query the Something storage from TemplatePallet
    // Adjust this based on your actual pallet storage items
    try {
      const something = await api.query.TemplatePallet.Something.getValue();
      console.log(`✅ Template storage value: ${something}`);
    } catch (error) {
      console.log('⚠️  Template pallet not yet integrated or storage item missing');
    }
  });

  // Example: Submit a transaction to the template pallet
  it.skip('should submit extrinsic to template pallet', async () => {
    if (!api) {
      console.log('⏭️  Skipping test - node not running');
      return;
    }

    // This is a placeholder - adjust based on your pallet's actual extrinsics
    // You'll need to:
    // 1. Import signing utilities from @polkadot-labs/hdkd
    // 2. Create a signer (e.g., from Alice's dev account)
    // 3. Build and submit the transaction

    console.log('⚠️  Transaction submission test not yet implemented');
    console.log('Implement this test based on your pallet\'s extrinsics');
  });

  // Unit test that doesn't require node connection
  it('should have PAPI client utilities available', () => {
    expect(createClient).toBeDefined();
    expect(getWsProvider).toBeDefined();
  });
});
