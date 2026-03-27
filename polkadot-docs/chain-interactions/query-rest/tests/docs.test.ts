import { describe, it, expect } from "vitest";

const SIDECAR_BASE_URL =
  "https://polkadot-asset-hub-public-sidecar.parity-chains.parity.io";

// USDT issuer account on Polkadot Asset Hub (known to have balance and asset activity)
const ACCOUNT_ID = "15uPcYeUE2XaMiMJuR6W7QGW2LsLdKXX7F3PxKG8gcizPh3X";

// USDT asset ID on Polkadot Asset Hub
const USDT_ASSET_ID = "1984";

async function sidecarGet(path: string): Promise<unknown> {
  const url = `${SIDECAR_BASE_URL}${path}`;
  const res = await fetch(url);
  expect(res.ok, `GET ${path} returned ${res.status}`).toBe(true);
  return res.json();
}

describe("Query On-Chain State with Sidecar REST API", () => {
  describe("1. Prerequisites", () => {
    it("public Sidecar endpoint is reachable", async () => {
      const res = await fetch(`${SIDECAR_BASE_URL}/blocks/head`);
      expect(res.ok).toBe(true);
    });
  });

  describe("2. Query Account Balance", () => {
    it("returns balance-info for an account", async () => {
      const data = (await sidecarGet(
        `/accounts/${ACCOUNT_ID}/balance-info`
      )) as Record<string, unknown>;

      expect(data).toHaveProperty("at");
      expect(data).toHaveProperty("nonce");
      expect(data).toHaveProperty("tokenSymbol");
      expect(data).toHaveProperty("free");
      expect(data).toHaveProperty("reserved");
      expect(data).toHaveProperty("frozen");
    });

    it("supports querying at a specific block height", async () => {
      const data = (await sidecarGet(
        `/accounts/${ACCOUNT_ID}/balance-info?at=1000000`
      )) as Record<string, unknown>;

      const at = data.at as Record<string, unknown>;
      expect(at).toHaveProperty("hash");
      expect(at).toHaveProperty("height");
      expect(at.height).toBe("1000000");
    });
  });

  describe("3. Query Asset Balances", () => {
    it("returns asset balances for an account", async () => {
      const data = (await sidecarGet(
        `/accounts/${ACCOUNT_ID}/asset-balances`
      )) as Record<string, unknown>;

      expect(data).toHaveProperty("at");
    });

    it("returns a specific asset balance (USDT)", async () => {
      const data = (await sidecarGet(
        `/accounts/${ACCOUNT_ID}/asset-balances?assets[]=${USDT_ASSET_ID}`
      )) as Record<string, unknown>;

      expect(data).toHaveProperty("at");
    });
  });

  describe("4. Query Asset Metadata", () => {
    it("returns metadata for USDT (asset 1984)", async () => {
      const data = (await sidecarGet(
        `/pallets/assets/storage/Metadata?keys[]=${USDT_ASSET_ID}`
      )) as Record<string, unknown>;

      expect(data).toHaveProperty("at");
      expect(data).toHaveProperty("pallet", "assets");
      expect(data).toHaveProperty("storageItem", "metadata");

      const value = (data as { value: Record<string, unknown> }).value;
      expect(value).toHaveProperty("decimals");
    });
  });

  describe("5. Query Asset Details", () => {
    it("returns asset configuration for USDT (asset 1984)", async () => {
      const data = (await sidecarGet(
        `/pallets/assets/storage/Asset?keys[]=${USDT_ASSET_ID}`
      )) as Record<string, unknown>;

      expect(data).toHaveProperty("at");
      expect(data).toHaveProperty("pallet", "assets");
      expect(data).toHaveProperty("storageItem", "asset");

      const value = (data as { value: Record<string, unknown> }).value;
      expect(value).toHaveProperty("owner");
      expect(value).toHaveProperty("supply");
      expect(value).toHaveProperty("isSufficient");
      expect(value).toHaveProperty("accounts");
    });
  });

  describe("6. Query Foreign Asset Balances", () => {
    it("returns foreign asset balances for an account", async () => {
      const data = (await sidecarGet(
        `/accounts/${ACCOUNT_ID}/foreign-asset-balances`
      )) as Record<string, unknown>;

      expect(data).toHaveProperty("at");
    });
  });

  describe("7. Query Block Information", () => {
    it("returns the latest block", async () => {
      const data = (await sidecarGet("/blocks/head")) as Record<
        string,
        unknown
      >;

      expect(data).toHaveProperty("number");
      expect(data).toHaveProperty("hash");
      expect(data).toHaveProperty("extrinsics");
    });

    it("returns a specific block by number", async () => {
      const data = (await sidecarGet("/blocks/1000000")) as Record<
        string,
        unknown
      >;

      expect(data).toHaveProperty("number");
      expect(data.number).toBe("1000000");
      expect(data).toHaveProperty("hash");
      expect(data).toHaveProperty("extrinsics");
    });
  });
});
