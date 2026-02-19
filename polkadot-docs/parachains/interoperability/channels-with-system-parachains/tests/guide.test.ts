import { describe, it, expect, afterAll } from "vitest";
import { execSync, spawn, ChildProcess } from "child_process";
import {
  existsSync,
  readFileSync,
  writeFileSync,
  unlinkSync,
  mkdirSync,
  statSync,
} from "fs";
import { join } from "path";
import { ApiPromise, WsProvider } from "@polkadot/api";
import { Keyring } from "@polkadot/keyring";

const PROJECT_DIR = process.cwd();
const WORKSPACE_DIR = join(PROJECT_DIR, ".test-workspace");
const TEMPLATE_DIR = join(WORKSPACE_DIR, "parachain-template");
const TEMPLATE_VERSION = process.env.TEMPLATE_VERSION!;
const POLKADOT_SDK_VERSION = process.env.POLKADOT_SDK_VERSION!;
const BIN_DIR = join(WORKSPACE_DIR, "bin");
const CHAIN_SPEC = join(WORKSPACE_DIR, "chain_spec.json");
const PID_FILE = join(WORKSPACE_DIR, "zombienet.pid");
const WASM_PATH = join(
  TEMPLATE_DIR,
  "target/release/wbuild/parachain-template-runtime/parachain_template_runtime.compact.compressed.wasm"
);
const POLKADOT_BINARY = join(BIN_DIR, "polkadot");
const POLKADOT_PREPARE_WORKER = join(BIN_DIR, "polkadot-prepare-worker");
const POLKADOT_EXECUTE_WORKER = join(BIN_DIR, "polkadot-execute-worker");
const POLKADOT_PARACHAIN_BINARY = join(BIN_DIR, "polkadot-parachain");

// RPC ports
const RELAY_RPC_PORT = 9944;
const ASSET_HUB_RPC_PORT = 9988;
const CUSTOM_PARA_RPC_PORT = 9989;

const RELAY_RPC_URL = `http://127.0.0.1:${RELAY_RPC_PORT}`;
const RELAY_WS_URL = `ws://127.0.0.1:${RELAY_RPC_PORT}`;
const ASSET_HUB_RPC_URL = `http://127.0.0.1:${ASSET_HUB_RPC_PORT}`;
const CUSTOM_PARA_RPC_URL = `http://127.0.0.1:${CUSTOM_PARA_RPC_PORT}`;

let zombienetProcess: ChildProcess | null = null;

/** Helper: JSON-RPC call to a given node */
async function rpcCall(url: string, method: string, params: unknown[] = []): Promise<unknown> {
  const response = await fetch(url, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ jsonrpc: "2.0", method, params, id: 1 }),
  });
  const json = (await response.json()) as { result?: unknown; error?: unknown };
  if (json.error) throw new Error(`RPC error: ${JSON.stringify(json.error)}`);
  return json.result;
}

describe("Channels with System Parachains Tutorial", () => {
  afterAll(async () => {
    await stopZombienet();
  });

  // ==================== ENVIRONMENT TESTS ====================
  describe("1. Environment Prerequisites", () => {
    it("should have Rust installed", () => {
      const result = execSync("rustc --version", { encoding: "utf-8" });
      expect(result).toMatch(/rustc \d+\.\d+/);
      console.log(`Rust: ${result.trim()}`);
    });

    it("should have cargo installed", () => {
      const result = execSync("cargo --version", { encoding: "utf-8" });
      expect(result).toMatch(/cargo \d+\.\d+/);
      console.log(`Cargo: ${result.trim()}`);
    });

    it("should have wasm32-unknown-unknown target", () => {
      const targets = execSync("rustup target list --installed", {
        encoding: "utf-8",
      });
      expect(targets).toContain("wasm32-unknown-unknown");
      console.log("wasm32-unknown-unknown target: installed");
    });

    it("should have chain-spec-builder installed", () => {
      try {
        const result = execSync("chain-spec-builder --version 2>&1", {
          encoding: "utf-8",
        });
        expect(result.length).toBeGreaterThan(0);
        console.log(`chain-spec-builder: ${result.trim()}`);
      } catch {
        console.log("Installing chain-spec-builder...");
        execSync(`cargo install staging-chain-spec-builder@${process.env.CHAIN_SPEC_BUILDER_VERSION} --locked`, {
          stdio: "inherit",
        });
      }
    });

    it("should have polkadot-omni-node installed", () => {
      try {
        const result = execSync("polkadot-omni-node --version 2>&1", {
          encoding: "utf-8",
        });
        expect(result.length).toBeGreaterThan(0);
        console.log(`polkadot-omni-node: ${result.trim()}`);
      } catch {
        console.log("Installing polkadot-omni-node...");
        execSync(`cargo install polkadot-omni-node@${process.env.POLKADOT_OMNI_NODE_VERSION} --locked`, {
          stdio: "inherit",
        });
      }
    });

    it("should have Zombienet installed", () => {
      const result = execSync("zombienet version 2>&1 || zombienet --version 2>&1", {
        encoding: "utf-8",
      });
      expect(result.length).toBeGreaterThan(0);
      console.log(`Zombienet: ${result.trim()}`);
    });

    it("should have git installed", () => {
      const result = execSync("git --version", { encoding: "utf-8" });
      expect(result).toMatch(/git version/);
      console.log(`Git: ${result.trim()}`);
    });
  });

  // ==================== BUILD RUNTIME ====================
  describe("2. Build Runtime", () => {
    it("should create workspace directory", () => {
      if (!existsSync(WORKSPACE_DIR)) {
        mkdirSync(WORKSPACE_DIR, { recursive: true });
      }
      expect(existsSync(WORKSPACE_DIR)).toBe(true);
    });

    it("should clone the parachain template repository", () => {
      if (existsSync(TEMPLATE_DIR)) {
        console.log(`Template already cloned, resetting to ${TEMPLATE_VERSION}...`);
        execSync(`git checkout -- . && git clean -fd -e target/ && git fetch --tags && git checkout ${TEMPLATE_VERSION}`, { cwd: TEMPLATE_DIR, encoding: "utf-8" });
      } else {
        console.log(`Cloning polkadot-sdk-parachain-template ${TEMPLATE_VERSION}...`);
        execSync(
          `git clone --branch ${TEMPLATE_VERSION} https://github.com/paritytech/polkadot-sdk-parachain-template.git ${TEMPLATE_DIR}`,
          { encoding: "utf-8", stdio: "inherit" }
        );
      }

      expect(existsSync(join(TEMPLATE_DIR, "Cargo.toml"))).toBe(true);
      console.log("Repository cloned successfully");
    }, 120000);

    it("should build the runtime", () => {
      console.log("Building parachain template runtime (this may take 15-30 minutes)...");
      execSync("cargo build --release", {
        cwd: TEMPLATE_DIR,
        encoding: "utf-8",
        stdio: "inherit",
        timeout: 1800000,
      });
      expect(existsSync(WASM_PATH)).toBe(true);
      const stats = statSync(WASM_PATH);
      const sizeKB = Math.round(stats.size / 1024);
      console.log(`WASM runtime size: ${sizeKB} KB`);
      expect(stats.size).toBeGreaterThan(100000);
    }, 1800000);
  });

  // ==================== GENERATE CHAIN SPEC ====================
  describe("3. Generate Chain Spec", () => {
    it("should generate chain spec for custom parachain (para ID 2000)", () => {
      console.log("Generating chain spec for parachain 2000...");
      execSync(
        `chain-spec-builder create -t development \
          --relay-chain rococo-local \
          --para-id 2000 \
          --runtime ${WASM_PATH} \
          named-preset development`,
        { encoding: "utf-8", cwd: WORKSPACE_DIR }
      );
      expect(existsSync(CHAIN_SPEC)).toBe(true);
      console.log("chain_spec.json generated for parachain 2000");
    }, 60000);
  });

  // ==================== DOWNLOAD BINARIES ====================
  describe("4. Download Binaries", () => {
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

      console.log(`Downloading Polkadot ${POLKADOT_SDK_VERSION} binaries...`);
      const baseUrl = `https://github.com/paritytech/polkadot-sdk/releases/download/${POLKADOT_SDK_VERSION}`;
      // On macOS, binaries have an architecture suffix on GitHub releases
      const suffix = process.platform === "darwin"
        ? `-${process.arch === "arm64" ? "aarch64" : "x86_64"}-apple-darwin`
        : "";
      for (const binary of ["polkadot", "polkadot-prepare-worker", "polkadot-execute-worker", "polkadot-parachain"]) {
        console.log(`Downloading ${binary}${suffix}...`);
        execSync(`curl -L -o ${binary} ${baseUrl}/${binary}${suffix}`, {
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
  });

  // ==================== SPAWN NETWORK ====================
  describe("5. Spawn Network", () => {
    it("should spawn the network with Zombienet", async () => {
      console.log("Spawning network with Zombienet (2 relay + Asset Hub + custom parachain)...");

      const configPath = join(PROJECT_DIR, "configs", "network.toml");

      zombienetProcess = spawn(
        "zombienet",
        ["spawn", configPath, "--provider", "native"],
        {
          cwd: WORKSPACE_DIR,
          stdio: ["ignore", "pipe", "pipe"],
          detached: true,
        }
      );

      if (zombienetProcess.pid) {
        writeFileSync(PID_FILE, zombienetProcess.pid.toString());
        console.log(`Zombienet started with PID: ${zombienetProcess.pid}`);
      }

      zombienetProcess.stdout?.on("data", (data: Buffer) => {
        const msg = data.toString();
        if (msg.includes("Network launched")) {
          console.log("Network launched successfully!");
        }
      });

      // Wait for relay chain RPC to be available
      const maxWaitTime = 120000;
      const startTime = Date.now();

      while (Date.now() - startTime < maxWaitTime) {
        try {
          const response = await fetch(RELAY_RPC_URL, {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({ jsonrpc: "2.0", method: "system_health", params: [], id: 1 }),
          });
          if (response.ok) {
            console.log("Relay chain RPC is ready!");
            return;
          }
        } catch {
          // Node not ready yet
        }
        await new Promise((resolve) => setTimeout(resolve, 2000));
      }
      throw new Error("Relay chain failed to start within 2 minutes");
    }, 180000);

    it("should wait for Asset Hub to produce blocks", async () => {
      console.log("Waiting for Asset Hub (1000) to produce blocks...");

      let blockNumber = 0;
      for (let attempt = 1; attempt <= 15; attempt++) {
        await new Promise((resolve) => setTimeout(resolve, 12000));
        try {
          const result = (await rpcCall(ASSET_HUB_RPC_URL, "chain_getHeader")) as { number: string };
          blockNumber = parseInt(result.number, 16);
          console.log(`Asset Hub block number (attempt ${attempt}): ${blockNumber}`);
          if (blockNumber > 0) break;
        } catch {
          console.log(`Attempt ${attempt}: Asset Hub not ready yet`);
        }
      }
      expect(blockNumber).toBeGreaterThan(0);
    }, 240000);

    it("should wait for custom parachain to produce blocks", async () => {
      console.log("Waiting for custom parachain (2000) to produce blocks...");

      let blockNumber = 0;
      for (let attempt = 1; attempt <= 15; attempt++) {
        await new Promise((resolve) => setTimeout(resolve, 12000));
        try {
          const result = (await rpcCall(CUSTOM_PARA_RPC_URL, "chain_getHeader")) as { number: string };
          blockNumber = parseInt(result.number, 16);
          console.log(`Custom parachain block number (attempt ${attempt}): ${blockNumber}`);
          if (blockNumber > 0) break;
        } catch {
          console.log(`Attempt ${attempt}: custom parachain not ready yet`);
        }
      }
      expect(blockNumber).toBeGreaterThan(0);
    }, 240000);
  });

  // ==================== OPEN BIDIRECTIONAL HRMP CHANNELS ====================
  describe("6. Open Bidirectional HRMP Channels (1000 <-> 2000)", () => {
    it("should force open HRMP channels in both directions via sudo on relay chain", async () => {
      console.log("Opening bidirectional HRMP channels 1000 <-> 2000 via sudo on relay chain...");

      const wsProvider = new WsProvider(RELAY_WS_URL);
      const api = await ApiPromise.create({ provider: wsProvider });

      const keyring = new Keyring({ type: "sr25519" });
      const alice = keyring.addFromUri("//Alice");

      // Channel 1: Asset Hub (1000) -> custom parachain (2000)
      console.log("Opening channel 1000 -> 2000...");
      const forceOpen1 = api.tx.hrmp.forceOpenHrmpChannel(1000, 2000, 8, 1048576);
      const sudoTx1 = api.tx.sudo.sudo(forceOpen1);

      await new Promise<void>((resolve, reject) => {
        sudoTx1.signAndSend(alice, ({ status, dispatchError, events }) => {
          if (dispatchError) {
            if (dispatchError.isModule) {
              const decoded = api.registry.findMetaError(dispatchError.asModule);
              reject(new Error(`${decoded.section}.${decoded.name}: ${decoded.docs.join(" ")}`));
            } else {
              reject(new Error(dispatchError.toString()));
            }
          }
          if (status.isInBlock) {
            const sudoEvent = events.find(({ event }) =>
              api.events.sudo.Sudid.is(event)
            );
            if (sudoEvent) {
              const [result] = sudoEvent.event.data as any;
              if (result.isErr) {
                reject(new Error(`Sudo call failed: ${result.asErr.toString()}`));
                return;
              }
            }
            console.log(`forceOpenHrmpChannel(1000->2000) included in block: ${status.asInBlock.toHex()}`);
            resolve();
          }
        });
      });

      // Channel 2: custom parachain (2000) -> Asset Hub (1000)
      console.log("Opening channel 2000 -> 1000...");
      const forceOpen2 = api.tx.hrmp.forceOpenHrmpChannel(2000, 1000, 8, 1048576);
      const sudoTx2 = api.tx.sudo.sudo(forceOpen2);

      await new Promise<void>((resolve, reject) => {
        sudoTx2.signAndSend(alice, ({ status, dispatchError, events }) => {
          if (dispatchError) {
            if (dispatchError.isModule) {
              const decoded = api.registry.findMetaError(dispatchError.asModule);
              reject(new Error(`${decoded.section}.${decoded.name}: ${decoded.docs.join(" ")}`));
            } else {
              reject(new Error(dispatchError.toString()));
            }
          }
          if (status.isInBlock) {
            const sudoEvent = events.find(({ event }) =>
              api.events.sudo.Sudid.is(event)
            );
            if (sudoEvent) {
              const [result] = sudoEvent.event.data as any;
              if (result.isErr) {
                reject(new Error(`Sudo call failed: ${result.asErr.toString()}`));
                return;
              }
            }
            console.log(`forceOpenHrmpChannel(2000->1000) included in block: ${status.asInBlock.toHex()}`);
            resolve();
          }
        });
      });

      await api.disconnect();
    }, 60000);
  });

  // ==================== VERIFY CHANNELS ESTABLISHED ====================
  describe("7. Verify Channels Established", () => {
    it("should have both HRMP channels established after session boundary", async () => {
      console.log("Waiting for both HRMP channels to be established (requires session boundary)...");

      const wsProvider = new WsProvider(RELAY_WS_URL);
      const api = await ApiPromise.create({ provider: wsProvider });

      let channel1Found = false;
      let channel2Found = false;

      for (let attempt = 1; attempt <= 25; attempt++) {
        if (!channel1Found) {
          const ch1 = await api.query.hrmp.hrmpChannels({
            sender: 1000,
            recipient: 2000,
          });
          const ch1Json = ch1.toJSON() as any;
          if (ch1Json && ch1Json.maxCapacity) {
            console.log(`HRMP channel 1000->2000 established: ${JSON.stringify(ch1Json)}`);
            channel1Found = true;
          }
        }

        if (!channel2Found) {
          const ch2 = await api.query.hrmp.hrmpChannels({
            sender: 2000,
            recipient: 1000,
          });
          const ch2Json = ch2.toJSON() as any;
          if (ch2Json && ch2Json.maxCapacity) {
            console.log(`HRMP channel 2000->1000 established: ${JSON.stringify(ch2Json)}`);
            channel2Found = true;
          }
        }

        if (channel1Found && channel2Found) break;

        console.log(`Attempt ${attempt}/25: waiting for channels (1000->2000: ${channel1Found}, 2000->1000: ${channel2Found})...`);
        await new Promise((resolve) => setTimeout(resolve, 12000));
      }

      await api.disconnect();
      expect(channel1Found).toBe(true);
      expect(channel2Found).toBe(true);
      console.log("Bidirectional HRMP channels 1000 <-> 2000 are active!");
    }, 360000);
  });

  // ==================== POST-CHANNEL VERIFICATION ====================
  describe("8. Post-Channel Verification", () => {
    it("should verify both parachains continue producing blocks", async () => {
      // Check Asset Hub
      const resultA = (await rpcCall(ASSET_HUB_RPC_URL, "chain_getHeader")) as { number: string };
      const blockA = parseInt(resultA.number, 16);
      console.log(`Asset Hub current block: ${blockA}`);
      expect(blockA).toBeGreaterThan(0);

      // Check custom parachain
      const resultB = (await rpcCall(CUSTOM_PARA_RPC_URL, "chain_getHeader")) as { number: string };
      const blockB = parseInt(resultB.number, 16);
      console.log(`Custom parachain current block: ${blockB}`);
      expect(blockB).toBeGreaterThan(0);
    }, 30000);

    it("should verify relay chain is still operational", async () => {
      const name = (await rpcCall(RELAY_RPC_URL, "system_name")) as string;
      expect(name).toBeDefined();
      console.log(`Relay chain system name: ${name}`);

      const result = (await rpcCall(RELAY_RPC_URL, "chain_getHeader")) as { number: string };
      const blockNumber = parseInt(result.number, 16);
      console.log(`Relay chain block: ${blockNumber}`);
      expect(blockNumber).toBeGreaterThan(0);
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

  try {
    execSync("pkill -f 'polkadot.*rococo-local' 2>/dev/null || true", { encoding: "utf-8" });
    execSync("pkill -f 'polkadot-omni-node' 2>/dev/null || true", { encoding: "utf-8" });
    execSync("pkill -f 'polkadot-parachain' 2>/dev/null || true", { encoding: "utf-8" });
  } catch {
    // Ignore
  }

  await new Promise((resolve) => setTimeout(resolve, 2000));
  console.log("Zombienet stopped");
}
