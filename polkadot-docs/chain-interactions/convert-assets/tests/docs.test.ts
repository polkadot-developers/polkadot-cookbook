import { describe, it, expect, afterAll, beforeAll } from "vitest";
import { ApiPromise, WsProvider } from "@polkadot/api";
import { Keyring } from "@polkadot/keyring";
import { cryptoWaitReady } from "@polkadot/util-crypto";
import { spawn, ChildProcess } from "child_process";

const CHOPSTICKS_PORT = 8000;
const CHOPSTICKS_WS = `ws://localhost:${CHOPSTICKS_PORT}`;

// PPM test token as used in the tutorial (asset ID 1112)
const TEST_ASSET_ID = 1112;

// Multilocation for DOT (relay chain asset from Asset Hub parachain perspective)
const DOT_MULTILOCATION = {
  parents: 1,
  interior: { Here: null },
};

// Multilocation for PPM token (Assets pallet instance 50, generalIndex 1112)
const TEST_ASSET_MULTILOCATION = {
  parents: 0,
  interior: {
    X2: [{ PalletInstance: 50 }, { GeneralIndex: TEST_ASSET_ID }],
  },
};

// Keyring and Alice account — hoisted to module scope to avoid re-deriving per test
const keyring = new Keyring({ type: "sr25519" });
const alice = keyring.addFromUri("//Alice");
const ALICE_ADDRESS = alice.address;

let chopsticksProcess: ChildProcess | null = null;

// ---------------------------------------------------------------------------
// Chopsticks lifecycle helpers
// ---------------------------------------------------------------------------

async function waitForChopsticks(
  maxRetries = 15,
  retryDelayMs = 3000
): Promise<void> {
  for (let i = 1; i <= maxRetries; i++) {
    try {
      await new Promise<void>((resolve, reject) => {
        const ws = new WebSocket(CHOPSTICKS_WS);
        const timer = setTimeout(() => {
          ws.close();
          reject(new Error("timeout"));
        }, 5000);
        ws.onopen = () => {
          ws.send(
            JSON.stringify({
              jsonrpc: "2.0",
              method: "system_health",
              params: [],
              id: 1,
            })
          );
        };
        ws.onmessage = () => {
          clearTimeout(timer);
          ws.close();
          resolve();
        };
        ws.onerror = () => {
          clearTimeout(timer);
          reject(new Error("ws error"));
        };
      });
      console.log(`Chopsticks ready after ${i} attempt(s)`);
      return;
    } catch {
      if (i < maxRetries) {
        await new Promise((r) => setTimeout(r, retryDelayMs));
      }
    }
  }
  throw new Error("Chopsticks did not become ready");
}

async function stopChopsticks(): Promise<void> {
  const proc = chopsticksProcess;
  chopsticksProcess = null;

  if (proc && !proc.killed) {
    // Kill the whole process group first (spawned with detached: true)
    try {
      process.kill(-proc.pid!, "SIGTERM");
    } catch {
      try {
        proc.kill("SIGTERM");
      } catch {
        // ignore
      }
    }

    await new Promise((r) => setTimeout(r, 2000));

    // Force-kill if still alive
    try {
      process.kill(-proc.pid!, "SIGKILL");
    } catch {
      try {
        proc.kill("SIGKILL");
      } catch {
        // ignore
      }
    }
  }

  await new Promise((r) => setTimeout(r, 500));
}

/**
 * Send a JSON-RPC call to Chopsticks over a fresh WebSocket connection.
 */
function rpcCall(
  method: string,
  params: unknown[] = [],
  timeout = 30000
): Promise<unknown> {
  return new Promise((resolve, reject) => {
    const ws = new WebSocket(CHOPSTICKS_WS);
    const timer = setTimeout(() => {
      ws.close();
      reject(new Error(`RPC call "${method}" timed out`));
    }, timeout);

    ws.onopen = () => {
      ws.send(JSON.stringify({ jsonrpc: "2.0", method, params, id: 1 }));
    };
    ws.onmessage = (event) => {
      clearTimeout(timer);
      const data = JSON.parse(String(event.data));
      ws.close();
      if (data.error) {
        reject(new Error(`RPC error: ${JSON.stringify(data.error)}`));
      } else {
        resolve(data.result);
      }
    };
    ws.onerror = () => {
      clearTimeout(timer);
      reject(new Error(`WebSocket error for ${method}`));
    };
  });
}

// ---------------------------------------------------------------------------
// Suite
// ---------------------------------------------------------------------------

describe("Convert Assets on Asset Hub Guide", () => {
  let api: ApiPromise;

  beforeAll(async () => {
    await stopChopsticks(); // clean up any stale process

    console.log("Starting Chopsticks (polkadot-asset-hub)...");
    chopsticksProcess = spawn(
      "npx",
      ["@acala-network/chopsticks", "-c", "polkadot-asset-hub"],
      {
        stdio: ["ignore", "pipe", "pipe"],
        detached: true,
      }
    );

    chopsticksProcess.stdout?.on("data", (data: Buffer) => {
      const line = data.toString().trim();
      if (line) console.log(`[chopsticks] ${line}`);
    });
    chopsticksProcess.stderr?.on("data", (data: Buffer) => {
      const line = data.toString().trim();
      if (line) console.log(`[chopsticks:err] ${line}`);
    });

    await waitForChopsticks();

    await cryptoWaitReady();
    const wsProvider = new WsProvider(CHOPSTICKS_WS);
    api = await ApiPromise.create({ provider: wsProvider });
    console.log("Polkadot.js API connected to Chopsticks");

    // ---------------------------------------------------------------------------
    // Bootstrap Alice's DOT balance and test asset via dev_setStorage
    // ---------------------------------------------------------------------------

    // Helper: encode a u128 as a 0x-prefixed 16-byte little-endian hex string
    const u128LeHex = (value: bigint): string => {
      const buf = Buffer.alloc(16);
      buf.writeBigUInt64LE(value & 0xffffffffffffffffn, 0);
      buf.writeBigUInt64LE(value >> 64n, 8);
      return "0x" + buf.toString("hex");
    };

    const DOT_DECIMALS = 10n;
    const ALICE_DOT = 10_000n * 10n ** DOT_DECIMALS; // 10 000 DOT
    const PPM_SUPPLY = 1_000n * 10n ** DOT_DECIMALS; // 1 000 PPM (10 decimals)

    // 1. Fund Alice with 10 000 DOT
    await rpcCall("dev_setStorage", [
      {
        System: {
          Account: [
            [
              [ALICE_ADDRESS],
              {
                providers: 1,
                data: { free: u128LeHex(ALICE_DOT) },
              },
            ],
          ],
        },
      },
    ]);
    console.log("Funded Alice with 10 000 DOT via dev_setStorage");

    // 2. Bootstrap PPM asset (ID 1112) — a fresh test token, not an existing mainnet asset
    await rpcCall("dev_setStorage", [
      {
        Assets: {
          Asset: [
            [
              [TEST_ASSET_ID],
              {
                owner: ALICE_ADDRESS,
                issuer: ALICE_ADDRESS,
                admin: ALICE_ADDRESS,
                freezer: ALICE_ADDRESS,
                supply: u128LeHex(PPM_SUPPLY),
                deposit: 0,
                minBalance: 1,
                isSufficient: true,
                accounts: 1,
                sufficients: 1,
                approvals: 0,
                status: "Live",
              },
            ],
          ],
        },
      },
    ]);
    console.log(`Created PPM asset (ID ${TEST_ASSET_ID}) via dev_setStorage`);

    // 3. Give Alice 1 000 PPM tokens
    await rpcCall("dev_setStorage", [
      {
        Assets: {
          Account: [
            [
              [TEST_ASSET_ID, ALICE_ADDRESS],
              {
                balance: u128LeHex(PPM_SUPPLY),
                status: "Liquid",
                reason: "Sufficient",
                extra: null,
              },
            ],
          ],
        },
      },
    ]);
    console.log(`Gave Alice ${PPM_SUPPLY} PPM tokens via dev_setStorage`);

    // 4. Advance chain by 1 block to commit storage changes
    await rpcCall("dev_newBlock", [{ count: 1 }], 60000);
    console.log("Advanced chain by 1 block");
  }, 180000);

  afterAll(async () => {
    if (api) await api.disconnect();
    await stopChopsticks();
  });

  // ==================== 1. Verify Asset Conversion Pallet ====================

  describe("1. Verify Asset Conversion Pallet", () => {
    it("should have the assetConversion pallet available", () => {
      expect(api.tx.assetConversion).toBeDefined();
      expect(api.tx.assetConversion.createPool).toBeDefined();
      expect(api.tx.assetConversion.addLiquidity).toBeDefined();
      expect(api.tx.assetConversion.swapExactTokensForTokens).toBeDefined();
      expect(api.tx.assetConversion.swapTokensForExactTokens).toBeDefined();
      expect(api.tx.assetConversion.removeLiquidity).toBeDefined();
      console.log("Asset Conversion pallet: all extrinsics available");
    });

    it("should have assetConversion query storage available", () => {
      expect(api.query.assetConversion).toBeDefined();
      expect(api.query.assetConversion.pools).toBeDefined();
      console.log("Asset Conversion pallet: query storage available");
    });
  });

  // ==================== 2. Query Existing Pools ====================

  describe("2. Query Existing Pools on Asset Hub", () => {
    it("should return the list of existing pools on the fork", async () => {
      const pools = await api.query.assetConversion.pools.entries();
      console.log(`Found ${pools.length} existing pool(s) on Asset Hub fork`);
      expect(Array.isArray(pools)).toBe(true);
      expect(pools.length).toBeGreaterThan(0); // mainnet fork has pools
    });

    it("should be able to query any pool and get pool info", async () => {
      const pools = await api.query.assetConversion.pools.entries();
      if (pools.length > 0) {
        const [key, poolData] = pools[0];
        console.log("Sample pool key:", key.toHex().substring(0, 40) + "...");
        console.log("Sample pool data:", poolData.toString().substring(0, 100));
        expect(poolData).toBeDefined();
      }
    });
  });

  // ==================== 3. Verify Test Asset Setup ====================

  describe("3. Verify Test Asset Setup", () => {
    it("should have Alice's DOT balance funded", async () => {
      const account = await api.query.system.account(ALICE_ADDRESS);
      const free = (account as any).data.free.toBigInt();
      console.log(`Alice's DOT balance: ${free}`);
      expect(free).toBeGreaterThan(0n);
    });

    it("should have test asset created with Alice as owner", async () => {
      const assetDetails = await api.query.assets.asset(TEST_ASSET_ID);
      expect((assetDetails as any).isSome).toBe(true);
      const details = (assetDetails as any).unwrap();
      console.log("Test asset status:", details.status.toString());
      expect(details.status.toString()).toBe("Live");
    });

    it("should have Alice funded with test asset tokens", async () => {
      const assetAccount = await api.query.assets.account(
        TEST_ASSET_ID,
        ALICE_ADDRESS
      );
      expect((assetAccount as any).isSome).toBe(true);
      const balance = (assetAccount as any).unwrap().balance.toBigInt();
      console.log(`Alice's test asset balance: ${balance}`);
      expect(balance).toBeGreaterThan(0n);
    });
  });

  // ==================== 4. Create Liquidity Pool ====================

  describe("4. Create Liquidity Pool", () => {
    it("should construct and get payment info for a createPool transaction", async () => {
      const tx = api.tx.assetConversion.createPool(
        DOT_MULTILOCATION,
        TEST_ASSET_MULTILOCATION
      );

      const paymentInfo = await tx.paymentInfo(alice.address);
      console.log(
        "createPool paymentInfo: fee =",
        paymentInfo.partialFee.toHuman()
      );
      expect(paymentInfo.partialFee.toBigInt()).toBeGreaterThan(0n);
    }, 30000);

    it("should create a DOT/PPM liquidity pool", async () => {
      // Check if the pool already exists (may exist from a prior Chopsticks session snapshot)
      const existingPool = await (api.query.assetConversion.pools as any)([
        DOT_MULTILOCATION,
        TEST_ASSET_MULTILOCATION,
      ]);
      if (existingPool.isSome) {
        console.log("Pool already exists (DOT/PPM) — skipping createPool submission");
        expect(existingPool.isSome).toBe(true);
        return;
      }

      const createPoolTx = api.tx.assetConversion.createPool(
        DOT_MULTILOCATION,
        TEST_ASSET_MULTILOCATION
      );

      const txHash = await new Promise<string>((resolve, reject) => {
        let unsub: (() => void) | undefined;
        createPoolTx
          .signAndSend(alice, { nonce: -1 }, ({ status, txHash, dispatchError, events }) => {
            if (status.isInBlock || status.isFinalized) {
              if (unsub) unsub();
              if (dispatchError) {
                const errInfo = dispatchError.isModule
                  ? api.registry.findMetaError(dispatchError.asModule)
                  : { name: dispatchError.type };
                reject(
                  new Error(
                    `createPool failed: ${errInfo.name} (${JSON.stringify(errInfo)})`
                  )
                );
              } else {
                const poolCreatedEvent = events.find(({ event }) =>
                  api.events.assetConversion.PoolCreated?.is(event)
                );
                expect(poolCreatedEvent).toBeDefined();
                console.log(
                  "PoolCreated event:",
                  poolCreatedEvent!.event.data.toString()
                );
                resolve(txHash.toHex());
              }
            }
          })
          .then((u) => {
            unsub = u;
          })
          .catch(reject);
      });

      console.log("Pool created, txHash:", txHash);
      expect(txHash).toBeDefined();
    }, 120000);
  });

  // ==================== 5. Add Liquidity ====================

  describe("5. Add Liquidity to Pool", () => {
    it("should add liquidity to the DOT/PPM pool", async () => {
      // Tutorial: 1 DOT + 1 PPM (both have 10 decimals → 1_000_000_000_000 each)
      const DOT_AMOUNT = 1_000_000_000_000n; // 1 DOT
      const ASSET_AMOUNT = 1_000_000_000_000n; // 1 PPM

      const addLiquidityTx = api.tx.assetConversion.addLiquidity(
        DOT_MULTILOCATION,
        TEST_ASSET_MULTILOCATION,
        DOT_AMOUNT,
        ASSET_AMOUNT,
        1n, // min DOT accepted
        1n, // min asset accepted
        alice.address // mintTo
      );

      const txHash = await new Promise<string>((resolve, reject) => {
        let unsub: (() => void) | undefined;
        addLiquidityTx
          .signAndSend(alice, { nonce: -1 }, ({ status, txHash, dispatchError, events }) => {
            if (status.isInBlock || status.isFinalized) {
              if (unsub) unsub();
              if (dispatchError) {
                const errInfo = dispatchError.isModule
                  ? api.registry.findMetaError(dispatchError.asModule)
                  : { name: dispatchError.type };
                reject(
                  new Error(
                    `addLiquidity failed: ${errInfo.name} (${JSON.stringify(errInfo)})`
                  )
                );
              } else {
                const liquidityEvent = events.find(({ event }) =>
                  api.events.assetConversion.LiquidityAdded?.is(event)
                );
                expect(liquidityEvent).toBeDefined();
                console.log(
                  "LiquidityAdded event:",
                  liquidityEvent!.event.data.toString()
                );
                resolve(txHash.toHex());
              }
            }
          })
          .then((u) => {
            unsub = u;
          })
          .catch(reject);
      });

      console.log("Liquidity added, txHash:", txHash);
      expect(txHash).toBeDefined();
    }, 120000);
  });

  // ==================== 6. Swap Exact Tokens For Tokens ====================

  describe("6. Swap Exact Tokens For Tokens", () => {
    it(
      "should swap an exact amount of DOT for PPM tokens",
      async () => {
        // Tutorial: swap 0.01 DOT (100_000_000_000) for at least some PPM
        const AMOUNT_IN = 100_000_000_000n; // 0.01 DOT
        const AMOUNT_OUT_MIN = 1n;

        const swapTx = api.tx.assetConversion.swapExactTokensForTokens(
          [DOT_MULTILOCATION, TEST_ASSET_MULTILOCATION], // path
          AMOUNT_IN,
          AMOUNT_OUT_MIN,
          alice.address,
          false // keepAlive
        );

        const txHash = await new Promise<string>((resolve, reject) => {
          let unsub: (() => void) | undefined;
          swapTx
            .signAndSend(alice, { nonce: -1 }, ({ status, txHash, dispatchError, events }) => {
              if (status.isInBlock || status.isFinalized) {
                if (unsub) unsub();
                if (dispatchError) {
                  const errInfo = dispatchError.isModule
                    ? api.registry.findMetaError(dispatchError.asModule)
                    : { name: dispatchError.type };
                  reject(
                    new Error(
                      `swapExactTokensForTokens failed: ${errInfo.name} (${JSON.stringify(errInfo)})`
                    )
                  );
                } else {
                  const swapEvent = events.find(({ event }) =>
                    api.events.assetConversion.SwapExecuted?.is(event)
                  );
                  expect(swapEvent).toBeDefined();
                  console.log(
                    "SwapExecuted event:",
                    swapEvent!.event.data.toString()
                  );
                  resolve(txHash.toHex());
                }
              }
            })
            .then((u) => {
              unsub = u;
            })
            .catch(reject);
        });

        console.log("Swap (exact input) completed, txHash:", txHash);
        expect(txHash).toBeDefined();
      },
      120000
    );
  });

  // ==================== 7. Swap Tokens For Exact Tokens ====================

  describe("7. Swap Tokens For Exact Tokens", () => {
    it(
      "should spend PPM to receive an exact amount of DOT",
      async () => {
        // Tutorial: "spend up to 0.04 PPM to receive exactly 0.01 DOT"
        // path = [PPM, DOT]: PPM is the input, DOT is the exact output.
        // Pool math: ~91.2B PPM needed to get 100B DOT → within 400B PPM limit.
        const AMOUNT_OUT = 100_000_000_000n; // 0.01 DOT exact out
        const AMOUNT_IN_MAX = 400_000_000_000n; // max 0.04 PPM in

        const swapTx = api.tx.assetConversion.swapTokensForExactTokens(
          [TEST_ASSET_MULTILOCATION, DOT_MULTILOCATION], // PPM → DOT (exact out)
          AMOUNT_OUT,
          AMOUNT_IN_MAX,
          alice.address,
          false // keepAlive
        );

        const txHash = await new Promise<string>((resolve, reject) => {
          let unsub: (() => void) | undefined;
          swapTx
            .signAndSend(alice, { nonce: -1 }, ({ status, txHash, dispatchError, events }) => {
              if (status.isInBlock || status.isFinalized) {
                if (unsub) unsub();
                if (dispatchError) {
                  const errInfo = dispatchError.isModule
                    ? api.registry.findMetaError(dispatchError.asModule)
                    : { name: dispatchError.type };
                  reject(
                    new Error(
                      `swapTokensForExactTokens failed: ${errInfo.name} (${JSON.stringify(errInfo)})`
                    )
                  );
                } else {
                  const swapEvent = events.find(({ event }) =>
                    api.events.assetConversion.SwapExecuted?.is(event)
                  );
                  expect(swapEvent).toBeDefined();
                  console.log(
                    "SwapExecuted event:",
                    swapEvent!.event.data.toString()
                  );
                  resolve(txHash.toHex());
                }
              }
            })
            .then((u) => {
              unsub = u;
            })
            .catch(reject);
        });

        console.log("Swap (exact output) completed, txHash:", txHash);
        expect(txHash).toBeDefined();
      },
      120000
    );
  });

  // ==================== 8. Remove Liquidity ====================

  describe("8. Remove Liquidity", () => {
    it(
      "should remove liquidity from the DOT/PPM pool",
      async () => {
        // Query the pool to find the LP token ID
        const poolInfo = await (api.query.assetConversion.pools as any)([
          DOT_MULTILOCATION,
          TEST_ASSET_MULTILOCATION,
        ]);

        if (!poolInfo.isSome) {
          throw new Error("Pool not found — createPool must have succeeded");
        }

        const lpTokenId = poolInfo.unwrap().lpToken.toNumber();
        console.log("LP Token ID:", lpTokenId);

        // Query Alice's LP token balance
        const lpAccount = await api.query.poolAssets.account(
          lpTokenId,
          ALICE_ADDRESS
        );
        const lpAmount = (lpAccount as any).isSome
          ? (lpAccount as any).unwrap().balance.toBigInt()
          : 0n;
        console.log(`Alice's LP token balance: ${lpAmount}`);
        expect(lpAmount).toBeGreaterThan(0n);

        // Tutorial: burn 0.05 LP tokens (10 decimals → 500_000_000n)
        const burnAmount = 500_000_000n;
        expect(lpAmount).toBeGreaterThanOrEqual(burnAmount);

        const removeLiquidityTx = api.tx.assetConversion.removeLiquidity(
          DOT_MULTILOCATION,
          TEST_ASSET_MULTILOCATION,
          burnAmount,
          1n, // min DOT to receive
          1n, // min PPM to receive
          alice.address
        );

        const txHash = await new Promise<string>((resolve, reject) => {
          let unsub: (() => void) | undefined;
          removeLiquidityTx
            .signAndSend(alice, { nonce: -1 }, ({ status, txHash, dispatchError, events }) => {
              if (status.isInBlock || status.isFinalized) {
                if (unsub) unsub();
                if (dispatchError) {
                  const errInfo = dispatchError.isModule
                    ? api.registry.findMetaError(dispatchError.asModule)
                    : { name: dispatchError.type };
                  reject(
                    new Error(
                      `removeLiquidity failed: ${errInfo.name} (${JSON.stringify(errInfo)})`
                    )
                  );
                } else {
                  const liquidityRemovedEvent = events.find(({ event }) =>
                    api.events.assetConversion.LiquidityRemoved?.is(event)
                  );
                  expect(liquidityRemovedEvent).toBeDefined();
                  console.log(
                    "LiquidityRemoved event:",
                    liquidityRemovedEvent!.event.data.toString()
                  );
                  resolve(txHash.toHex());
                }
              }
            })
            .then((u) => {
              unsub = u;
            })
            .catch(reject);
        });

        console.log("Liquidity removed, txHash:", txHash);
        expect(txHash).toBeDefined();
      },
      120000
    );
  });
});
