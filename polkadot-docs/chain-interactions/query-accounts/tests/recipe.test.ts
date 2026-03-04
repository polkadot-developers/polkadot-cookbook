import { describe, it, expect, afterAll } from "vitest";
import { ApiPromise, WsProvider } from "@polkadot/api";
import { DedotClient, WsProvider as DedotWsProvider } from "dedot";

const WS_ENDPOINT = "wss://asset-hub-paseo.dotters.network";
const ACCOUNT_ADDRESS = "5GgbDVeKZwCmMHzn58iFSgSZDTojRMM52arXnuNXto28R7mg";

// ---------------------------------------------------------------------------
// Polkadot.js API
// ---------------------------------------------------------------------------

describe("1. Polkadot.js API — Query Account", () => {
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

  it("should query system.account and return valid account info", async () => {
    const accountInfo = await api.query.system.account(ACCOUNT_ADDRESS);

    console.log(`Polkadot.js: Querying account ${ACCOUNT_ADDRESS}`);
    console.log(`  Nonce: ${accountInfo.nonce.toString()}`);
    console.log(`  Consumers: ${accountInfo.consumers.toString()}`);
    console.log(`  Providers: ${accountInfo.providers.toString()}`);
    console.log(`  Sufficients: ${accountInfo.sufficients.toString()}`);
    console.log(`  Free: ${accountInfo.data.free.toString()}`);
    console.log(`  Reserved: ${accountInfo.data.reserved.toString()}`);
    console.log(`  Frozen: ${accountInfo.data.frozen.toString()}`);

    expect(accountInfo.nonce).toBeDefined();
    expect(accountInfo.consumers).toBeDefined();
    expect(accountInfo.providers).toBeDefined();
    expect(accountInfo.sufficients).toBeDefined();
    expect(accountInfo.data.free).toBeDefined();
    expect(accountInfo.data.reserved).toBeDefined();
    expect(accountInfo.data.frozen).toBeDefined();
  });
});

// ---------------------------------------------------------------------------
// Dedot
// ---------------------------------------------------------------------------

describe("2. Dedot — Query Account", () => {
  let client: DedotClient;

  afterAll(async () => {
    if (client) await client.disconnect();
  });

  it("should connect to Asset Hub Paseo", async () => {
    const provider = new DedotWsProvider(WS_ENDPOINT);
    client = await DedotClient.new(provider);
    expect(client).toBeDefined();
    console.log("Dedot: Connected to Asset Hub Paseo");
  });

  it("should query system.account and return valid account info", async () => {
    const accountInfo = await client.query.system.account(ACCOUNT_ADDRESS);

    console.log(`Dedot: Querying account ${ACCOUNT_ADDRESS}`);
    console.log(`  Nonce: ${accountInfo.nonce}`);
    console.log(`  Consumers: ${accountInfo.consumers}`);
    console.log(`  Providers: ${accountInfo.providers}`);
    console.log(`  Sufficients: ${accountInfo.sufficients}`);
    console.log(`  Free: ${accountInfo.data.free}`);
    console.log(`  Reserved: ${accountInfo.data.reserved}`);
    console.log(`  Frozen: ${accountInfo.data.frozen}`);

    expect(accountInfo.nonce).toBeDefined();
    expect(accountInfo.consumers).toBeDefined();
    expect(accountInfo.providers).toBeDefined();
    expect(accountInfo.sufficients).toBeDefined();
    expect(accountInfo.data.free).toBeDefined();
    expect(accountInfo.data.reserved).toBeDefined();
    expect(accountInfo.data.frozen).toBeDefined();
  });
});
