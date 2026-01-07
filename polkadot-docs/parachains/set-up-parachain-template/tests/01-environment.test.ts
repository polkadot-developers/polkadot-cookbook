import { describe, it, expect } from "vitest";
import { execSync } from "child_process";
import { existsSync } from "fs";

describe("Environment Prerequisites", () => {
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
      // Try to install it
      console.log("Installing chain-spec-builder...");
      execSync("cargo install staging-chain-spec-builder@10.0.0 --locked", {
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
      // Try to install it
      console.log("Installing polkadot-omni-node...");
      execSync("cargo install polkadot-omni-node@0.5.0 --locked", {
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
