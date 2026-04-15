import { describe, it, expect, afterAll, beforeAll } from "vitest";
import { ApiPromise, WsProvider } from "@polkadot/api";
import { Keyring } from "@polkadot/keyring";
import { cryptoWaitReady } from "@polkadot/util-crypto";
import { spawn, exec, ChildProcess } from "child_process";

// Chopsticks forks polkadot-asset-hub on port 8000
const ASSET_HUB_WS = "ws://localhost:8000";

// Alice's well-known dev address (funded via the polkadot-asset-hub Chopsticks config)
const ALICE_URI = "//Alice";
const ALICE_ADDRESS = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY";

// Astar parachain ID = 2006
const ASTAR_PARA_ID = 2006;

// Sovereign (sibling) account of Astar on Asset Hub:
//   bytes = b'sibling' (7 bytes) + u32LE(2006) (4 bytes) + zeros (21 bytes)
//   = 7369626c696e67d607000000000000000000000000000000000000000000000000
//   SS58 (prefix=0): 13cKp891KZjJw5RdsU6CGvY2kusdT6ubEc2uFu4mC3kHKcax
const ASTAR_SOVEREIGN = "13cKp891KZjJw5RdsU6CGvY2kusdT6ubEc2uFu4mC3kHKcax";

// Minimum balance for the foreign asset
const MIN_BALANCE = 100_000n;

// Multilocation for Astar foreign asset from Asset Hub's perspective
// (parents=1, X1:[Parachain(2006)]) — matches the guide's example
const ASTAR_ASSET_MULTILOCATION = {
  parents: 1,
  interior: {
    X1: [{ Parachain: ASTAR_PARA_ID }],
  },
};

let chopsticksProcess: ChildProcess | null = null;

// ---------------------------------------------------------------------------
// Chopsticks lifecycle
// ---------------------------------------------------------------------------

async function waitForChopsticks(
  maxRetries = 40,
  retryDelayMs = 3000
): Promise<void> {
  for (let i = 1; i <= maxRetries; i++) {
    try {
      await new Promise<void>((resolve, reject) => {
        const ws = new WebSocket(ASSET_HUB_WS);
        const timer = setTimeout(() => {
          ws.close();
          reject(new Error("timeout"));
        }, 5000);
        ws.onopen = () => {
          ws.send(
            JSON.stringify({ jsonrpc: "2.0", method: "system_health", params: [], id: 1 })
          );
        };
        ws.onmessage = () => {
          clearTimeout(timer);
          ws.close();
          resolve();
        };
        ws.onerror = () => {
          clearTimeout(timer);
          reject(new Error("ws error"));
        };
      });
      console.log(`Chopsticks ready after ${i} attempt(s)`);
      return;
    } catch {
      if (i < maxRetries) {
        await new Promise((r) => setTimeout(r, retryDelayMs));
      }
    }
  }
  throw new Error("Chopsticks did not become ready");
}

async function stopChopsticks(): Promise<void> {
  const proc = chopsticksProcess;
  chopsticksProcess = null;

  if (proc && !proc.killed) {
    try {
      process.kill(-proc.pid!, "SIGTERM");
    } catch {
      try {
        proc.kill("SIGTERM");
      } catch {
        // ignore
      }
    }

    await new Promise((r) => setTimeout(r, 2000));

    try {
      process.kill(-proc.pid!, "SIGKILL");
    } catch {
      try {
        proc.kill("SIGKILL");
      } catch {
        // ignore
      }
    }
  }

  // Best-effort cleanup — never throws
  await new Promise<void>((resolve) => {
    exec("pkill -f '@acala-network/chopsticks' 2>/dev/null || true", () => resolve());
  });

  await new Promise((r) => setTimeout(r, 500));
}

/**
 * Send a JSON-RPC call over WebSocket and return the result.
 */
function rpcCall(
  method: string,
  params: unknown[] = [],
  timeout = 30000
): Promise<unknown> {
  return new Promise((resolve, reject) => {
    const ws = new WebSocket(ASSET_HUB_WS);
    const timer = setTimeout(() => {
      ws.close();
      reject(new Error(`RPC call "${method}" timed out`));
    }, timeout);

    ws.onopen = () => {
      ws.send(JSON.stringify({ jsonrpc: "2.0", method, params, id: 1 }));
    };
    ws.onmessage = (event) => {
      clearTimeout(timer);
      const data = JSON.parse(String(event.data));
      ws.close();
      if (data.error) {
        reject(new Error(`RPC error: ${JSON.stringify(data.error)}`));
      } else {
        resolve(data.result);
      }
    };
    ws.onerror = () => {
      clearTimeout(timer);
      reject(new Error(`WebSocket error for ${method}`));
    };
  });
}

// ---------------------------------------------------------------------------
// Suite
// ---------------------------------------------------------------------------

describe("Register a Foreign Asset on Polkadot Hub Guide", () => {
  let api: ApiPromise;

  beforeAll(async () => {
    await stopChopsticks(); // clean up any stale process

    console.log("Starting Chopsticks (polkadot-asset-hub)...");
    chopsticksProcess = spawn(
      "npx",
      ["@acala-network/chopsticks", "-c", "polkadot-asset-hub"],
      {
        cwd: process.cwd(),
        stdio: ["ignore", "pipe", "pipe"],
        detached: true,
      }
    );

    chopsticksProcess.stdout?.on("data", (data: Buffer) => {
      const line = data.toString().trim();
      if (line) console.log(`[chopsticks] ${line}`);
    });
    chopsticksProcess.stderr?.on("data", (data: Buffer) => {
      const line = data.toString().trim();
      if (line) console.log(`[chopsticks:err] ${line}`);
    });

    await waitForChopsticks();
    await cryptoWaitReady();

    api = await ApiPromise.create({ provider: new WsProvider(ASSET_HUB_WS) });
    console.log("Connected to Polkadot Asset Hub via Chopsticks");

    // Fund the Astar sovereign account on Asset Hub (simulating it having DOT for XCM fees)
    await rpcCall("dev_setStorage", [
      {
        System: {
          Account: [
            [
              [ASTAR_SOVEREIGN],
              {
                providers: 1,
                data: {
                  free: "0x00000000000000008ac7230489e80000", // 10000 DOT
                },
              },
            ],
          ],
        },
      },
    ]);
    console.log("Funded Astar sovereign account on Asset Hub");

    // Advance chain to commit state
    await rpcCall("dev_newBlock", [{ count: 1 }], 60000);
    console.log("Advanced chain by 1 block");
  }, 180000);

  afterAll(async () => {
    if (api) await api.disconnect();
    await stopChopsticks();
  });

  // ==================== 1. Verify foreignAssets Pallet ====================

  describe("1. Verify foreignAssets Pallet on Asset Hub", () => {
    it("should have the foreignAssets pallet available with all required extrinsics", () => {
      expect(api.tx.foreignAssets).toBeDefined();
      expect(api.tx.foreignAssets.create).toBeDefined();
      expect(api.tx.foreignAssets.setMetadata).toBeDefined();
      expect(api.tx.foreignAssets.mint).toBeDefined();
      expect(api.tx.foreignAssets.freeze).toBeDefined();
      console.log("foreignAssets pallet: all expected extrinsics available");
    });

    it("should be able to query foreignAssets storage entries", () => {
      expect(api.query.foreignAssets).toBeDefined();
      expect(api.query.foreignAssets.asset).toBeDefined();
      expect(api.query.foreignAssets.metadata).toBeDefined();
      expect(api.query.foreignAssets.account).toBeDefined();
      console.log("foreignAssets pallet: all storage queries available");
    });

    it("should list existing foreign assets on the Asset Hub fork", async () => {
      const assets = await api.query.foreignAssets.asset.entries();
      console.log(`Found ${assets.length} existing foreign asset(s) on Asset Hub fork`);
      expect(assets.length).toBeGreaterThan(0);

      // Show the first few assets with their Multilocation IDs
      for (const [key] of assets.slice(0, 3)) {
        const multilocation = key.args[0].toHuman();
        console.log("  Foreign asset:", JSON.stringify(multilocation));
      }
    });

    it("should find a sibling parachain foreign asset in the existing state", async () => {
      const assets = await api.query.foreignAssets.asset.entries();
      const siblingAssets = assets.filter(([key]) => {
        const human = JSON.stringify(key.args[0].toHuman());
        return human.includes("Parachain") && human.includes('"parents":"1"');
      });

      console.log(`Found ${siblingAssets.length} sibling parachain foreign asset(s)`);
      expect(siblingAssets.length).toBeGreaterThan(0);

      // Show the first sibling asset (similar to the guide's Astar example)
      const [key, value] = siblingAssets[0];
      const multilocation = key.args[0].toHuman();
      const details = (value as any).toHuman();
      console.log("Sample sibling foreign asset:", JSON.stringify(multilocation));
      console.log("  Status:", details.status);
      console.log("  Min balance:", details.minBalance);
    });
  });

  // ==================== 2. Construct the foreignAssets.create Extrinsic ====================

  describe("2. Construct the foreignAssets.create Extrinsic", () => {
    it("should verify the Astar sovereign account is funded on Asset Hub", async () => {
      const account = await api.query.system.account(ASTAR_SOVEREIGN);
      const free = (account as any).data.free.toBigInt();
      console.log(`Astar sovereign balance on Asset Hub: ${free}`);
      expect(free).toBeGreaterThan(0n);
    });

    it("should construct the foreignAssets.create extrinsic matching the guide", () => {
      // This is the call described in the guide:
      // foreignAssets.create(
      //   id: { parents: 1, interior: { X1: [{ Parachain: 2006 }] } },
      //   admin: <sovereign_account>,
      //   minBalance: 100000
      // )
      // Note: In production this must be called via XCM from the source parachain
      const tx = api.tx.foreignAssets.create(
        ASTAR_ASSET_MULTILOCATION,
        ASTAR_SOVEREIGN,
        MIN_BALANCE
      );

      expect(tx).toBeDefined();
      const encodedHex = tx.method.toHex();
      expect(encodedHex).toBeDefined();
      expect(encodedHex.startsWith("0x35")).toBe(true); // foreignAssets pallet index
      console.log(
        "foreignAssets.create extrinsic encoded:",
        encodedHex.slice(0, 60) + "..."
      );
    });
  });

  // ==================== 3. Simulate Foreign Asset Registration via dev_setStorage ====================

  describe("3. Simulate Foreign Asset Registration (XCM State Injection)", () => {
    // In the guide, foreignAssets.create is called via XCM from the Astar parachain.
    // We simulate this by directly injecting storage state (as Chopsticks XCM would do).

    it("should inject a foreign asset entry for Astar via dev_setStorage", async () => {
      await rpcCall("dev_setStorage", [
        {
          ForeignAssets: {
            Asset: [
              [
                [ASTAR_ASSET_MULTILOCATION],
                {
                  owner: ASTAR_SOVEREIGN,
                  issuer: ASTAR_SOVEREIGN,
                  admin: ASTAR_SOVEREIGN,
                  freezer: ASTAR_SOVEREIGN,
                  supply: 0,
                  deposit: 2019000000,
                  minBalance: MIN_BALANCE.toString(),
                  isSufficient: false,
                  accounts: 0,
                  sufficients: 0,
                  approvals: 0,
                  status: "Live",
                },
              ],
            ],
          },
        },
      ]);

      // Advance by 1 block to commit the storage change
      await rpcCall("dev_newBlock", [{ count: 1 }], 60000);
      console.log("Injected Astar foreign asset via dev_setStorage and advanced chain");
    }, 60000);

    it("should query the Astar foreign asset after registration", async () => {
      const assetDetails = await api.query.foreignAssets.asset(ASTAR_ASSET_MULTILOCATION);

      expect((assetDetails as any).isSome).toBe(true);
      const details = (assetDetails as any).unwrap();
      console.log("Astar foreign asset registered! Details:");
      console.log("  Status:", details.status.toString());
      console.log("  Admin:", details.admin.toString());
      console.log("  Min balance:", details.minBalance.toBigInt().toString());
      expect(details.status.toString()).toBe("Live");
      expect(details.admin.toString()).toBe(ASTAR_SOVEREIGN);
      expect(details.minBalance.toBigInt()).toBe(MIN_BALANCE);
    }, 30000);

    it("should confirm total foreign asset count increased", async () => {
      const assets = await api.query.foreignAssets.asset.entries();
      console.log(`Total foreign assets after registration: ${assets.length}`);
      expect(assets.length).toBeGreaterThan(46); // was 46 before injection
    });
  });

  // ==================== 4. Verify the Registration in the Explorer Context ====================

  describe("4. Verify Asset Registration (Explorer/Event Context)", () => {
    it("should verify Alice is funded (simulating the funded sovereign account in the guide)", async () => {
      const account = await api.query.system.account(ALICE_ADDRESS);
      const free = (account as any).data.free.toBigInt();
      console.log(`Alice DOT balance: ${free}`);
      expect(free).toBeGreaterThan(0n);
    });

    it("should be able to query metadata for the registered Astar foreign asset", async () => {
      // After registration, no metadata is set yet (that's a separate call in the real flow)
      const metadata = await api.query.foreignAssets.metadata(ASTAR_ASSET_MULTILOCATION);
      const meta = metadata as any;
      // Metadata exists but may be empty until setMetadata is called
      console.log("Foreign asset metadata:", {
        name: meta.name?.toHuman?.() ?? "not set",
        symbol: meta.symbol?.toHuman?.() ?? "not set",
        decimals: meta.decimals?.toString() ?? "0",
      });
      expect(meta).toBeDefined();
    }, 30000);

    it("should confirm the registered foreign asset Multilocation matches the guide", async () => {
      // The guide uses this Multilocation for the Astar asset:
      // { parents: 1, interior: { X1: [{ Parachain: 2006 }] } }
      const assets = await api.query.foreignAssets.asset.entries();
      const astarAsset = assets.find(([key]) => {
        const human = JSON.stringify(key.args[0].toHuman());
        return human.includes("2,006") || human.includes("2006");
      });

      expect(astarAsset).toBeDefined();
      const [key] = astarAsset!;
      const multilocation = key.args[0].toHuman();
      console.log("Astar foreign asset Multilocation:", JSON.stringify(multilocation));
      // Verify it's a sibling parachain location (parents=1)
      expect(JSON.stringify(multilocation)).toContain('"parents":"1"');
    }, 30000);
  });
});
