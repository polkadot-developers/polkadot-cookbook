import { describe, it, expect, afterAll, beforeAll } from "vitest";
import { ApiPromise, WsProvider } from "@polkadot/api";
import { Keyring } from "@polkadot/keyring";
import { cryptoWaitReady } from "@polkadot/util-crypto";
import { spawn, execSync, ChildProcess } from "child_process";
import { join } from "path";

const CHOPSTICKS_PORT = 8000;
const CHOPSTICKS_WS = `ws://localhost:${CHOPSTICKS_PORT}`;
const TARGET_ADDRESS = "14E5nqKAp3oAJcmzgZhUD2RcptBeUBScxKHgJKU4HPNcKVf3";
const TRANSFER_AMOUNT = 3_000_000_000n;
const USDT_ASSET_ID = 1984;

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
        const ws = new WebSocket(CHOPSTICKS_WS);
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
  if (chopsticksProcess && !chopsticksProcess.killed) {
    try {
      process.kill(-chopsticksProcess.pid!, "SIGTERM");
    } catch {
      chopsticksProcess.kill("SIGTERM");
    }
    chopsticksProcess = null;
  }
  execSync("pkill -f 'chopsticks' 2>/dev/null || true", { encoding: "utf-8" });
  await new Promise((r) => setTimeout(r, 2000));
}

// ---------------------------------------------------------------------------
// Suite
// ---------------------------------------------------------------------------

describe("Pay Fees with Different Tokens Guide", () => {
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
  }, 180000);

  afterAll(async () => {
    await stopChopsticks();
  });

  // ==================== 1. PAPI ====================

  describe("1. PAPI — Pay Fees with USDT", () => {
    let client: any;

    afterAll(async () => {
      if (client) await client.destroy();
    });

    it("should send a transfer paying fees in USDT", async () => {
      const { sr25519CreateDerive } = await import("@polkadot-labs/hdkd");
      const { DEV_PHRASE, entropyToMiniSecret, mnemonicToEntropy } =
        await import("@polkadot-labs/hdkd-helpers");
      const { getPolkadotSigner } = await import("polkadot-api/signer");
      const { createClient } = await import("polkadot-api");
      const { withPolkadotSdkCompat } = await import(
        "polkadot-api/polkadot-sdk-compat"
      );
      const { getWsProvider } = await import("polkadot-api/ws-provider/node");
      const { assetHub, MultiAddress } = await import(
        "@polkadot-api/descriptors"
      );

      // Create signer
      const entropy = mnemonicToEntropy(DEV_PHRASE);
      const miniSecret = entropyToMiniSecret(entropy);
      const derive = sr25519CreateDerive(miniSecret);
      const hdkdKeyPair = derive("//Alice");
      const signer = getPolkadotSigner(
        hdkdKeyPair.publicKey,
        "Sr25519",
        hdkdKeyPair.sign
      );

      // Connect
      client = createClient(
        withPolkadotSdkCompat(getWsProvider(CHOPSTICKS_WS))
      );
      const api = client.getTypedApi(assetHub);

      // Create and submit tx
      const tx = api.tx.Balances.transfer_keep_alive({
        dest: MultiAddress.Id(TARGET_ADDRESS),
        value: BigInt(TRANSFER_AMOUNT),
      });

      const result = await tx.signAndSubmit(signer, {
        asset: {
          parents: 0,
          interior: {
            type: "X2",
            value: [
              { type: "PalletInstance", value: 50 },
              { type: "GeneralIndex", value: BigInt(USDT_ASSET_ID) },
            ],
          },
        },
      });

      console.log(`PAPI: Tx finalized: ${result.txHash} (ok=${result.ok})`);
      expect(result.txHash).toBeDefined();
      expect(result.ok).toBe(true);
    });
  });

  // ==================== 2. Polkadot.js ====================

  describe("2. Polkadot.js — Pay Fees with USDT", () => {
    let api: ApiPromise;

    afterAll(async () => {
      if (api) await api.disconnect();
    });

    it("should send a transfer paying fees in USDT", async () => {
      await cryptoWaitReady();

      const keyring = new Keyring({ type: "sr25519" });
      const alice = keyring.addFromUri("//Alice");

      const wsProvider = new WsProvider(CHOPSTICKS_WS);
      api = await ApiPromise.create({ provider: wsProvider });
      console.log("Polkadot.js: Connected to Chopsticks fork");

      const tx = api.tx.balances.transferKeepAlive(
        TARGET_ADDRESS,
        TRANSFER_AMOUNT
      );

      const assetId = {
        parents: 0,
        interior: {
          X2: [{ PalletInstance: 50 }, { GeneralIndex: USDT_ASSET_ID }],
        },
      };

      const result = await new Promise<string>((resolve, reject) => {
        let unsub: (() => void) | undefined;
        tx.signAndSend(
          alice,
          { assetId },
          ({ status, txHash, dispatchError }) => {
            if (status.isFinalized) {
              if (unsub) unsub();
              if (dispatchError) {
                reject(new Error(dispatchError.toString()));
              } else {
                resolve(txHash.toHex());
              }
            }
          }
        )
          .then((u) => {
            unsub = u;
          })
          .catch(reject);
      });

      console.log(`Polkadot.js: Tx finalized: ${result}`);
      expect(result).toBeDefined();
    });
  });

  // ==================== 3. Subxt ====================

  describe("3. Subxt — Pay Fees with USDT", () => {
    it(
      "should send a transfer paying fees in USDT",
      () => {
        const result = execSync("cargo run --bin fee_payment_transaction", {
          cwd: join(__dirname, "subxt-pay-fees"),
          encoding: "utf-8",
          timeout: 120000,
        });
        console.log(result);
        expect(result).toContain("Transaction finalized in block");
        expect(result).toContain("Events:");
      },
      120000
    );
  });
});
