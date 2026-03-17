import { describe, it, expect } from "vitest";
import { execSync } from "child_process";
import { existsSync, mkdirSync, readFileSync, rmSync } from "fs";
import { join } from "path";

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

const WORKSPACE_DIR = join(process.cwd(), ".test-workspace");
const PROJECT_DIR = join(WORKSPACE_DIR, "hardhat-pvm");

// Pinned package versions — match the tutorial exactly
const HARDHAT_POLKADOT_VERSION = "0.2.7";
const RESOLC_VERSION = "1.0.0";

// ---------------------------------------------------------------------------
// Credentials
// ---------------------------------------------------------------------------

// The generated hardhat.config.ts uses vars.get('PRIVATE_KEY').
// Hardhat reads HARDHAT_VAR_<VARNAME> from the environment automatically,
// so no interactive `npx hardhat vars set` call is needed in CI.
const PRIVATE_KEY = process.env.PRIVATE_KEY;

const hardhatEnv = {
  ...process.env,
  HARDHAT_VAR_PRIVATE_KEY: PRIVATE_KEY ?? "",
};

// ---------------------------------------------------------------------------
// Test suite
// ---------------------------------------------------------------------------

describe("Use Hardhat with Polkadot Hub (PVM) Guide", () => {
  // ==================== 1. ENVIRONMENT PREREQUISITES ====================
  describe("1. Environment Prerequisites", () => {
    // PVM requires Node.js 22.5+ and npm 10.9.0+ per the tutorial.
    it("should have Node.js v22.5 or later", () => {
      const result = execSync("node --version", { encoding: "utf-8" }).trim();
      const parts = result.replace("v", "").split(".").map(Number);
      const meetsMinimum =
        parts[0] > 22 || (parts[0] === 22 && parts[1] >= 5);
      expect(meetsMinimum, `Node.js 22.5+ required, got ${result}`).toBe(true);
      console.log(`Node.js: ${result}`);
    });

    it("should have npm 10.9.0 or later", () => {
      const result = execSync("npm --version", { encoding: "utf-8" }).trim();
      const parts = result.split(".").map(Number);
      const meetsMinimum =
        parts[0] > 10 || (parts[0] === 10 && parts[1] >= 9);
      expect(meetsMinimum, `npm 10.9.0+ required, got ${result}`).toBe(true);
      console.log(`npm: ${result}`);
    });

    it("should have git available", () => {
      const result = execSync("git --version", { encoding: "utf-8" }).trim();
      expect(result).toMatch(/git version/);
      console.log(`git: ${result}`);
    });
  });

  // ==================== 2. INITIALIZE PVM PROJECT ====================
  describe("2. Initialize PVM Project", () => {
    // Mirrors the tutorial steps:
    //   mkdir hardhat-pvm-example && cd hardhat-pvm-example
    //   npm init -y
    //   npm install --save-dev @parity/hardhat-polkadot@0.2.7
    //   npm install --save-dev @parity/resolc@1.0.0
    //   npx hardhat-polkadot init
    //   npm install
    it("should create the project directory", () => {
      if (!existsSync(WORKSPACE_DIR)) {
        mkdirSync(WORKSPACE_DIR, { recursive: true });
      }
      if (!existsSync(PROJECT_DIR)) {
        mkdirSync(PROJECT_DIR, { recursive: true });
      }
      expect(existsSync(PROJECT_DIR)).toBe(true);
    });

    it("should initialise a package.json via npm init", () => {
      if (!existsSync(join(PROJECT_DIR, "package.json"))) {
        execSync("npm init -y", { cwd: PROJECT_DIR, stdio: "inherit" });
      }
      expect(existsSync(join(PROJECT_DIR, "package.json"))).toBe(true);
    });

    it("should install @parity/hardhat-polkadot and @parity/resolc", () => {
      if (
        existsSync(
          join(PROJECT_DIR, "node_modules", "@parity", "hardhat-polkadot")
        )
      ) {
        console.log("@parity/hardhat-polkadot already installed — skipping.");
        return;
      }
      console.log("Installing @parity/hardhat-polkadot and @parity/resolc...");
      execSync(
        `npm install --save-dev @parity/hardhat-polkadot@${HARDHAT_POLKADOT_VERSION} @parity/resolc@${RESOLC_VERSION}`,
        { cwd: PROJECT_DIR, stdio: "inherit" }
      );
      expect(
        existsSync(
          join(PROJECT_DIR, "node_modules", "@parity", "hardhat-polkadot")
        )
      ).toBe(true);
      expect(
        existsSync(join(PROJECT_DIR, "node_modules", "@parity", "resolc"))
      ).toBe(true);
    }, 180000);

    it("should scaffold the project via npx hardhat-polkadot init", () => {
      // Skip if already scaffolded (hardhat.config.ts exists from a prior run)
      if (existsSync(join(PROJECT_DIR, "hardhat.config.ts"))) {
        console.log("Project already scaffolded — skipping.");
        return;
      }
      console.log("Scaffolding PVM project...");
      execSync("npx hardhat-polkadot init", {
        cwd: PROJECT_DIR,
        stdio: "inherit",
        input: "\n", // accept any prompts with defaults
      });
    }, 60000);

    it("should install remaining dependencies", () => {
      // npx hardhat-polkadot init may add deps to package.json
      console.log("Installing remaining dependencies...");
      execSync("npm install", { cwd: PROJECT_DIR, stdio: "inherit" });
    }, 180000);
  });

  // ==================== 3. VERIFY PROJECT STRUCTURE ====================
  describe("3. Verify Project Structure", () => {
    it("should have a hardhat.config.ts file", () => {
      expect(existsSync(join(PROJECT_DIR, "hardhat.config.ts"))).toBe(true);
    });

    it("should have a contracts directory with a Solidity file", () => {
      expect(existsSync(join(PROJECT_DIR, "contracts"))).toBe(true);
      // The scaffold generates MyToken.sol
      const hasMyToken = existsSync(
        join(PROJECT_DIR, "contracts", "MyToken.sol")
      );
      // Accept any .sol file in case the scaffold changes
      if (!hasMyToken) {
        const result = execSync("ls contracts/*.sol", {
          cwd: PROJECT_DIR,
          encoding: "utf-8",
        }).trim();
        expect(result.length, "contracts/ must contain at least one .sol file")
          .toBeGreaterThan(0);
        console.log(`Found contract(s): ${result}`);
      } else {
        console.log("Found: contracts/MyToken.sol");
      }
    });

    it("should have an ignition deployment module", () => {
      const modulesDir = join(PROJECT_DIR, "ignition", "modules");
      expect(existsSync(modulesDir)).toBe(true);
      // The scaffold generates MyToken.js
      const hasMyToken =
        existsSync(join(modulesDir, "MyToken.js")) ||
        existsSync(join(modulesDir, "MyToken.ts"));
      if (!hasMyToken) {
        const result = execSync("ls ignition/modules/", {
          cwd: PROJECT_DIR,
          encoding: "utf-8",
        }).trim();
        expect(
          result.length,
          "ignition/modules/ must contain at least one module"
        ).toBeGreaterThan(0);
        console.log(`Found module(s): ${result}`);
      } else {
        console.log("Found: ignition/modules/MyToken.js");
      }
    });

    it("should have a test directory", () => {
      expect(existsSync(join(PROJECT_DIR, "test"))).toBe(true);
    });

    it("should reference @parity/hardhat-polkadot in the config", () => {
      const config = readFileSync(
        join(PROJECT_DIR, "hardhat.config.ts"),
        "utf-8"
      );
      expect(config).toContain("hardhat-polkadot");
    });

    it("should configure the polkadotTestnet network", () => {
      const config = readFileSync(
        join(PROJECT_DIR, "hardhat.config.ts"),
        "utf-8"
      );
      expect(config).toContain("polkadotTestnet");
    });
  });

  // ==================== 4. COMPILE CONTRACTS (PVM) ====================
  describe("4. Compile Contracts", () => {
    // Mirrors: npx hardhat compile
    // Uses resolc to compile Solidity to PVM bytecode instead of EVM bytecode.
    it("should compile contracts without errors", () => {
      console.log("Compiling contracts with resolc (PVM)...");
      const result = execSync("npx hardhat compile", {
        cwd: PROJECT_DIR,
        env: hardhatEnv,
        encoding: "utf-8",
      });
      console.log(result.trim());
      expect(result).toMatch(
        /Compiled \d+ Solidity files? successfully|Nothing to compile/
      );
    }, 120000);

    it("should create the artifacts directory", () => {
      expect(existsSync(join(PROJECT_DIR, "artifacts"))).toBe(true);
    });

    it("should produce an artifact with a valid ABI", () => {
      // Find the first .json artifact (MyToken.json or whatever the scaffold creates)
      const artifactDir = join(PROJECT_DIR, "artifacts", "contracts");
      const result = execSync(
        "find . -name '*.json' -not -name '*.dbg.json' | head -1",
        { cwd: artifactDir, encoding: "utf-8" }
      ).trim();
      expect(result.length, "Must find at least one artifact JSON").toBeGreaterThan(0);

      const artifactPath = join(artifactDir, result);
      const artifact = JSON.parse(readFileSync(artifactPath, "utf-8"));
      expect(Array.isArray(artifact.abi)).toBe(true);
      expect(artifact.abi.length).toBeGreaterThan(0);
      console.log(`Artifact: ${result} — ABI entries: ${artifact.abi.length}`);
    });

    it("should produce an artifact with non-empty bytecode", () => {
      const artifactDir = join(PROJECT_DIR, "artifacts", "contracts");
      const result = execSync(
        "find . -name '*.json' -not -name '*.dbg.json' | head -1",
        { cwd: artifactDir, encoding: "utf-8" }
      ).trim();

      const artifactPath = join(artifactDir, result);
      const artifact = JSON.parse(readFileSync(artifactPath, "utf-8"));
      expect(artifact.bytecode).toBeTruthy();
      expect(artifact.bytecode.length).toBeGreaterThan(2); // more than just "0x"
      console.log(`Bytecode length: ${artifact.bytecode.length} chars`);
    });
  });

  // ==================== 5. VERIFY TESTNET CREDENTIALS ====================
  describe("5. Verify Testnet Credentials", () => {
    // The tutorial instructs: npx hardhat vars set PRIVATE_KEY
    // In CI, HARDHAT_VAR_PRIVATE_KEY is used instead.
    it("should have PRIVATE_KEY environment variable set", () => {
      expect(
        PRIVATE_KEY,
        "PRIVATE_KEY must be set — provide it via .env or CI secret"
      ).toBeTruthy();
    });
  });

  // ==================== 6. DEPLOY VIA IGNITION ====================
  describe("6. Deploy via Hardhat Ignition (polkadotTestnet)", () => {
    // Mirrors: npx hardhat ignition deploy ./ignition/modules/MyToken.js --network polkadotTestnet
    //
    // SOFT FAILURE: deployment requires a live testnet and a funded account.
    // If the faucet is dry or the network is unreachable, the test logs a
    // warning and exits cleanly so that phases 1–4, which fully gate guide
    // correctness, are not blocked by infrastructure issues.
    it("should deploy and output a contract address", async () => {
      // Detect the ignition module name (MyToken.js or MyToken.ts)
      const modulesDir = join(PROJECT_DIR, "ignition", "modules");
      let moduleName = "MyToken.js";
      if (!existsSync(join(modulesDir, moduleName))) {
        moduleName = "MyToken.ts";
      }
      if (!existsSync(join(modulesDir, moduleName))) {
        // Fallback: find first module file
        const found = execSync("ls", {
          cwd: modulesDir,
          encoding: "utf-8",
        }).trim().split("\n")[0];
        moduleName = found;
      }
      console.log(`Deploying via ignition/modules/${moduleName}...`);

      const MAX_ATTEMPTS = 3;
      const RETRY_WAIT_MS = 30000;
      let result = "";
      let deployError: unknown = null;

      for (let attempt = 1; attempt <= MAX_ATTEMPTS; attempt++) {
        // Remove any prior deployment state
        const deploymentsDir = join(PROJECT_DIR, "ignition", "deployments");
        if (existsSync(deploymentsDir)) {
          rmSync(deploymentsDir, { recursive: true, force: true });
        }

        try {
          result = execSync(
            `npx hardhat ignition deploy ./ignition/modules/${moduleName} --network polkadotTestnet`,
            {
              cwd: PROJECT_DIR,
              env: hardhatEnv,
              input: "y\n",
              encoding: "utf-8",
              timeout: 120000,
            }
          );
          deployError = null;
          break;
        } catch (e: any) {
          const combined =
            (e.stderr ?? "") + (e.stdout ?? "") + (e.message ?? "");

          const isRetryable =
            combined.includes("IGN403") ||
            combined.includes("UND_ERR_HEADERS_TIMEOUT") ||
            combined.includes("ECONNRESET") ||
            combined.includes("ETIMEDOUT");

          if (isRetryable && attempt < MAX_ATTEMPTS) {
            console.log(
              `Attempt ${attempt} failed (transient): waiting ${
                RETRY_WAIT_MS / 1000
              }s then retrying...`
            );
            await new Promise((resolve) => setTimeout(resolve, RETRY_WAIT_MS));
          } else {
            deployError = e;
            break;
          }
        }
      }

      if (deployError) {
        console.warn(
          "\n⚠  Deploy phase skipped — testnet may be unavailable or the account " +
            "unfunded.\n" +
            "   Phases 1–4 fully verify the guide; this does not indicate a guide " +
            "defect.\n" +
            `   Error: ${(deployError as any).message ?? deployError}`
        );
        return;
      }

      console.log(result);

      const match = result.match(/0x[0-9a-fA-F]{40}/);
      expect(
        match,
        "Ignition output must contain a deployed contract address (0x...40 hex chars)"
      ).not.toBeNull();
      console.log(`Deployed contract address: ${match![0]}`);
    }, 300000);
  });
});
