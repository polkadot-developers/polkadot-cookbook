import { describe, it, expect, beforeAll, afterAll } from 'vitest';
import { setupContext } from '@acala-network/chopsticks-testing';
import { createChainClient, getBalance, createParachainDestination, createAccountBeneficiary, createFungibleAsset } from '../src/xcm-helpers.js';
import type { TypedApi } from 'polkadot-api';

/**
 * XCM Recipe Tests using Polkadot API (PAPI) with Chopsticks
 *
 * These tests demonstrate XCM operations between relay chain and parachains
 * using the modern Polkadot API and Chopsticks for multi-chain testing.
 */

describe('XCM Recipe Tests with PAPI', () => {
  let ctx: any;
  let relayChainApi: TypedApi<any>;
  let parachainApi: TypedApi<any>;

  beforeAll(async () => {
    // Setup Chopsticks context for multi-chain testing
    // To run these tests, start Chopsticks in another terminal: npm run chopsticks
    try {
      // Try to connect to locally running Chopsticks instances
      const relayChainClient = await createChainClient('ws://localhost:8000');
      const parachainClient = await createChainClient('ws://localhost:8001');

      relayChainApi = relayChainClient.getTypedApi({} as any);
      parachainApi = parachainClient.getTypedApi({} as any);

      console.log('✓ Connected to Chopsticks instances');
    } catch (error) {
      console.log('⏭️  Chopsticks not running - tests will be skipped');
      console.log('   To run these tests: npm run chopsticks (in another terminal)');
    }
  }, 5000);

  afterAll(async () => {
    if (ctx) {
      await ctx.teardown();
    }
  });

  it('should connect to relay chain using PAPI', async () => {
    if (!relayChainApi) return;

    // Query chain information
    const chainName = await relayChainApi.constants.System.Version();
    expect(chainName).toBeDefined();
    console.log(`Connected to relay chain: ${chainName.spec_name}`);
  });

  it('should connect to parachain using PAPI', async () => {
    if (!parachainApi) return;

    const chainName = await parachainApi.constants.System.Version();
    expect(chainName).toBeDefined();
    console.log(`Connected to parachain: ${chainName.spec_name}`);
  });

  it('should get account balance on relay chain', async () => {
    if (!relayChainApi) return;

    // Alice's account address (SS58 format)
    const aliceAddress = '5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY';

    const balance = await getBalance(relayChainApi, aliceAddress);
    expect(balance).toBeGreaterThan(0n);

    console.log(`Alice's balance: ${balance.toString()}`);
  });

  it('should create XCM destination for parachain', () => {
    const paraId = 1000; // Asset Hub
    const destination = createParachainDestination(paraId);

    expect(destination.V3.interior.X1.Parachain).toBe(paraId);
  });

  it('should create beneficiary account location', () => {
    const accountId = '0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef';
    const beneficiary = createAccountBeneficiary(accountId);

    expect(beneficiary.V3.interior.X1.AccountId32.id).toBe(accountId);
  });

  it('should create fungible asset', () => {
    const amount = 1000000000000n; // 1 DOT (12 decimals)
    const asset = createFungibleAsset(amount);

    expect(asset.V3[0].fun.Fungible).toBe(amount);
  });

  // Example of a complete XCM teleport test
  it.todo('should teleport assets from relay chain to parachain');

  // Example test structure for teleport:
  /*
  it('should teleport assets from relay chain to parachain', async () => {
    if (!relayChainApi || !parachainApi) return;

    const destination = createParachainDestination(1000);
    const beneficiary = createAccountBeneficiary('BENEFICIARY_ACCOUNT_ID');
    const assets = createFungibleAsset(1000000000000n);

    const tx = await teleportAssets(
      relayChainApi,
      destination,
      beneficiary,
      assets
    );

    // Sign and send with Alice
    const result = await tx.signAndSubmit(alice);
    await waitForFinalization(result);

    // Verify balance on destination chain
    const finalBalance = await getBalance(parachainApi, 'BENEFICIARY_ACCOUNT_ID');
    expect(finalBalance).toBeGreaterThan(0n);
  });
  */

  it.todo('should verify XCM execution on destination chain');
  it.todo('should handle XCM errors gracefully');
});
