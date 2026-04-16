import { describe, it, expect, afterAll } from "vitest";
import { ApiPromise, WsProvider } from "@polkadot/api";
import { DedotClient, WsProvider as DedotWsProvider } from "dedot";
import { execSync } from "child_process";
import { join } from "path";

const WS_ENDPOINT = "wss://asset-hub-paseo.dotters.network";
const ACCOUNT_ADDRESS = "5GgbDVeKZwCmMHzn58iFSgSZDTojRMM52arXnuNXto28R7mg";

// ---------------------------------------------------------------------------
// 1. PAPI — Runtime API Calls
// ---------------------------------------------------------------------------

describe("1. PAPI — Runtime API Calls", () => {
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

  it("should call AccountNonceApi.account_nonce", async () => {
    const { polkadotTestNet } = await import("@polkadot-api/descriptors");
    const api = client.getTypedApi(polkadotTestNet);

    const nonce = await api.apis.AccountNonceApi.account_nonce(ACCOUNT_ADDRESS);

    console.log(`PAPI: AccountNonceApi.account_nonce(${ACCOUNT_ADDRESS})`);
    console.log(`  Account Nonce: ${nonce}`);

    expect(nonce).toBeDefined();
    expect(Number(nonce)).toBeGreaterThanOrEqual(0);
  });

  it("should call Metadata.metadata_versions", async () => {
    const { polkadotTestNet } = await import("@polkadot-api/descriptors");
    const api = client.getTypedApi(polkadotTestNet);

    const versions = await api.apis.Metadata.metadata_versions();

    console.log("PAPI: Metadata.metadata_versions()");
    console.log(`  Supported Metadata Versions: [${versions.join(", ")}]`);

    expect(Array.isArray(versions)).toBe(true);
    expect(versions.length).toBeGreaterThan(0);
    expect(versions).toContain(14);
  });
});

// ---------------------------------------------------------------------------
// 2. Polkadot.js — Runtime API Calls
// ---------------------------------------------------------------------------

describe("2. Polkadot.js — Runtime API Calls", () => {
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

  it("should call accountNonceApi.accountNonce", async () => {
    const nonce = await api.call.accountNonceApi.accountNonce(ACCOUNT_ADDRESS);

    console.log(`Polkadot.js: accountNonceApi.accountNonce(${ACCOUNT_ADDRESS})`);
    console.log(`  Account Nonce: ${nonce.toString()}`);

    expect(nonce).toBeDefined();
    expect(nonce.toNumber()).toBeGreaterThanOrEqual(0);
  });

  it("should call metadata.metadataVersions", async () => {
    const versions = await api.call.metadata.metadataVersions();

    console.log("Polkadot.js: metadata.metadataVersions()");
    console.log(
      `  Supported Metadata Versions: [${versions.map((v) => v.toString()).join(", ")}]`
    );

    expect(versions).toBeDefined();
    expect(versions.length).toBeGreaterThan(0);
  });
});

// ---------------------------------------------------------------------------
// 3. Dedot — Runtime API Calls
// ---------------------------------------------------------------------------

describe("3. Dedot — Runtime API Calls", () => {
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

  it("should call accountNonceApi.accountNonce", async () => {
    const nonce = await client.call.accountNonceApi.accountNonce(ACCOUNT_ADDRESS);

    console.log(`Dedot: accountNonceApi.accountNonce(${ACCOUNT_ADDRESS})`);
    console.log(`  Account Nonce: ${nonce}`);

    expect(nonce).toBeDefined();
    expect(Number(nonce)).toBeGreaterThanOrEqual(0);
  });

  it("should call metadata.metadataVersions", async () => {
    const versions = await client.call.metadata.metadataVersions();

    console.log("Dedot: metadata.metadataVersions()");
    console.log(`  Supported Metadata Versions: [${versions.join(", ")}]`);

    expect(Array.isArray(versions)).toBe(true);
    expect(versions.length).toBeGreaterThan(0);
  });
});

// ---------------------------------------------------------------------------
// 4. Python Substrate Interface — Runtime API Calls
// ---------------------------------------------------------------------------

describe("4. Python Substrate Interface — Runtime API Calls", () => {
  it("should call runtime APIs using substrate-interface", () => {
    const result = execSync(
      `python3 ${join(__dirname, "runtime_apis.py")}`,
      { encoding: "utf-8", timeout: 120000 }
    );
    console.log(result);
    expect(result).toContain("Account Nonce:");
    expect(result).toContain("Spec Name:");
    expect(result).toContain("Spec Version:");
  });
});

// ---------------------------------------------------------------------------
// 5. Subxt — Runtime API Calls
// ---------------------------------------------------------------------------

describe("5. Subxt — Runtime API Calls", () => {
  it(
    "should call runtime APIs using subxt",
    () => {
      const result = execSync("cargo run --bin runtime_apis", {
        cwd: join(__dirname, "subxt-runtime-api-calls"),
        encoding: "utf-8",
        timeout: 600000,
      });
      console.log(result);
      expect(result).toContain("Account Nonce:");
      expect(result).toContain("Supported Metadata Versions:");
    },
    600000
  );
});
