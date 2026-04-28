import { describe, it, expect } from "vitest";
import {
  EvmBuilder,
  getSupportedAssets,
  EXTERNAL_CHAINS,
} from "@paraspell/sdk-pjs";

// ---------------------------------------------------------------------------
// Environment guards
// ---------------------------------------------------------------------------

// Live bridge submission requires an Ethereum private key with WETH on mainnet.
// Set ETH_PRIVATE_KEY to opt-in to the live submission test.
const ETH_PRIVATE_KEY = process.env.ETH_PRIVATE_KEY;
const hasEthKey = !!ETH_PRIVATE_KEY;

// Recipient Polkadot address for tests that need one (public, well-known)
const RECIPIENT_ADDRESS = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"; // Alice

// A non-dust amount of WETH (0.001 WETH in wei = 10^15)
const WETH_AMOUNT = 1_000_000_000_000_000n;

// ---------------------------------------------------------------------------
// 1. getSupportedAssets — Ethereum → AssetHubPolkadot
// ---------------------------------------------------------------------------

describe("1. ParaSpell — getSupportedAssets (Ethereum → AssetHubPolkadot)", () => {
  it("should return a non-empty list of supported assets for the bridge route", () => {
    const assets = getSupportedAssets("Ethereum", "AssetHubPolkadot");

    expect(Array.isArray(assets)).toBe(true);
    expect(assets.length).toBeGreaterThan(0);

    console.log(
      "Supported assets (Ethereum → AssetHubPolkadot):",
      assets.map((a) => a.symbol).join(", ")
    );
  });

  it("should include WETH in the supported asset list", () => {
    const assets = getSupportedAssets("Ethereum", "AssetHubPolkadot");
    const weth = assets.find(
      (a) => a.symbol?.toUpperCase() === "WETH"
    );

    expect(weth).toBeDefined();
    console.log("WETH asset found:", JSON.stringify(weth));
  });

  it("should return asset objects with required fields (symbol)", () => {
    const assets = getSupportedAssets("Ethereum", "AssetHubPolkadot");

    for (const asset of assets) {
      expect(typeof asset.symbol).toBe("string");
      expect(asset.symbol.length).toBeGreaterThan(0);
    }
  });
});

// ---------------------------------------------------------------------------
// 2. getSupportedAssets — Ethereum → Hydration
// ---------------------------------------------------------------------------

describe("2. ParaSpell — getSupportedAssets (Ethereum → Hydration)", () => {
  it("should return supported assets for the Ethereum → Hydration route", () => {
    const assets = getSupportedAssets("Ethereum", "Hydration");

    expect(Array.isArray(assets)).toBe(true);
    // Hydration may have fewer assets; we just verify the call succeeds
    console.log(
      "Supported assets (Ethereum → Hydration):",
      assets.map((a) => a.symbol).join(", ")
    );
  });
});

// ---------------------------------------------------------------------------
// 3. EXTERNAL_CHAINS constant — SDK recognizes Ethereum as an external chain
// ---------------------------------------------------------------------------

describe("3. ParaSpell — EXTERNAL_CHAINS constant", () => {
  it("should include Ethereum in the list of external (non-parachain) chains", () => {
    // EXTERNAL_CHAINS lists chains like Ethereum that sit outside the Polkadot
    // relay/parachain topology and use specialized bridge paths.
    expect(Array.isArray(EXTERNAL_CHAINS)).toBe(true);
    expect(EXTERNAL_CHAINS).toContain("Ethereum");

    console.log("EXTERNAL_CHAINS:", EXTERNAL_CHAINS.join(", "));
  });

  it("getSupportedAssets should work for multiple Ethereum destination chains", () => {
    // Verify the bridge route lookup works for at least two known destinations
    const destinations = ["AssetHubPolkadot", "Hydration"] as const;

    for (const dest of destinations) {
      const assets = getSupportedAssets("Ethereum", dest);
      expect(Array.isArray(assets)).toBe(true);
      expect(assets.length).toBeGreaterThan(0);
      console.log(`Ethereum → ${dest}: ${assets.length} supported asset(s)`);
    }
  });
});

// ---------------------------------------------------------------------------
// 4. EvmBuilder — Build transfer call (no signing, no network call)
// ---------------------------------------------------------------------------

describe("4. ParaSpell — EvmBuilder construction", () => {
  it("should construct an EvmBuilder for Ethereum → AssetHubPolkadot without errors", () => {
    // EvmBuilder() returns a builder instance; calling methods on it should
    // not throw at the construction stage (before .build() is invoked).
    const builder = EvmBuilder()
      .from("Ethereum")
      .to("AssetHubPolkadot")
      .currency({ symbol: "WETH", amount: WETH_AMOUNT })
      .recipient(RECIPIENT_ADDRESS);

    expect(builder).toBeDefined();
    console.log("EvmBuilder instance created successfully");
  });

  it("should construct an EvmBuilder for Ethereum → Hydration without errors", () => {
    const assets = getSupportedAssets("Ethereum", "Hydration");
    const weth = assets.find((a) => a.symbol?.toUpperCase() === "WETH");

    if (!weth) {
      console.log("WETH not supported on Hydration — skipping builder test");
      return;
    }

    const builder = EvmBuilder()
      .from("Ethereum")
      .to("Hydration")
      .currency({ symbol: "WETH", amount: WETH_AMOUNT })
      .recipient(RECIPIENT_ADDRESS);

    expect(builder).toBeDefined();
    console.log("EvmBuilder (Ethereum → Hydration) created successfully");
  });
});

// ---------------------------------------------------------------------------
// 5. Live bridge transfer — skipped unless ETH_PRIVATE_KEY is set
// ---------------------------------------------------------------------------

describe("5. ParaSpell — Live bridge transfer (requires ETH_PRIVATE_KEY)", () => {
  it.skipIf(!hasEthKey)(
    "should approve WETH and submit a bridge transfer from Ethereum to AssetHubPolkadot",
    async () => {
      // This test requires:
      //   - ETH_PRIVATE_KEY: funded Ethereum wallet with WETH
      //   - Live Ethereum mainnet RPC access (ethers default provider)
      //   - ~0.001 WETH + ETH gas
      const { ethers } = await import("ethers");
      const provider = new ethers.JsonRpcProvider(
        process.env.ETH_RPC_URL ?? "https://eth.llamarpc.com"
      );
      const signer = new ethers.Wallet(ETH_PRIVATE_KEY!, provider);

      const tx = await EvmBuilder(provider)
        .from("Ethereum")
        .to("AssetHubPolkadot")
        .currency({ symbol: "WETH", amount: WETH_AMOUNT })
        .recipient(RECIPIENT_ADDRESS)
        .signer(signer)
        .build();

      expect(tx).toBeDefined();
      console.log("Bridge transfer submitted:", tx);
    },
    300000
  );
});
