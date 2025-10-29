import { describe, it, expect } from 'vitest';
import { ahp } from '@polkadot-api/descriptors';
import { teleportAssets } from '../src/teleport.js';

/**
 * XCM Teleport Tests
 *
 * These tests verify the XCM teleport functionality structure and types.
 * To test with live chains, run `npm run chopsticks` in another terminal.
 */

describe('XCM Teleport Tests', () => {
  it('should have valid PAPI descriptor for Asset Hub', () => {
    // Verify the ahp descriptor was generated correctly
    expect(ahp).toBeDefined();
    expect(typeof ahp).toBe('object');
    console.log('✓ Asset Hub descriptor loaded successfully');
  });

  it('should export teleportAssets function', () => {
    expect(teleportAssets).toBeDefined();
    expect(typeof teleportAssets).toBe('function');
    console.log('✓ Teleport function is defined');
  });

  it('should have XCM types available from descriptor', async () => {
    // Verify XCM types are properly exported
    const { XcmV5Instruction, XcmVersionedXcm, XcmV5AssetFilter } = await import(
      '@polkadot-api/descriptors'
    );

    expect(XcmV5Instruction).toBeDefined();
    expect(XcmVersionedXcm).toBeDefined();
    expect(XcmV5AssetFilter).toBeDefined();
    console.log('✓ XCM v5 types are available');
  });

  // Integration test note
  it.todo('Integration test: teleport assets between chains (requires Chopsticks)');

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

  async function getBalance(api: any, address: string): Promise<bigint> {
    const accountInfo = await api.query.System.Account.getValue(address);
    return accountInfo.data.free;
  }
  */
});
