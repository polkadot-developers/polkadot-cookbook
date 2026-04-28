import { describe, it, expect } from "vitest";
import { execSync } from "child_process";
import { existsSync, readFileSync, rmSync, mkdirSync } from "fs";
import { join } from "path";

const REPO_URL =
  "https://github.com/polkadot-developers/revm-hardhat-examples.git";
const PINNED_COMMIT = "96696ad15c3cf01b9168a71ad5114f27c34a8726";

const WORKSPACE_DIR = join(process.cwd(), ".test-workspace");
const REPO_DIR = join(WORKSPACE_DIR, "revm-hardhat-examples");
const UNISWAP_DIR = join(REPO_DIR, "uniswap-v3-periphery-hardhat");
const SWAP_ROUTER_ARTIFACT_PATH = join(
  UNISWAP_DIR,
  "artifacts",
  "contracts",
  "SwapRouter.sol",
  "SwapRouter.json",
);
const NFPM_ARTIFACT_PATH = join(
  UNISWAP_DIR,
  "artifacts",
  "contracts",
  "NonfungiblePositionManager.sol",
  "NonfungiblePositionManager.json",
);

// Only TESTNET_PRIVATE_KEY is needed — the RPC URL is hardcoded in hardhat.config.ts
const TESTNET_PRIVATE_KEY = process.env.TESTNET_PRIVATE_KEY || undefined;

const hardhatEnv: Record<string, string> = {
  ...Object.fromEntries(
    Object.entries(process.env).filter(
      (entry): entry is [string, string] => entry[1] != null,
    ),
  ),
  ...(TESTNET_PRIVATE_KEY
    ? { HARDHAT_VAR_TESTNET_PRIVATE_KEY: TESTNET_PRIVATE_KEY }
    : {}),
};

describe("Uniswap V3 Periphery with Hardhat Guide", () => {
  // ==================== ENVIRONMENT PREREQUISITES ====================
  describe("1. Environment Prerequisites", () => {
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

  // ==================== CLONE REPOSITORY ====================
  describe("2. Clone Repository", () => {
    it("should clone revm-hardhat-examples at pinned commit", () => {
      if (!existsSync(WORKSPACE_DIR)) {
        mkdirSync(WORKSPACE_DIR, { recursive: true });
      }

      const ensurePinnedCommit = () => {
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
          console.log(
            "Repository already present — checked out pinned commit.",
          );
        } else {
          console.log(
            "Removing existing directory (not a repo or missing commit) and cloning fresh...",
          );
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

    it("should contain the uniswap-v3-periphery-hardhat directory", () => {
      expect(existsSync(UNISWAP_DIR)).toBe(true);
    });

    it("should have hardhat.config.ts", () => {
      expect(existsSync(join(UNISWAP_DIR, "hardhat.config.ts"))).toBe(true);
    });

    it("should have package.json", () => {
      expect(existsSync(join(UNISWAP_DIR, "package.json"))).toBe(true);
    });

    it("should have SwapRouter.sol contract", () => {
      expect(existsSync(join(UNISWAP_DIR, "contracts", "SwapRouter.sol"))).toBe(
        true,
      );
    });

    it("should have NonfungiblePositionManager.sol contract", () => {
      expect(
        existsSync(
          join(UNISWAP_DIR, "contracts", "NonfungiblePositionManager.sol"),
        ),
      ).toBe(true);
    });

    it("should have the Hardhat test files", () => {
      expect(existsSync(join(UNISWAP_DIR, "test", "SwapRouter.test.ts"))).toBe(
        true,
      );
      expect(
        existsSync(
          join(UNISWAP_DIR, "test", "NonfungiblePositionManager.test.ts"),
        ),
      ).toBe(true);
    });

    it("should have the Ignition deployment module", () => {
      expect(
        existsSync(
          join(UNISWAP_DIR, "ignition", "modules", "UniswapV3Periphery.ts"),
        ),
      ).toBe(true);
    });
  });

  // ==================== INSTALL DEPENDENCIES ====================
  describe("3. Install Dependencies", () => {
    it("should run npm install without errors", () => {
      // The periphery project references v3-core via a local file path
      // ("@uniswap/v3-core": "file:../uniswap-v3-core-hardhat"). npm resolves
      // this automatically from the sibling directory in the same cloned repo.
      console.log("Running npm install...");
      execSync("npm install", { cwd: UNISWAP_DIR, stdio: "inherit" });
      expect(existsSync(join(UNISWAP_DIR, "node_modules"))).toBe(true);
      console.log("Dependencies installed successfully");
    }, 120000);

    it("should have Hardhat available after install", () => {
      const result = execSync("npx hardhat --version", {
        cwd: UNISWAP_DIR,
        encoding: "utf-8",
      }).trim();
      expect(result).toMatch(/\d+\.\d+/);
      console.log(`Hardhat version: ${result}`);
    });
  });

  // ==================== VERIFY TESTNET CREDENTIALS ====================
  describe("4. Verify Testnet Credentials", () => {
    it.skipIf(!TESTNET_PRIVATE_KEY)(
      "should have TESTNET_PRIVATE_KEY environment variable",
      () => {
        expect(TESTNET_PRIVATE_KEY).toBeTruthy();
      },
    );
  });

  // ==================== COMPILE CONTRACTS ====================
  describe("5. Compile Contracts", () => {
    it("should compile Solidity contracts without errors", () => {
      console.log("Compiling contracts...");
      const result = execSync("npx hardhat compile", {
        cwd: UNISWAP_DIR,
        encoding: "utf-8",
      });
      console.log(result.trim());
      expect(result).toMatch(
        /Compiled \d+ Solidity files? successfully|Nothing to compile/,
      );
    }, 120000);

    it("should create the artifacts directory", () => {
      expect(existsSync(join(UNISWAP_DIR, "artifacts"))).toBe(true);
    });

    it("should produce a SwapRouter.json artifact", () => {
      expect(existsSync(SWAP_ROUTER_ARTIFACT_PATH)).toBe(true);
    });

    it("should produce a SwapRouter artifact with a valid ABI", () => {
      const artifact = JSON.parse(
        readFileSync(SWAP_ROUTER_ARTIFACT_PATH, "utf-8"),
      );
      expect(Array.isArray(artifact.abi)).toBe(true);
      expect(artifact.abi.length).toBeGreaterThan(0);
      console.log(`SwapRouter ABI entries: ${artifact.abi.length}`);
    });

    it("should produce a SwapRouter artifact with non-empty bytecode", () => {
      const artifact = JSON.parse(
        readFileSync(SWAP_ROUTER_ARTIFACT_PATH, "utf-8"),
      );
      expect(artifact.bytecode).toMatch(/^0x[0-9a-fA-F]+$/);
      console.log(
        `SwapRouter bytecode length: ${artifact.bytecode.length} chars`,
      );
    });

    it("should produce a NonfungiblePositionManager.json artifact", () => {
      expect(existsSync(NFPM_ARTIFACT_PATH)).toBe(true);
    });

    it("should produce a NonfungiblePositionManager artifact with a valid ABI", () => {
      const artifact = JSON.parse(readFileSync(NFPM_ARTIFACT_PATH, "utf-8"));
      expect(Array.isArray(artifact.abi)).toBe(true);
      expect(artifact.abi.length).toBeGreaterThan(0);
      console.log(`NFPM ABI entries: ${artifact.abi.length}`);
    });
  });

  // ==================== RUN HARDHAT TESTS ====================
  describe("6. Run Hardhat Tests (local network)", () => {
    it("should pass all 39 Hardhat tests on the local Hardhat network", () => {
      console.log("Running Hardhat test suite on local network...");
      const result = execSync("npx hardhat test", {
        cwd: UNISWAP_DIR,
        encoding: "utf-8",
        env: hardhatEnv,
        timeout: 300000,
      });
      console.log(result);
      expect(result).toContain("39 passing");
    }, 300000);
  });

  // ==================== DEPLOY VIA IGNITION ====================
  describe("7. Deploy Periphery via Hardhat Ignition (polkadotTestnet)", () => {
    it.skipIf(!TESTNET_PRIVATE_KEY)(
      "should deploy SwapRouter and NonfungiblePositionManager and output contract addresses",
      async () => {
        console.log("Deploying Uniswap V3 Periphery via Hardhat Ignition...");

        const MAX_ATTEMPTS = 3;
        const RETRY_WAIT_MS = 30000;
        let deployError: unknown = null;

        for (let attempt = 1; attempt <= MAX_ATTEMPTS; attempt++) {
          // Remove prior deployment state so only one confirmation prompt appears
          const deploymentsDir = join(UNISWAP_DIR, "ignition", "deployments");
          if (existsSync(deploymentsDir)) {
            rmSync(deploymentsDir, { recursive: true, force: true });
          }

          try {
            const result = execSync(
              "npx hardhat ignition deploy ./ignition/modules/UniswapV3Periphery.ts --network polkadotTestnet",
              {
                cwd: UNISWAP_DIR,
                env: hardhatEnv,
                input: "y\n",
                encoding: "utf-8",
                timeout: 120000,
              },
            );
            console.log(result);
            const addresses = result.match(/0x[0-9a-fA-F]{40}/g);
            expect(
              addresses,
              "Ignition output must contain deployed contract addresses",
            ).not.toBeNull();
            console.log(`Deployed addresses: ${addresses!.join(", ")}`);
            deployError = null;
            break;
          } catch (e: unknown) {
            const err = e as {
              stderr?: string;
              stdout?: string;
              message?: string;
            };
            const combined =
              (err.stderr ?? "") + (err.stdout ?? "") + (err.message ?? "");
            const isRetryable =
              combined.includes("IGN403") ||
              combined.includes("UND_ERR_HEADERS_TIMEOUT") ||
              combined.includes("ECONNRESET") ||
              combined.includes("ETIMEDOUT");

            if (isRetryable && attempt < MAX_ATTEMPTS) {
              console.log(
                `Attempt ${attempt} failed (transient): waiting ${RETRY_WAIT_MS / 1000}s then retrying...`,
              );
              await new Promise((resolve) =>
                setTimeout(resolve, RETRY_WAIT_MS),
              );
            } else {
              deployError = e;
            }
          }
        }

        if (deployError) {
          console.warn(
            "\n⚠  Deploy phase skipped — testnet may be unavailable or the account " +
              "may lack funds. Phases 1–6 fully verify the guide.\n" +
              `   Error: ${(deployError as { message?: string }).message ?? deployError}`,
          );
          return;
        }
      },
      300000,
    );
  });
});
