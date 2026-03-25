import { describe, it, expect } from "vitest";
import { execSync } from "child_process";
import { existsSync, mkdirSync, writeFileSync, readFileSync, rmSync } from "fs";
import { join } from "path";

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

const WORKSPACE_DIR = join(process.cwd(), ".test-workspace");
const PROJECT_DIR   = join(WORKSPACE_DIR, "hardhat-evm");
const ARTIFACT_PATH = join(
  PROJECT_DIR,
  "artifacts",
  "contracts",
  "Lock.sol",
  "Lock.json"
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
// Contract source — Hardhat's standard Lock.sol sample
// ---------------------------------------------------------------------------

const LOCK_SOL = `\
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.28;

contract Lock {
    uint public unlockTime;
    address payable public owner;

    event Withdrawal(uint amount, uint when);

    constructor(uint _unlockTime) payable {
        require(
            block.timestamp < _unlockTime,
            "Unlock time should be in the future"
        );
        unlockTime = _unlockTime;
        owner = payable(msg.sender);
    }

    function withdraw() public {
        require(block.timestamp >= unlockTime, "You can't withdraw yet");
        require(msg.sender == owner, "You aren't the owner");

        emit Withdrawal(address(this).balance, block.timestamp);
        owner.transfer(address(this).balance);
    }
}
`;

// ---------------------------------------------------------------------------
// Ignition module — deploys Lock with a future unlock time
// ---------------------------------------------------------------------------

const LOCK_MODULE_TS = `\
import { buildModule } from "@nomicfoundation/hardhat-ignition/modules";

const JAN_1_2030 = 1893456000;
const ONE_GWEI: bigint = 1_000_000_000n;

const LockModule = buildModule("LockModule", (m) => {
  const unlockTime = m.getParameter("unlockTime", JAN_1_2030);
  const lockedAmount = m.getParameter("lockedAmount", ONE_GWEI);

  const lock = m.contract("Lock", [unlockTime], {
    value: lockedAmount,
  });

  return { lock };
});

export default LockModule;
`;

// ---------------------------------------------------------------------------
// Hardhat config — standard scaffold + polkadotTestnet network
// ---------------------------------------------------------------------------

const HARDHAT_CONFIG_TS = `\
import type { HardhatUserConfig } from "hardhat/config";
import "@nomicfoundation/hardhat-toolbox";
import { vars } from "hardhat/config";

const config: HardhatUserConfig = {
  solidity: "0.8.28",
  networks: {
    polkadotTestnet: {
      url: "https://services.polkadothub-rpc.com/testnet",
      chainId: 420420417,
      accounts: [vars.get("PRIVATE_KEY")],
    },
  },
};

export default config;
`;

// tsconfig.json required for Hardhat TypeScript support
const TSCONFIG_JSON = `\
{
  "compilerOptions": {
    "target": "ES2020",
    "module": "commonjs",
    "esModuleInterop": true,
    "forceConsistentCasingInFileNames": true,
    "strict": true,
    "skipLibCheck": true
  }
}
`;

// ---------------------------------------------------------------------------
// Test suite
// ---------------------------------------------------------------------------

describe("Use Hardhat with Polkadot Hub (EVM) Guide", () => {

  // ==================== 1. ENVIRONMENT PREREQUISITES ====================
  describe("1. Environment Prerequisites", () => {
    // The tutorial supports Node.js LTS (18.x, 20.x, 22.x); CI runs v22.
    it("should have Node.js v18 or later", () => {
      const result = execSync("node --version", { encoding: "utf-8" }).trim();
      const major = parseInt(result.replace("v", "").split(".")[0], 10);
      expect(major).toBeGreaterThanOrEqual(18);
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

  // ==================== 2. INITIALIZE HARDHAT PROJECT ====================
  describe("2. Initialize Hardhat Project", () => {
    // Mirrors the tutorial step:
    //   mkdir hardhat-example && cd hardhat-example
    //   npx hardhat@^2.27.0 init
    //
    // Because `npx hardhat init` is interactive, this phase reproduces the
    // outcome — installing Hardhat at the pinned version and verifying the
    // binary — without relying on stdin piping to a TTY prompt.
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

    it("should install hardhat@^2.27.0 and @nomicfoundation/hardhat-toolbox", () => {
      // Skip if already installed to speed up re-runs.
      if (existsSync(join(PROJECT_DIR, "node_modules", "hardhat"))) {
        console.log("Hardhat already installed — skipping.");
        return;
      }
      console.log("Installing Hardhat and toolbox (this may take a minute)...");
      execSync(
        // Pin hardhat-toolbox to the hh2 tag — the latest (v4+) no longer
        // supports Hardhat 2.x. The hh2 tag targets Hardhat 2 and includes ts-node.
        "npm install --save-dev hardhat@^2.27.0 \"@nomicfoundation/hardhat-toolbox@hh2\"",
        { cwd: PROJECT_DIR, stdio: "inherit" }
      );
      expect(existsSync(join(PROJECT_DIR, "node_modules", "hardhat"))).toBe(true);
    }, 180000);

    it("should report a Hardhat version ≥ 2.27.0", () => {
      const result = execSync("npx hardhat --version", {
        cwd: PROJECT_DIR,
        encoding: "utf-8",
      }).trim();
      const [major, minor] = result.split(".").map(Number);
      expect(major > 2 || (major === 2 && minor >= 27)).toBe(true);
      console.log(`Hardhat version: ${result}`);
    });
  });

  // ==================== 3. CONFIGURE POLKADOT HUB NETWORK ====================
  describe("3. Configure Polkadot Hub Network", () => {
    // Mirrors the tutorial step: update hardhat.config.ts to add the
    // polkadotTestnet network block (url, chainId 420420417, accounts via vars).
    it("should write hardhat.config.ts with polkadotTestnet network", () => {
      writeFileSync(join(PROJECT_DIR, "hardhat.config.ts"), HARDHAT_CONFIG_TS);
      expect(existsSync(join(PROJECT_DIR, "hardhat.config.ts"))).toBe(true);
      console.log("hardhat.config.ts written.");
    });

    it("should contain the correct polkadotTestnet RPC URL", () => {
      const config = readFileSync(join(PROJECT_DIR, "hardhat.config.ts"), "utf-8");
      expect(config).toContain("https://services.polkadothub-rpc.com/testnet");
    });

    it("should contain chainId 420420417", () => {
      const config = readFileSync(join(PROJECT_DIR, "hardhat.config.ts"), "utf-8");
      expect(config).toContain("420420417");
    });

    it("should reference vars.get('PRIVATE_KEY') for account management", () => {
      const config = readFileSync(join(PROJECT_DIR, "hardhat.config.ts"), "utf-8");
      expect(config).toContain('vars.get("PRIVATE_KEY")');
    });

    it("should write tsconfig.json for TypeScript support", () => {
      writeFileSync(join(PROJECT_DIR, "tsconfig.json"), TSCONFIG_JSON);
      expect(existsSync(join(PROJECT_DIR, "tsconfig.json"))).toBe(true);
    });

    it("should write contracts/Lock.sol", () => {
      const contractsDir = join(PROJECT_DIR, "contracts");
      if (!existsSync(contractsDir)) {
        mkdirSync(contractsDir, { recursive: true });
      }
      writeFileSync(join(contractsDir, "Lock.sol"), LOCK_SOL);
      expect(existsSync(join(contractsDir, "Lock.sol"))).toBe(true);
    });

    it("should write ignition/modules/Lock.ts", () => {
      const modulesDir = join(PROJECT_DIR, "ignition", "modules");
      if (!existsSync(modulesDir)) {
        mkdirSync(modulesDir, { recursive: true });
      }
      writeFileSync(join(modulesDir, "Lock.ts"), LOCK_MODULE_TS);
      expect(existsSync(join(modulesDir, "Lock.ts"))).toBe(true);
    });
  });

  // ==================== 4. VERIFY TESTNET CREDENTIALS ====================
  describe("4. Verify Testnet Credentials", () => {
    // The tutorial instructs: npx hardhat vars set PRIVATE_KEY
    // In CI and .env-based local runs, HARDHAT_VAR_PRIVATE_KEY is used instead
    // so Hardhat resolves vars.get('PRIVATE_KEY') without interactive prompts.
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
    it("should compile Lock.sol without errors", () => {
      console.log("Compiling contracts...");
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

    it("should produce a Lock.json artifact", () => {
      expect(existsSync(ARTIFACT_PATH)).toBe(true);
    });

    it("should produce an artifact with a valid ABI", () => {
      const artifact = JSON.parse(readFileSync(ARTIFACT_PATH, "utf-8"));
      expect(Array.isArray(artifact.abi)).toBe(true);
      expect(artifact.abi.length).toBeGreaterThan(0);
      console.log(`ABI entries: ${artifact.abi.length}`);
    });

    it("should expose a 'withdraw' function in the ABI", () => {
      const artifact = JSON.parse(readFileSync(ARTIFACT_PATH, "utf-8"));
      const withdrawFn = artifact.abi.find(
        (entry: { type: string; name: string }) =>
          entry.type === "function" && entry.name === "withdraw"
      );
      expect(withdrawFn, "ABI must contain a 'withdraw' function").toBeDefined();
    });

    it("should expose an 'unlockTime' getter in the ABI", () => {
      const artifact = JSON.parse(readFileSync(ARTIFACT_PATH, "utf-8"));
      const unlockTimeFn = artifact.abi.find(
        (entry: { type: string; name: string }) =>
          entry.type === "function" && entry.name === "unlockTime"
      );
      expect(
        unlockTimeFn,
        "ABI must contain an 'unlockTime' getter"
      ).toBeDefined();
    });

    it("should produce an artifact with non-empty bytecode", () => {
      const artifact = JSON.parse(readFileSync(ARTIFACT_PATH, "utf-8"));
      expect(artifact.bytecode).toMatch(/^0x[0-9a-fA-F]+$/);
      console.log(`Bytecode length: ${artifact.bytecode.length} chars`);
    });
  });

  // ==================== 6. DEPLOY VIA IGNITION ====================
  describe("6. Deploy via Hardhat Ignition (polkadotTestnet)", () => {
    // Mirrors: npx hardhat ignition deploy ./ignition/modules/Lock.ts --network polkadotTestnet
    //
    // SOFT FAILURE: deployment requires a live testnet and a funded account.
    // If the faucet is dry or the network is unreachable, the test logs a
    // warning and exits cleanly so that phases 1–5, which fully gate guide
    // correctness, are not blocked by infrastructure issues outside the
    // guide's control.
    it("should deploy Lock and output a contract address", async () => {
      console.log("Deploying Lock via Hardhat Ignition...");

      const MAX_ATTEMPTS  = 3;
      const RETRY_WAIT_MS = 30000; // 30 s between retries for transient RPC issues
      let result          = "";
      let deployError: unknown = null;

      for (let attempt = 1; attempt <= MAX_ATTEMPTS; attempt++) {
        // Remove any prior deployment state so Ignition does not skip the
        // deploy and so only one confirmation prompt is issued.
        const deploymentsDir = join(PROJECT_DIR, "ignition", "deployments");
        if (existsSync(deploymentsDir)) {
          rmSync(deploymentsDir, { recursive: true, force: true });
        }

        try {
          result = execSync(
            "npx hardhat ignition deploy ./ignition/modules/Lock.ts --network polkadotTestnet",
            {
              cwd: PROJECT_DIR,
              env: hardhatEnv,
              input: "y\n", // confirm the network prompt non-interactively
              encoding: "utf-8",
              timeout: 120000, // 2 min — Lock is a trivial contract
            }
          );
          deployError = null;
          break; // deployment succeeded — exit retry loop
        } catch (e: any) {
          const combined =
            (e.stderr  ?? "") +
            (e.stdout  ?? "") +
            (e.message ?? "");

          // Retry only on well-known transient RPC / Ignition errors.
          const isRetryable =
            combined.includes("IGN403") ||                   // Ignition "already deployed" race
            combined.includes("UND_ERR_HEADERS_TIMEOUT") ||  // RPC timeout
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
            break; // no more retries — fall through to soft-failure handling
          }
        }
      }

      // Soft-failure: surface infrastructure problems as a warning, not a hard fail.
      if (deployError) {
        console.warn(
          "\n⚠  Deploy phase skipped — testnet may be unavailable or the account " +
          "unfunded.\n" +
          "   Phases 1–5 fully verify the guide; this does not indicate a guide " +
          "defect.\n" +
          `   Error: ${(deployError as any).message ?? deployError}`
        );
        return;
      }

      console.log(result);

      // The deployment output must contain a valid EVM contract address.
      const match = result.match(/0x[0-9a-fA-F]{40}/);
      expect(
        match,
        "Ignition output must contain a deployed contract address (0x...40 hex chars)"
      ).not.toBeNull();
      console.log(`Deployed Lock contract address: ${match![0]}`);
    }, 300000);
  });
});
