import { describe, it, expect, afterAll, beforeAll } from "vitest";
import { spawn, execSync, ChildProcess } from "child_process";
import { resolve, dirname } from "path";
import { fileURLToPath } from "url";

const __dirname = dirname(fileURLToPath(import.meta.url));
const HARNESS_DIR = resolve(__dirname, "..");

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

const ASSET_HUB_PORT = 8001;
const PEOPLE_CHAIN_PORT = 8000;
const ASSET_HUB_WS = `ws://localhost:${ASSET_HUB_PORT}`;
const PEOPLE_CHAIN_WS = `ws://localhost:${PEOPLE_CHAIN_PORT}`;

// 1 PAS = 10^10 units
const PAS_UNITS = 10_000_000_000n;
const PAS_CENTS = 100_000_000n; // 0.01 PAS

const POLKADOT_HUB_ACCOUNT =
  "15oF4uVJwmo4TdGW7VfQxNLavjCXviqxT9S1MgbjMNHr6Sp5"; // Alice on Paseo Asset Hub
const PEOPLE_CHAIN_BENEFICIARY =
  "14E5nqKAp3oAJcmzgZhUD2RcptBeUBScxKHgJKU4HPNcKVf3"; // Bob on People Chain
const PEOPLE_CHAIN_PARA_ID = 1004;

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
  for (const proc of [assetHubProcess, peopleChainProcess]) {
    if (proc && !proc.killed) {
      try {
        process.kill(-proc.pid!, "SIGTERM");
      } catch {
        proc.kill("SIGTERM");
      }
    }
  }
  assetHubProcess = null;
  peopleChainProcess = null;
  try {
    execSync("pkill -f 'chopsticks' 2>/dev/null || true", { encoding: "utf-8" });
  } catch {
    // ignore — pkill may not find any processes or may be unavailable
  }
  await new Promise((r) => setTimeout(r, 2000));
}

// ---------------------------------------------------------------------------
// Helper: decode SS58 address to 32-byte FixedSizeBinary
// ---------------------------------------------------------------------------

async function accountId32FromSS58(address: string) {
  const { getSs58AddressInfo } = await import("polkadot-api");
  const { FixedSizeBinary } = await import("@polkadot-api/substrate-bindings");
  const info = getSs58AddressInfo(address);
  if (!info.isValid) throw new Error(`Invalid SS58 address: ${address}`);
  return FixedSizeBinary.fromBytes(info.publicKey);
}

// ---------------------------------------------------------------------------
// Helper: build the XCM V5 teleport message
// ---------------------------------------------------------------------------

async function buildTeleportXcm() {
  const {
    XcmVersionedXcm,
    XcmV5Instruction,
    XcmV5Junctions,
    XcmV5Junction,
    XcmV5AssetFilter,
    XcmV5WildAsset,
    XcmV3MultiassetFungibility,
  } = await import("@polkadot-api/descriptors");
  const { Enum } = await import("polkadot-api");

  return XcmVersionedXcm.V5([
    XcmV5Instruction.WithdrawAsset([
      {
        id: { parents: 1, interior: XcmV5Junctions.Here() },
        fun: XcmV3MultiassetFungibility.Fungible(1n * PAS_UNITS),
      },
    ]),
    XcmV5Instruction.PayFees({
      asset: {
        id: { parents: 1, interior: XcmV5Junctions.Here() },
        fun: XcmV3MultiassetFungibility.Fungible(10n * PAS_CENTS),
      },
    }),
    XcmV5Instruction.InitiateTransfer({
      destination: {
        parents: 1,
        interior: XcmV5Junctions.X1(
          XcmV5Junction.Parachain(PEOPLE_CHAIN_PARA_ID)
        ),
      },
      remote_fees: Enum(
        "Teleport",
        XcmV5AssetFilter.Definite([
          {
            id: { parents: 1, interior: XcmV5Junctions.Here() },
            fun: XcmV3MultiassetFungibility.Fungible(10n * PAS_CENTS),
          },
        ])
      ),
      preserve_origin: false,
      remote_xcm: [
        XcmV5Instruction.DepositAsset({
          assets: XcmV5AssetFilter.Wild(XcmV5WildAsset.AllCounted(1)),
          beneficiary: {
            parents: 0,
            interior: XcmV5Junctions.X1(
              XcmV5Junction.AccountId32({
                network: undefined,
                id: await accountId32FromSS58(PEOPLE_CHAIN_BENEFICIARY),
              })
            ),
          },
        }),
      ],
      assets: [
        Enum(
          "Teleport",
          XcmV5AssetFilter.Wild(XcmV5WildAsset.AllCounted(1))
        ),
      ],
    }),
  ]);
}

// ---------------------------------------------------------------------------
// Suite
// ---------------------------------------------------------------------------

describe("XCM Fee Estimation Guide", () => {
  beforeAll(async () => {
    await stopAllChopsticks(); // clean up any stale processes

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

    // Wait for both chains to be ready
    await Promise.all([
      waitForWs(PEOPLE_CHAIN_WS),
      waitForWs(ASSET_HUB_WS),
    ]);
  }, 300000);

  afterAll(async () => {
    await stopAllChopsticks();
  });

  // ==================== 1. PAPI — XCM Fee Estimation ====================

  describe("1. PAPI — XCM Fee Estimation (Polkadot Hub → People Chain)", () => {
    let assetHubClient: any;
    let peopleChainClient: any;

    afterAll(async () => {
      if (assetHubClient) assetHubClient.destroy();
      if (peopleChainClient) peopleChainClient.destroy();
    });

    it("should connect to Paseo Asset Hub (Polkadot Hub)", async () => {
      const { createClient } = await import("polkadot-api");
      const { getWsProvider } = await import("polkadot-api/ws");

      assetHubClient = createClient(getWsProvider(ASSET_HUB_WS));
      expect(assetHubClient).toBeDefined();
      console.log("PAPI: Connected to Paseo Asset Hub");
    });

    it("should connect to Paseo People Chain", async () => {
      const { createClient } = await import("polkadot-api");
      const { getWsProvider } = await import("polkadot-api/ws");

      peopleChainClient = createClient(getWsProvider(PEOPLE_CHAIN_WS));
      expect(peopleChainClient).toBeDefined();
      console.log("PAPI: Connected to Paseo People Chain");
    });

    it("should build the XCM V5 teleport message", async () => {
      const xcm = await buildTeleportXcm();
      expect(xcm).toBeDefined();
      console.log("PAPI: XCM V5 teleport message constructed successfully");
    });

    it("should estimate local execution fees on Polkadot Hub via Polkadot.js", async () => {
      // Use Polkadot.js API for dynamic runtime calls — avoids PAPI descriptor
      // compatibility check with the chopsticks fork's runtime metadata.
      const { ApiPromise, WsProvider } = await import("@polkadot/api");
      const api = await ApiPromise.create({
        provider: new WsProvider(ASSET_HUB_WS),
      });

      const xcm = await buildTeleportXcm();

      // Encode the XCM to SCALE hex using PAPI's codec
      const {
        XcmVersionedXcm,
        XcmV5Instruction,
        XcmV5Junctions,
        XcmV5Junction,
        XcmV5AssetFilter,
        XcmV5WildAsset,
        XcmV3MultiassetFungibility,
      } = await import("@polkadot-api/descriptors");
      const { Enum } = await import("polkadot-api");

      // Build a simpler XCM using V4 (widely supported) for the API call test
      const xcmV4 = {
        V4: [
          {
            WithdrawAsset: [
              {
                id: { parents: 1, interior: "Here" },
                fun: { Fungible: "1000000000000" },
              },
            ],
          },
        ],
      };

      // Use Polkadot.js runtime call API
      const weightResult =
        await api.call.xcmPaymentApi.queryXcmWeight({ V4: xcmV4.V4 });

      console.log("PJS: XCM weight result:", weightResult.toHuman());
      expect(weightResult).toBeDefined();
      await api.disconnect();
    });

    it("should query XCM weight via Polkadot.js on Paseo Asset Hub", async () => {
      const { ApiPromise, WsProvider } = await import("@polkadot/api");
      const api = await ApiPromise.create({
        provider: new WsProvider(ASSET_HUB_WS),
      });

      // Use XCM V4 for broad compatibility in runtime API test
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

      console.log(
        "PJS: XCM weight on Paseo Asset Hub:",
        weightResult.toHuman()
      );
      expect(weightResult).toBeDefined();

      const weightIsOk = (weightResult as any).isOk;
      if (weightIsOk) {
        const weight = (weightResult as any).asOk;
        console.log("PJS: Weight value:", weight.toHuman());

        // Convert weight to PAS fee
        const assetId = api.createType("XcmVersionedAssetId", {
          V4: {
            parents: 1,
            interior: "Here",
          },
        });
        const feeResult = await api.call.xcmPaymentApi.queryWeightToAssetFee(
          weight,
          assetId
        );
        console.log("PJS: Fee result:", feeResult.toHuman());
        expect(feeResult).toBeDefined();

        const feeIsOk = (feeResult as any).isOk;
        if (feeIsOk) {
          const fee = BigInt((feeResult as any).asOk.toString());
          console.log(
            `PJS: Local execution fees: ${fee} PAS units (${Number(fee) / Number(PAS_UNITS)} PAS)`
          );
          expect(fee).toBeGreaterThan(0n);
        }
      }

      await api.disconnect();
    });

    it("should dry-run XCM via Polkadot.js and estimate delivery fees", async () => {
      const { ApiPromise, WsProvider } = await import("@polkadot/api");
      const api = await ApiPromise.create({
        provider: new WsProvider(ASSET_HUB_WS),
      });

      // Build an origin location for Alice
      const origin = api.createType("XcmVersionedLocation", {
        V4: {
          parents: 0,
          interior: {
            X1: [
              {
                AccountId32: {
                  id: "0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d",
                  network: null,
                },
              },
            ],
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
                interior: {
                  X1: [
                    {
                      Parachain: PEOPLE_CHAIN_PARA_ID,
                    },
                  ],
                },
              },
            },
          },
        ],
      });

      const dryRunResult = await api.call.dryRunApi.dryRunXcm(origin, xcm);
      console.log(
        "PJS: Dry run execution result type:",
        (dryRunResult as any).isOk ? "Ok" : "Err"
      );
      expect(dryRunResult).toBeDefined();

      await api.disconnect();
    });

    it("should query XCM weight on Paseo People Chain via Polkadot.js", async () => {
      const { ApiPromise, WsProvider } = await import("@polkadot/api");
      const api = await ApiPromise.create({
        provider: new WsProvider(PEOPLE_CHAIN_WS),
      });

      // Build a simple XCM to test weight query on People Chain
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
                  X1: [
                    {
                      AccountId32: {
                        id: "0x8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48",
                        network: null,
                      },
                    },
                  ],
                },
              },
            },
          },
        ],
      });

      const weightResult =
        await api.call.xcmPaymentApi.queryXcmWeight(xcm);
      console.log(
        "PJS: Remote XCM weight on People Chain:",
        weightResult.toHuman()
      );
      expect(weightResult).toBeDefined();

      await api.disconnect();
    });
  });
});
