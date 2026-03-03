import { describe, it, expect } from "vitest";
import { execSync } from "child_process";
import { existsSync, rmSync, readFileSync, writeFileSync } from "fs";
import { join } from "path";

const WORKSPACE_DIR = join(process.cwd(), ".test-workspace");
const REPO_URL = "https://github.com/papermoonio/v2-periphery-polkadot.git";
const REPO_BRANCH = "revm";
const REPO_COMMIT = "bbc9fb3d2a067ac9f387d495cc6f59b90851c834";

describe("Uniswap V2 Periphery REVM Migration", () => {
  describe("1. Environment Prerequisites", () => {
    it("should have Node.js installed", () => {
      const result = execSync("node --version", { encoding: "utf-8" });
      expect(result).toMatch(/v\d+\.\d+/);
      console.log(`Node.js: ${result.trim()}`);
    });

    it("should have npm installed", () => {
      const result = execSync("npm --version", { encoding: "utf-8" });
      expect(result).toMatch(/\d+\.\d+/);
      console.log(`npm: ${result.trim()}`);
    });

    it("should have git installed", () => {
      const result = execSync("git --version", { encoding: "utf-8" });
      expect(result).toMatch(/git version/);
      console.log(`Git: ${result.trim()}`);
    });
  });

  describe("2. Clone Repository", () => {
    it("should clone v2-periphery-polkadot at pinned commit", () => {
      // Clean up if exists
      if (existsSync(WORKSPACE_DIR)) {
        console.log("Cleaning up existing workspace...");
        rmSync(WORKSPACE_DIR, { recursive: true, force: true });
      }

      console.log(`Cloning repository at commit ${REPO_COMMIT}...`);

      // Clone specific branch with depth 1, then pin to exact commit
      execSync(`git clone --depth 1 --branch ${REPO_BRANCH} ${REPO_URL} ${WORKSPACE_DIR}`, {
        encoding: "utf-8",
        stdio: "inherit",
      });

      // Fetch and checkout the specific commit
      execSync(
        `git fetch --depth 1 origin ${REPO_COMMIT} && git checkout ${REPO_COMMIT}`,
        { cwd: WORKSPACE_DIR, encoding: "utf-8", stdio: "inherit" }
      );

      expect(existsSync(join(WORKSPACE_DIR, "package.json"))).toBe(true);
      expect(existsSync(join(WORKSPACE_DIR, "hardhat.config.ts"))).toBe(true);

      // Patch local network URL: eth-rpc binds to IPv6 [::1]:8545,
      // so 127.0.0.1 (IPv4) won't reach it — use localhost instead
      const configPath = join(WORKSPACE_DIR, "hardhat.config.ts");
      const config = readFileSync(configPath, "utf-8");
      writeFileSync(configPath, config.replace("http://127.0.0.1:8545", "http://localhost:8545"));

      console.log("Repository cloned successfully");
    }, 120000);

    it("should verify pinned commit", () => {
      const currentCommit = execSync("git rev-parse HEAD", {
        cwd: WORKSPACE_DIR,
        encoding: "utf-8",
      }).trim();

      expect(currentCommit).toBe(REPO_COMMIT);
      console.log(`Verified commit: ${currentCommit}`);
    });
  });

  describe("3. Install Dependencies", () => {
    it("should install npm dependencies", () => {
      console.log("Installing dependencies...");

      execSync("npm ci", {
        cwd: WORKSPACE_DIR,
        encoding: "utf-8",
        stdio: "inherit",
      });

      expect(existsSync(join(WORKSPACE_DIR, "node_modules"))).toBe(true);
      console.log("Dependencies installed successfully");
    }, 300000);
  });

  describe("4. Run REVM Tests", () => {
    it("should compile contracts", () => {
      console.log("Compiling contracts...");

      execSync("npx hardhat compile", {
        cwd: WORKSPACE_DIR,
        encoding: "utf-8",
        stdio: "inherit",
      });

      expect(existsSync(join(WORKSPACE_DIR, "artifacts"))).toBe(true);
      console.log("Contracts compiled successfully");
    }, 300000);

    it("should run tests on pallet-revive dev node", () => {
      console.log("Running tests on pallet-revive dev node...");

      execSync("npx hardhat test --network local", {
        cwd: WORKSPACE_DIR,
        encoding: "utf-8",
        stdio: "inherit",
        timeout: 600000,
      });

      console.log("Tests completed successfully on pallet-revive dev node");
    }, 900000);
  });
});
