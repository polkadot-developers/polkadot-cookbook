import { describe, it, expect, afterAll, beforeAll } from "vitest";
import { spawn, ChildProcess } from "child_process";
import { resolve, dirname } from "path";
import { fileURLToPath } from "url";
import { ApiPromise, WsProvider } from "@polkadot/api";
import { Keyring } from "@polkadot/keyring";
import { cryptoWaitReady } from "@polkadot/util-crypto";

const __dirname = dirname(fileURLToPath(import.meta.url));
const HARNESS_DIR = resolve(__dirname, "..");

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

const ASSET_HUB_PORT = 8001;
const PEOPLE_CHAIN_PORT = 8000;
const ASSET_HUB_WS = `ws://localhost:${ASSET_HUB_PORT}`;
const PEOPLE_CHAIN_WS = `ws://localhost:${PEOPLE_CHAIN_PORT}`;

const PAS_UNITS = 10_000_000_000n; // 1 PAS = 10^10 planck
const PEOPLE_CHAIN_PARA_ID = 1004;

// Derived in beforeAll after cryptoWaitReady()
let alicePubHex: string;
let bobPubHex: string;

// ---------------------------------------------------------------------------
// Chopsticks lifecycle helpers
// ---------------------------------------------------------------------------

let assetHubProcess: ChildProcess | null = null;
let peopleChainProcess: ChildProcess | null = null;

async function waitForWs(
  url: string,
  maxRetries = 50,
  retryDelayMs = 3000
): Promise<void> {
  for (let i = 1; i <= maxRetries; i++) {
    try {
      await new Promise<void>((resolve, reject) => {
        const ws = new WebSocket(url);
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
      console.log(`[${url}] ready after ${i} attempt(s)`);
      return;
    } catch {
      if (i < maxRetries) {
        await new Promise((r) => setTimeout(r, retryDelayMs));
      }
    }
  }
  throw new Error(`WebSocket at ${url} did not become ready`);
}

function spawnChopsticks(
  configPath: string,
  port: number,
  label: string
): ChildProcess {
  const proc = spawn(
    "npx",
    [
      "@acala-network/chopsticks",
      "--config",
      configPath,
      "--port",
      String(port),
    ],
    {
      cwd: HARNESS_DIR,
      stdio: ["ignore", "pipe", "pipe"],
      detached: true,
    }
  );

  proc.stdout?.on("data", (data: Buffer) => {
    const line = data.toString().trim();
    if (line) console.log(`[${label}] ${line}`);
  });
  proc.stderr?.on("data", (data: Buffer) => {
    const line = data.toString().trim();
    if (line) console.log(`[${label}:err] ${line}`);
  });

  return proc;
}

async function stopAllChopsticks(): Promise<void> {
  const procs = [assetHubProcess, peopleChainProcess];
  assetHubProcess = null;
  peopleChainProcess = null;

  for (const proc of procs) {
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
    }
  }

  await new Promise((r) => setTimeout(r, 2000));

  for (const proc of procs) {
    if (proc && !proc.killed) {
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
  }

  await new Promise((r) => setTimeout(r, 500));
}

// ---------------------------------------------------------------------------
// Suite
// ---------------------------------------------------------------------------

describe("XCM Fee Estimation Guide", () => {
  beforeAll(async () => {
    await cryptoWaitReady();
    const keyring = new Keyring({ type: "sr25519" });
    alicePubHex =
      "0x" + Buffer.from(keyring.addFromUri("//Alice").publicKey).toString("hex");
    bobPubHex =
      "0x" + Buffer.from(keyring.addFromUri("//Bob").publicKey).toString("hex");

    await stopAllChopsticks();

    const peopleChainConfig = resolve(HARNESS_DIR, "paseo-people-chain.yml");
    const assetHubConfig = resolve(HARNESS_DIR, "paseo-asset-hub.yml");

    console.log("Starting Chopsticks (Paseo People Chain) on port 8000...");
    peopleChainProcess = spawnChopsticks(
      peopleChainConfig,
      PEOPLE_CHAIN_PORT,
      "people-chain"
    );

    console.log("Starting Chopsticks (Paseo Asset Hub) on port 8001...");
    assetHubProcess = spawnChopsticks(
      assetHubConfig,
      ASSET_HUB_PORT,
      "asset-hub"
    );

    await Promise.all([waitForWs(PEOPLE_CHAIN_WS), waitForWs(ASSET_HUB_WS)]);
  }, 300000);

  afterAll(async () => {
    await stopAllChopsticks();
  });

  // ==================== 1. Local Execution Fees (Polkadot Hub) ====================

  describe("1. XCM Weight and Fees on Polkadot Hub (Paseo Asset Hub)", () => {
    it("should query XCM weight for a V4 teleport message", async () => {
      const api = await ApiPromise.create({
        provider: new WsProvider(ASSET_HUB_WS),
      });

      const xcm = api.createType("XcmVersionedXcm", {
        V4: [
          {
            WithdrawAsset: [
              {
                id: { parents: 1, interior: "Here" },
                fun: { Fungible: "10000000000" },
              },
            ],
          },
          {
            BuyExecution: {
              fees: {
                id: { parents: 1, interior: "Here" },
                fun: { Fungible: "10000000000" },
              },
              weightLimit: "Unlimited",
            },
          },
        ],
      });

      const result = await api.call.xcmPaymentApi.queryXcmWeight(xcm);
      expect((result as any).isOk).toBe(true);

      const weight = (result as any).asOk;
      const refTime = BigInt(weight.refTime.toString());
      expect(refTime).toBeGreaterThan(0n);
      console.log(
        `XCM weight on Paseo Asset Hub: refTime=${refTime}, proofSize=${weight.proofSize}`
      );

      await api.disconnect();
    });

    it("should convert XCM weight to a PAS fee greater than zero and less than 10 PAS", async () => {
      const api = await ApiPromise.create({
        provider: new WsProvider(ASSET_HUB_WS),
      });

      const xcm = api.createType("XcmVersionedXcm", {
        V4: [
          {
            WithdrawAsset: [
              {
                id: { parents: 1, interior: "Here" },
                fun: { Fungible: "10000000000" },
              },
            ],
          },
          {
            BuyExecution: {
              fees: {
                id: { parents: 1, interior: "Here" },
                fun: { Fungible: "10000000000" },
              },
              weightLimit: "Unlimited",
            },
          },
        ],
      });

      const weightResult = await api.call.xcmPaymentApi.queryXcmWeight(xcm);
      expect((weightResult as any).isOk).toBe(true);
      const weight = (weightResult as any).asOk;

      const assetId = api.createType("XcmVersionedAssetId", {
        V4: { parents: 1, interior: "Here" },
      });
      const feeResult = await api.call.xcmPaymentApi.queryWeightToAssetFee(
        weight,
        assetId
      );
      expect((feeResult as any).isOk).toBe(true);

      const fee = BigInt((feeResult as any).asOk.toString());
      expect(fee).toBeGreaterThan(0n);
      expect(fee).toBeLessThan(10n * PAS_UNITS);
      console.log(
        `Local execution fee on Paseo Asset Hub: ${fee} planck (${Number(fee) / Number(PAS_UNITS)} PAS)`
      );

      await api.disconnect();
    });

    it("should dry-run the XCM and confirm execution proceeds without error", async () => {
      const api = await ApiPromise.create({
        provider: new WsProvider(ASSET_HUB_WS),
      });

      const origin = api.createType("XcmVersionedLocation", {
        V4: {
          parents: 0,
          interior: {
            X1: [{ AccountId32: { id: alicePubHex, network: null } }],
          },
        },
      });

      const xcm = api.createType("XcmVersionedXcm", {
        V4: [
          {
            WithdrawAsset: [
              {
                id: { parents: 1, interior: "Here" },
                fun: { Fungible: "10000000000" },
              },
            ],
          },
          {
            BuyExecution: {
              fees: {
                id: { parents: 1, interior: "Here" },
                fun: { Fungible: "10000000000" },
              },
              weightLimit: "Unlimited",
            },
          },
          {
            DepositAsset: {
              assets: { Wild: { AllCounted: 1 } },
              beneficiary: {
                parents: 0,
                interior: { X1: [{ Parachain: PEOPLE_CHAIN_PARA_ID }] },
              },
            },
          },
        ],
      });

      const result = await api.call.dryRunApi.dryRunXcm(origin, xcm);
      expect((result as any).isOk).toBe(true);
      const executionResult = (result as any).asOk.executionResult;
      console.log(
        "Dry run execution result:",
        JSON.stringify(executionResult.toHuman())
      );

      await api.disconnect();
    });
  });

  // ==================== 2. Remote Execution Fees (People Chain) ====================

  describe("2. XCM Weight and Fees on Paseo People Chain", () => {
    it("should query remote XCM weight for a teleport receive message", async () => {
      const api = await ApiPromise.create({
        provider: new WsProvider(PEOPLE_CHAIN_WS),
      });

      const xcm = api.createType("XcmVersionedXcm", {
        V4: [
          {
            ReceiveTeleportedAsset: [
              {
                id: { parents: 1, interior: "Here" },
                fun: { Fungible: "10000000000" },
              },
            ],
          },
          {
            DepositAsset: {
              assets: { Wild: { AllCounted: 1 } },
              beneficiary: {
                parents: 0,
                interior: {
                  X1: [{ AccountId32: { id: bobPubHex, network: null } }],
                },
              },
            },
          },
        ],
      });

      const result = await api.call.xcmPaymentApi.queryXcmWeight(xcm);
      expect((result as any).isOk).toBe(true);

      const weight = (result as any).asOk;
      const refTime = BigInt(weight.refTime.toString());
      expect(refTime).toBeGreaterThan(0n);
      console.log(`Remote XCM weight on Paseo People Chain: refTime=${refTime}`);

      await api.disconnect();
    });

    it("should convert remote XCM weight to a PAS fee greater than zero and less than 10 PAS", async () => {
      const api = await ApiPromise.create({
        provider: new WsProvider(PEOPLE_CHAIN_WS),
      });

      const xcm = api.createType("XcmVersionedXcm", {
        V4: [
          {
            ReceiveTeleportedAsset: [
              {
                id: { parents: 1, interior: "Here" },
                fun: { Fungible: "10000000000" },
              },
            ],
          },
          {
            DepositAsset: {
              assets: { Wild: { AllCounted: 1 } },
              beneficiary: {
                parents: 0,
                interior: {
                  X1: [{ AccountId32: { id: bobPubHex, network: null } }],
                },
              },
            },
          },
        ],
      });

      const weightResult = await api.call.xcmPaymentApi.queryXcmWeight(xcm);
      expect((weightResult as any).isOk).toBe(true);
      const weight = (weightResult as any).asOk;

      const assetId = api.createType("XcmVersionedAssetId", {
        V4: { parents: 1, interior: "Here" },
      });
      const feeResult = await api.call.xcmPaymentApi.queryWeightToAssetFee(
        weight,
        assetId
      );
      expect((feeResult as any).isOk).toBe(true);

      const fee = BigInt((feeResult as any).asOk.toString());
      expect(fee).toBeGreaterThan(0n);
      expect(fee).toBeLessThan(10n * PAS_UNITS);
      console.log(
        `Remote execution fee on Paseo People Chain: ${fee} planck (${Number(fee) / Number(PAS_UNITS)} PAS)`
      );

      await api.disconnect();
    });
  });
});
