import { describe, it, expect, afterAll } from "vitest";
import { execSync, spawn, ChildProcess } from "child_process";

const PROJECT_DIR = process.cwd();
const CHOPSTICKS_PORT = 8000;
const CHOPSTICKS_WS = `ws://localhost:${CHOPSTICKS_PORT}`;

let chopsticksProcess: ChildProcess | null = null;

/**
 * Send a JSON-RPC request over WebSocket and return the result.
 * Uses Node.js 22 native WebSocket API.
 */
function rpcCall(
  method: string,
  params: unknown[] = [],
  timeout = 30000
): Promise<unknown> {
  return new Promise((resolve, reject) => {
    const ws = new WebSocket(CHOPSTICKS_WS);
    const timer = setTimeout(() => {
      ws.close();
      reject(new Error(`RPC call "${method}" timed out after ${timeout}ms`));
    }, timeout);

    ws.onopen = () => {
      ws.send(
        JSON.stringify({
          jsonrpc: "2.0",
          method,
          params,
          id: 1,
        })
      );
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

    ws.onerror = (event) => {
      clearTimeout(timer);
      ws.close();
      reject(new Error(`WebSocket error: ${event}`));
    };
  });
}

/**
 * Wait for Chopsticks WebSocket to become available.
 */
async function waitForChopsticks(
  maxWaitMs = 120000,
  intervalMs = 3000
): Promise<void> {
  const start = Date.now();
  while (Date.now() - start < maxWaitMs) {
    try {
      await rpcCall("system_health", [], 5000);
      return;
    } catch {
      // Not ready yet
    }
    await new Promise((r) => setTimeout(r, intervalMs));
  }
  throw new Error(
    `Chopsticks did not become available within ${maxWaitMs / 1000}s`
  );
}

describe("Fork a Parachain Guide", () => {
  afterAll(async () => {
    await stopChopsticks();
  });

  // ==================== PREREQUISITES ====================
  describe("1. Prerequisites", () => {
    it("should have Node.js installed", () => {
      const result = execSync("node --version", { encoding: "utf-8" });
      expect(result).toMatch(/v\d+\.\d+/);
      console.log(`Node.js: ${result.trim()}`);
    });

    it("should have npx available", () => {
      const result = execSync("npx --version", { encoding: "utf-8" });
      expect(result.trim().length).toBeGreaterThan(0);
      console.log(`npx: ${result.trim()}`);
    });
  });

  // ==================== VERIFY CHOPSTICKS ====================
  describe("2. Verify Chopsticks", () => {
    it("should have Chopsticks accessible via npx", () => {
      const result = execSync("npx @acala-network/chopsticks --help 2>&1", {
        encoding: "utf-8",
        cwd: PROJECT_DIR,
        timeout: 60000,
      });
      expect(result).toContain("chopsticks");
      console.log("Chopsticks CLI is accessible");
    }, 60000);
  });

  // ==================== START CHOPSTICKS ====================
  describe("3. Start Chopsticks", () => {
    it("should start Chopsticks and fork Polkadot", async () => {
      console.log("Starting Chopsticks to fork Polkadot...");

      chopsticksProcess = spawn(
        "npx",
        ["@acala-network/chopsticks", "--config=configs/chopsticks.yml"],
        {
          cwd: PROJECT_DIR,
          stdio: ["ignore", "pipe", "pipe"],
          detached: true,
        }
      );

      // Collect output for debugging
      let stdout = "";
      let stderr = "";

      chopsticksProcess.stdout?.on("data", (data) => {
        const line = data.toString();
        stdout += line;
        if (line.includes("listening")) {
          console.log(`Chopsticks: ${line.trim()}`);
        }
      });

      chopsticksProcess.stderr?.on("data", (data) => {
        stderr += data.toString();
      });

      chopsticksProcess.on("error", (err) => {
        console.error("Chopsticks process error:", err.message);
      });

      try {
        await waitForChopsticks();
        console.log("Chopsticks is ready!");
      } catch {
        console.log("stdout:", stdout);
        console.log("stderr:", stderr);
        throw new Error("Chopsticks failed to start");
      }
    }, 180000);
  });

  // ==================== VERIFY FORK ====================
  describe("4. Verify Fork", () => {
    it("should respond to system_health", async () => {
      const health = (await rpcCall("system_health")) as Record<
        string,
        unknown
      >;
      expect(health).toBeDefined();
      console.log("system_health:", JSON.stringify(health));
    });

    it("should report as Polkadot chain", async () => {
      const chain = (await rpcCall("system_chain")) as string;
      expect(chain).toContain("Polkadot");
      console.log(`Chain: ${chain}`);
    });

    it("should have a valid block header", async () => {
      const header = (await rpcCall("chain_getHeader")) as Record<
        string,
        unknown
      >;
      expect(header).toBeDefined();
      expect(header.number).toBeDefined();
      const blockNumber = parseInt(header.number as string, 16);
      expect(blockNumber).toBeGreaterThan(0);
      console.log(`Current block number: ${blockNumber}`);
    });
  });

  // ==================== DEV RPC COMMANDS ====================
  describe("5. Dev RPC Commands", () => {
    it("should create a new block with dev_newBlock", async () => {
      // Get current block number
      const headerBefore = (await rpcCall("chain_getHeader")) as Record<
        string,
        unknown
      >;
      const blockBefore = parseInt(headerBefore.number as string, 16);
      console.log(`Block before dev_newBlock: ${blockBefore}`);

      // Create a new block
      const newBlock = await rpcCall("dev_newBlock", [{ count: 1 }]);
      expect(newBlock).toBeDefined();
      console.log("dev_newBlock result:", JSON.stringify(newBlock));

      // Verify block number incremented
      const headerAfter = (await rpcCall("chain_getHeader")) as Record<
        string,
        unknown
      >;
      const blockAfter = parseInt(headerAfter.number as string, 16);
      console.log(`Block after dev_newBlock: ${blockAfter}`);

      expect(blockAfter).toBe(blockBefore + 1);
    });

    it("should modify storage with dev_setStorage", async () => {
      // Alice's account on Polkadot
      // System.Account storage key for Alice (5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY)
      const aliceStorageKey =
        "0x26aa394eea5630e07c48ae0c9558cef7b99d880ec681799c0cf30e8886371da9de1e86a9a8c739864cf3cc5ec2bea59fd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d";

      // Read current storage value
      const storageBefore = (await rpcCall("state_getStorage", [
        aliceStorageKey,
      ])) as string | null;
      console.log(
        `Storage before: ${storageBefore ? storageBefore.substring(0, 40) + "..." : "null"}`
      );

      // Set a new storage value using dev_setStorage
      // Set Alice's free balance to a known value (100 DOT = 1_000_000_000_000 planck)
      // System.Account is a Map keyed by AccountId, so values are [[key, value]]
      const setStorageParams = [
        {
          System: {
            Account: [
              [
                ["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],
                {
                  providers: 1,
                  data: {
                    free: "0x00000000000000000000e8d4a5100000",
                  },
                },
              ],
            ],
          },
        },
      ];

      const setResult = await rpcCall("dev_setStorage", setStorageParams);
      console.log("dev_setStorage result:", JSON.stringify(setResult));

      // Read storage after modification
      const storageAfter = (await rpcCall("state_getStorage", [
        aliceStorageKey,
      ])) as string | null;
      console.log(
        `Storage after: ${storageAfter ? storageAfter.substring(0, 40) + "..." : "null"}`
      );

      // Verify storage was modified
      expect(storageAfter).not.toEqual(storageBefore);
    });

    it("should time travel with dev_timeTravel", async () => {
      // Get current timestamp by querying the Timestamp pallet
      const timestampStorageKey =
        "0xf0c365c3cf59d671eb72da0e7a4113c49f1f0515f462cdcf84e0f1d6045dfcbb";
      const timestampBefore = (await rpcCall("state_getStorage", [
        timestampStorageKey,
      ])) as string | null;
      console.log(`Timestamp storage before: ${timestampBefore}`);

      // Time travel to a future date (2030-01-01T00:00:00Z = 1893456000000 ms)
      const futureTimestamp = "2030-01-01T00:00:00Z";
      const timeTravelResult = await rpcCall("dev_timeTravel", [
        futureTimestamp,
      ]);
      console.log("dev_timeTravel result:", JSON.stringify(timeTravelResult));

      // Create a new block so the timestamp takes effect
      await rpcCall("dev_newBlock", [{ count: 1 }]);

      // Read timestamp after time travel
      const timestampAfter = (await rpcCall("state_getStorage", [
        timestampStorageKey,
      ])) as string | null;
      console.log(`Timestamp storage after: ${timestampAfter}`);

      // Verify timestamp changed
      expect(timestampAfter).not.toEqual(timestampBefore);
    });
  });
});

async function stopChopsticks(): Promise<void> {
  console.log("Stopping Chopsticks...");

  if (chopsticksProcess && !chopsticksProcess.killed) {
    try {
      process.kill(-chopsticksProcess.pid!, "SIGTERM");
    } catch {
      chopsticksProcess.kill("SIGTERM");
    }
    chopsticksProcess = null;
  }

  // Kill any lingering Chopsticks processes
  try {
    execSync("pkill -f 'chopsticks' 2>/dev/null || true", {
      encoding: "utf-8",
    });
  } catch {
    // Ignore errors
  }

  await new Promise((resolve) => setTimeout(resolve, 2000));
  console.log("Chopsticks stopped");
}
