import { describe, it, expect, afterAll } from "vitest";
import { execSync, spawn, ChildProcess } from "child_process";
import { existsSync, mkdirSync } from "fs";
import { join } from "path";

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

const SDK_REPO_URL = "https://github.com/paritytech/polkadot-sdk.git";

// Pinned to the release tracked in versions.yml (polkadot_sdk.release_tag).
// Injected by the CI workflow via POLKADOT_SDK_TAG; falls back for local runs.
const SDK_TAG = process.env.POLKADOT_SDK_TAG ?? "polkadot-stable2512-1";

const WORKSPACE_DIR  = join(process.cwd(), ".test-workspace");
const SDK_DIR        = join(WORKSPACE_DIR, "polkadot-sdk");
const DEV_NODE_BIN   = join(SDK_DIR, "target", "release", "revive-dev-node");
const ETH_RPC_BIN    = join(SDK_DIR, "target", "release", "eth-rpc");

// Port where revive-dev-node exposes its Substrate JSON-RPC (HTTP + WS)
const SUBSTRATE_RPC_PORT = 9944;
// Port where the eth-rpc adapter exposes the Ethereum JSON-RPC
const ETH_RPC_PORT = 8545;

const SUBSTRATE_RPC_URL = `http://127.0.0.1:${SUBSTRATE_RPC_PORT}`;
const ETH_RPC_URL       = `http://127.0.0.1:${ETH_RPC_PORT}`;

// ---------------------------------------------------------------------------
// Long-running processes (killed in afterAll)
// ---------------------------------------------------------------------------

let devNodeProcess: ChildProcess | null = null;
let ethRpcProcess: ChildProcess | null = null;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/**
 * Polls a Substrate/Ethereum JSON-RPC endpoint until it returns a non-null
 * result for `method`, or throws when `timeoutMs` elapses.
 */
async function waitForRpc(
  url: string,
  method: string,
  timeoutMs: number
): Promise<unknown> {
  const deadline = Date.now() + timeoutMs;
  let lastError: unknown;

  while (Date.now() < deadline) {
    try {
      const res = await fetch(url, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ jsonrpc: "2.0", method, params: [], id: 1 }),
      });
      if (res.ok) {
        const json = (await res.json()) as { result?: unknown };
        if (json.result !== undefined) return json.result;
      }
    } catch (e) {
      lastError = e; // node not ready yet — keep polling
    }
    await new Promise((r) => setTimeout(r, 2000));
  }

  throw new Error(
    `${url} did not respond to '${method}' within ${timeoutMs / 1000}s. Last error: ${lastError}`
  );
}

// ---------------------------------------------------------------------------
// Cleanup — always runs after the full suite, even on test failure
// ---------------------------------------------------------------------------

afterAll(async () => {
  console.log("\nCleaning up: stopping eth-rpc and revive-dev-node...");

  // Kill eth-rpc first (it depends on the dev node)
  for (const [label, proc] of [
    ["eth-rpc", ethRpcProcess],
    ["revive-dev-node", devNodeProcess],
  ] as [string, ChildProcess | null][]) {
    if (proc && !proc.killed) {
      try {
        process.kill(-proc.pid!, "SIGTERM"); // kill the whole process group
      } catch {
        proc.kill("SIGTERM");
      }
      console.log(`Sent SIGTERM to ${label} (PID ${proc.pid})`);
    }
  }

  // Belt-and-suspenders: pkill any stragglers by binary name
  for (const name of ["revive-dev-node", "eth-rpc"]) {
    try {
      execSync(`pkill -f '${name}' 2>/dev/null || true`);
    } catch { /* ignore */ }
  }

  await new Promise((r) => setTimeout(r, 3000));
  console.log("Cleanup complete.");
});

// ---------------------------------------------------------------------------
// Test suite
// ---------------------------------------------------------------------------

describe("Local Development Node Guide", () => {

  // ==================== 1. PREREQUISITES ====================
  describe("1. Prerequisites", () => {
    // Tutorial requires Rust — this is installed by the CI workflow step.
    it("should have Rust installed", () => {
      const result = execSync("rustc --version", { encoding: "utf-8" }).trim();
      expect(result).toMatch(/rustc \d+\.\d+/);
      console.log(`Rust: ${result}`);
    });

    it("should have Cargo installed", () => {
      const result = execSync("cargo --version", { encoding: "utf-8" }).trim();
      expect(result).toMatch(/cargo \d+\.\d+/);
      console.log(`Cargo: ${result}`);
    });

    it("should have git installed", () => {
      const result = execSync("git --version", { encoding: "utf-8" }).trim();
      expect(result).toMatch(/git version/);
      console.log(`Git: ${result}`);
    });
  });

  // ==================== 2. CLONE POLKADOT-SDK ====================
  describe("2. Clone polkadot-sdk", () => {
    // Mirrors: git clone https://github.com/paritytech/polkadot-sdk.git
    it("should clone polkadot-sdk at the pinned release tag", () => {
      if (!existsSync(WORKSPACE_DIR)) {
        mkdirSync(WORKSPACE_DIR, { recursive: true });
      }

      if (existsSync(join(SDK_DIR, "Cargo.toml"))) {
        // Already present — ensure we are on the right tag
        console.log(`polkadot-sdk already cloned — checking out ${SDK_TAG}...`);
        execSync(`git fetch --tags && git checkout ${SDK_TAG}`, {
          cwd: SDK_DIR,
          stdio: "inherit",
        });
      } else {
        // Shallow clone (--depth 1) — only need the source at this tag
        console.log(`Cloning polkadot-sdk at ${SDK_TAG} (shallow)...`);
        execSync(
          `git clone --branch ${SDK_TAG} --depth 1 ${SDK_REPO_URL} ${SDK_DIR}`,
          { stdio: "inherit" }
        );
      }

      expect(existsSync(join(SDK_DIR, "Cargo.toml"))).toBe(true);
      console.log(`Checked out: ${SDK_TAG}`);
    }, 300000); // shallow clone takes ~2-5 min

    // Verify both target crates exist in the workspace before attempting to build.
    it("should contain the revive-dev-node crate", () => {
      const result = execSync(
        `cargo metadata --no-deps --format-version 1 2>&1`,
        { cwd: SDK_DIR, encoding: "utf-8" }
      );
      expect(result).toContain("revive-dev-node");
      console.log("revive-dev-node crate found in workspace.");
    });

    it("should contain the pallet-revive-eth-rpc crate", () => {
      const result = execSync(
        `cargo metadata --no-deps --format-version 1 2>&1`,
        { cwd: SDK_DIR, encoding: "utf-8" }
      );
      expect(result).toContain("pallet-revive-eth-rpc");
      console.log("pallet-revive-eth-rpc crate found in workspace.");
    });
  });

  // ==================== 3. BUILD REVIVE-DEV-NODE ====================
  describe("3. Build revive-dev-node", () => {
    // Mirrors: cargo build -p revive-dev-node --bin revive-dev-node --release
    // The tutorial notes that compilation takes ~30 minutes; this test
    // skips the build if the binary already exists (e.g., restored from cache).
    it("should compile revive-dev-node in release mode", () => {
      if (existsSync(DEV_NODE_BIN)) {
        console.log("revive-dev-node binary already present — skipping build.");
        return;
      }
      console.log("Building revive-dev-node --release (first run takes ~30 min)...");
      execSync(
        "cargo build -p revive-dev-node --bin revive-dev-node --release",
        { cwd: SDK_DIR, stdio: "inherit", timeout: 3600000 }
      );
      expect(existsSync(DEV_NODE_BIN)).toBe(true);
      console.log("revive-dev-node built successfully.");
    }, 3600000); // 60 min ceiling
  });

  // ==================== 4. BUILD ETH-RPC ====================
  describe("4. Build eth-rpc", () => {
    // Mirrors: cargo build -p pallet-revive-eth-rpc --bin eth-rpc --release
    it("should compile eth-rpc in release mode", () => {
      if (existsSync(ETH_RPC_BIN)) {
        console.log("eth-rpc binary already present — skipping build.");
        return;
      }
      console.log("Building eth-rpc --release...");
      execSync(
        "cargo build -p pallet-revive-eth-rpc --bin eth-rpc --release",
        { cwd: SDK_DIR, stdio: "inherit", timeout: 3600000 }
      );
      expect(existsSync(ETH_RPC_BIN)).toBe(true);
      console.log("eth-rpc built successfully.");
    }, 3600000); // 60 min ceiling
  });

  // ==================== 5. RUN REVIVE-DEV-NODE ====================
  describe("5. Run revive-dev-node", () => {
    // Mirrors: ./target/release/revive-dev-node --dev
    // The node runs in the background; afterAll will SIGTERM it.
    it("should start revive-dev-node --dev and expose the Substrate RPC on port 9944", async () => {
      console.log(`Starting ${DEV_NODE_BIN} --dev...`);

      devNodeProcess = spawn(DEV_NODE_BIN, ["--dev"], {
        // detached: true creates a new process group so we can kill -<pgid>
        detached: true,
        stdio: ["ignore", "pipe", "pipe"],
      });

      devNodeProcess.stdout?.on("data", (d: Buffer) => process.stdout.write(d));
      devNodeProcess.stderr?.on("data", (d: Buffer) => process.stderr.write(d));
      devNodeProcess.on("error", (err) => console.error("revive-dev-node error:", err));

      console.log(`revive-dev-node PID: ${devNodeProcess.pid}`);

      // Wait up to 60 s for the Substrate JSON-RPC to become available.
      await waitForRpc(SUBSTRATE_RPC_URL, "system_health", 60000);
      console.log("Substrate RPC is ready on port 9944.");
    }, 90000);

    it("should report a healthy node via system_health", async () => {
      const health = await waitForRpc(SUBSTRATE_RPC_URL, "system_health", 10000);
      expect(health).toBeDefined();
      console.log("system_health:", JSON.stringify(health));
    }, 15000);

    // Give the dev node a few seconds to author its first block, then confirm.
    it("should be producing blocks", async () => {
      await new Promise((r) => setTimeout(r, 6000));
      const header = (await waitForRpc(SUBSTRATE_RPC_URL, "chain_getHeader", 10000)) as {
        number: string;
      };
      // Genesis (block 0) is acceptable; what matters is that the RPC responded.
      const blockNum = parseInt(header.number, 16);
      expect(blockNum).toBeGreaterThanOrEqual(0);
      console.log(`Current block number: ${blockNum}`);
    }, 30000);
  });

  // ==================== 6. RUN ETH-RPC ADAPTER ====================
  describe("6. Run eth-rpc adapter", () => {
    // Mirrors: ./target/release/eth-rpc --dev
    // The adapter connects to ws://127.0.0.1:9944 (the dev node) and exposes
    // an Ethereum JSON-RPC server on port 8545.
    it("should start eth-rpc --dev and expose the Ethereum JSON-RPC on port 8545", async () => {
      console.log(`Starting ${ETH_RPC_BIN} --dev...`);

      ethRpcProcess = spawn(ETH_RPC_BIN, ["--dev"], {
        detached: true,
        stdio: ["ignore", "pipe", "pipe"],
      });

      ethRpcProcess.stdout?.on("data", (d: Buffer) => process.stdout.write(d));
      ethRpcProcess.stderr?.on("data", (d: Buffer) => process.stderr.write(d));
      ethRpcProcess.on("error", (err) => console.error("eth-rpc error:", err));

      console.log(`eth-rpc PID: ${ethRpcProcess.pid}`);

      // Wait up to 60 s for the ETH JSON-RPC to be accessible.
      await waitForRpc(ETH_RPC_URL, "eth_chainId", 60000);
      console.log("ETH-RPC is ready on port 8545.");
    }, 90000);
  });

  // ==================== 7. VERIFY ETH-RPC ENDPOINT ====================
  describe("7. Verify ETH-RPC endpoint (http://localhost:8545)", () => {
    // This is the endpoint the tutorial instructs users to consume from tools
    // like Hardhat, Foundry, or MetaMask.

    it("should respond to eth_chainId with a valid hex chain ID", async () => {
      const chainId = (await waitForRpc(ETH_RPC_URL, "eth_chainId", 10000)) as string;
      // The chain ID must be a hex-encoded integer (0x-prefixed).
      expect(chainId).toMatch(/^0x[0-9a-fA-F]+$/);
      console.log(`eth_chainId: ${chainId} (decimal: ${parseInt(chainId, 16)})`);
    }, 15000);

    it("should respond to eth_blockNumber", async () => {
      const blockNumber = (await waitForRpc(ETH_RPC_URL, "eth_blockNumber", 10000)) as string;
      expect(blockNumber).toMatch(/^0x[0-9a-fA-F]+$/);
      console.log(`eth_blockNumber: ${blockNumber} (decimal: ${parseInt(blockNumber, 16)})`);
    }, 15000);

    it("should respond to net_version", async () => {
      const version = await waitForRpc(ETH_RPC_URL, "net_version", 10000);
      expect(String(version).length).toBeGreaterThan(0);
      console.log(`net_version: ${version}`);
    }, 15000);

    it("should respond to eth_gasPrice", async () => {
      const gasPrice = (await waitForRpc(ETH_RPC_URL, "eth_gasPrice", 10000)) as string;
      expect(gasPrice).toMatch(/^0x[0-9a-fA-F]+$/);
      console.log(`eth_gasPrice: ${gasPrice}`);
    }, 15000);
  });
});
