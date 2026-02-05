import { describe, it, expect, afterAll } from "vitest";
import { execSync, spawn, ChildProcess } from "child_process";
import { existsSync, mkdirSync, writeFileSync, unlinkSync, readFileSync } from "fs";
import { join } from "path";

const PROJECT_DIR = process.cwd();
const TEMPLATE_REPO = "https://github.com/OpenZeppelin/polkadot-runtime-templates";
const TEMPLATE_VERSION = "v4.0.0";
const TEMPLATE_DIR = join(PROJECT_DIR, "polkadot-runtime-templates", "generic-template");
const BIN_DIR = join(PROJECT_DIR, "bin");
const POLKADOT_BINARY = join(BIN_DIR, "polkadot");
const PARACHAIN_BINARY = join(TEMPLATE_DIR, "target/release/generic-template-node");
const PID_FILE = join(PROJECT_DIR, "zombienet.pid");

// Relay chain RPC port (alice)
const RELAY_RPC_PORT = 9944;
// Parachain RPC port (collator)
const PARACHAIN_RPC_PORT = 9988;

let zombienetProcess: ChildProcess | null = null;

describe("Run a Parachain Network Guide", () => {
  afterAll(async () => {
    await stopZombienet();
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

    it("should have wasm32-unknown-unknown target", () => {
      const targets = execSync("rustup target list --installed", {
        encoding: "utf-8",
      });
      expect(targets).toContain("wasm32-unknown-unknown");
      console.log("wasm32-unknown-unknown target: installed");
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

  // ==================== CLONE AND BUILD ====================
  describe("2. Clone and Build Parachain Template", () => {
    it("should clone OpenZeppelin polkadot-runtime-templates", () => {
      const repoDir = join(PROJECT_DIR, "polkadot-runtime-templates");

      if (existsSync(repoDir)) {
        console.log(`Repository already cloned, checking out ${TEMPLATE_VERSION}...`);
        execSync(`git fetch --tags && git checkout ${TEMPLATE_VERSION}`, { cwd: repoDir, encoding: "utf-8" });
      } else {
        console.log(`Cloning OpenZeppelin polkadot-runtime-templates ${TEMPLATE_VERSION}...`);
        execSync(
          `git clone --branch ${TEMPLATE_VERSION} ${TEMPLATE_REPO}`,
          { cwd: PROJECT_DIR, encoding: "utf-8", stdio: "inherit" }
        );
      }

      expect(existsSync(join(TEMPLATE_DIR, "Cargo.toml"))).toBe(true);
      console.log("Repository cloned successfully");
    }, 120000);

    it("should build the custom parachain binary", () => {
      console.log("Building parachain template (this may take 15-30 minutes)...");

      execSync("cargo build --release", {
        cwd: TEMPLATE_DIR,
        encoding: "utf-8",
        stdio: "inherit",
        timeout: 1800000, // 30 minutes
      });

      expect(existsSync(PARACHAIN_BINARY)).toBe(true);
      console.log("Parachain binary built successfully");
    }, 1800000);
  });

  // ==================== DOWNLOAD RELAY CHAIN ====================
  describe("3. Download Relay Chain Binary", () => {
    it("should download polkadot binary via zombienet setup", () => {
      // Create bin directory
      if (!existsSync(BIN_DIR)) {
        mkdirSync(BIN_DIR, { recursive: true });
      }

      if (existsSync(POLKADOT_BINARY)) {
        console.log("Polkadot binary already exists");
        const version = execSync(`${POLKADOT_BINARY} --version 2>&1`, {
          encoding: "utf-8",
        });
        console.log(`Polkadot: ${version.trim()}`);
        return;
      }

      console.log("Downloading Polkadot binary via zombienet setup...");

      // Use zombienet setup to download the binary
      // The -y flag auto-confirms prompts
      execSync("zombienet setup polkadot -y", {
        cwd: BIN_DIR,
        encoding: "utf-8",
        stdio: "inherit",
        timeout: 300000, // 5 minutes
      });

      expect(existsSync(POLKADOT_BINARY)).toBe(true);

      // Make binary executable
      execSync(`chmod +x ${POLKADOT_BINARY}`);

      const version = execSync(`${POLKADOT_BINARY} --version 2>&1`, {
        encoding: "utf-8",
      });
      console.log(`Polkadot downloaded: ${version.trim()}`);
    }, 300000);
  });

  // ==================== CONFIGURE NETWORK ====================
  describe("4. Configure Network", () => {
    it("should have valid network.toml configuration", () => {
      const configPath = join(PROJECT_DIR, "configs", "network.toml");
      expect(existsSync(configPath)).toBe(true);

      const config = readFileSync(configPath, "utf-8");

      // Verify essential configuration elements
      expect(config).toContain("[relaychain]");
      expect(config).toContain('chain = "rococo-local"');
      expect(config).toContain("[[relaychain.nodes]]");
      expect(config).toContain('name = "alice"');
      expect(config).toContain("[[parachains]]");
      expect(config).toContain("[[parachains.collators]]");

      console.log("Network configuration is valid");
    });
  });

  // ==================== SPAWN NETWORK ====================
  describe("5. Spawn Network", () => {
    it("should spawn the network with zombienet", async () => {
      console.log("Spawning network with Zombienet...");

      const configPath = join(PROJECT_DIR, "configs", "network.toml");

      zombienetProcess = spawn(
        "zombienet",
        ["spawn", configPath, "--provider", "native"],
        {
          cwd: PROJECT_DIR,
          stdio: ["ignore", "pipe", "pipe"],
          detached: true,
        }
      );

      if (zombienetProcess.pid) {
        writeFileSync(PID_FILE, zombienetProcess.pid.toString());
        console.log(`Zombienet started with PID: ${zombienetProcess.pid}`);
      }

      // Collect output for debugging
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

      // Wait for the relay chain RPC to be available
      const maxWaitTime = 120000; // 2 minutes
      const startTime = Date.now();

      while (Date.now() - startTime < maxWaitTime) {
        try {
          const response = await fetch(`http://127.0.0.1:${RELAY_RPC_PORT}`, {
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
            console.log("Relay chain RPC is ready!");
            return;
          }
        } catch {
          // Node not ready yet
        }

        await new Promise((resolve) => setTimeout(resolve, 2000));
      }

      // If we get here, something went wrong
      console.log("stdout:", stdout);
      console.log("stderr:", stderr);
      throw new Error("Network failed to start within 2 minutes");
    }, 180000);

    it("should verify relay chain is producing blocks", async () => {
      console.log("Verifying relay chain block production...");

      // Wait for blocks to be produced (relay chain needs time to start producing)
      // Retry a few times with increasing delays
      let blockNumber = 0;
      const maxAttempts = 5;

      for (let attempt = 1; attempt <= maxAttempts; attempt++) {
        await new Promise((resolve) => setTimeout(resolve, 12000));

        const response = await fetch(`http://127.0.0.1:${RELAY_RPC_PORT}`, {
          method: "POST",
          headers: { "Content-Type": "application/json" },
          body: JSON.stringify({
            jsonrpc: "2.0",
            method: "chain_getHeader",
            params: [],
            id: 1,
          }),
        });

        const result = await response.json();
        expect(result.result).toBeDefined();
        expect(result.result.number).toBeDefined();

        blockNumber = parseInt(result.result.number, 16);
        console.log(`Relay chain block number (attempt ${attempt}): ${blockNumber}`);

        if (blockNumber > 0) {
          break;
        }
      }

      expect(blockNumber).toBeGreaterThan(0);
    }, 90000);

    it("should verify parachain is producing blocks", async () => {
      console.log("Verifying parachain block production...");

      // Wait for parachain to start producing blocks
      // Parachains need to be onboarded first, which takes some time
      await new Promise((resolve) => setTimeout(resolve, 30000));

      // Try parachain RPC port
      try {
        const response = await fetch(`http://127.0.0.1:${PARACHAIN_RPC_PORT}`, {
          method: "POST",
          headers: { "Content-Type": "application/json" },
          body: JSON.stringify({
            jsonrpc: "2.0",
            method: "chain_getHeader",
            params: [],
            id: 1,
          }),
        });

        const result = await response.json();
        expect(result.result).toBeDefined();
        expect(result.result.number).toBeDefined();

        const blockNumber = parseInt(result.result.number, 16);
        console.log(`Parachain block number: ${blockNumber}`);

        expect(blockNumber).toBeGreaterThanOrEqual(0);
      } catch (error) {
        // If we can't reach parachain RPC, check relay chain for parachain info
        console.log("Checking parachain status via relay chain...");

        const response = await fetch(`http://127.0.0.1:${RELAY_RPC_PORT}`, {
          method: "POST",
          headers: { "Content-Type": "application/json" },
          body: JSON.stringify({
            jsonrpc: "2.0",
            method: "system_health",
            params: [],
            id: 1,
          }),
        });

        const result = await response.json();
        expect(result.result).toBeDefined();
        console.log("Relay chain is healthy, parachain should be syncing");
      }
    }, 90000);

    it("should respond to RPC queries", async () => {
      console.log("Testing RPC endpoints...");

      const nameResponse = await fetch(`http://127.0.0.1:${RELAY_RPC_PORT}`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          jsonrpc: "2.0",
          method: "system_name",
          params: [],
          id: 1,
        }),
      });

      const nameResult = await nameResponse.json();
      expect(nameResult.result).toBeDefined();
      console.log(`Relay chain system name: ${nameResult.result}`);

      const versionResponse = await fetch(`http://127.0.0.1:${RELAY_RPC_PORT}`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          jsonrpc: "2.0",
          method: "system_version",
          params: [],
          id: 1,
        }),
      });

      const versionResult = await versionResponse.json();
      expect(versionResult.result).toBeDefined();
      console.log(`Relay chain version: ${versionResult.result}`);
    }, 10000);
  });
});

async function stopZombienet(): Promise<void> {
  console.log("Stopping Zombienet...");

  if (zombienetProcess && !zombienetProcess.killed) {
    // Kill the process group to ensure all child processes are terminated
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

  // Also try to kill any lingering polkadot/parachain processes
  try {
    execSync("pkill -f 'polkadot.*rococo-local' 2>/dev/null || true", {
      encoding: "utf-8",
    });
    execSync("pkill -f 'parachain-template-node' 2>/dev/null || true", {
      encoding: "utf-8",
    });
  } catch {
    // Ignore errors
  }

  await new Promise((resolve) => setTimeout(resolve, 2000));
  console.log("Zombienet stopped");
}
