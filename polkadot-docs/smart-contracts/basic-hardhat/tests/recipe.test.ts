import { describe, it, expect } from "vitest";
import { execSync } from "child_process";
import { existsSync, readFileSync, mkdirSync, rmSync } from "fs";
import { join } from "path";

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

const REPO_URL =
  "https://github.com/polkadot-developers/revm-hardhat-examples.git";

// Update this SHA whenever upstream changes are intentionally pulled in.
const PINNED_COMMIT = "3c89476e52f19439978112a688a9086b27a63be1";

const WORKSPACE_DIR = join(process.cwd(), ".test-workspace");
const REPO_DIR      = join(WORKSPACE_DIR, "revm-hardhat-examples");
const PROJECT_DIR   = join(REPO_DIR, "basic-hardhat");
const ARTIFACT_PATH = join(
  PROJECT_DIR,
  "artifacts",
  "contracts",
  "Storage.sol",
  "Storage.json"
);

// ---------------------------------------------------------------------------
// Credentials
// ---------------------------------------------------------------------------

// The tutorial's hardhat.config.ts uses vars.get('PRIVATE_KEY').
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

describe("Deploy a Basic Contract with Hardhat Guide", () => {

  // ==================== 1. ENVIRONMENT PREREQUISITES ====================
  describe("1. Environment Prerequisites", () => {
    // The tutorial requires Node.js v22.13.1 or later.
    it("should have Node.js v22 or later", () => {
      const result = execSync("node --version", { encoding: "utf-8" }).trim();
      const major = parseInt(result.replace("v", "").split(".")[0], 10);
      expect(major).toBeGreaterThanOrEqual(22);
      console.log(`Node.js: ${result}`);
    });

    it("should have npm available", () => {
      const result = execSync("npm --version", { encoding: "utf-8" }).trim();
      expect(result).toMatch(/^\d+\.\d+/);
      console.log(`npm: ${result}`);
    });

    it("should have git available", () => {
      const result = execSync("git --version", { encoding: "utf-8" }).trim();
      expect(result).toMatch(/git version/);
      console.log(`git: ${result}`);
    });
  });

  // ==================== 2. CLONE REPOSITORY ====================
  describe("2. Clone Repository", () => {
    // Mirrors the tutorial step: git clone + navigate into the project.
    it("should clone revm-hardhat-examples at the pinned commit", () => {
      if (!existsSync(WORKSPACE_DIR)) {
        mkdirSync(WORKSPACE_DIR, { recursive: true });
      }

      // Reuse an existing clone if it is already at the right commit,
      // otherwise nuke and re-clone for a clean state.
      const ensurePinnedCommit = (): boolean => {
        try {
          execSync(`git fetch origin ${PINNED_COMMIT}`, {
            cwd: REPO_DIR,
            stdio: "pipe",
          });
          execSync(`git checkout ${PINNED_COMMIT}`, {
            cwd: REPO_DIR,
            stdio: "inherit",
          });
          return true;
        } catch {
          return false;
        }
      };

      if (existsSync(REPO_DIR)) {
        const isGitRepo = existsSync(join(REPO_DIR, ".git"));
        if (isGitRepo && ensurePinnedCommit()) {
          console.log("Repository already present — checked out pinned commit.");
        } else {
          console.log("Removing stale directory and cloning fresh...");
          rmSync(REPO_DIR, { recursive: true, force: true });
        }
      }

      if (!existsSync(REPO_DIR)) {
        console.log(`Cloning ${REPO_URL}...`);
        execSync(`git clone ${REPO_URL} ${REPO_DIR}`, { stdio: "inherit" });
        execSync(`git checkout ${PINNED_COMMIT}`, {
          cwd: REPO_DIR,
          stdio: "inherit",
        });
      }

      expect(existsSync(REPO_DIR)).toBe(true);
      console.log(`Checked out: ${PINNED_COMMIT}`);
    }, 60000);

    // Verify the tutorial project structure exists inside the cloned repo.
    it("should contain the basic-hardhat directory", () => {
      expect(existsSync(PROJECT_DIR)).toBe(true);
    });

    it("should have hardhat.config.ts", () => {
      expect(existsSync(join(PROJECT_DIR, "hardhat.config.ts"))).toBe(true);
    });

    it("should have package.json", () => {
      expect(existsSync(join(PROJECT_DIR, "package.json"))).toBe(true);
    });

    // The tutorial's core contract file.
    it("should have Storage.sol contract", () => {
      expect(
        existsSync(join(PROJECT_DIR, "contracts", "Storage.sol"))
      ).toBe(true);
    });

    // The tutorial's Ignition deployment module.
    it("should have the Ignition deployment module", () => {
      expect(
        existsSync(join(PROJECT_DIR, "ignition", "modules", "Storage.ts"))
      ).toBe(true);
    });
  });

  // ==================== 3. INSTALL DEPENDENCIES ====================
  describe("3. Install Dependencies", () => {
    // Mirrors: cd basic-hardhat && npm install
    it("should run npm install without errors", () => {
      console.log("Running npm install...");
      execSync("npm install", { cwd: PROJECT_DIR, stdio: "inherit" });
      expect(existsSync(join(PROJECT_DIR, "node_modules"))).toBe(true);
      console.log("Dependencies installed successfully.");
    }, 120000);

    it("should have Hardhat available after install", () => {
      const result = execSync("npx hardhat --version", {
        cwd: PROJECT_DIR,
        encoding: "utf-8",
      }).trim();
      expect(result).toMatch(/\d+\.\d+/);
      console.log(`Hardhat version: ${result}`);
    });
  });

  // ==================== 4. VERIFY TESTNET CREDENTIALS ====================
  describe("4. Verify Testnet Credentials", () => {
    // The tutorial uses vars.get('PRIVATE_KEY'). We surface the check here
    // so the error message is informative rather than a cryptic Hardhat failure.
    it("should have PRIVATE_KEY environment variable set", () => {
      expect(
        PRIVATE_KEY,
        "PRIVATE_KEY must be set — provide it via .env or CI secret"
      ).toBeTruthy();
    });
  });

  // ==================== 5. COMPILE CONTRACTS ====================
  describe("5. Compile Contracts", () => {
    // Mirrors: npx hardhat compile
    it("should compile Storage.sol without errors", () => {
      console.log("Compiling contracts...");
      // Inject HARDHAT_VAR_PRIVATE_KEY so the config resolves without prompts.
      const result = execSync("npx hardhat compile", {
        cwd: PROJECT_DIR,
        env: hardhatEnv,
        encoding: "utf-8",
      });
      console.log(result.trim());
      expect(result).toMatch(
        /Compiled \d+ Solidity files? successfully|Nothing to compile/
      );
    }, 60000);

    it("should create the artifacts directory", () => {
      expect(existsSync(join(PROJECT_DIR, "artifacts"))).toBe(true);
    });

    // The artifact file is Hardhat's compilation output for Storage.sol.
    it("should produce a Storage.json artifact", () => {
      expect(existsSync(ARTIFACT_PATH)).toBe(true);
    });

    it("should produce an artifact with a valid ABI", () => {
      const artifact = JSON.parse(readFileSync(ARTIFACT_PATH, "utf-8"));
      expect(Array.isArray(artifact.abi)).toBe(true);
      expect(artifact.abi.length).toBeGreaterThan(0);
      console.log(`ABI entries: ${artifact.abi.length}`);
    });

    // ABI must expose the two functions defined in the tutorial's Storage.sol.
    it("should expose a 'store' function in the ABI", () => {
      const artifact = JSON.parse(readFileSync(ARTIFACT_PATH, "utf-8"));
      const storeFn = artifact.abi.find(
        (entry: { type: string; name: string }) =>
          entry.type === "function" && entry.name === "store"
      );
      expect(storeFn, "ABI must contain a 'store' function").toBeDefined();
    });

    it("should expose a 'retrieve' function in the ABI", () => {
      const artifact = JSON.parse(readFileSync(ARTIFACT_PATH, "utf-8"));
      const retrieveFn = artifact.abi.find(
        (entry: { type: string; name: string }) =>
          entry.type === "function" && entry.name === "retrieve"
      );
      expect(retrieveFn, "ABI must contain a 'retrieve' function").toBeDefined();
    });

    it("should produce an artifact with non-empty bytecode", () => {
      const artifact = JSON.parse(readFileSync(ARTIFACT_PATH, "utf-8"));
      expect(artifact.bytecode).toMatch(/^0x[0-9a-fA-F]+$/);
      console.log(`Bytecode length: ${artifact.bytecode.length} chars`);
    });
  });

  // ==================== 6. DEPLOY VIA IGNITION ====================
  describe("6. Deploy via Hardhat Ignition (polkadotTestnet)", () => {
    // Mirrors: npx hardhat ignition deploy ignition/modules/Storage.ts --network polkadotTestnet
    it("should deploy Storage and output a contract address", async () => {
      console.log("Deploying Storage via Hardhat Ignition...");

      const MAX_ATTEMPTS  = 3;
      const RETRY_WAIT_MS = 30000; // 30 s between retries for transient RPC issues
      let result          = "";

      for (let attempt = 1; attempt <= MAX_ATTEMPTS; attempt++) {
        // Remove any prior deployment state so Ignition does not skip the deploy
        // and so only one confirmation prompt is issued.
        const deploymentsDir = join(PROJECT_DIR, "ignition", "deployments");
        if (existsSync(deploymentsDir)) {
          rmSync(deploymentsDir, { recursive: true, force: true });
        }

        try {
          result = execSync(
            "npx hardhat ignition deploy ./ignition/modules/Storage.ts --network polkadotTestnet",
            {
              cwd: PROJECT_DIR,
              env: hardhatEnv,
              input: "y\n", // confirm the network prompt non-interactively
              encoding: "utf-8",
              timeout: 120000, // 2 min — Storage is a trivial contract, deploys fast
            }
          );
          break; // deployment succeeded — exit retry loop
        } catch (e: any) {
          const combined =
            (e.stderr  ?? "") +
            (e.stdout  ?? "") +
            (e.message ?? "");

          // Retry only on well-known transient RPC / Ignition errors.
          const isRetryable =
            combined.includes("IGN403") ||                 // Ignition "already deployed" race
            combined.includes("UND_ERR_HEADERS_TIMEOUT") || // RPC timeout
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
            throw e; // non-retryable — surface the error immediately
          }
        }
      }

      console.log(result);

      // The deployment output must contain a valid EVM contract address.
      const match = result.match(/0x[0-9a-fA-F]{40}/);
      expect(
        match,
        "Ignition output must contain a deployed contract address (0x...40 hex chars)"
      ).not.toBeNull();
      console.log(`Deployed Storage contract address: ${match![0]}`);
    }, 300000);
  });
});
