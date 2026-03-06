import { describe, it, expect, afterAll } from "vitest";
import { execSync, spawn, ChildProcess } from "child_process";
import {
  existsSync,
  mkdirSync,
  writeFileSync,
  unlinkSync,
  readFileSync,
} from "fs";
import { join } from "path";

const PROJECT_DIR = process.cwd();
const BIN_DIR = join(PROJECT_DIR, "bin");
const CONFIGS_DIR = join(PROJECT_DIR, "configs");
const PID_FILE = join(PROJECT_DIR, "zombienet.pid");

// RPC ports (matching configs/network.toml)
const RELAY_RPC_PORT = 9944;
const PARACHAIN_RPC_PORT = 9988;

// Well-known dev account addresses (SS58)
const ALICE = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY";
const BOB = "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty";

let zombienetProcess: ChildProcess | null = null;

async function rpcCall(
  port: number,
  method: string,
  params: unknown[] = [],
): Promise<any> {
  const response = await fetch(`http://127.0.0.1:${port}`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ jsonrpc: "2.0", method, params, id: 1 }),
  });
  return response.json();
}

async function stopZombienet(): Promise<void> {
  console.log("Stopping Zombienet...");

  if (zombienetProcess && !zombienetProcess.killed) {
    try {
      process.kill(-zombienetProcess.pid!, "SIGTERM");
    } catch {
      zombienetProcess.kill("SIGTERM");
    }
    zombienetProcess = null;
  }

  if (existsSync(PID_FILE)) {
    try {
      const pid = parseInt(readFileSync(PID_FILE, "utf-8"));
      process.kill(-pid, "SIGTERM");
    } catch {
      // Process might already be dead
    }
    unlinkSync(PID_FILE);
  }

  try {
    execSync("pkill -f 'polkadot.*zombie' 2>/dev/null || true", {
      encoding: "utf-8",
    });
    execSync("pkill -f 'polkadot-parachain.*zombie' 2>/dev/null || true", {
      encoding: "utf-8",
    });
  } catch {
    // Ignore
  }

  await new Promise((resolve) => setTimeout(resolve, 2000));
  console.log("Zombienet stopped");
}

describe("Network Configuration Tests", () => {
  afterAll(async () => {
    await stopZombienet();
  });

  // ==================== VALIDATE CONFIGS ====================
  describe("1. Validate Configuration", () => {
    it("should have zombienet network.toml", () => {
      const configPath = join(CONFIGS_DIR, "network.toml");
      expect(existsSync(configPath)).toBe(true);

      const config = readFileSync(configPath, "utf-8");
      expect(config).toContain("[relaychain]");
      expect(config).toContain('chain_spec_path = "configs/paseo-local.json"');
      expect(config).toContain('name = "alice"');
      expect(config).toContain('name = "bob"');
      expect(config).toContain("[[parachains]]");
      expect(config).toContain(
        'chain_spec_path = "configs/asset-hub-paseo-local.json"',
      );
      expect(config).toContain("polkadot-parachain");
      console.log("Zombienet configuration is valid");
    });

    it("should have chopsticks configuration", () => {
      const configPath = join(CONFIGS_DIR, "chopsticks.yml");
      expect(existsSync(configPath)).toBe(true);

      const config = readFileSync(configPath, "utf-8");
      expect(config).toContain("relaychain:");
      expect(config).toContain("parachains:");
      console.log("Chopsticks configuration is valid");
    });
  });

  // ==================== CHECK PREREQUISITES ====================
  describe("2. Prerequisites", () => {
    it("should have Zombienet installed", () => {
      const result = execSync(
        "zombienet version 2>&1 || zombienet --version 2>&1",
        { encoding: "utf-8" },
      );
      expect(result.length).toBeGreaterThan(0);
      console.log(`Zombienet: ${result.trim()}`);
    });

    it("should have required binaries", () => {
      const binaries = [
        "polkadot",
        "polkadot-prepare-worker",
        "polkadot-execute-worker",
        "polkadot-parachain",
      ];
      for (const binary of binaries) {
        const binPath = join(BIN_DIR, binary);
        expect(existsSync(binPath)).toBe(true);
        const version = execSync(`${binPath} --version 2>&1`, {
          encoding: "utf-8",
        });
        console.log(`${binary}: ${version.trim()}`);
      }
    });

    it("should have generated chain specs", () => {
      const paseoSpec = join(CONFIGS_DIR, "paseo-local.json");
      const assetHubSpec = join(CONFIGS_DIR, "asset-hub-paseo-local.json");

      expect(existsSync(paseoSpec)).toBe(true);
      expect(existsSync(assetHubSpec)).toBe(true);

      const paseo = JSON.parse(readFileSync(paseoSpec, "utf-8"));
      console.log(`Paseo chain spec: ${paseo.name} (${paseo.id})`);

      const assetHub = JSON.parse(readFileSync(assetHubSpec, "utf-8"));
      expect(assetHub.relay_chain).toBeDefined();
      expect(assetHub.para_id).toBe(1000);
      console.log(
        `Asset Hub chain spec: ${assetHub.name} (paraId: ${assetHub.para_id})`,
      );
    });
  });

  // ==================== SPAWN & VERIFY NETWORK ====================
  describe("3. Spawn Network", () => {
    it("should spawn the network with zombienet", async () => {
      console.log("Spawning network with Zombienet...");

      const configPath = join(CONFIGS_DIR, "network.toml");

      zombienetProcess = spawn(
        "zombienet",
        ["spawn", configPath, "--provider", "native"],
        {
          cwd: PROJECT_DIR,
          stdio: ["ignore", "pipe", "pipe"],
          detached: true,
        },
      );

      if (zombienetProcess.pid) {
        writeFileSync(PID_FILE, zombienetProcess.pid.toString());
        console.log(`Zombienet started with PID: ${zombienetProcess.pid}`);
      }

      let stdout = "";
      let stderr = "";

      zombienetProcess.stdout?.on("data", (data) => {
        stdout += data.toString();
        if (data.toString().includes("Network launched")) {
          console.log("Network launched successfully!");
        }
      });

      zombienetProcess.stderr?.on("data", (data) => {
        stderr += data.toString();
      });

      // Wait for relay chain RPC to be available
      const maxWaitTime = 120000;
      const startTime = Date.now();

      while (Date.now() - startTime < maxWaitTime) {
        try {
          const result = await rpcCall(RELAY_RPC_PORT, "system_health");
          if (result.result) {
            console.log("Relay chain RPC is ready!");
            return;
          }
        } catch {
          // Node not ready yet
        }
        await new Promise((resolve) => setTimeout(resolve, 2000));
      }

      console.log("stdout:", stdout.slice(-2000));
      console.log("stderr:", stderr.slice(-2000));
      throw new Error("Network failed to start within 2 minutes");
    }, 180000);

    it("should verify relay chain is producing blocks", async () => {
      console.log("Verifying relay chain block production...");

      let blockNumber = 0;
      const maxAttempts = 5;

      for (let attempt = 1; attempt <= maxAttempts; attempt++) {
        await new Promise((resolve) => setTimeout(resolve, 12000));

        const result = await rpcCall(RELAY_RPC_PORT, "chain_getHeader");
        expect(result.result).toBeDefined();
        expect(result.result.number).toBeDefined();

        blockNumber = parseInt(result.result.number, 16);
        console.log(
          `Relay chain block number (attempt ${attempt}): ${blockNumber}`,
        );

        if (blockNumber > 0) break;
      }

      expect(blockNumber).toBeGreaterThan(0);
      console.log(`Relay chain producing blocks (current: #${blockNumber})`);
    }, 90000);

    it("should connect to parachain", async () => {
      console.log("Verifying parachain connectivity...");

      let blockNumber = -1;
      const maxAttempts = 10;

      for (let attempt = 1; attempt <= maxAttempts; attempt++) {
        await new Promise((resolve) => setTimeout(resolve, 15000));
        try {
          const result = await rpcCall(PARACHAIN_RPC_PORT, "chain_getHeader");
          if (result.result?.number !== undefined) {
            blockNumber = parseInt(result.result.number, 16);
            console.log(
              `Parachain block number (attempt ${attempt}): ${blockNumber}`,
            );
            if (blockNumber >= 0) break;
          }
        } catch {
          console.log(`Parachain not ready yet (attempt ${attempt})`);
        }
      }

      expect(blockNumber).toBeGreaterThanOrEqual(0);
      console.log(`Parachain connected (current block: #${blockNumber})`);
    }, 180000);
  });

  // ==================== VERIFY DEV ACCOUNTS ====================
  describe("4. Verify Dev Accounts", () => {
    it("should have Alice account on relay chain", async () => {
      const result = await rpcCall(RELAY_RPC_PORT, "system_accountNextIndex", [
        ALICE,
      ]);
      expect(result.result).toBeDefined();
      console.log(`Alice (relay) nonce: ${result.result}`);

      // Verify accounts exist in storage
      const systemAccountPrefix =
        "0x26aa394eea5630e07c48ae0c9558cef7b99d880ec681799c0cf30e8886371da9";
      const keysResult = await rpcCall(RELAY_RPC_PORT, "state_getKeysPaged", [
        systemAccountPrefix,
        10,
        null,
      ]);
      expect(keysResult.result.length).toBeGreaterThan(0);
      console.log(
        `Relay chain has ${keysResult.result.length} accounts in storage`,
      );
    }, 10000);

    it("should have Bob account on relay chain", async () => {
      const result = await rpcCall(RELAY_RPC_PORT, "system_accountNextIndex", [
        BOB,
      ]);
      expect(result.result).toBeDefined();
      console.log(`Bob (relay) nonce: ${result.result}`);
    }, 10000);

    it("should have Alice account on Asset Hub", async () => {
      const result = await rpcCall(
        PARACHAIN_RPC_PORT,
        "system_accountNextIndex",
        [ALICE],
      );
      expect(result.result).toBeDefined();
      console.log(`Alice (Asset Hub) nonce: ${result.result}`);
    }, 10000);

    it("should have Bob account on Asset Hub", async () => {
      const result = await rpcCall(
        PARACHAIN_RPC_PORT,
        "system_accountNextIndex",
        [BOB],
      );
      expect(result.result).toBeDefined();
      console.log(`Bob (Asset Hub) nonce: ${result.result}`);
    }, 10000);

    it("should respond to RPC queries on both chains", async () => {
      const relayName = await rpcCall(RELAY_RPC_PORT, "system_name");
      expect(relayName.result).toBeDefined();
      console.log(`Relay chain: ${relayName.result}`);

      const parachainName = await rpcCall(PARACHAIN_RPC_PORT, "system_name");
      expect(parachainName.result).toBeDefined();
      console.log(`Parachain: ${parachainName.result}`);
    }, 10000);
  });
});
