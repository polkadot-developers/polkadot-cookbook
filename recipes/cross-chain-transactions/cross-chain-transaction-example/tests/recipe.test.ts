import { describe, it, expect, afterAll } from "vitest";
import { execSync, spawn, ChildProcess } from "child_process";
import { existsSync, readFileSync } from "fs";
import { join } from "path";

const PROJECT_DIR = process.cwd();
const REPO_URL = "https://github.com/brunopgalvao/recipe-xcm-example";
const REPO_VERSION = "v1.1.0";
const REPO_DIR = join(PROJECT_DIR, "recipe-xcm-example");

let chopsticksProcess: ChildProcess | null = null;

describe("Cross-Chain Transaction Example Recipe", () => {
  afterAll(async () => {
    await stopChopsticks();
  });

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
        console.log(`Cloning recipe-xcm-example ${REPO_VERSION}...`);
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

  // ==================== START CHOPSTICKS ====================
  describe("4. Start Chopsticks", () => {
    it("should start Chopsticks for local multi-chain testing", async () => {
      console.log("Starting Chopsticks...");

      chopsticksProcess = spawn("npm", ["run", "chopsticks"], {
        cwd: REPO_DIR,
        stdio: ["ignore", "pipe", "pipe"],
        detached: true,
      });

      // Wait for Chopsticks to be ready (both relay chain and parachain)
      const maxWaitTime = 300000; // 5 minutes
      let rpcCount = 0;
      const requiredRpcCount = 2; // relay chain + parachain

      await new Promise<void>((resolve, reject) => {
        const timeout = setTimeout(() => {
          reject(new Error(`Chopsticks failed to start within 5 minutes (${rpcCount}/${requiredRpcCount} chains ready)`));
        }, maxWaitTime);

        const checkOutput = (output: string) => {
          const matches = output.match(/RPC listening/g);
          if (matches) {
            rpcCount += matches.length;
            if (rpcCount >= requiredRpcCount) {
              clearTimeout(timeout);
              console.log("Chopsticks is ready (both chains up)!");
              resolve();
            }
          }
        };

        chopsticksProcess!.stdout?.on("data", (data) => {
          const output = data.toString();
          console.log(`[chopsticks:stdout] ${output.trim()}`);
          checkOutput(output);
        });

        chopsticksProcess!.stderr?.on("data", (data) => {
          const output = data.toString();
          console.log(`[chopsticks:stderr] ${output.trim()}`);
          checkOutput(output);
        });
      });

      expect(rpcCount).toBeGreaterThanOrEqual(requiredRpcCount);
    }, 360000);
  });

  // ==================== TEST ====================
  describe("5. Run Tests", () => {
    it("should pass all XCM tests", () => {
      console.log("Running XCM tests...");
      execSync("npm test", {
        cwd: REPO_DIR,
        encoding: "utf-8",
        stdio: "inherit",
      });
      console.log("All tests passed");
    }, 120000);
  });
});

async function stopChopsticks(): Promise<void> {
  console.log("Stopping Chopsticks...");

  if (chopsticksProcess && !chopsticksProcess.killed) {
    try {
      process.kill(-chopsticksProcess.pid!, "SIGTERM");
    } catch {
      chopsticksProcess.kill("SIGTERM");
    }
    chopsticksProcess = null;
  }

  // Kill any lingering chopsticks processes
  try {
    execSync("pkill -f chopsticks 2>/dev/null || true", { encoding: "utf-8" });
  } catch {
    // Ignore errors
  }

  await new Promise((resolve) => setTimeout(resolve, 2000));
  console.log("Chopsticks stopped");
}
