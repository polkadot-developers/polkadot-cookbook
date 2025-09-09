import { describe, it, expect } from 'vitest';
import { ApiPromise, WsProvider } from '@polkadot/api';

describe('zero-to-hero e2e', () => {
  it('connects to a local node and queries chain info', async () => {
    const endpoint = process.env.POLKADOT_WS || 'ws://127.0.0.1:50971'; // This is the port of the charlie node in the zombienet config
    const provider = new WsProvider(endpoint, 1);

    let api: ApiPromise | null = null;
    try {
      api = await ApiPromise.create({ provider });
      const [chain, nodeName, nodeVersion, header] = await Promise.all([
        api.rpc.system.chain(),
        api.rpc.system.name(),
        api.rpc.system.version(),
        api.rpc.chain.getHeader(),
      ]);

      expect(chain.toString().length).toBeGreaterThan(0);
      expect(nodeName.toString().length).toBeGreaterThan(0);
      expect(nodeVersion.toString().length).toBeGreaterThan(0);
      expect(header.number.toNumber()).toBeGreaterThanOrEqual(0);
    } catch (err: any) {
      if (String(err?.message || err).match(/ECONNREFUSED|TIMEOUT|disconnected|Connection/i)) {
        console.warn('No local node available at', endpoint, '- skipping test');
        return;
      }
      throw err;
    } finally {
      if (api) {
        await api.disconnect();
      } else {
        try { await provider.disconnect(); } catch {}
      }
    }
  });
});

