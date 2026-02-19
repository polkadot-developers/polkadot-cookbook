import { describe, it, expect, afterAll } from "vitest";
import { execSync, spawn, ChildProcess } from "child_process";
import {
  existsSync,
  readFileSync,
  writeFileSync,
  unlinkSync,
  mkdirSync,
  statSync,
} from "fs";
import { join } from "path";
import { ApiPromise, WsProvider } from "@polkadot/api";
import { Keyring } from "@polkadot/keyring";
import { bnToU8a, u8aToHex, stringToU8a } from "@polkadot/util";

const PROJECT_DIR = process.cwd();
const WORKSPACE_DIR = join(PROJECT_DIR, ".test-workspace");
const TEMPLATE_DIR = join(WORKSPACE_DIR, "parachain-template");
const TEMPLATE_VERSION = process.env.TEMPLATE_VERSION!;
const POLKADOT_SDK_VERSION = process.env.POLKADOT_SDK_VERSION!;
const BIN_DIR = join(WORKSPACE_DIR, "bin");
const CHAIN_SPEC_1000 = join(WORKSPACE_DIR, "chain_spec_1000.json");
const CHAIN_SPEC_1001 = join(WORKSPACE_DIR, "chain_spec_1001.json");
const PID_FILE = join(WORKSPACE_DIR, "zombienet.pid");
const WASM_PATH = join(
  TEMPLATE_DIR,
  "target/release/wbuild/parachain-template-runtime/parachain_template_runtime.compact.compressed.wasm"
);
const POLKADOT_BINARY = join(BIN_DIR, "polkadot");
const POLKADOT_PREPARE_WORKER = join(BIN_DIR, "polkadot-prepare-worker");
const POLKADOT_EXECUTE_WORKER = join(BIN_DIR, "polkadot-execute-worker");

// RPC ports
const RELAY_RPC_PORT = 9944;
const PARA_A_RPC_PORT = 9988;
const PARA_B_RPC_PORT = 9989;

const RELAY_RPC_URL = `http://127.0.0.1:${RELAY_RPC_PORT}`;
const RELAY_WS_URL = `ws://127.0.0.1:${RELAY_RPC_PORT}`;
const PARA_A_RPC_URL = `http://127.0.0.1:${PARA_A_RPC_PORT}`;
const PARA_A_WS_URL = `ws://127.0.0.1:${PARA_A_RPC_PORT}`;
const PARA_B_RPC_URL = `http://127.0.0.1:${PARA_B_RPC_PORT}`;
const PARA_B_WS_URL = `ws://127.0.0.1:${PARA_B_RPC_PORT}`;

let zombienetProcess: ChildProcess | null = null;

/** Helper: JSON-RPC call to a given node */
async function rpcCall(url: string, method: string, params: unknown[] = []): Promise<unknown> {
  const response = await fetch(url, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ jsonrpc: "2.0", method, params, id: 1 }),
  });
  const json = (await response.json()) as { result?: unknown; error?: unknown };
  if (json.error) throw new Error(`RPC error: ${JSON.stringify(json.error)}`);
  return json.result;
}

/**
 * Calculate the sovereign account for a parachain on the relay chain.
 * Format: prefix "para" (0x70617261) + LE-encoded u32 paraId + zero-pad to 32 bytes.
 */
function sovereignAccount(paraId: number): string {
  const prefix = stringToU8a("para");
  const idBytes = bnToU8a(paraId, { bitLength: 32, isLe: true });
  const account = new Uint8Array(32);
  account.set(prefix, 0);
  account.set(idBytes, prefix.length);
  return u8aToHex(account);
}

describe("Channels Between Parachains Tutorial", () => {
  afterAll(async () => {
    await stopZombienet();
  });

  // ==================== ENVIRONMENT TESTS ====================
  describe("1. Environment Prerequisites", () => {
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
      } catch {
        console.log("Installing chain-spec-builder...");
        execSync(`cargo install staging-chain-spec-builder@${process.env.CHAIN_SPEC_BUILDER_VERSION} --locked`, {
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
      } catch {
        console.log("Installing polkadot-omni-node...");
        execSync(`cargo install polkadot-omni-node@${process.env.POLKADOT_OMNI_NODE_VERSION} --locked`, {
          stdio: "inherit",
        });
      }
    });

    it("should have Zombienet installed", () => {
      const result = execSync("zombienet version 2>&1 || zombienet --version 2>&1", {
        encoding: "utf-8",
      });
      expect(result.length).toBeGreaterThan(0);
      console.log(`Zombienet: ${result.trim()}`);
    });

    it("should have git installed", () => {
      const result = execSync("git --version", { encoding: "utf-8" });
      expect(result).toMatch(/git version/);
      console.log(`Git: ${result.trim()}`);
    });
  });

  // ==================== BUILD RUNTIME ====================
  describe("2. Build Runtime", () => {
    it("should create workspace directory", () => {
      if (!existsSync(WORKSPACE_DIR)) {
        mkdirSync(WORKSPACE_DIR, { recursive: true });
      }
      expect(existsSync(WORKSPACE_DIR)).toBe(true);
    });

    it("should clone the parachain template repository", () => {
      if (existsSync(TEMPLATE_DIR)) {
        console.log(`Template already cloned, resetting to ${TEMPLATE_VERSION}...`);
        execSync(`git checkout -- . && git clean -fd -e target/ && git fetch --tags && git checkout ${TEMPLATE_VERSION}`, { cwd: TEMPLATE_DIR, encoding: "utf-8" });
      } else {
        console.log(`Cloning polkadot-sdk-parachain-template ${TEMPLATE_VERSION}...`);
        execSync(
          `git clone --branch ${TEMPLATE_VERSION} https://github.com/paritytech/polkadot-sdk-parachain-template.git ${TEMPLATE_DIR}`,
          { encoding: "utf-8", stdio: "inherit" }
        );
      }

      expect(existsSync(join(TEMPLATE_DIR, "Cargo.toml"))).toBe(true);
      console.log("Repository cloned successfully");
    }, 120000);

    it("should build the runtime", () => {
      console.log("Building parachain template runtime (this may take 15-30 minutes)...");
      execSync("cargo build --release", {
        cwd: TEMPLATE_DIR,
        encoding: "utf-8",
        stdio: "inherit",
        timeout: 1800000,
      });
      expect(existsSync(WASM_PATH)).toBe(true);
      const stats = statSync(WASM_PATH);
      const sizeKB = Math.round(stats.size / 1024);
      console.log(`WASM runtime size: ${sizeKB} KB`);
      expect(stats.size).toBeGreaterThan(100000);
    }, 1800000);

    it("should generate chain specs for both parachains", () => {
      // Generate chain spec for para ID 1000
      console.log("Generating chain spec for parachain 1000...");
      execSync(
        `chain-spec-builder create -t development \
          --relay-chain rococo-local \
          --para-id 1000 \
          --runtime ${WASM_PATH} \
          named-preset development`,
        { encoding: "utf-8", cwd: WORKSPACE_DIR }
      );
      // chain-spec-builder always outputs chain_spec.json — rename it
      const defaultSpec = join(WORKSPACE_DIR, "chain_spec.json");
      expect(existsSync(defaultSpec)).toBe(true);
      execSync(`mv ${defaultSpec} ${CHAIN_SPEC_1000}`);
      expect(existsSync(CHAIN_SPEC_1000)).toBe(true);
      console.log("chain_spec_1000.json generated");

      // Generate chain spec for para ID 1001
      console.log("Generating chain spec for parachain 1001...");
      execSync(
        `chain-spec-builder create -t development \
          --relay-chain rococo-local \
          --para-id 1001 \
          --runtime ${WASM_PATH} \
          named-preset development`,
        { encoding: "utf-8", cwd: WORKSPACE_DIR }
      );
      expect(existsSync(defaultSpec)).toBe(true);
      execSync(`mv ${defaultSpec} ${CHAIN_SPEC_1001}`);
      expect(existsSync(CHAIN_SPEC_1001)).toBe(true);
      console.log("chain_spec_1001.json generated");
    }, 60000);
  });

  // ==================== DOWNLOAD RELAY CHAIN BINARIES ====================
  describe("3. Download Relay Chain Binaries", () => {
    it("should download polkadot binaries", () => {
      if (!existsSync(BIN_DIR)) {
        mkdirSync(BIN_DIR, { recursive: true });
      }

      if (existsSync(POLKADOT_BINARY) && existsSync(POLKADOT_PREPARE_WORKER) && existsSync(POLKADOT_EXECUTE_WORKER)) {
        try {
          const version = execSync(`${POLKADOT_BINARY} --version 2>&1`, { encoding: "utf-8" });
          console.log(`Polkadot already downloaded: ${version.trim()}`);
          return;
        } catch {
          console.log("Existing binaries not executable, re-downloading...");
        }
      }

      const platform = process.platform;
      if (platform !== "linux") {
        console.log(`Platform ${platform} detected. Using zombienet setup...`);
        execSync(`zombienet setup polkadot -y`, {
          cwd: BIN_DIR,
          encoding: "utf-8",
          stdio: "inherit",
          timeout: 300000,
        });
      } else {
        console.log(`Downloading Polkadot ${POLKADOT_SDK_VERSION} binaries...`);
        const baseUrl = `https://github.com/paritytech/polkadot-sdk/releases/download/${POLKADOT_SDK_VERSION}`;
        for (const binary of ["polkadot", "polkadot-prepare-worker", "polkadot-execute-worker"]) {
          console.log(`Downloading ${binary}...`);
          execSync(`curl -L -o ${binary} ${baseUrl}/${binary}`, {
            cwd: BIN_DIR,
            encoding: "utf-8",
            stdio: "inherit",
            timeout: 300000,
          });
          execSync(`chmod +x ${binary}`, { cwd: BIN_DIR });
        }
      }

      expect(existsSync(POLKADOT_BINARY)).toBe(true);
      const version = execSync(`${POLKADOT_BINARY} --version 2>&1`, { encoding: "utf-8" });
      console.log(`Polkadot: ${version.trim()}`);
    }, 300000);
  });

  // ==================== SPAWN NETWORK ====================
  describe("4. Spawn Network", () => {
    it("should spawn the network with Zombienet", async () => {
      console.log("Spawning network with Zombienet (2 relay + 2 parachain nodes)...");

      const configPath = join(PROJECT_DIR, "configs", "network.toml");

      zombienetProcess = spawn(
        "zombienet",
        ["spawn", configPath, "--provider", "native"],
        {
          cwd: WORKSPACE_DIR,
          stdio: ["ignore", "pipe", "pipe"],
          detached: true,
        }
      );

      if (zombienetProcess.pid) {
        writeFileSync(PID_FILE, zombienetProcess.pid.toString());
        console.log(`Zombienet started with PID: ${zombienetProcess.pid}`);
      }

      zombienetProcess.stdout?.on("data", (data: Buffer) => {
        const msg = data.toString();
        if (msg.includes("Network launched")) {
          console.log("Network launched successfully!");
        }
      });

      // Wait for relay chain RPC to be available
      const maxWaitTime = 120000;
      const startTime = Date.now();

      while (Date.now() - startTime < maxWaitTime) {
        try {
          const response = await fetch(RELAY_RPC_URL, {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({ jsonrpc: "2.0", method: "system_health", params: [], id: 1 }),
          });
          if (response.ok) {
            console.log("Relay chain RPC is ready!");
            return;
          }
        } catch {
          // Node not ready yet
        }
        await new Promise((resolve) => setTimeout(resolve, 2000));
      }
      throw new Error("Relay chain failed to start within 2 minutes");
    }, 180000);

    it("should wait for parachain A to produce blocks", async () => {
      console.log("Waiting for parachain A (1000) to produce blocks...");

      let blockNumber = 0;
      for (let attempt = 1; attempt <= 15; attempt++) {
        await new Promise((resolve) => setTimeout(resolve, 12000));
        try {
          const result = (await rpcCall(PARA_A_RPC_URL, "chain_getHeader")) as { number: string };
          blockNumber = parseInt(result.number, 16);
          console.log(`Parachain A block number (attempt ${attempt}): ${blockNumber}`);
          if (blockNumber > 0) break;
        } catch {
          console.log(`Attempt ${attempt}: parachain A not ready yet`);
        }
      }
      expect(blockNumber).toBeGreaterThan(0);
    }, 240000);

    it("should wait for parachain B to produce blocks", async () => {
      console.log("Waiting for parachain B (1001) to produce blocks...");

      let blockNumber = 0;
      for (let attempt = 1; attempt <= 15; attempt++) {
        await new Promise((resolve) => setTimeout(resolve, 12000));
        try {
          const result = (await rpcCall(PARA_B_RPC_URL, "chain_getHeader")) as { number: string };
          blockNumber = parseInt(result.number, 16);
          console.log(`Parachain B block number (attempt ${attempt}): ${blockNumber}`);
          if (blockNumber > 0) break;
        } catch {
          console.log(`Attempt ${attempt}: parachain B not ready yet`);
        }
      }
      expect(blockNumber).toBeGreaterThan(0);
    }, 240000);
  });

  // ==================== FUND SOVEREIGN ACCOUNTS ====================
  describe("5. Fund Sovereign Accounts", () => {
    it("should fund parachain 1000 sovereign account on relay chain", async () => {
      const account1000 = sovereignAccount(1000);
      console.log(`Parachain 1000 sovereign account: ${account1000}`);

      const wsProvider = new WsProvider(RELAY_WS_URL);
      const api = await ApiPromise.create({ provider: wsProvider });

      const keyring = new Keyring({ type: "sr25519" });
      const alice = keyring.addFromUri("//Alice");

      // Transfer 10 ROC (10 * 10^12 planck)
      const amount = BigInt(10) * BigInt(10 ** 12);

      await new Promise<void>((resolve, reject) => {
        api.tx.balances
          .transferKeepAlive(account1000, amount)
          .signAndSend(alice, ({ status, dispatchError }) => {
            if (dispatchError) {
              if (dispatchError.isModule) {
                const decoded = api.registry.findMetaError(dispatchError.asModule);
                reject(new Error(`${decoded.section}.${decoded.name}: ${decoded.docs.join(" ")}`));
              } else {
                reject(new Error(dispatchError.toString()));
              }
            }
            if (status.isInBlock) {
              console.log(`Funded parachain 1000 sovereign account in block: ${status.asInBlock.toHex()}`);
              resolve();
            }
          });
      });

      // Verify balance
      const { data: balance } = (await api.query.system.account(account1000)) as any;
      console.log(`Parachain 1000 sovereign balance: ${balance.free.toString()}`);
      expect(BigInt(balance.free.toString())).toBeGreaterThan(BigInt(0));

      await api.disconnect();
    }, 60000);

    it("should fund parachain 1001 sovereign account on relay chain", async () => {
      const account1001 = sovereignAccount(1001);
      console.log(`Parachain 1001 sovereign account: ${account1001}`);

      const wsProvider = new WsProvider(RELAY_WS_URL);
      const api = await ApiPromise.create({ provider: wsProvider });

      const keyring = new Keyring({ type: "sr25519" });
      const alice = keyring.addFromUri("//Alice");

      const amount = BigInt(10) * BigInt(10 ** 12);

      await new Promise<void>((resolve, reject) => {
        api.tx.balances
          .transferKeepAlive(account1001, amount)
          .signAndSend(alice, ({ status, dispatchError }) => {
            if (dispatchError) {
              if (dispatchError.isModule) {
                const decoded = api.registry.findMetaError(dispatchError.asModule);
                reject(new Error(`${decoded.section}.${decoded.name}: ${decoded.docs.join(" ")}`));
              } else {
                reject(new Error(dispatchError.toString()));
              }
            }
            if (status.isInBlock) {
              console.log(`Funded parachain 1001 sovereign account in block: ${status.asInBlock.toHex()}`);
              resolve();
            }
          });
      });

      const { data: balance } = (await api.query.system.account(account1001)) as any;
      console.log(`Parachain 1001 sovereign balance: ${balance.free.toString()}`);
      expect(BigInt(balance.free.toString())).toBeGreaterThan(BigInt(0));

      await api.disconnect();
    }, 60000);
  });

  // ==================== OPEN HRMP CHANNEL 1000 -> 1001 ====================
  describe("6. Open HRMP Channel (1000 -> 1001)", () => {
    it("should send XCM from parachain 1000 to initiate HRMP channel", async () => {
      console.log("Opening HRMP channel from parachain 1000 to 1001...");

      // Connect to relay chain to encode the hrmp.hrmpInitOpenChannel call
      const relayWs = new WsProvider(RELAY_WS_URL);
      const relayApi = await ApiPromise.create({ provider: relayWs });

      const initCall = relayApi.tx.hrmp.hrmpInitOpenChannel(1001, 8, 1048576);
      const encodedCall = initCall.method.toHex();
      console.log(`Encoded hrmpInitOpenChannel call: ${encodedCall}`);

      await relayApi.disconnect();

      // Connect to parachain A to send the XCM
      const paraWs = new WsProvider(PARA_A_WS_URL);
      const paraApi = await ApiPromise.create({ provider: paraWs });

      const keyring = new Keyring({ type: "sr25519" });
      const alice = keyring.addFromUri("//Alice");

      // Build the XCM message to relay chain
      const dest = { V4: { parents: 1, interior: "Here" } };
      const message = {
        V4: [
          {
            WithdrawAsset: [
              {
                id: { parents: 0, interior: "Here" },
                fun: { Fungible: BigInt(10) * BigInt(10 ** 12) },
              },
            ],
          },
          {
            BuyExecution: {
              fees: {
                id: { parents: 0, interior: "Here" },
                fun: { Fungible: BigInt(10) * BigInt(10 ** 12) },
              },
              weightLimit: "Unlimited",
            },
          },
          {
            Transact: {
              originKind: "Native",
              requireWeightAtMost: { refTime: BigInt(1_000_000_000), proofSize: BigInt(65536) },
              call: { encoded: encodedCall },
            },
          },
          "RefundSurplus",
          {
            DepositAsset: {
              assets: { Wild: "All" },
              beneficiary: {
                parents: 0,
                interior: {
                  X1: [
                    {
                      AccountId32: {
                        id: alice.publicKey,
                        network: null,
                      },
                    },
                  ],
                },
              },
            },
          },
        ],
      };

      const xcmTx = paraApi.tx.polkadotXcm.send(dest, message);
      const sudoTx = paraApi.tx.sudo.sudo(xcmTx);

      await new Promise<void>((resolve, reject) => {
        sudoTx.signAndSend(alice, ({ status, dispatchError }) => {
          if (dispatchError) {
            if (dispatchError.isModule) {
              const decoded = paraApi.registry.findMetaError(dispatchError.asModule);
              reject(new Error(`${decoded.section}.${decoded.name}: ${decoded.docs.join(" ")}`));
            } else {
              reject(new Error(dispatchError.toString()));
            }
          }
          if (status.isInBlock) {
            console.log(`XCM sent from parachain 1000 in block: ${status.asInBlock.toHex()}`);
            resolve();
          }
        });
      });

      await paraApi.disconnect();

      // Wait for the XCM to be relayed
      console.log("Waiting for XCM to be relayed to the relay chain...");
      await new Promise((resolve) => setTimeout(resolve, 24000));
    }, 120000);

    it("should have a pending HRMP channel request", async () => {
      const relayWs = new WsProvider(RELAY_WS_URL);
      const relayApi = await ApiPromise.create({ provider: relayWs });

      let found = false;
      for (let attempt = 1; attempt <= 10; attempt++) {
        const requests = await relayApi.query.hrmp.hrmpOpenChannelRequestsList();
        const requestsJson = requests.toJSON() as any[];
        console.log(`HRMP open channel requests (attempt ${attempt}): ${JSON.stringify(requestsJson)}`);

        if (requestsJson && requestsJson.length > 0) {
          found = true;
          break;
        }
        await new Promise((resolve) => setTimeout(resolve, 12000));
      }

      await relayApi.disconnect();
      expect(found).toBe(true);
      console.log("HRMP channel request from 1000 -> 1001 is pending");
    }, 180000);
  });

  // ==================== ACCEPT HRMP CHANNEL ====================
  describe("7. Accept HRMP Channel", () => {
    it("should send XCM from parachain 1001 to accept HRMP channel", async () => {
      console.log("Accepting HRMP channel from parachain 1001...");

      // Connect to relay chain to encode the hrmp.hrmpAcceptOpenChannel call
      const relayWs = new WsProvider(RELAY_WS_URL);
      const relayApi = await ApiPromise.create({ provider: relayWs });

      const acceptCall = relayApi.tx.hrmp.hrmpAcceptOpenChannel(1000);
      const encodedCall = acceptCall.method.toHex();
      console.log(`Encoded hrmpAcceptOpenChannel call: ${encodedCall}`);

      await relayApi.disconnect();

      // Connect to parachain B to send the XCM
      const paraWs = new WsProvider(PARA_B_WS_URL);
      const paraApi = await ApiPromise.create({ provider: paraWs });

      const keyring = new Keyring({ type: "sr25519" });
      const alice = keyring.addFromUri("//Alice");

      const dest = { V4: { parents: 1, interior: "Here" } };
      const message = {
        V4: [
          {
            WithdrawAsset: [
              {
                id: { parents: 0, interior: "Here" },
                fun: { Fungible: BigInt(10) * BigInt(10 ** 12) },
              },
            ],
          },
          {
            BuyExecution: {
              fees: {
                id: { parents: 0, interior: "Here" },
                fun: { Fungible: BigInt(10) * BigInt(10 ** 12) },
              },
              weightLimit: "Unlimited",
            },
          },
          {
            Transact: {
              originKind: "Native",
              requireWeightAtMost: { refTime: BigInt(1_000_000_000), proofSize: BigInt(65536) },
              call: { encoded: encodedCall },
            },
          },
          "RefundSurplus",
          {
            DepositAsset: {
              assets: { Wild: "All" },
              beneficiary: {
                parents: 0,
                interior: {
                  X1: [
                    {
                      AccountId32: {
                        id: alice.publicKey,
                        network: null,
                      },
                    },
                  ],
                },
              },
            },
          },
        ],
      };

      const xcmTx = paraApi.tx.polkadotXcm.send(dest, message);
      const sudoTx = paraApi.tx.sudo.sudo(xcmTx);

      await new Promise<void>((resolve, reject) => {
        sudoTx.signAndSend(alice, ({ status, dispatchError }) => {
          if (dispatchError) {
            if (dispatchError.isModule) {
              const decoded = paraApi.registry.findMetaError(dispatchError.asModule);
              reject(new Error(`${decoded.section}.${decoded.name}: ${decoded.docs.join(" ")}`));
            } else {
              reject(new Error(dispatchError.toString()));
            }
          }
          if (status.isInBlock) {
            console.log(`XCM sent from parachain 1001 in block: ${status.asInBlock.toHex()}`);
            resolve();
          }
        });
      });

      await paraApi.disconnect();

      console.log("Waiting for XCM to be relayed to the relay chain...");
      await new Promise((resolve) => setTimeout(resolve, 24000));
    }, 120000);

    it("should have an established HRMP channel after session boundary", async () => {
      console.log("Waiting for HRMP channel to be established (requires session boundary)...");

      const relayWs = new WsProvider(RELAY_WS_URL);
      const relayApi = await ApiPromise.create({ provider: relayWs });

      let channelFound = false;
      for (let attempt = 1; attempt <= 25; attempt++) {
        const channels = await relayApi.query.hrmp.hrmpChannels({
          sender: 1000,
          recipient: 1001,
        });
        const channelJson = channels.toJSON() as any;

        if (channelJson && channelJson.maxCapacity) {
          console.log(`HRMP channel 1000->1001 established: ${JSON.stringify(channelJson)}`);
          channelFound = true;
          break;
        }

        console.log(`Attempt ${attempt}/25: channel not yet active, waiting for session boundary...`);
        await new Promise((resolve) => setTimeout(resolve, 12000));
      }

      await relayApi.disconnect();
      expect(channelFound).toBe(true);
      console.log("HRMP channel 1000 -> 1001 is active!");
    }, 360000);
  });

  // ==================== POST-CHANNEL VERIFICATION ====================
  describe("8. Post-Channel Verification", () => {
    it("should verify both parachains continue producing blocks", async () => {
      // Check parachain A
      const resultA = (await rpcCall(PARA_A_RPC_URL, "chain_getHeader")) as { number: string };
      const blockA = parseInt(resultA.number, 16);
      console.log(`Parachain A current block: ${blockA}`);
      expect(blockA).toBeGreaterThan(0);

      // Check parachain B
      const resultB = (await rpcCall(PARA_B_RPC_URL, "chain_getHeader")) as { number: string };
      const blockB = parseInt(resultB.number, 16);
      console.log(`Parachain B current block: ${blockB}`);
      expect(blockB).toBeGreaterThan(0);
    }, 30000);

    it("should verify relay chain is still operational", async () => {
      const name = (await rpcCall(RELAY_RPC_URL, "system_name")) as string;
      expect(name).toBeDefined();
      console.log(`Relay chain system name: ${name}`);

      const result = (await rpcCall(RELAY_RPC_URL, "chain_getHeader")) as { number: string };
      const blockNumber = parseInt(result.number, 16);
      console.log(`Relay chain block: ${blockNumber}`);
      expect(blockNumber).toBeGreaterThan(0);
    }, 10000);
  });
});

async function stopZombienet(): Promise<void> {
  console.log("Stopping Zombienet...");

  if (zombienetProcess && !zombienetProcess.killed) {
    try {
      process.kill(-zombienetProcess.pid!, "SIGTERM");
    } catch {
      zombienetProcess.kill("SIGTERM");
    }
    zombienetProcess = null;
  }

  if (existsSync(PID_FILE)) {
    try {
      const pid = parseInt(readFileSync(PID_FILE, "utf-8"));
      process.kill(-pid, "SIGTERM");
    } catch {
      // Process might already be dead
    }
    unlinkSync(PID_FILE);
  }

  try {
    execSync("pkill -f 'polkadot.*rococo-local' 2>/dev/null || true", { encoding: "utf-8" });
    execSync("pkill -f 'polkadot-omni-node' 2>/dev/null || true", { encoding: "utf-8" });
  } catch {
    // Ignore
  }

  await new Promise((resolve) => setTimeout(resolve, 2000));
  console.log("Zombienet stopped");
}
