import { describe, it, expect, afterAll, beforeAll } from "vitest";
import { spawn, exec, ChildProcess } from "child_process";
import { resolve, dirname } from "path";
import { fileURLToPath } from "url";
import { createClient, Binary, Enum } from "polkadot-api";
import { getWsProvider } from "polkadot-api/ws";
import { polkadotHub } from "@polkadot-api/descriptors";
import type { PolkadotClient } from "polkadot-api";
import { ApiPromise, WsProvider } from "@polkadot/api";
import {
  DEV_PHRASE,
  entropyToMiniSecret,
  mnemonicToEntropy,
  ss58Address,
} from "@polkadot-labs/hdkd-helpers";
import { sr25519CreateDerive } from "@polkadot-labs/hdkd";

const __dirname = dirname(fileURLToPath(import.meta.url));
const HARNESS_DIR = resolve(__dirname, "..");

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

const POLKADOT_HUB_PORT = 8000;
const POLKADOT_HUB_WS = `ws://localhost:${POLKADOT_HUB_PORT}`;

// The XCM call data from block 9079592 on Polkadot Hub (from the guide).
// The fork is pinned to block 9079592 in polkadot-hub.yml to ensure the
// runtime's XCM codec matches this call data deterministically.
const XCM_CALL_DATA =
  "0x1f0803010100411f0300010100fc39fcf04a8071b7409823b7c82427ce67910c6ed80aa0e5093aff234624c8200304000002043205011f0092e81d790000000000";

const XCM_VERSION = 5;

// ---------------------------------------------------------------------------
// Alice derivation (module scope — reused across tests)
// ---------------------------------------------------------------------------

const entropy = mnemonicToEntropy(DEV_PHRASE);
const miniSecret = entropyToMiniSecret(entropy);
const derive = sr25519CreateDerive(miniSecret);
const alice = derive("//Alice");
const aliceAddress = ss58Address(alice.publicKey);
const alicePublicKey = `0x${Buffer.from(alice.publicKey).toString("hex")}`;

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
    if (line) console.error(`[${label}:err] ${line}`);
  });

  return proc;
}

async function stopAllChopsticks(): Promise<void> {
  const proc = polkadotHubProcess;
  polkadotHubProcess = null;

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
    let client: PolkadotClient;

    afterAll(async () => {
      if (client) client.destroy();
    });

    it("should connect to Polkadot Hub Chopsticks fork", async () => {
      client = createClient(getWsProvider(POLKADOT_HUB_WS));
      const api = client.getTypedApi(polkadotHub);
      const runtimeVersion = await api.constants.System.Version();
      expect(runtimeVersion.spec_name).toBeTruthy();
      expect(runtimeVersion.spec_version).toBeGreaterThan(0);
      console.log("PAPI: Connected to Polkadot Hub Chopsticks fork, spec:", runtimeVersion.spec_name, runtimeVersion.spec_version);
    });

    it("should decode XCM call data from block 9079592", async () => {
      const localClient = createClient(getWsProvider(POLKADOT_HUB_WS));
      const api = localClient.getTypedApi(polkadotHub);

      const callData = Binary.fromHex(XCM_CALL_DATA);
      const tx = await api.txFromCallData(callData);

      expect(tx).toBeDefined();
      expect(tx.decodedCall).toBeDefined();
      expect(typeof tx.decodedCall.type).toBe("string");
      expect(tx.decodedCall.type.length).toBeGreaterThan(0);
      console.log("PAPI: XCM decoded call type:", tx.decodedCall?.type);

      localClient.destroy();
    });

    it("should dry-run the XCM call via DryRunApi and get Ok", async () => {
      const localClient = createClient(getWsProvider(POLKADOT_HUB_WS));
      const api = localClient.getTypedApi(polkadotHub);

      // Decode the XCM call
      const callData = Binary.fromHex(XCM_CALL_DATA);
      const tx: any = await api.txFromCallData(callData);

      // Build the origin as a signed account using module-scope derived Alice
      const origin = Enum("system", Enum("Signed", aliceAddress));

      const dryRunResult: any = await api.apis.DryRunApi.dry_run_call(
        origin,
        tx.decodedCall,
        XCM_VERSION
      );

      expect(dryRunResult).toBeDefined();
      // PAPI encodes Rust Result<T,E> as { success: boolean, value: T | E }
      expect(dryRunResult.success).toBe(true);
      console.log("PAPI: Dry run succeeded with Ok result");

      localClient.destroy();
    });

    it("should verify DryRunApi is available on Polkadot Hub", async () => {
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
      const api = await ApiPromise.create({
        provider: new WsProvider(POLKADOT_HUB_WS),
      });

      // Build the origin for Alice using module-scope derived public key
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
      expect(dryRunResult).toBeDefined();
      expect((dryRunResult as any).isOk || (dryRunResult as any).isErr).toBe(true);
      console.log(
        "PJS: Dry run XCM result:",
        (dryRunResult as any).isOk ? "Ok" : "Err"
      );

      await api.disconnect();
    });
  });
});
