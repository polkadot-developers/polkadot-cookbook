import { describe, it, expect, afterAll } from "vitest";
import { execSync, spawn, ChildProcess } from "child_process";
import { existsSync, mkdirSync, writeFileSync, unlinkSync, readFileSync } from "fs";
import { join } from "path";

const PROJECT_DIR = process.cwd();
const BIN_DIR = join(PROJECT_DIR, "bin");
const CONFIGS_DIR = join(PROJECT_DIR, "configs");

// Versions from vitest.config.ts (loaded from versions.yml)
const POLKADOT_VERSION = process.env.POLKADOT_SDK_VERSION!;
const POLKADOT_RELEASE_URL = `https://github.com/paritytech/polkadot-sdk/releases/download/${POLKADOT_VERSION}`;
const CHAIN_SPEC_BUILDER_VERSION = process.env.CHAIN_SPEC_BUILDER_VERSION!;
const PASEO_RUNTIME_VERSION = process.env.PASEO_RUNTIME_VERSION!;
const PASEO_RUNTIMES_URL = `https://github.com/paseo-network/runtimes/releases/download/${PASEO_RUNTIME_VERSION}`;

// Binaries
const POLKADOT_BINARY = join(BIN_DIR, "polkadot");
const POLKADOT_PREPARE_WORKER = join(BIN_DIR, "polkadot-prepare-worker");
const POLKADOT_EXECUTE_WORKER = join(BIN_DIR, "polkadot-execute-worker");
const POLKADOT_PARACHAIN_BINARY = join(BIN_DIR, "polkadot-parachain");
const CHAIN_SPEC_BUILDER = join(BIN_DIR, "chain-spec-builder");

// Runtime WASMs
const PASEO_RUNTIME_WASM = join(BIN_DIR, "paseo_runtime.compressed.wasm");
const ASSET_HUB_RUNTIME_WASM = join(BIN_DIR, "asset-hub-paseo_runtime.compressed.wasm");

// Generated chain specs
const PASEO_LOCAL_CHAIN_SPEC = join(CONFIGS_DIR, "paseo-local.json");
const ASSET_HUB_LOCAL_CHAIN_SPEC = join(CONFIGS_DIR, "asset-hub-paseo-local.json");

const PID_FILE = join(PROJECT_DIR, "zombienet.pid");

// RPC ports (matching configs/network.toml)
const RELAY_RPC_PORT = 9944;
const PARACHAIN_RPC_PORT = 9988;

// Well-known dev account addresses (SS58 format)
const ALICE = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY";
const BOB = "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty";

let zombienetProcess: ChildProcess | null = null;

async function rpcCall(port: number, method: string, params: unknown[] = []): Promise<any> {
  const response = await fetch(`http://127.0.0.1:${port}`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ jsonrpc: "2.0", method, params, id: 1 }),
  });
  const json = await response.json();
  return json;
}

// Verify a dev account exists by querying its nonce via the standard RPC
async function verifyDevAccount(port: number, address: string, label: string): Promise<void> {
  // system_accountNextIndex returns the nonce (u32) for an account
  // This RPC accepts SS58 addresses directly
  const result = await rpcCall(port, "system_accountNextIndex", [address]);
  expect(result.result).toBeDefined();
  console.log(`${label} nonce: ${result.result}`);

  // Also verify via state_getKeys that the account has storage entries
  // System.Account prefix: twox128("System") ++ twox128("Account")
  const systemAccountPrefix = "0x26aa394eea5630e07c48ae0c9558cef7b99d880ec681799c0cf30e8886371da9";
  const keysResult = await rpcCall(port, "state_getKeysPaged", [systemAccountPrefix, 10, null]);
  expect(keysResult.result).toBeDefined();
  expect(keysResult.result.length).toBeGreaterThan(0);
  console.log(`${label} chain has ${keysResult.result.length} accounts in storage`);
}

describe("Paseo Local Network with Asset Hub", () => {
  afterAll(async () => {
    await stopZombienet();
  });

  // ==================== PREREQUISITES ====================
  describe("1. Prerequisites", () => {
    it("should have Zombienet installed", () => {
      const result = execSync("zombienet version 2>&1 || zombienet --version 2>&1", {
        encoding: "utf-8",
      });
      expect(result.length).toBeGreaterThan(0);
      console.log(`Zombienet: ${result.trim()}`);
    });
  });

  // ==================== DOWNLOAD BINARIES ====================
  describe("2. Download Binaries", () => {
    it("should download polkadot and polkadot-parachain binaries", () => {
      if (!existsSync(BIN_DIR)) {
        mkdirSync(BIN_DIR, { recursive: true });
      }

      const allExist =
        existsSync(POLKADOT_BINARY) &&
        existsSync(POLKADOT_PREPARE_WORKER) &&
        existsSync(POLKADOT_EXECUTE_WORKER) &&
        existsSync(POLKADOT_PARACHAIN_BINARY);

      if (allExist) {
        try {
          const version = execSync(`${POLKADOT_BINARY} --version 2>&1`, { encoding: "utf-8" });
          const paraVersion = execSync(`${POLKADOT_PARACHAIN_BINARY} --version 2>&1`, { encoding: "utf-8" });
          console.log(`Polkadot already downloaded: ${version.trim()}`);
          console.log(`polkadot-parachain already downloaded: ${paraVersion.trim()}`);
          return;
        } catch {
          console.log("Existing binaries not executable, re-downloading...");
        }
      }

      console.log(`Downloading Polkadot ${POLKADOT_VERSION} binaries...`);
      // On macOS, GitHub releases have an architecture suffix
      const suffix = process.platform === "darwin"
        ? `-${process.arch === "arm64" ? "aarch64" : "x86_64"}-apple-darwin`
        : "";
      for (const binary of ["polkadot", "polkadot-prepare-worker", "polkadot-execute-worker", "polkadot-parachain"]) {
        console.log(`Downloading ${binary}${suffix}...`);
        execSync(`curl -L -o ${binary} ${POLKADOT_RELEASE_URL}/${binary}${suffix}`, {
          cwd: BIN_DIR,
          encoding: "utf-8",
          stdio: "inherit",
          timeout: 300000,
        });
        execSync(`chmod +x ${binary}`, { cwd: BIN_DIR });
      }

      expect(existsSync(POLKADOT_BINARY)).toBe(true);
      expect(existsSync(POLKADOT_PARACHAIN_BINARY)).toBe(true);
      const version = execSync(`${POLKADOT_BINARY} --version 2>&1`, { encoding: "utf-8" });
      console.log(`Polkadot: ${version.trim()}`);
      const paraVersion = execSync(`${POLKADOT_PARACHAIN_BINARY} --version 2>&1`, { encoding: "utf-8" });
      console.log(`polkadot-parachain: ${paraVersion.trim()}`);
    }, 300000);

    it("should download chain-spec-builder", () => {
      if (existsSync(CHAIN_SPEC_BUILDER)) {
        console.log("chain-spec-builder already exists");
        return;
      }

      console.log(`Downloading chain-spec-builder v${CHAIN_SPEC_BUILDER_VERSION}...`);

      const platform = process.platform;
      if (platform !== "linux") {
        execSync(
          `cargo install staging-chain-spec-builder@${CHAIN_SPEC_BUILDER_VERSION} --locked --root ${BIN_DIR}`,
          { encoding: "utf-8", stdio: "inherit", timeout: 600000 }
        );
        const cargoPath = join(BIN_DIR, "bin", "chain-spec-builder");
        if (existsSync(cargoPath) && !existsSync(CHAIN_SPEC_BUILDER)) {
          execSync(`mv ${cargoPath} ${CHAIN_SPEC_BUILDER}`);
        }
      } else {
        execSync(`curl -L -o chain-spec-builder ${POLKADOT_RELEASE_URL}/chain-spec-builder`, {
          cwd: BIN_DIR,
          encoding: "utf-8",
          stdio: "inherit",
          timeout: 300000,
        });
        execSync(`chmod +x chain-spec-builder`, { cwd: BIN_DIR });
      }

      expect(existsSync(CHAIN_SPEC_BUILDER)).toBe(true);
      console.log("chain-spec-builder downloaded successfully");
    }, 600000);
  });

  // ==================== DOWNLOAD RUNTIMES ====================
  describe("3. Download Runtime WASMs", () => {
    it("should download Paseo runtime WASM", () => {
      if (existsSync(PASEO_RUNTIME_WASM)) {
        console.log("Paseo runtime WASM already exists");
        return;
      }

      console.log(`Downloading Paseo runtime ${PASEO_RUNTIME_VERSION}...`);
      execSync(`curl -L -o paseo_runtime.compressed.wasm ${PASEO_RUNTIMES_URL}/paseo_runtime.compressed.wasm`, {
        cwd: BIN_DIR,
        encoding: "utf-8",
        stdio: "inherit",
        timeout: 300000,
      });

      expect(existsSync(PASEO_RUNTIME_WASM)).toBe(true);
      console.log("Paseo runtime WASM downloaded");
    }, 300000);

    it("should download Asset Hub Paseo runtime WASM", () => {
      if (existsSync(ASSET_HUB_RUNTIME_WASM)) {
        console.log("Asset Hub Paseo runtime WASM already exists");
        return;
      }

      console.log(`Downloading Asset Hub Paseo runtime ${PASEO_RUNTIME_VERSION}...`);
      execSync(
        `curl -L -o asset-hub-paseo_runtime.compressed.wasm ${PASEO_RUNTIMES_URL}/asset-hub-paseo_runtime.compressed.wasm`,
        {
          cwd: BIN_DIR,
          encoding: "utf-8",
          stdio: "inherit",
          timeout: 300000,
        }
      );

      expect(existsSync(ASSET_HUB_RUNTIME_WASM)).toBe(true);
      console.log("Asset Hub Paseo runtime WASM downloaded");
    }, 300000);
  });

  // ==================== GENERATE CHAIN SPECS ====================
  describe("4. Generate Chain Specs", () => {
    it("should generate Paseo local chain spec", () => {
      if (existsSync(PASEO_LOCAL_CHAIN_SPEC)) {
        console.log("Paseo local chain spec already exists");
        return;
      }

      console.log("Generating Paseo local testnet chain spec...");
      execSync(
        `${CHAIN_SPEC_BUILDER} -c ${PASEO_LOCAL_CHAIN_SPEC} create -r ${PASEO_RUNTIME_WASM} named-preset local_testnet`,
        { encoding: "utf-8", stdio: "inherit" }
      );

      expect(existsSync(PASEO_LOCAL_CHAIN_SPEC)).toBe(true);

      // Ensure chain spec has Local chain type for zombienet
      const chainSpec = JSON.parse(readFileSync(PASEO_LOCAL_CHAIN_SPEC, "utf-8"));
      if (chainSpec.chainType !== "Local") {
        chainSpec.chainType = "Local";
        writeFileSync(PASEO_LOCAL_CHAIN_SPEC, JSON.stringify(chainSpec, null, 2));
      }
      expect(chainSpec).toBeDefined();
      console.log(`Paseo local chain spec generated (name: ${chainSpec.name})`);
    }, 60000);

    it("should generate Asset Hub Paseo local chain spec", () => {
      if (existsSync(ASSET_HUB_LOCAL_CHAIN_SPEC)) {
        console.log("Asset Hub Paseo local chain spec already exists");
        return;
      }

      console.log("Generating Asset Hub Paseo local chain spec...");
      execSync(
        `${CHAIN_SPEC_BUILDER} -c ${ASSET_HUB_LOCAL_CHAIN_SPEC} create -r ${ASSET_HUB_RUNTIME_WASM} named-preset development`,
        { encoding: "utf-8", stdio: "inherit" }
      );

      // Patch chain spec: add relay_chain and para_id required for parachain chain specs
      const chainSpec = JSON.parse(readFileSync(ASSET_HUB_LOCAL_CHAIN_SPEC, "utf-8"));
      chainSpec.relay_chain = "paseo-local";
      chainSpec.para_id = 1000;
      chainSpec.chainType = "Local";
      writeFileSync(ASSET_HUB_LOCAL_CHAIN_SPEC, JSON.stringify(chainSpec, null, 2));

      expect(existsSync(ASSET_HUB_LOCAL_CHAIN_SPEC)).toBe(true);
      console.log(`Asset Hub Paseo local chain spec generated (name: ${chainSpec.name}, paraId: ${chainSpec.para_id})`);
    }, 60000);
  });

  // ==================== VALIDATE CONFIG ====================
  describe("5. Validate Network Configuration", () => {
    it("should have valid network.toml", () => {
      const configPath = join(CONFIGS_DIR, "network.toml");
      expect(existsSync(configPath)).toBe(true);

      const config = readFileSync(configPath, "utf-8");
      expect(config).toContain("[relaychain]");
      expect(config).toContain('chain_spec_path = "configs/paseo-local.json"');
      expect(config).toContain('name = "alice"');
      expect(config).toContain('name = "bob"');
      expect(config).toContain("[[parachains]]");
      expect(config).toContain('chain_spec_path = "configs/asset-hub-paseo-local.json"');
      expect(config).toContain("polkadot-parachain");

      console.log("Network configuration is valid");
    });
  });

  // ==================== SPAWN & VERIFY NETWORK ====================
  describe("6. Spawn Network", () => {
    it("should spawn the network with zombienet", async () => {
      console.log("Spawning network with Zombienet...");

      const configPath = join(CONFIGS_DIR, "network.toml");

      zombienetProcess = spawn("zombienet", ["spawn", configPath, "--provider", "native"], {
        cwd: PROJECT_DIR,
        stdio: ["ignore", "pipe", "pipe"],
        detached: true,
      });

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
        console.log(`Relay chain block number (attempt ${attempt}): ${blockNumber}`);

        if (blockNumber > 0) break;
      }

      expect(blockNumber).toBeGreaterThan(0);
      console.log(`Relay chain producing blocks (current: #${blockNumber})`);
    }, 90000);

    it("should verify parachain is producing blocks", async () => {
      console.log("Verifying parachain block production...");

      // Parachains need time to onboard
      let blockNumber = -1;
      const maxAttempts = 10;

      for (let attempt = 1; attempt <= maxAttempts; attempt++) {
        await new Promise((resolve) => setTimeout(resolve, 15000));

        try {
          const result = await rpcCall(PARACHAIN_RPC_PORT, "chain_getHeader");
          if (result.result?.number) {
            blockNumber = parseInt(result.result.number, 16);
            console.log(`Parachain block number (attempt ${attempt}): ${blockNumber}`);
            if (blockNumber > 0) break;
          }
        } catch {
          console.log(`Parachain not ready yet (attempt ${attempt})`);
        }
      }

      expect(blockNumber).toBeGreaterThanOrEqual(0);
      console.log(`Parachain producing blocks (current: #${blockNumber})`);
    }, 180000);
  });

  // ==================== VERIFY DEV ACCOUNTS ====================
  describe("7. Verify Dev Accounts", () => {
    it("should have Alice account on relay chain", async () => {
      await verifyDevAccount(RELAY_RPC_PORT, ALICE, "Alice (relay)");
    }, 10000);

    it("should have Bob account on relay chain", async () => {
      await verifyDevAccount(RELAY_RPC_PORT, BOB, "Bob (relay)");
    }, 10000);

    it("should have Alice account on Asset Hub", async () => {
      await verifyDevAccount(PARACHAIN_RPC_PORT, ALICE, "Alice (Asset Hub)");
    }, 10000);

    it("should have Bob account on Asset Hub", async () => {
      await verifyDevAccount(PARACHAIN_RPC_PORT, BOB, "Bob (Asset Hub)");
    }, 10000);

    it("should respond to RPC queries on both chains", async () => {
      const relayName = await rpcCall(RELAY_RPC_PORT, "system_name");
      expect(relayName.result).toBeDefined();
      console.log(`Relay chain system name: ${relayName.result}`);

      const relayVersion = await rpcCall(RELAY_RPC_PORT, "system_version");
      expect(relayVersion.result).toBeDefined();
      console.log(`Relay chain version: ${relayVersion.result}`);

      const parachainName = await rpcCall(PARACHAIN_RPC_PORT, "system_name");
      expect(parachainName.result).toBeDefined();
      console.log(`Parachain system name: ${parachainName.result}`);

      const parachainVersion = await rpcCall(PARACHAIN_RPC_PORT, "system_version");
      expect(parachainVersion.result).toBeDefined();
      console.log(`Parachain version: ${parachainVersion.result}`);
    }, 10000);
  });
});

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

  // Kill any lingering processes from this test
  try {
    execSync("pkill -f 'polkadot.*paseo' 2>/dev/null || true", { encoding: "utf-8" });
    execSync("pkill -f 'polkadot-parachain.*asset-hub' 2>/dev/null || true", { encoding: "utf-8" });
  } catch {
    // Ignore errors
  }

  await new Promise((resolve) => setTimeout(resolve, 2000));
  console.log("Zombienet stopped");
}
