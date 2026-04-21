import { describe, it, expect, afterAll, beforeAll } from "vitest";
import { ApiPromise, WsProvider } from "@polkadot/api";
import { Keyring } from "@polkadot/keyring";
import { cryptoWaitReady, addressEq } from "@polkadot/util-crypto";
import { spawn, exec, ChildProcess } from "child_process";
import type { KeyringPair } from "@polkadot/keyring/types";
import type { ISubmittableResult } from "@polkadot/types/types";

const ASSET_HUB_WS = "ws://localhost:8000";

// Alice's well-known dev address; funded in beforeAll via dev_setStorage
const ALICE_ADDRESS = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY";
// Bob / Charlie / Dave for admin / issuer / freezer roles in setTeam
const BOB_ADDRESS = "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty";
const CHARLIE_ADDRESS = "5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y";
const DAVE_ADDRESS = "5DAAnrj7VHTznn2AWBemMuyBwZWs6FNFjdyVXUeYum3PTXFy";

// Guide-style asset parameters (name / symbol / decimals / minimum balance).
// Asset ID is picked dynamically from the live fork state to avoid collision.
const ASSET_NAME = "Cookbook Test Token";
const ASSET_SYMBOL = "CBK";
const ASSET_DECIMALS = 10;
const MIN_BALANCE = 1_000_000n;

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
            JSON.stringify({
              jsonrpc: "2.0",
              method: "system_health",
              params: [],
              id: 1,
            })
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

  await new Promise<void>((resolve) => {
    exec("pkill -f '@acala-network/chopsticks' 2>/dev/null || true", () =>
      resolve()
    );
  });

  await new Promise((r) => setTimeout(r, 500));
}

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

// Submit an extrinsic, wait until it's in a block, advance one block, and
// return the events. Fails fast if any ExtrinsicFailed event is emitted.
async function submitAndFinalize(
  api: ApiPromise,
  tx: ReturnType<ApiPromise["tx"]["assets"]["create"]>,
  signer: KeyringPair
): Promise<ISubmittableResult> {
  const result = await new Promise<ISubmittableResult>((resolve, reject) => {
    let settled = false;
    tx.signAndSend(signer, (r) => {
      if (settled) return;
      if (r.dispatchError) {
        settled = true;
        reject(new Error(`dispatchError: ${r.dispatchError.toString()}`));
        return;
      }
      if (r.isInBlock || r.isFinalized) {
        settled = true;
        resolve(r);
      }
    }).catch((e) => {
      if (!settled) {
        settled = true;
        reject(e);
      }
    });
  });

  const failed = result.events.find(({ event }) =>
    api.events.system.ExtrinsicFailed.is(event)
  );
  if (failed) {
    throw new Error(`ExtrinsicFailed: ${failed.event.data.toString()}`);
  }

  // Advance the chain one block so subsequent queries observe the update
  await rpcCall("dev_newBlock", [{ count: 1 }], 60000);
  return result;
}

// ---------------------------------------------------------------------------
// Suite
// ---------------------------------------------------------------------------

describe("Register a Local Asset on Polkadot Hub Guide", () => {
  let api: ApiPromise;
  let alice: KeyringPair;
  let assetId: number;
  let initialAssetCount = 0;

  beforeAll(async () => {
    await stopChopsticks();

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

    // Fund Alice with enough DOT to cover the 10 DOT + ~0.201 DOT metadata
    // deposit required by the guide, plus transaction fees.
    // 0x00000000000000008ac7230489e80000 = 10000 DOT (little-endian u128)
    await rpcCall("dev_setStorage", [
      {
        System: {
          Account: [
            [
              [ALICE_ADDRESS],
              {
                providers: 1,
                data: { free: "0x00000000000000008ac7230489e80000" },
              },
            ],
          ],
        },
      },
    ]);
    await rpcCall("dev_newBlock", [{ count: 1 }], 60000);

    const balance = await api.query.system.account(ALICE_ADDRESS);
    console.log(
      `Alice DOT balance on fork: ${(balance as any).data.free.toBigInt()}`
    );

    const keyring = new Keyring({ type: "sr25519", ss58Format: 0 });
    alice = keyring.addFromUri("//Alice");

    // Enumerate existing assets and snapshot the count so later tests can
    // assert that registration increased it.
    const existingAssets = await api.query.assets.asset.entries();
    initialAssetCount = existingAssets.length;
    console.log(
      `Found ${initialAssetCount} existing local asset(s) on Asset Hub fork`
    );

    // Polkadot Asset Hub's pallet-assets enforces sequential IDs via the
    // `NextAssetId` storage item — `assets.create` rejects any other ID with
    // `BadAssetId`. Use the runtime-issued next ID when set, otherwise pick a
    // unique one far above all existing IDs.
    const nextAssetId = (await api.query.assets.nextAssetId()) as any;
    if (nextAssetId.isSome) {
      assetId = Number(nextAssetId.unwrap().toString());
      console.log(`Using runtime-issued next asset ID ${assetId}`);
    } else {
      const existingIds = existingAssets.map(([key]) =>
        Number((key.args[0] as any).toString())
      );
      const maxId = existingIds.length > 0 ? Math.max(...existingIds) : 0;
      assetId = maxId + 100_000;
      console.log(`Chose asset ID ${assetId} (no NextAssetId set)`);
    }
  }, 240000);

  afterAll(async () => {
    if (api) await api.disconnect();
    await stopChopsticks();
  });

  // ==================== 1. Verify assets Pallet ====================

  describe("1. Verify assets Pallet on Polkadot Hub", () => {
    it("should have the assets pallet available with all required extrinsics", () => {
      expect(api.tx.assets).toBeDefined();
      expect(api.tx.assets.create).toBeDefined();
      expect(api.tx.assets.setMetadata).toBeDefined();
      expect(api.tx.assets.setTeam).toBeDefined();
      expect(api.tx.assets.mint).toBeDefined();
      expect(api.tx.assets.freeze).toBeDefined();
      console.log("assets pallet: all guide extrinsics available");
    });

    it("should expose assets storage queries", () => {
      expect(api.query.assets).toBeDefined();
      expect(api.query.assets.asset).toBeDefined();
      expect(api.query.assets.metadata).toBeDefined();
      expect(api.query.assets.account).toBeDefined();
      console.log("assets pallet: all storage queries available");
    });

    it("should confirm the chosen asset ID is unique on the fork", async () => {
      const details = await api.query.assets.asset(assetId);
      expect((details as any).isNone).toBe(true);
      console.log(`Asset ID ${assetId} is available`);
    });
  });

  // ==================== 2. Create the Asset ====================

  describe("2. Create the Asset (assets.create)", () => {
    it("should sign and submit assets.create with Alice as admin", async () => {
      const tx = api.tx.assets.create(assetId, ALICE_ADDRESS, MIN_BALANCE);
      await submitAndFinalize(api, tx, alice);

      const details = await api.query.assets.asset(assetId);
      expect((details as any).isSome).toBe(true);

      const asset = (details as any).unwrap();
      expect(addressEq(asset.owner.toString(), ALICE_ADDRESS)).toBe(true);
      expect(addressEq(asset.admin.toString(), ALICE_ADDRESS)).toBe(true);
      expect(asset.minBalance.toBigInt()).toBe(MIN_BALANCE);
      expect(asset.status.toString()).toBe("Live");
      console.log(
        `Asset ${assetId} created — status=${asset.status.toString()}, minBalance=${asset.minBalance.toBigInt()}`
      );
    }, 90000);

    it("should have incremented the total asset count", async () => {
      const assets = await api.query.assets.asset.entries();
      console.log(
        `Total local assets: ${assets.length} (was ${initialAssetCount})`
      );
      expect(assets.length).toBe(initialAssetCount + 1);
    });
  });

  // ==================== 3. Set Metadata ====================

  describe("3. Set Metadata (assets.setMetadata)", () => {
    it("should sign and submit assets.setMetadata and persist name/symbol/decimals", async () => {
      const tx = api.tx.assets.setMetadata(
        assetId,
        ASSET_NAME,
        ASSET_SYMBOL,
        ASSET_DECIMALS
      );
      await submitAndFinalize(api, tx, alice);

      const metadata = (await api.query.assets.metadata(assetId)) as any;
      const name = Buffer.from(metadata.name.toU8a(true)).toString("utf-8");
      const symbol = Buffer.from(metadata.symbol.toU8a(true)).toString("utf-8");
      const decimals = metadata.decimals.toNumber();

      expect(name).toBe(ASSET_NAME);
      expect(symbol).toBe(ASSET_SYMBOL);
      expect(decimals).toBe(ASSET_DECIMALS);
      console.log(
        `Metadata: name="${name}", symbol="${symbol}", decimals=${decimals}`
      );
    }, 90000);
  });

  // ==================== 4. Assign Team ====================

  describe("4. Assign Admin / Issuer / Freezer (assets.setTeam)", () => {
    it("should sign and submit assets.setTeam and update all three roles", async () => {
      const tx = api.tx.assets.setTeam(
        assetId,
        BOB_ADDRESS,
        CHARLIE_ADDRESS,
        DAVE_ADDRESS
      );
      await submitAndFinalize(api, tx, alice);

      const asset = ((await api.query.assets.asset(assetId)) as any).unwrap();
      expect(addressEq(asset.issuer.toString(), BOB_ADDRESS)).toBe(true);
      expect(addressEq(asset.admin.toString(), CHARLIE_ADDRESS)).toBe(true);
      expect(addressEq(asset.freezer.toString(), DAVE_ADDRESS)).toBe(true);
      // Owner is unchanged — only the team is updated
      expect(addressEq(asset.owner.toString(), ALICE_ADDRESS)).toBe(true);
      console.log(
        `Team: owner=Alice, issuer=Bob, admin=Charlie, freezer=Dave`
      );
    }, 90000);
  });
});
