import { describe, it, expect } from "vitest";
import { execSync } from "child_process";
import { existsSync, mkdirSync } from "fs";
import { join } from "path";

const WORKSPACE_DIR = join(process.cwd(), ".test-workspace");
const SDK_DIR = join(WORKSPACE_DIR, "polkadot-sdk");

describe("Install Polkadot SDK Guide", () => {
  // ==================== RUST INSTALLATION ====================
  describe("1. Rust Installation", () => {
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

    it("should have rustup installed", () => {
      const result = execSync("rustup --version", { encoding: "utf-8" });
      expect(result).toMatch(/rustup \d+\.\d+/);
      console.log(`Rustup: ${result.trim()}`);
    });

    it("should have stable toolchain as default", () => {
      const result = execSync("rustup default", { encoding: "utf-8" });
      expect(result).toMatch(/stable/);
      console.log(`Default toolchain: ${result.trim()}`);
    });
  });

  // ==================== RUST CONFIGURATION ====================
  describe("2. Rust Configuration", () => {
    it("should have wasm32-unknown-unknown target installed", () => {
      const targets = execSync("rustup target list --installed", {
        encoding: "utf-8",
      });
      expect(targets).toContain("wasm32-unknown-unknown");
      console.log("wasm32-unknown-unknown target: installed");
    });

    it("should have rust-src component installed", () => {
      const components = execSync("rustup component list --installed", {
        encoding: "utf-8",
      });
      expect(components).toContain("rust-src");
      console.log("rust-src component: installed");
    });
  });

  // ==================== SYSTEM DEPENDENCIES ====================
  describe("3. System Dependencies", () => {
    it("should have git installed", () => {
      const result = execSync("git --version", { encoding: "utf-8" });
      expect(result).toMatch(/git version/);
      console.log(`Git: ${result.trim()}`);
    });

    it("should have protobuf compiler installed", () => {
      const result = execSync("protoc --version", { encoding: "utf-8" });
      expect(result).toMatch(/libprotoc/);
      console.log(`Protobuf: ${result.trim()}`);
    });

    it("should have clang installed", () => {
      const result = execSync("clang --version", { encoding: "utf-8" });
      expect(result.length).toBeGreaterThan(0);
      const firstLine = result.split("\n")[0];
      console.log(`Clang: ${firstLine}`);
    });

    it("should have make installed", () => {
      const result = execSync("make --version", { encoding: "utf-8" });
      expect(result).toMatch(/GNU Make|make/i);
      const firstLine = result.split("\n")[0];
      console.log(`Make: ${firstLine}`);
    });
  });

  // ==================== CLONE POLKADOT SDK ====================
  describe("4. Clone Polkadot SDK", () => {
    it("should clone polkadot-sdk repository", () => {
      // Create workspace directory
      if (!existsSync(WORKSPACE_DIR)) {
        mkdirSync(WORKSPACE_DIR, { recursive: true });
      }

      if (existsSync(SDK_DIR)) {
        console.log("polkadot-sdk already cloned, updating...");
        execSync("git fetch --depth 1", {
          cwd: SDK_DIR,
          encoding: "utf-8",
          stdio: "inherit",
        });
      } else {
        console.log("Cloning polkadot-sdk (shallow clone for speed)...");
        execSync(
          `git clone --depth 1 https://github.com/paritytech/polkadot-sdk.git ${SDK_DIR}`,
          { encoding: "utf-8", stdio: "inherit" }
        );
      }

      expect(existsSync(join(SDK_DIR, "Cargo.toml"))).toBe(true);
      expect(existsSync(join(SDK_DIR, "polkadot"))).toBe(true);
      expect(existsSync(join(SDK_DIR, "substrate"))).toBe(true);
      expect(existsSync(join(SDK_DIR, "cumulus"))).toBe(true);
      console.log("polkadot-sdk cloned successfully");
    }, 120000);

    it("should have valid workspace structure", () => {
      // Verify key directories exist
      const expectedDirs = [
        "polkadot",
        "substrate",
        "cumulus",
        "bridges",
      ];

      for (const dir of expectedDirs) {
        const dirPath = join(SDK_DIR, dir);
        expect(existsSync(dirPath)).toBe(true);
        console.log(`Found: ${dir}/`);
      }
    });
  });

  // ==================== VERIFY WORKSPACE ====================
  describe("5. Verify Workspace", () => {
    it("should be able to read workspace metadata", () => {
      console.log("Reading workspace metadata with cargo metadata...");

      // Use cargo metadata to verify the workspace is valid
      // This is much faster than cargo check/build
      const result = execSync(
        "cargo metadata --format-version 1 --no-deps 2>&1 | head -1",
        {
          cwd: SDK_DIR,
          encoding: "utf-8",
          timeout: 60000,
        }
      );

      // If cargo metadata starts successfully, it means the workspace is valid
      // The output should start with { for JSON
      expect(result.trim().startsWith("{")).toBe(true);
      console.log("Workspace metadata is valid");
    }, 120000);

    it("should have polkadot binary crate", () => {
      const cargoToml = join(SDK_DIR, "polkadot/cli/Cargo.toml");
      expect(existsSync(cargoToml)).toBe(true);
      console.log("Found polkadot CLI crate");
    });

    it("should have substrate-node binary crate", () => {
      // Check for substrate node (kitchensink or minimal node)
      const kitchensinkToml = join(SDK_DIR, "substrate/bin/node/cli/Cargo.toml");
      const minimalToml = join(SDK_DIR, "substrate/bin/minimal/node/Cargo.toml");

      const hasKitchensink = existsSync(kitchensinkToml);
      const hasMinimal = existsSync(minimalToml);

      expect(hasKitchensink || hasMinimal).toBe(true);
      if (hasKitchensink) console.log("Found substrate kitchensink node crate");
      if (hasMinimal) console.log("Found substrate minimal node crate");
    });
  });
});
