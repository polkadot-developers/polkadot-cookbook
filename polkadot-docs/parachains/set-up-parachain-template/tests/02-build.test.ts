import { describe, it, expect, beforeAll, afterAll } from "vitest";
import { execSync, exec } from "child_process";
import { existsSync, rmSync, mkdirSync } from "fs";
import { join } from "path";

const TEMPLATE_DIR = join(process.cwd(), ".test-workspace", "parachain-template");
const WORKSPACE_DIR = join(process.cwd(), ".test-workspace");

describe("Parachain Template Build", () => {
  beforeAll(() => {
    // Create workspace directory
    if (!existsSync(WORKSPACE_DIR)) {
      mkdirSync(WORKSPACE_DIR, { recursive: true });
    }
  });

  it("should clone the parachain template repository", () => {
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
  }, 120000); // 2 minute timeout for clone

  it("should build the parachain template", () => {
    console.log("Building parachain template (this may take 15-30 minutes)...");

    // Build with locked dependencies
    execSync("cargo build --release --locked", {
      cwd: TEMPLATE_DIR,
      encoding: "utf-8",
      stdio: "inherit",
      timeout: 1800000, // 30 minute timeout
    });

    const wasmPath = join(
      TEMPLATE_DIR,
      "target/release/wbuild/parachain-template-runtime/parachain_template_runtime.compact.compressed.wasm"
    );

    expect(existsSync(wasmPath)).toBe(true);
    console.log("WASM runtime built successfully");
  }, 1800000); // 30 minute timeout for build

  it("should have generated the WASM runtime", () => {
    const wasmPath = join(
      TEMPLATE_DIR,
      "target/release/wbuild/parachain-template-runtime/parachain_template_runtime.compact.compressed.wasm"
    );

    expect(existsSync(wasmPath)).toBe(true);

    // Check WASM size
    const stats = require("fs").statSync(wasmPath);
    const sizeKB = Math.round(stats.size / 1024);
    console.log(`WASM runtime size: ${sizeKB} KB`);

    // WASM should be at least 100KB (sanity check)
    expect(stats.size).toBeGreaterThan(100000);
  });
});
