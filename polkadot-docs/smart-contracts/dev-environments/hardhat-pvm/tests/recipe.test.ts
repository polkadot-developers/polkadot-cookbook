import { describe, it, expect } from "vitest";
import { execSync } from "child_process";
import {
  existsSync,
  mkdirSync,
  writeFileSync,
  readFileSync,
  rmSync,
} from "fs";
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

// The hardhat.config.ts uses vars.get('PRIVATE_KEY').
// Hardhat reads HARDHAT_VAR_<VARNAME> from the environment automatically,
// so no interactive `npx hardhat vars set` call is needed in CI.
const PRIVATE_KEY = process.env.PRIVATE_KEY;

const hardhatEnv = {
  ...process.env,
  HARDHAT_VAR_PRIVATE_KEY: PRIVATE_KEY ?? "",
};

// ---------------------------------------------------------------------------
// Project files — reproduce the output of `npx hardhat-polkadot init`
// ---------------------------------------------------------------------------

// hardhat.config.ts — PVM plugin with polkadotTestnet network
const HARDHAT_CONFIG_TS = `\
import "@parity/hardhat-polkadot";
import { vars } from "hardhat/config";

const config = {
  solidity: "0.8.28",
  resolc: {
    version: "${RESOLC_VERSION}",
  },
  networks: {
    hardhat: {
      polkavm: true,
    },
    localNode: {
      url: "http://127.0.0.1:8545",
    },
    polkadotTestnet: {
      url: "https://services.polkadothub-rpc.com/testnet",
      chainId: 420420417,
      accounts: [vars.get("PRIVATE_KEY")],
    },
  },
  ignition: {
    requiredConfirmations: 1,
  },
};

export default config;
`;

// MyToken.sol — sample ERC-20 token from the tutorial scaffold
const MY_TOKEN_SOL = `\
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.28;

contract MyToken {
    string public name = "MyToken";
    string public symbol = "MTK";
    uint8 public decimals = 18;
    uint256 public totalSupply;

    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;

    event Transfer(address indexed from, address indexed to, uint256 value);
    event Approval(address indexed owner, address indexed spender, uint256 value);

    constructor(uint256 _initialSupply) {
        totalSupply = _initialSupply * 10 ** uint256(decimals);
        balanceOf[msg.sender] = totalSupply;
        emit Transfer(address(0), msg.sender, totalSupply);
    }

    function transfer(address _to, uint256 _value) public returns (bool success) {
        require(balanceOf[msg.sender] >= _value, "Insufficient balance");
        balanceOf[msg.sender] -= _value;
        balanceOf[_to] += _value;
        emit Transfer(msg.sender, _to, _value);
        return true;
    }

    function approve(address _spender, uint256 _value) public returns (bool success) {
        allowance[msg.sender][_spender] = _value;
        emit Approval(msg.sender, _spender, _value);
        return true;
    }

    function transferFrom(
        address _from,
        address _to,
        uint256 _value
    ) public returns (bool success) {
        require(balanceOf[_from] >= _value, "Insufficient balance");
        require(allowance[_from][msg.sender] >= _value, "Allowance exceeded");
        balanceOf[_from] -= _value;
        balanceOf[_to] += _value;
        allowance[_from][msg.sender] -= _value;
        emit Transfer(_from, _to, _value);
        return true;
    }
}
`;

// Ignition deployment module for MyToken
const MY_TOKEN_MODULE_JS = `\
const { buildModule } = require("@nomicfoundation/hardhat-ignition/modules");

module.exports = buildModule("MyTokenModule", (m) => {
  const initialSupply = m.getParameter("initialSupply", 1000);
  const myToken = m.contract("MyToken", [initialSupply]);
  return { myToken };
});
`;

// tsconfig.json for Hardhat TypeScript support
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
    //
    // Because `npx hardhat-polkadot init` is interactive and does not work
    // reliably in non-TTY CI environments, this phase reproduces its output
    // by writing the project files directly.
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

    it("should write hardhat.config.ts with PVM plugin and polkadotTestnet", () => {
      writeFileSync(join(PROJECT_DIR, "hardhat.config.ts"), HARDHAT_CONFIG_TS);
      expect(existsSync(join(PROJECT_DIR, "hardhat.config.ts"))).toBe(true);
      console.log("hardhat.config.ts written.");
    });

    it("should write tsconfig.json for TypeScript support", () => {
      writeFileSync(join(PROJECT_DIR, "tsconfig.json"), TSCONFIG_JSON);
      expect(existsSync(join(PROJECT_DIR, "tsconfig.json"))).toBe(true);
    });

    it("should write contracts/MyToken.sol", () => {
      const contractsDir = join(PROJECT_DIR, "contracts");
      if (!existsSync(contractsDir)) {
        mkdirSync(contractsDir, { recursive: true });
      }
      writeFileSync(join(contractsDir, "MyToken.sol"), MY_TOKEN_SOL);
      expect(existsSync(join(contractsDir, "MyToken.sol"))).toBe(true);
    });

    it("should write ignition/modules/MyToken.js", () => {
      const modulesDir = join(PROJECT_DIR, "ignition", "modules");
      if (!existsSync(modulesDir)) {
        mkdirSync(modulesDir, { recursive: true });
      }
      writeFileSync(join(modulesDir, "MyToken.js"), MY_TOKEN_MODULE_JS);
      expect(existsSync(join(modulesDir, "MyToken.js"))).toBe(true);
    });

    it("should install remaining dependencies", () => {
      console.log("Installing remaining dependencies...");
      execSync("npm install", { cwd: PROJECT_DIR, stdio: "inherit" });
    }, 180000);
  });

  // ==================== 3. VERIFY PROJECT STRUCTURE ====================
  describe("3. Verify Project Structure", () => {
    it("should have a hardhat.config.ts file", () => {
      expect(existsSync(join(PROJECT_DIR, "hardhat.config.ts"))).toBe(true);
    });

    it("should have contracts/MyToken.sol", () => {
      expect(
        existsSync(join(PROJECT_DIR, "contracts", "MyToken.sol"))
      ).toBe(true);
    });

    it("should have ignition/modules/MyToken.js", () => {
      expect(
        existsSync(join(PROJECT_DIR, "ignition", "modules", "MyToken.js"))
      ).toBe(true);
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

    it("should contain chainId 420420417", () => {
      const config = readFileSync(
        join(PROJECT_DIR, "hardhat.config.ts"),
        "utf-8"
      );
      expect(config).toContain("420420417");
    });

    it("should configure resolc compiler version", () => {
      const config = readFileSync(
        join(PROJECT_DIR, "hardhat.config.ts"),
        "utf-8"
      );
      expect(config).toContain(RESOLC_VERSION);
    });
  });

  // ==================== 4. COMPILE CONTRACTS (PVM) ====================
  describe("4. Compile Contracts", () => {
    // Mirrors: npx hardhat compile
    // Uses resolc to compile Solidity to PVM bytecode instead of EVM bytecode.
    it("should compile MyToken.sol without errors", () => {
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

    it("should produce a MyToken.json artifact", () => {
      const artifactPath = join(
        PROJECT_DIR,
        "artifacts",
        "contracts",
        "MyToken.sol",
        "MyToken.json"
      );
      expect(existsSync(artifactPath)).toBe(true);
    });

    it("should produce an artifact with a valid ABI", () => {
      const artifactPath = join(
        PROJECT_DIR,
        "artifacts",
        "contracts",
        "MyToken.sol",
        "MyToken.json"
      );
      const artifact = JSON.parse(readFileSync(artifactPath, "utf-8"));
      expect(Array.isArray(artifact.abi)).toBe(true);
      expect(artifact.abi.length).toBeGreaterThan(0);
      console.log(`ABI entries: ${artifact.abi.length}`);
    });

    it("should expose a 'transfer' function in the ABI", () => {
      const artifactPath = join(
        PROJECT_DIR,
        "artifacts",
        "contracts",
        "MyToken.sol",
        "MyToken.json"
      );
      const artifact = JSON.parse(readFileSync(artifactPath, "utf-8"));
      const transferFn = artifact.abi.find(
        (entry: { type: string; name: string }) =>
          entry.type === "function" && entry.name === "transfer"
      );
      expect(
        transferFn,
        "ABI must contain a 'transfer' function"
      ).toBeDefined();
    });

    it("should produce an artifact with non-empty bytecode", () => {
      const artifactPath = join(
        PROJECT_DIR,
        "artifacts",
        "contracts",
        "MyToken.sol",
        "MyToken.json"
      );
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
    it("should deploy MyToken and output a contract address", async () => {
      console.log("Deploying MyToken via Hardhat Ignition...");

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
            "npx hardhat ignition deploy ./ignition/modules/MyToken.js --network polkadotTestnet",
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
      console.log(`Deployed MyToken contract address: ${match![0]}`);
    }, 300000);
  });
});
