#!/usr/bin/env bash
set -euo pipefail

if [ $# -lt 1 ]; then
  echo "Usage: $0 <tutorial-slug>"
  exit 1
fi

SLUG="$1"
DIR="tutorials/$SLUG"

mkdir -p "$DIR/tests" "$DIR/scripts" "$DIR/${SLUG}-code"

cat > "$DIR/justfile" << 'EOF'
default:
  @just --list

say-hello:
  echo "Hello, world!"
EOF

cat > "$DIR/tests/example.test.ts" << 'EOF'
import { describe, it, expect } from 'vitest';
import { ApiPromise, WsProvider } from '@polkadot/api';
import net from 'node:net';

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

describe('tutorial e2e', () => {
  it('connects and reads chain info', async () => {
    const endpoint = process.env.POLKADOT_WS || 'ws://127.0.0.1:9944';
    const { hostname, port } = new URL(endpoint.replace('ws://', 'http://'));
    if (!(await isPortReachable(hostname, Number(port || 9944), 1000))) return;

    const api = await ApiPromise.create({ provider: new WsProvider(endpoint, 1) });
    const header = await api.rpc.chain.getHeader();
    expect(header.number.toNumber()).toBeGreaterThanOrEqual(0);
    await api.disconnect();
  });
});
EOF

cat > "$DIR/tutorial.yml" << 'EOF'
name: Tutorial
description: Replace with a short description.
type: sdk # or contracts
EOF

echo "Scaffolded tutorial at $DIR"


