import { describe, it, expect } from "vitest";
import { execSync } from "child_process";
import { existsSync } from "fs";
import { join } from "path";

const PROJECT_DIR = process.cwd();
const REPO_URL = "https://github.com/brunopgalvao/recipe-pallet-example";
const REPO_VERSION = "v1.0.0";
const REPO_DIR = join(PROJECT_DIR, "recipe-pallet-example");

describe("Pallet Example Recipe", () => {
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
        console.log(`Cloning recipe-pallet-example ${REPO_VERSION}...`);
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

  // ==================== TEST ====================
  describe("3. Run Tests", () => {
    it("should pass all cargo tests", () => {
      console.log("Running cargo tests (this may take several minutes on first build)...");
      execSync("cargo test --all-features", {
        cwd: REPO_DIR,
        encoding: "utf-8",
        stdio: "inherit",
        timeout: 2700000, // 45 minutes
      });
      console.log("All tests passed");
    }, 2700000);
  });
});
