import { describe, it, expect, afterAll } from "vitest";
import { ApiPromise, WsProvider } from "@polkadot/api";

const WS_ENDPOINT = "wss://asset-hub-paseo.dotters.network";
const ALICE_ADDRESS = "5GgbDVeKZwCmMHzn58iFSgSZDTojRMM52arXnuNXto28R7mg";
const BOB_ADDRESS = "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty";

// Amount to transfer (1 DOT = 10^10 plancks)
const AMOUNT = 10_000_000_000n;

// ---------------------------------------------------------------------------
// 1. PAPI — Calculate Transaction Fees
// ---------------------------------------------------------------------------

describe("1. PAPI — Calculate Transaction Fees", () => {
  let client: any;

  afterAll(async () => {
    if (client) await client.destroy();
  });

  it("should connect to Asset Hub Paseo", async () => {
    const { createClient } = await import("polkadot-api");
    const { getWsProvider } = await import("polkadot-api/ws");

    client = createClient(getWsProvider(WS_ENDPOINT));
    expect(client).toBeDefined();
    console.log("PAPI: Connected to Asset Hub Paseo");
  });

  it("should estimate fees for a transfer_keep_alive transaction", async () => {
    const { polkadotTestNet } = await import("@polkadot-api/descriptors");
    const api = client.getTypedApi(polkadotTestNet);

    const tx = api.tx.Balances.transfer_keep_alive({
      dest: {
        type: "Id",
        value: BOB_ADDRESS,
      },
      value: AMOUNT,
    });

    const estimatedFees = await tx.getEstimatedFees(ALICE_ADDRESS);

    console.log(`PAPI: Estimated fee: ${estimatedFees} plancks`);
    console.log(`PAPI: Estimated fee: ${Number(estimatedFees) / 1e10} DOT`);

    expect(estimatedFees).toBeDefined();
    expect(estimatedFees).toBeGreaterThan(0n);
  });
});

// ---------------------------------------------------------------------------
// 2. Polkadot.js — Calculate Transaction Fees
// ---------------------------------------------------------------------------

describe("2. Polkadot.js — Calculate Transaction Fees", () => {
  let api: ApiPromise;

  afterAll(async () => {
    if (api) await api.disconnect();
  });

  it("should connect to Asset Hub Paseo", async () => {
    const wsProvider = new WsProvider(WS_ENDPOINT);
    api = await ApiPromise.create({ provider: wsProvider });
    expect(api.isConnected).toBe(true);
    console.log("Polkadot.js: Connected to Asset Hub Paseo");
  });

  it("should estimate fees for a transferKeepAlive transaction", async () => {
    const tx = api.tx.balances.transferKeepAlive(BOB_ADDRESS, AMOUNT);

    const paymentInfo = await tx.paymentInfo(ALICE_ADDRESS);

    console.log(
      `Polkadot.js: Estimated fee: ${paymentInfo.partialFee.toString()} plancks`
    );
    console.log(
      `Polkadot.js: Estimated fee: ${Number(paymentInfo.partialFee.toBigInt()) / 1e10} DOT`
    );

    expect(paymentInfo.partialFee).toBeDefined();
    expect(paymentInfo.partialFee.toBigInt()).toBeGreaterThan(0n);
  });
});
