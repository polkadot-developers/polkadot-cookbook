import { describe, it, expect, afterAll } from "vitest";
import { ApiPromise, WsProvider } from "@polkadot/api";
import { DedotClient, WsProvider as DedotWsProvider } from "dedot";
import { hexToString } from "dedot/utils";
import { execSync } from "child_process";
import { join } from "path";

const WS_ENDPOINT = "wss://asset-hub-paseo.dotters.network";
const ACCOUNT_ADDRESS = "5GgbDVeKZwCmMHzn58iFSgSZDTojRMM52arXnuNXto28R7mg";
const USDT_ASSET_ID = 1984;
const USDT_HOLDER_ADDRESS =
  "13rxtPcR9nsAMzLKJsj6UevMR9TzGmyRohJVgQ6U6K2xeqU3";

// ---------------------------------------------------------------------------
// 1. PAPI — Query Balance
// ---------------------------------------------------------------------------

describe("1. PAPI — Query Balance", () => {
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

  it("should query system.account and return valid account info", async () => {
    const { polkadotTestNet } = await import("@polkadot-api/descriptors");

    const api = client.getTypedApi(polkadotTestNet);
    const accountInfo = await api.query.System.Account.getValue(
      ACCOUNT_ADDRESS
    );

    console.log(`PAPI: Querying account ${ACCOUNT_ADDRESS}`);
    console.log(`  Nonce: ${accountInfo.nonce}`);
    console.log(`  Free: ${accountInfo.data.free}`);
    console.log(`  Reserved: ${accountInfo.data.reserved}`);
    console.log(`  Frozen: ${accountInfo.data.frozen}`);

    expect(accountInfo.nonce).toBeDefined();
    expect(accountInfo.data.free).toBeDefined();
    expect(accountInfo.data.reserved).toBeDefined();
    expect(accountInfo.data.frozen).toBeDefined();
  });
});

// ---------------------------------------------------------------------------
// 2. PAPI — Query Asset
// ---------------------------------------------------------------------------

describe("2. PAPI — Query Asset", () => {
  let client: any;

  afterAll(async () => {
    if (client) await client.destroy();
  });

  it("should connect and query asset metadata", async () => {
    const { createClient, Binary } = await import("polkadot-api");
    const { getWsProvider } = await import("polkadot-api/ws");
    const { polkadotTestNet } = await import("@polkadot-api/descriptors");

    client = createClient(getWsProvider(WS_ENDPOINT));
    const api = client.getTypedApi(polkadotTestNet);

    const assetMetadata = await api.query.Assets.Metadata.getValue(
      USDT_ASSET_ID
    );

    console.log(`PAPI: Querying asset metadata for asset ID ${USDT_ASSET_ID}`);
    console.log(`  Name: ${Binary.toText(assetMetadata.name)}`);
    console.log(`  Symbol: ${Binary.toText(assetMetadata.symbol)}`);
    console.log(`  Decimals: ${assetMetadata.decimals}`);

    expect(Binary.toText(assetMetadata.name)).toBeDefined();
    expect(Binary.toText(assetMetadata.symbol)).toBeDefined();
    expect(assetMetadata.decimals).toBeDefined();
  });

  it("should query asset details", async () => {
    const { polkadotTestNet } = await import("@polkadot-api/descriptors");
    const api = client.getTypedApi(polkadotTestNet);

    const assetDetails = await api.query.Assets.Asset.getValue(USDT_ASSET_ID);

    console.log(`PAPI: Querying asset details for asset ID ${USDT_ASSET_ID}`);
    console.log(`  Owner: ${assetDetails.owner}`);
    console.log(`  Supply: ${assetDetails.supply}`);

    expect(assetDetails.owner).toBeDefined();
    expect(assetDetails.supply).toBeDefined();
  });

  it("should query asset account balance", async () => {
    const { polkadotTestNet } = await import("@polkadot-api/descriptors");
    const api = client.getTypedApi(polkadotTestNet);

    const assetAccount = await api.query.Assets.Account.getValue(
      USDT_ASSET_ID,
      USDT_HOLDER_ADDRESS
    );

    console.log(
      `PAPI: Querying asset account for ${USDT_HOLDER_ADDRESS}`
    );
    console.log(`  Balance: ${assetAccount?.balance}`);

    expect(assetAccount).toBeDefined();
    expect(assetAccount?.balance).toBeDefined();
  });
});

// ---------------------------------------------------------------------------
// 3. Polkadot.js — Query Balance
// ---------------------------------------------------------------------------

describe("3. Polkadot.js — Query Balance", () => {
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
    console.log(`  Free: ${accountInfo.data.free.toString()}`);
    console.log(`  Reserved: ${accountInfo.data.reserved.toString()}`);
    console.log(`  Frozen: ${accountInfo.data.frozen.toString()}`);

    expect(accountInfo.nonce).toBeDefined();
    expect(accountInfo.data.free).toBeDefined();
    expect(accountInfo.data.reserved).toBeDefined();
    expect(accountInfo.data.frozen).toBeDefined();
  });
});

// ---------------------------------------------------------------------------
// 4. Polkadot.js — Query Asset
// ---------------------------------------------------------------------------

describe("4. Polkadot.js — Query Asset", () => {
  let api: ApiPromise;

  afterAll(async () => {
    if (api) await api.disconnect();
  });

  it("should connect and query asset metadata", async () => {
    const wsProvider = new WsProvider(WS_ENDPOINT);
    api = await ApiPromise.create({ provider: wsProvider });

    const assetMetadata = await api.query.assets.metadata(USDT_ASSET_ID);

    console.log(
      `Polkadot.js: Querying asset metadata for asset ID ${USDT_ASSET_ID}`
    );
    console.log(`  Name: ${assetMetadata.name.toUtf8()}`);
    console.log(`  Symbol: ${assetMetadata.symbol.toUtf8()}`);
    console.log(`  Decimals: ${assetMetadata.decimals.toString()}`);

    expect(assetMetadata.name.toUtf8()).toBeDefined();
    expect(assetMetadata.symbol.toUtf8()).toBeDefined();
    expect(assetMetadata.decimals).toBeDefined();
  });

  it("should query asset details", async () => {
    const assetDetails = await api.query.assets.asset(USDT_ASSET_ID);

    console.log(
      `Polkadot.js: Querying asset details for asset ID ${USDT_ASSET_ID}`
    );

    expect(assetDetails.isSome).toBe(true);
    if (assetDetails.isSome) {
      const details = assetDetails.unwrap();
      console.log(`  Owner: ${details.owner.toString()}`);
      console.log(`  Supply: ${details.supply.toString()}`);
      expect(details.owner).toBeDefined();
      expect(details.supply).toBeDefined();
    }
  });

  it("should query asset account balance", async () => {
    const assetAccount = await api.query.assets.account(
      USDT_ASSET_ID,
      USDT_HOLDER_ADDRESS
    );

    console.log(
      `Polkadot.js: Querying asset account for ${USDT_HOLDER_ADDRESS}`
    );

    expect(assetAccount.isSome).toBe(true);
    if (assetAccount.isSome) {
      console.log(`  Balance: ${assetAccount.unwrap().balance.toString()}`);
      expect(assetAccount.unwrap().balance).toBeDefined();
    }
  });
});

// ---------------------------------------------------------------------------
// 5. Dedot — Query Balance
// ---------------------------------------------------------------------------

describe("5. Dedot — Query Balance", () => {
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
    console.log(`  Free: ${accountInfo.data.free}`);
    console.log(`  Reserved: ${accountInfo.data.reserved}`);
    console.log(`  Frozen: ${accountInfo.data.frozen}`);

    expect(accountInfo.nonce).toBeDefined();
    expect(accountInfo.data.free).toBeDefined();
    expect(accountInfo.data.reserved).toBeDefined();
    expect(accountInfo.data.frozen).toBeDefined();
  });
});

// ---------------------------------------------------------------------------
// 6. Dedot — Query Asset
// ---------------------------------------------------------------------------

describe("6. Dedot — Query Asset", () => {
  let client: DedotClient;

  afterAll(async () => {
    if (client) await client.disconnect();
  });

  it("should connect and query asset metadata", async () => {
    const provider = new DedotWsProvider(WS_ENDPOINT);
    client = await DedotClient.new(provider);

    const assetMetadata = await client.query.assets.metadata(USDT_ASSET_ID);

    console.log(
      `Dedot: Querying asset metadata for asset ID ${USDT_ASSET_ID}`
    );
    console.log(`  Name: ${hexToString(assetMetadata.name)}`);
    console.log(`  Symbol: ${hexToString(assetMetadata.symbol)}`);
    console.log(`  Decimals: ${assetMetadata.decimals}`);

    expect(hexToString(assetMetadata.name)).toBeDefined();
    expect(hexToString(assetMetadata.symbol)).toBeDefined();
    expect(assetMetadata.decimals).toBeDefined();
  });

  it("should query asset details", async () => {
    const assetDetails = await client.query.assets.asset(USDT_ASSET_ID);

    console.log(
      `Dedot: Querying asset details for asset ID ${USDT_ASSET_ID}`
    );

    expect(assetDetails).toBeDefined();
    if (assetDetails) {
      console.log(`  Owner: ${assetDetails.owner}`);
      console.log(`  Supply: ${assetDetails.supply}`);
      expect(assetDetails.owner).toBeDefined();
      expect(assetDetails.supply).toBeDefined();
    }
  });

  it("should query asset account balance", async () => {
    const assetAccount = await client.query.assets.account([
      USDT_ASSET_ID,
      USDT_HOLDER_ADDRESS,
    ]);

    console.log(
      `Dedot: Querying asset account for ${USDT_HOLDER_ADDRESS}`
    );

    expect(assetAccount).toBeDefined();
    if (assetAccount) {
      console.log(`  Balance: ${assetAccount.balance}`);
      expect(assetAccount.balance).toBeDefined();
    }
  });
});

// ---------------------------------------------------------------------------
// 7. Python — Query Balance
// ---------------------------------------------------------------------------

describe("7. Python — Query Balance", () => {
  it("should query account info using substrate-interface", () => {
    const result = execSync(
      `python3 ${join(__dirname, "query_balance.py")}`,
      { encoding: "utf-8", timeout: 120000 }
    );
    console.log(result);
    expect(result).toContain("Nonce:");
    expect(result).toContain("Free Balance:");
    expect(result).toContain("Reserved Balance:");
    expect(result).toContain("Frozen Balance:");
  });
});

// ---------------------------------------------------------------------------
// 8. Python — Query Asset
// ---------------------------------------------------------------------------

describe("8. Python — Query Asset", () => {
  it("should query asset info using substrate-interface", () => {
    const result = execSync(
      `python3 ${join(__dirname, "query_asset.py")}`,
      { encoding: "utf-8", timeout: 120000 }
    );
    console.log(result);
    expect(result).toContain("Asset Name:");
    expect(result).toContain("Asset Symbol:");
    expect(result).toContain("Asset Owner:");
    expect(result).toContain("Asset Supply:");
    expect(result).toContain("Asset Balance:");
  });
});

// ---------------------------------------------------------------------------
// 9. Subxt — Query Balance
// ---------------------------------------------------------------------------

describe("9. Subxt — Query Balance", () => {
  it(
    "should query account info using subxt",
    () => {
      const result = execSync("cargo run --bin query_balance", {
        cwd: join(__dirname, "subxt-query-sdks"),
        encoding: "utf-8",
        timeout: 600000,
      });
      console.log(result);
      expect(result).toContain("Nonce:");
      expect(result).toContain("Free Balance:");
      expect(result).toContain("Reserved Balance:");
      expect(result).toContain("Frozen Balance:");
    },
    600000
  );
});

// ---------------------------------------------------------------------------
// 10. Subxt — Query Asset
// ---------------------------------------------------------------------------

describe("10. Subxt — Query Asset", () => {
  it(
    "should query asset info using subxt",
    () => {
      const result = execSync("cargo run --bin query_asset", {
        cwd: join(__dirname, "subxt-query-sdks"),
        encoding: "utf-8",
        timeout: 600000,
      });
      console.log(result);
      expect(result).toContain("Asset Name:");
      expect(result).toContain("Asset Symbol:");
      expect(result).toContain("Asset Owner:");
      expect(result).toContain("Asset Supply:");
      expect(result).toContain("Asset Balance:");
    },
    600000
  );
});
