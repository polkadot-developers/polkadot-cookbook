import { describe, it, expect } from "vitest";
import { execSync } from "child_process";
import { existsSync } from "fs";
import { join } from "path";

const PROJECT_DIR = process.cwd();
const REPO_URL = "https://github.com/brunopgalvao/recipe-contracts-example";
const REPO_VERSION = "v1.0.0";
const REPO_DIR = join(PROJECT_DIR, "recipe-contracts-example");

describe("Contracts Example Recipe", () => {
  // ==================== PREREQUISITES ====================
  describe("1. Prerequisites", () => {
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
        console.log(`Cloning recipe-contracts-example ${REPO_VERSION}...`);
        execSync(`git clone --branch ${REPO_VERSION} ${REPO_URL}`, {
          cwd: PROJECT_DIR,
          encoding: "utf-8",
          stdio: "inherit",
        });
      }

      expect(existsSync(join(REPO_DIR, "package.json"))).toBe(true);
      console.log("Repository cloned successfully");
    }, 120000);
  });

  // ==================== INSTALL ====================
  describe("3. Install Dependencies", () => {
    it("should install npm dependencies", () => {
      console.log("Installing dependencies...");
      execSync("npm ci", {
        cwd: REPO_DIR,
        encoding: "utf-8",
        stdio: "inherit",
      });
      expect(existsSync(join(REPO_DIR, "node_modules"))).toBe(true);
      console.log("Dependencies installed successfully");
    }, 120000);
  });

  // ==================== BUILD ====================
  describe("4. Compile Contracts", () => {
    it("should compile Solidity contracts with Hardhat", () => {
      console.log("Compiling contracts...");
      execSync("npx hardhat compile", {
        cwd: REPO_DIR,
        encoding: "utf-8",
        stdio: "inherit",
      });
      console.log("Contracts compiled successfully");
    }, 120000);
  });

  // ==================== TEST ====================
  describe("5. Run Tests", () => {
    it("should pass all Hardhat tests", () => {
      console.log("Running Hardhat tests on pallet-revive dev node...");
      const result = execSync("npx hardhat test --network localhost", {
        cwd: REPO_DIR,
        encoding: "utf-8",
        stdio: "inherit",
      });
      console.log("All tests passed");
    }, 120000);
  });
});
