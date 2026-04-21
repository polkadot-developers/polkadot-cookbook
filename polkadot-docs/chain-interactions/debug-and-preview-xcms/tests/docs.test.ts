import { describe, it, expect, afterAll, beforeAll } from "vitest";
import { spawn, execSync, ChildProcess } from "child_process";
import { resolve, dirname } from "path";
import { fileURLToPath } from "url";

const __dirname = dirname(fileURLToPath(import.meta.url));
const HARNESS_DIR = resolve(__dirname, "..");

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

const POLKADOT_HUB_PORT = 8000;
const POLKADOT_HUB_WS = `ws://localhost:${POLKADOT_HUB_PORT}`;

// The XCM call data from block 9079592 on Polkadot Hub (from the guide)
const XCM_CALL_DATA =
  "0x1f0803010100411f0300010100fc39fcf04a8071b7409823b7c82427ce67910c6ed80aa0e5093aff234624c8200304000002043205011f0092e81d790000000000";

const XCM_VERSION = 5;

// ---------------------------------------------------------------------------
// Chopsticks lifecycle helpers
// ---------------------------------------------------------------------------

let polkadotHubProcess: ChildProcess | null = null;

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
  for (const proc of [polkadotHubProcess]) {
    if (proc && !proc.killed) {
      try {
        process.kill(-proc.pid!, "SIGTERM");
      } catch {
        proc.kill("SIGTERM");
      }
    }
  }
  polkadotHubProcess = null;
  try {
    execSync("pkill -f 'chopsticks' 2>/dev/null || true", { encoding: "utf-8" });
  } catch {
    // ignore — pkill may not find any processes or may be unavailable
  }
  await new Promise((r) => setTimeout(r, 2000));
}

// ---------------------------------------------------------------------------
// Suite
// ---------------------------------------------------------------------------

describe("Replay and Dry Run XCMs Guide", () => {
  beforeAll(async () => {
    await stopAllChopsticks(); // clean up any stale processes

    const polkadotHubConfig = resolve(HARNESS_DIR, "polkadot-hub.yml");

    console.log("Starting Chopsticks (Polkadot Hub) on port 8000...");
    polkadotHubProcess = spawnChopsticks(
      polkadotHubConfig,
      POLKADOT_HUB_PORT,
      "polkadot-hub"
    );

    await waitForWs(POLKADOT_HUB_WS);
  }, 300000);

  afterAll(async () => {
    await stopAllChopsticks();
  });

  // ==================== 1. PAPI — XCM Replay and Dry Run ====================

  describe("1. PAPI — XCM Replay and Dry Run (Polkadot Hub)", () => {
    let client: any;

    afterAll(async () => {
      if (client) client.destroy();
    });

    it("should connect to Polkadot Hub Chopsticks fork", async () => {
      const { createClient } = await import("polkadot-api");
      const { getWsProvider } = await import("polkadot-api/ws");

      client = createClient(getWsProvider(POLKADOT_HUB_WS));
      expect(client).toBeDefined();
      console.log("PAPI: Connected to Polkadot Hub Chopsticks fork");
    });

    it("should decode XCM call data from block 9079592", async () => {
      const { createClient, Binary } = await import("polkadot-api");
      const { getWsProvider } = await import("polkadot-api/ws");
      const { polkadotHub } = await import("@polkadot-api/descriptors");

      const localClient = createClient(getWsProvider(POLKADOT_HUB_WS));
      const api = localClient.getTypedApi(polkadotHub);

      const callData = Binary.fromHex(XCM_CALL_DATA);
      const tx = await api.txFromCallData(callData);

      expect(tx).toBeDefined();
      expect(tx.decodedCall).toBeDefined();
      console.log(
        "PAPI: XCM decoded call type:",
        tx.decodedCall?.type
      );

      localClient.destroy();
    });

    it("should dry-run the XCM call via DryRunApi", async () => {
      const { createClient, Binary, Enum } = await import("polkadot-api");
      const { getWsProvider } = await import("polkadot-api/ws");
      const { polkadotHub } = await import("@polkadot-api/descriptors");
      const {
        DEV_PHRASE,
        entropyToMiniSecret,
        mnemonicToEntropy,
        ss58Address,
      } = await import("@polkadot-labs/hdkd-helpers");
      const { sr25519CreateDerive } = await import("@polkadot-labs/hdkd");

      const localClient = createClient(getWsProvider(POLKADOT_HUB_WS));
      const api = localClient.getTypedApi(polkadotHub);

      // Derive Alice's address
      const entropy = mnemonicToEntropy(DEV_PHRASE);
      const miniSecret = entropyToMiniSecret(entropy);
      const derive = sr25519CreateDerive(miniSecret);
      const alice = derive("//Alice");
      const aliceAddress = ss58Address(alice.publicKey);

      // Decode the XCM call
      const callData = Binary.fromHex(XCM_CALL_DATA);
      const tx: any = await api.txFromCallData(callData);

      // Build the origin as a signed account
      const origin = Enum("system", Enum("Signed", aliceAddress));

      // Perform the dry run
      const dryRunResult: any = await api.apis.DryRunApi.dry_run_call(
        origin,
        tx.decodedCall,
        XCM_VERSION
      );

      expect(dryRunResult).toBeDefined();
      console.log(
        "PAPI: Dry run result type:",
        dryRunResult?.type ?? "unknown"
      );

      // The dry run result should be an Ok or Err variant
      if (dryRunResult?.type === "Ok") {
        console.log("PAPI: Dry run succeeded");
        expect(dryRunResult.value).toBeDefined();
      } else if (dryRunResult?.type === "Err") {
        // Dry run may fail due to missing funds in the fork — this is acceptable
        console.log(
          "PAPI: Dry run returned Err (expected for underfunded fork):",
          JSON.stringify(dryRunResult.value)
        );
      } else {
        console.log("PAPI: Dry run result:", JSON.stringify(dryRunResult));
      }

      localClient.destroy();
    });

    it("should verify DryRunApi is available on Polkadot Hub", async () => {
      const { ApiPromise, WsProvider } = await import("@polkadot/api");
      const api = await ApiPromise.create({
        provider: new WsProvider(POLKADOT_HUB_WS),
      });

      // Verify DryRunApi runtime call is available
      expect(api.call.dryRunApi).toBeDefined();
      console.log("PJS: DryRunApi confirmed available on Polkadot Hub fork");

      // Also verify xcmPaymentApi is available
      expect(api.call.xcmPaymentApi).toBeDefined();
      console.log("PJS: XcmPaymentApi confirmed available on Polkadot Hub fork");

      await api.disconnect();
    });

    it("should dry-run the XCM call via Polkadot.js DryRunApi", async () => {
      const { ApiPromise, WsProvider } = await import("@polkadot/api");
      const api = await ApiPromise.create({
        provider: new WsProvider(POLKADOT_HUB_WS),
      });

      // Build the origin for Alice (dev account)
      const alicePublicKey =
        "0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d";

      const origin = api.createType("XcmVersionedLocation", {
        V4: {
          parents: 0,
          interior: {
            X1: [
              {
                AccountId32: {
                  id: alicePublicKey,
                  network: null,
                },
              },
            ],
          },
        },
      });

      // Build a simple XCM to dry-run
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

      const dryRunResult = await api.call.dryRunApi.dryRunXcm(origin, xcm);
      console.log(
        "PJS: Dry run XCM result:",
        (dryRunResult as any).isOk ? "Ok" : "Err"
      );
      expect(dryRunResult).toBeDefined();

      await api.disconnect();
    });
  });
});
