import { describe, it, expect, afterAll } from "vitest";
import { execSync, spawn, ChildProcess } from "child_process";
import { existsSync, writeFileSync, unlinkSync, readFileSync } from "fs";
import { join } from "path";

const PROJECT_DIR = process.cwd();
const REPO_URL = "https://github.com/brunopgalvao/recipe-parachain-example";
const REPO_VERSION = "v1.1.0";
const REPO_DIR = join(PROJECT_DIR, "recipe-parachain-example");
const RUNTIME_WASM = join(
  REPO_DIR,
  "target/release/wbuild/parachain-example-runtime/parachain_example_runtime.compact.compressed.wasm"
);
const CHAIN_SPEC = join(REPO_DIR, "chain-spec.json");
const PID_FILE = join(PROJECT_DIR, "node.pid");
const RPC_PORT = 9944;

let nodeProcess: ChildProcess | null = null;

describe("Parachain Example Recipe", () => {
  afterAll(async () => {
    await stopNode();
  });

  // ==================== PREREQUISITES ====================
  describe("1. Prerequisites", () => {
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

    it("should have polkadot-omni-node installed", () => {
      const result = execSync("polkadot-omni-node --version", { encoding: "utf-8" });
      expect(result).toMatch(/polkadot-omni-node/);
      console.log(`polkadot-omni-node: ${result.trim()}`);
    });

    it("should have Node.js installed", () => {
      const result = execSync("node --version", { encoding: "utf-8" });
      expect(result).toMatch(/v\d+\.\d+/);
      console.log(`Node.js: ${result.trim()}`);
    });

    it("should have git installed", () => {
      const result = execSync("git --version", { encoding: "utf-8" });
      expect(result).toMatch(/git version/);
      console.log(`Git: ${result.trim()}`);
    });
  });

  // ==================== CLONE ====================
  describe("2. Clone Repository", () => {
    it("should clone the recipe repository", () => {
      if (existsSync(REPO_DIR)) {
        console.log(`Repository already cloned, checking out ${REPO_VERSION}...`);
        execSync(`git fetch --tags && git checkout ${REPO_VERSION}`, {
          cwd: REPO_DIR,
          encoding: "utf-8",
        });
      } else {
        console.log(`Cloning recipe-parachain-example ${REPO_VERSION}...`);
        execSync(`git clone --branch ${REPO_VERSION} ${REPO_URL}`, {
          cwd: PROJECT_DIR,
          encoding: "utf-8",
          stdio: "inherit",
        });
      }

      expect(existsSync(join(REPO_DIR, "Cargo.toml"))).toBe(true);
      console.log("Repository cloned successfully");
    }, 120000);
  });

  // ==================== BUILD ====================
  describe("3. Build Runtime", () => {
    it("should build the parachain runtime WASM", () => {
      console.log("Building parachain runtime (this may take 30-45 minutes on CI)...");
      execSync("cargo build --release", {
        cwd: REPO_DIR,
        encoding: "utf-8",
        stdio: "inherit",
        timeout: 2700000, // 45 minutes
      });
      expect(existsSync(RUNTIME_WASM)).toBe(true);
      console.log("Parachain runtime built successfully");
    }, 2700000);
  });

  // ==================== INSTALL NODE DEPS ====================
  describe("4. Install Node.js Dependencies", () => {
    it("should install npm dependencies", () => {
      console.log("Installing Node.js dependencies...");
      execSync("npm ci", {
        cwd: REPO_DIR,
        encoding: "utf-8",
        stdio: "inherit",
      });
      expect(existsSync(join(REPO_DIR, "node_modules"))).toBe(true);
      console.log("Dependencies installed successfully");
    }, 120000);
  });

  // ==================== START NODE ====================
  describe("5. Start Development Node", () => {
    it("should start the parachain node with polkadot-omni-node", async () => {
      console.log("Starting parachain development node via polkadot-omni-node...");

      expect(existsSync(CHAIN_SPEC)).toBe(true);

      nodeProcess = spawn(
        "polkadot-omni-node",
        ["--chain", CHAIN_SPEC, "--tmp", "--rpc-port", String(RPC_PORT), "--rpc-cors", "all", "--rpc-methods", "unsafe"],
        {
          cwd: REPO_DIR,
          stdio: ["ignore", "pipe", "pipe"],
          detached: true,
        }
      );

      if (nodeProcess.pid) {
        writeFileSync(PID_FILE, nodeProcess.pid.toString());
        console.log(`Node started with PID: ${nodeProcess.pid}`);
      }

      nodeProcess.stderr?.on("data", (data) => {
        const output = data.toString();
        console.log(`[omni-node] ${output.trim()}`);
      });

      // Wait for the RPC to be available
      const maxWaitTime = 120000; // 2 minutes
      const startTime = Date.now();

      while (Date.now() - startTime < maxWaitTime) {
        try {
          const response = await fetch(`http://127.0.0.1:${RPC_PORT}`, {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({
              jsonrpc: "2.0",
              method: "system_health",
              params: [],
              id: 1,
            }),
          });

          if (response.ok) {
            console.log("Node RPC is ready!");
            return;
          }
        } catch {
          // Node not ready yet
        }

        await new Promise((resolve) => setTimeout(resolve, 2000));
      }

      throw new Error("Node failed to start within 2 minutes");
    }, 180000);
  });

  // ==================== TEST ====================
  describe("6. Run Tests", () => {
    it("should pass all PAPI integration tests", () => {
      console.log("Running PAPI integration tests...");
      execSync("npm test", {
        cwd: REPO_DIR,
        encoding: "utf-8",
        stdio: "inherit",
      });
      console.log("All tests passed");
    }, 120000);
  });
});

async function stopNode(): Promise<void> {
  console.log("Stopping parachain node...");

  if (nodeProcess && !nodeProcess.killed) {
    try {
      process.kill(-nodeProcess.pid!, "SIGTERM");
    } catch {
      nodeProcess.kill("SIGTERM");
    }
    nodeProcess = null;
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

  // Kill any lingering node processes
  try {
    execSync("pkill -f 'polkadot-omni-node' 2>/dev/null || true", {
      encoding: "utf-8",
    });
  } catch {
    // Ignore errors
  }

  await new Promise((resolve) => setTimeout(resolve, 2000));
  console.log("Node stopped");
}
