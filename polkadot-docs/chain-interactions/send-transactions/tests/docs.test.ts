import { describe, it, expect, afterAll } from "vitest";
import { ApiPromise, WsProvider } from "@polkadot/api";
import { Keyring } from "@polkadot/keyring";
import { DedotClient, WsProvider as DedotWsProvider } from "dedot";
import { execSync } from "child_process";
import { join } from "path";

const WS_ENDPOINT = "wss://asset-hub-paseo.dotters.network";
// A well-known Paseo testnet address used as recipient when no env var is set
const FALLBACK_DEST = "5GgbDVeKZwCmMHzn58iFSgSZDTojRMM52arXnuNXto28R7mg";

const SENDER_MNEMONIC = process.env.SENDER_MNEMONIC;
const DEST_ADDRESS = process.env.DEST_ADDRESS || FALLBACK_DEST;
const hasMnemonic = !!SENDER_MNEMONIC;

// ---------------------------------------------------------------------------
// 1. PAPI — Connect & Construct
// ---------------------------------------------------------------------------

describe("1. PAPI — Connect and Construct", () => {
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

  it("should construct a balance transfer transaction", async () => {
    const { polkadotTestNet } = await import("@polkadot-api/descriptors");
    const api = client.getTypedApi(polkadotTestNet);

    const tx = api.tx.Balances.transfer_keep_alive({
      dest: DEST_ADDRESS,
      value: 1000n,
    });

    expect(tx).toBeDefined();
    console.log("PAPI: Balance transfer transaction constructed");
  });
});

// ---------------------------------------------------------------------------
// 2. PAPI — Send Balance Transfer
// ---------------------------------------------------------------------------

describe("2. PAPI — Send Balance Transfer", () => {
  let client: any;

  afterAll(async () => {
    if (client) await client.destroy();
  });

  it.skipIf(!hasMnemonic)(
    "should sign and send a balance transfer",
    async () => {
      const { createClient } = await import("polkadot-api");
      const { getWsProvider } = await import("polkadot-api/ws");
      const { getPolkadotSigner } = await import("polkadot-api/signer");
      const { polkadotTestNet } = await import("@polkadot-api/descriptors");
      const { waitReady } = await import("@polkadot/wasm-crypto");

      await waitReady();
      const keyring = new Keyring({ type: "sr25519" });
      const pair = keyring.addFromUri(SENDER_MNEMONIC!);
      const signer = getPolkadotSigner(
        pair.publicKey,
        "Sr25519",
        (input) => pair.sign(input)
      );

      client = createClient(getWsProvider(WS_ENDPOINT));
      const api = client.getTypedApi(polkadotTestNet);

      const tx = api.tx.Balances.transfer_keep_alive({
        dest: DEST_ADDRESS,
        value: 1000n,
      });

      const result = await tx.signAndSend(signer);
      console.log(`PAPI: Transaction submitted. Result: ${result}`);
      expect(result).toBeDefined();
    }
  );
});

// ---------------------------------------------------------------------------
// 3. Polkadot.js — Connect & Construct
// ---------------------------------------------------------------------------

describe("3. Polkadot.js — Connect and Construct", () => {
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

  it("should construct a balance transfer transaction", async () => {
    const tx = api.tx.balances.transferKeepAlive(DEST_ADDRESS, 1000);
    expect(tx).toBeDefined();
    expect(tx.method.toHex()).toBeDefined();
    console.log("Polkadot.js: Balance transfer transaction constructed");
  });
});

// ---------------------------------------------------------------------------
// 4. Polkadot.js — Send Balance Transfer
// ---------------------------------------------------------------------------

describe("4. Polkadot.js — Send Balance Transfer", () => {
  let api: ApiPromise;

  afterAll(async () => {
    if (api) await api.disconnect();
  });

  it.skipIf(!hasMnemonic)(
    "should sign and send a balance transfer",
    async () => {
      const wsProvider = new WsProvider(WS_ENDPOINT);
      api = await ApiPromise.create({ provider: wsProvider });

      const keyring = new Keyring({ type: "sr25519" });
      const sender = keyring.addFromUri(SENDER_MNEMONIC!);

      const transfer = api.tx.balances.transferKeepAlive(DEST_ADDRESS, 1000);
      const hash = await transfer.signAndSend(sender);

      console.log(`Polkadot.js: Transfer sent with hash ${hash.toHex()}`);
      expect(hash.toHex()).toBeDefined();
    }
  );
});

// ---------------------------------------------------------------------------
// 5. Dedot — Connect & Construct
// ---------------------------------------------------------------------------

describe("5. Dedot — Connect and Construct", () => {
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

  it("should construct a balance transfer transaction", async () => {
    const tx = client.tx.balances.transferKeepAlive(DEST_ADDRESS, 1000n);
    expect(tx).toBeDefined();
    console.log("Dedot: Balance transfer transaction constructed");
  });
});

// ---------------------------------------------------------------------------
// 6. Dedot — Send Balance Transfer
// ---------------------------------------------------------------------------

describe("6. Dedot — Send Balance Transfer", () => {
  let client: DedotClient;

  afterAll(async () => {
    if (client) await client.disconnect();
  });

  it.skipIf(!hasMnemonic)(
    "should sign and send a balance transfer",
    async () => {
      const { waitReady } = await import("@polkadot/wasm-crypto");
      await waitReady();

      const keyring = new Keyring({ type: "sr25519" });
      const sender = keyring.addFromUri(SENDER_MNEMONIC!);

      const provider = new DedotWsProvider(WS_ENDPOINT);
      client = await DedotClient.new(provider);

      const result = await client.tx.balances
        .transferKeepAlive(DEST_ADDRESS, 1000n)
        .signAndSend(sender);

      console.log(`Dedot: Transaction submitted. Hash: ${result.txHash}`);
      expect(result.txHash).toBeDefined();
    }
  );
});

// ---------------------------------------------------------------------------
// 7. Python — Send Balance Transfer
// ---------------------------------------------------------------------------

describe("7. Python — Send Balance Transfer", () => {
  it.skipIf(!hasMnemonic)(
    "should sign and send a balance transfer using substrate-interface",
    () => {
      const result = execSync(
        `python3 ${join(__dirname, "send_transfer.py")}`,
        {
          encoding: "utf-8",
          timeout: 120000,
          env: {
            ...process.env,
            SENDER_MNEMONIC: SENDER_MNEMONIC!,
            DEST_ADDRESS,
          },
        }
      );
      console.log(result);
      expect(result).toContain("Extrinsic hash:");
      expect(result).toContain("Block hash:");
    }
  );
});

// ---------------------------------------------------------------------------
// 8. Subxt — Send Balance Transfer
// ---------------------------------------------------------------------------

describe("8. Subxt — Send Balance Transfer", () => {
  it.skipIf(!hasMnemonic)(
    "should sign and send a balance transfer using subxt",
    () => {
      const result = execSync("cargo run --bin send_transfer", {
        cwd: join(__dirname, "subxt-send-transactions"),
        encoding: "utf-8",
        timeout: 600000,
        env: {
          ...process.env,
          SENDER_MNEMONIC: SENDER_MNEMONIC!,
          DEST_ADDRESS,
        },
      });
      console.log(result);
      expect(result).toContain("Transaction submitted");
      expect(result).toContain("Block hash:");
    },
    600000
  );
});
