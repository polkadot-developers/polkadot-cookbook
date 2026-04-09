import { describe, it, expect, afterAll } from "vitest";
import { execSync, spawn, ChildProcess } from "child_process";
import { existsSync, readFileSync, writeFileSync, unlinkSync, mkdirSync, statSync } from "fs";
import { join } from "path";

const WORKSPACE_DIR = join(process.cwd(), ".test-workspace");
const TEMPLATE_DIR = join(WORKSPACE_DIR, "parachain-template");
const CHAIN_SPEC_PATH = join(WORKSPACE_DIR, "chain_spec.json");
const PID_FILE = join(WORKSPACE_DIR, "node.pid");
const WASM_PATH = join(
  TEMPLATE_DIR,
  "target/release/wbuild/parachain-template-runtime/parachain_template_runtime.compact.compressed.wasm"
);

let nodeProcess: ChildProcess | null = null;

// All tests in a single describe block to ensure sequential execution
describe("Parachain Template Guide", () => {
  afterAll(async () => {
    await stopNode();
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
      } catch (error) {
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
      } catch (error) {
        console.log("Installing polkadot-omni-node...");
        execSync(`cargo install polkadot-omni-node@${process.env.POLKADOT_OMNI_NODE_VERSION} --locked`, {
          stdio: "inherit",
        });
      }
    });

    it("should have git installed", () => {
      const result = execSync("git --version", { encoding: "utf-8" });
      expect(result).toMatch(/git version/);
      console.log(`Git: ${result.trim()}`);
    });
  });

  // ==================== BUILD TESTS ====================
  describe("2. Parachain Template Build", () => {
    it("should clone the parachain template repository", () => {
      // Create workspace directory
      if (!existsSync(WORKSPACE_DIR)) {
        mkdirSync(WORKSPACE_DIR, { recursive: true });
      }

      if (existsSync(TEMPLATE_DIR)) {
        console.log("Template already cloned, pulling latest...");
        execSync("git pull", { cwd: TEMPLATE_DIR, encoding: "utf-8" });
      } else {
        console.log("Cloning polkadot-sdk-parachain-template...");
        execSync(
          `git clone https://github.com/paritytech/polkadot-sdk-parachain-template.git ${TEMPLATE_DIR}`,
          { encoding: "utf-8", stdio: "inherit" }
        );
      }

      expect(existsSync(join(TEMPLATE_DIR, "Cargo.toml"))).toBe(true);
      expect(existsSync(join(TEMPLATE_DIR, "runtime"))).toBe(true);
      console.log("Repository cloned successfully");
    }, 120000);

    it("should build the parachain template", () => {
      console.log("Building parachain template (this may take 15-30 minutes)...");

      execSync("cargo build --release --locked", {
        cwd: TEMPLATE_DIR,
        encoding: "utf-8",
        stdio: "inherit",
        timeout: 1800000,
      });

      expect(existsSync(WASM_PATH)).toBe(true);
      console.log("WASM runtime built successfully");
    }, 1800000);

    it("should have generated the WASM runtime", () => {
      expect(existsSync(WASM_PATH)).toBe(true);

      const stats = statSync(WASM_PATH);
      const sizeKB = Math.round(stats.size / 1024);
      console.log(`WASM runtime size: ${sizeKB} KB`);

      expect(stats.size).toBeGreaterThan(100000);
    });
  });

  // ==================== RUNTIME TESTS ====================
  describe("3. Parachain Runtime", () => {
    it("should generate chain specification", () => {
      console.log("Generating chain specification...");

      if (!existsSync(WASM_PATH)) {
        throw new Error(`WASM runtime not found at ${WASM_PATH}. Build must complete first.`);
      }

      execSync(
        `chain-spec-builder create -t development \
          --relay-chain paseo \
          --para-id 1000 \
          --runtime ${WASM_PATH} \
          named-preset development`,
        { encoding: "utf-8", cwd: WORKSPACE_DIR }
      );

      expect(existsSync(CHAIN_SPEC_PATH)).toBe(true);

      const chainSpec = JSON.parse(readFileSync(CHAIN_SPEC_PATH, "utf-8"));
      expect(chainSpec.name).toBeDefined();
      expect(chainSpec.id).toBeDefined();

      console.log(`Chain spec generated: ${chainSpec.name}`);
    }, 60000);

    it("should start the parachain node", async () => {
      console.log("Starting parachain node...");

      nodeProcess = spawn(
        "polkadot-omni-node",
        ["--chain", CHAIN_SPEC_PATH, "--dev"],
        {
          cwd: WORKSPACE_DIR,
          stdio: ["ignore", "pipe", "pipe"],
          detached: true,
        }
      );

      if (nodeProcess.pid) {
        writeFileSync(PID_FILE, nodeProcess.pid.toString());
        console.log(`Node started with PID: ${nodeProcess.pid}`);
      }

      const maxWaitTime = 60000;
      const startTime = Date.now();

      while (Date.now() - startTime < maxWaitTime) {
        try {
          const response = await fetch("http://127.0.0.1:9944", {
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
            console.log("Node is ready!");
            return;
          }
        } catch {
          // Node not ready yet
        }

        await new Promise((resolve) => setTimeout(resolve, 1000));
      }

      throw new Error("Node failed to start within 60 seconds");
    }, 90000);

    it("should produce blocks", async () => {
      console.log("Verifying block production...");

      await new Promise((resolve) => setTimeout(resolve, 6000));

      const response = await fetch("http://127.0.0.1:9944", {
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
      console.log(`Current block number: ${blockNumber}`);

      expect(blockNumber).toBeGreaterThan(0);
    }, 30000);

    it("should respond to RPC queries", async () => {
      console.log("Testing RPC endpoints...");

      const nameResponse = await fetch("http://127.0.0.1:9944", {
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
      console.log(`System name: ${nameResult.result}`);

      const versionResponse = await fetch("http://127.0.0.1:9944", {
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
      console.log(`System version: ${versionResult.result}`);
    }, 10000);
  });
});

async function stopNode(): Promise<void> {
  console.log("Stopping parachain node...");

  if (nodeProcess && !nodeProcess.killed) {
    nodeProcess.kill("SIGTERM");
    nodeProcess = null;
  }

  if (existsSync(PID_FILE)) {
    try {
      const pid = parseInt(readFileSync(PID_FILE, "utf-8"));
      process.kill(pid, "SIGTERM");
    } catch {
      // Process might already be dead
    }
    unlinkSync(PID_FILE);
  }

  await new Promise((resolve) => setTimeout(resolve, 2000));
  console.log("Node stopped");
}
