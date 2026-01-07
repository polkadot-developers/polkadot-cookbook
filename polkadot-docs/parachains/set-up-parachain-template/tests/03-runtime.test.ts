import { describe, it, expect, beforeAll, afterAll } from "vitest";
import { execSync, spawn, ChildProcess } from "child_process";
import { existsSync, readFileSync, writeFileSync, unlinkSync } from "fs";
import { join } from "path";

const WORKSPACE_DIR = join(process.cwd(), ".test-workspace");
const TEMPLATE_DIR = join(WORKSPACE_DIR, "parachain-template");
const CHAIN_SPEC_PATH = join(WORKSPACE_DIR, "chain_spec.json");
const PID_FILE = join(WORKSPACE_DIR, "node.pid");
const LOG_FILE = join(WORKSPACE_DIR, "node.log");

let nodeProcess: ChildProcess | null = null;

describe("Parachain Runtime", () => {
  beforeAll(() => {
    // Ensure build was completed
    const wasmPath = join(
      TEMPLATE_DIR,
      "target/release/wbuild/parachain-template-runtime/parachain_template_runtime.compact.compressed.wasm"
    );

    if (!existsSync(wasmPath)) {
      throw new Error(
        "WASM runtime not found. Run build tests first."
      );
    }
  });

  afterAll(async () => {
    // Cleanup: stop node if running
    await stopNode();
  });

  it("should generate chain specification", () => {
    console.log("Generating chain specification...");

    const wasmPath = join(
      TEMPLATE_DIR,
      "target/release/wbuild/parachain-template-runtime/parachain_template_runtime.compact.compressed.wasm"
    );

    // Generate chain spec
    execSync(
      `chain-spec-builder create -t development \
        --relay-chain paseo \
        --para-id 1000 \
        --runtime ${wasmPath} \
        named-preset development`,
      { encoding: "utf-8", cwd: WORKSPACE_DIR }
    );

    // Move to expected location
    if (existsSync(join(WORKSPACE_DIR, "chain_spec.json"))) {
      // Already in the right place
    }

    expect(existsSync(CHAIN_SPEC_PATH)).toBe(true);

    // Verify chain spec is valid JSON
    const chainSpec = JSON.parse(readFileSync(CHAIN_SPEC_PATH, "utf-8"));
    expect(chainSpec.name).toBeDefined();
    expect(chainSpec.id).toBeDefined();

    console.log(`Chain spec generated: ${chainSpec.name}`);
  }, 60000);

  it("should start the parachain node", async () => {
    console.log("Starting parachain node...");

    // Start node in background
    nodeProcess = spawn(
      "polkadot-omni-node",
      ["--chain", CHAIN_SPEC_PATH, "--dev"],
      {
        cwd: WORKSPACE_DIR,
        stdio: ["ignore", "pipe", "pipe"],
        detached: true,
      }
    );

    // Save PID for cleanup
    if (nodeProcess.pid) {
      writeFileSync(PID_FILE, nodeProcess.pid.toString());
      console.log(`Node started with PID: ${nodeProcess.pid}`);
    }

    // Wait for node to be ready
    const maxWaitTime = 60000; // 60 seconds
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
        // Node not ready yet, wait and retry
      }

      await new Promise((resolve) => setTimeout(resolve, 1000));
    }

    throw new Error("Node failed to start within 60 seconds");
  }, 90000);

  it("should produce blocks", async () => {
    console.log("Verifying block production...");

    // Wait a few seconds for block production
    await new Promise((resolve) => setTimeout(resolve, 6000));

    // Query current block number
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

    // Should have produced at least 1 block
    expect(blockNumber).toBeGreaterThan(0);
  }, 30000);

  it("should respond to RPC queries", async () => {
    console.log("Testing RPC endpoints...");

    // Test system_name
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

    // Test system_version
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

async function stopNode(): Promise<void> {
  console.log("Stopping parachain node...");

  // Kill via process reference
  if (nodeProcess && !nodeProcess.killed) {
    nodeProcess.kill("SIGTERM");
    nodeProcess = null;
  }

  // Also try to kill via PID file
  if (existsSync(PID_FILE)) {
    try {
      const pid = parseInt(readFileSync(PID_FILE, "utf-8"));
      process.kill(pid, "SIGTERM");
    } catch {
      // Process might already be dead
    }
    unlinkSync(PID_FILE);
  }

  // Give it a moment to shutdown
  await new Promise((resolve) => setTimeout(resolve, 2000));
  console.log("Node stopped");
}
