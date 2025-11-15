import { describe, it, expect } from 'vitest';
import { createChainClient, getBalance, getCurrentBlockNumber, getChainInfo } from '../src/chain-client.js';

/**
 * Basic Chain Interaction Tests using Polkadot API (PAPI)
 *
 * These tests demonstrate basic chain interactions using the modern Polkadot API.
 *
 * Note: These tests require a running node. To run against a live chain:
 * 1. Update the endpoint to a public RPC endpoint (e.g., wss://westend-rpc.polkadot.io)
 * 2. Or run a local node and connect to ws://localhost:9944
 */

describe('Basic Chain Interaction Tests with PAPI', () => {
  // Skip tests by default - require explicit node endpoint
  const CHAIN_ENDPOINT = process.env.CHAIN_ENDPOINT || '';
  const SKIP_TESTS = !CHAIN_ENDPOINT;

  it.skipIf(SKIP_TESTS)('should connect to chain using PAPI', async () => {
    const client = await createChainClient(CHAIN_ENDPOINT);
    const api = client.getTypedApi({} as any);

    // Query chain information
    const chainInfo = await getChainInfo(api);
    expect(chainInfo.specName).toBeDefined();
    console.log(`Connected to chain: ${chainInfo.specName}`);

    client.destroy();
  });

  it.skipIf(SKIP_TESTS)('should get current block number', async () => {
    const client = await createChainClient(CHAIN_ENDPOINT);
    const api = client.getTypedApi({} as any);

    const blockNumber = await getCurrentBlockNumber(api);
    expect(blockNumber).toBeGreaterThan(0);
    console.log(`Current block number: ${blockNumber}`);

    client.destroy();
  });

  it.skipIf(SKIP_TESTS)('should get account balance', async () => {
    const client = await createChainClient(CHAIN_ENDPOINT);
    const api = client.getTypedApi({} as any);

    // Alice's account address (SS58 format - adjust for your chain)
    const aliceAddress = '5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY';

    const balance = await getBalance(api, aliceAddress);
    expect(balance).toBeGreaterThanOrEqual(0n);
    console.log(`Balance: ${balance.toString()}`);

    client.destroy();
  });

  // Unit tests that don't require a node connection
  it('should export chain client utilities', () => {
    expect(createChainClient).toBeDefined();
    expect(getBalance).toBeDefined();
    expect(getCurrentBlockNumber).toBeDefined();
    expect(getChainInfo).toBeDefined();
  });
});
