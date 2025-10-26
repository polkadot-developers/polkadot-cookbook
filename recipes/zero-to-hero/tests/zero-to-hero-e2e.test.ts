import { describe, it, expect } from 'vitest';
import { ApiPromise, WsProvider } from '@polkadot/api';
import { spawn } from 'node:child_process';
import net from 'node:net';
import fs from 'node:fs';
import path from 'node:path';

async function isPortReachable(host: string, port: number, timeoutMs: number): Promise<boolean> {
  return new Promise((resolve) => {
    const socket = new net.Socket();
    const done = (ok: boolean) => { try { socket.destroy(); } catch {} ; resolve(ok); };
    socket.setTimeout(timeoutMs);
    socket.once('error', () => done(false));
    socket.once('timeout', () => done(false));
    socket.connect(port, host, () => done(true));
  });
}

async function commandExists(cmd: string): Promise<boolean> {
  return new Promise((resolve) => {
    try {
      const proc = spawn(cmd, ['--version'], { stdio: 'ignore' });
      proc.on('error', () => resolve(false));
      proc.on('exit', () => resolve(true));
    } catch {
      resolve(false);
    }
  });
}

async function startOmniNode(repoRoot: string, chainSpecPath: string, host: string, port: number): Promise<ReturnType<typeof spawn> | null> {
  if (!fs.existsSync(chainSpecPath)) return null;
  let nodeProc: ReturnType<typeof spawn> | null = null;
  try {
    nodeProc = spawn('polkadot-omni-node', ['--chain', chainSpecPath, '--dev', '--dev-block-time', '1000'], {
      cwd: repoRoot,
      stdio: 'ignore',
      detached: true,
    });
    // Wait for RPC to come up (max ~20s)
    for (let i = 0; i < 10; i++) {
      const ok = await isPortReachable(host, port, 1000);
      if (ok) return nodeProc;
      await new Promise((r) => setTimeout(r, 2000));
    }
  } catch {}
  // Ensure cleanup
  if (nodeProc && nodeProc.pid) {
    try { process.kill(-nodeProc.pid, 'SIGTERM'); } catch {}
  }
  return null;
}

async function ensureNodeAvailable(endpoint: string) {
  const { hostname, port } = new URL(endpoint.replace('ws://', 'http://'));
  const targetPort = Number(port || 9944);

  if (await isPortReachable(hostname, targetPort, 1000)) {
    return { ok: true as const, nodeProc: null as ReturnType<typeof spawn> | null };
  }

  const repoRoot = path.resolve(__dirname, '../../../');
  const chainSpec = path.join(repoRoot, 'kitchensink-parachain', 'chain_spec.json');
  const hasOmni = await commandExists('polkadot-omni-node');

  if (hasOmni) {
    const nodeProc = await startOmniNode(repoRoot, chainSpec, hostname, targetPort);
    if (nodeProc) return { ok: true as const, nodeProc };
  }

  // In CI, skip fast if we cannot start a node (binary not installed or failed to start)
  return { ok: false as const, nodeProc: null };
}

describe('zero-to-hero integration', () => {
  it('connects and sees zero-to-hero metadata', async () => {
    const endpoint = process.env.POLKADOT_WS || 'ws://127.0.0.1:9944';

    const result = await ensureNodeAvailable(endpoint);

    // If node is not available it should fail
    expect(result.ok).toBe(true);

    const { nodeProc } = result;
    try {
      const api = await ApiPromise.create({ provider: new WsProvider(endpoint, 1) });
      const pallets = api.runtimeMetadata.asLatest.pallets.map((p) => p.name.toString().toLowerCase());
      const hasCustomPallet = pallets.includes('custompallet');
      expect(hasCustomPallet).toBe(true);
      await api.disconnect();
    } finally {
      if (nodeProc && nodeProc.pid) {
        try { process.kill(-nodeProc.pid, 'SIGTERM'); } catch {}
      }
    }
  });
});