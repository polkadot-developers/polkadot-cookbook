import { describe, it, expect } from "vitest";
import { Builder, hasDryRunSupport } from "@paraspell/sdk";
import {
  entropyToMiniSecret,
  mnemonicToEntropy,
  ss58Address,
} from "@polkadot-labs/hdkd-helpers";
import { sr25519CreateDerive } from "@polkadot-labs/hdkd";
import { getPolkadotSigner } from "polkadot-api/signer";

// PAS token has 10 decimals
const PAS_UNITS = 10_000_000_000n;
const AMOUNT = 10n * PAS_UNITS;

// Public Paseo testnet address used when no mnemonic is provided (dry-run/info only)
const FALLBACK_ADDRESS = "5GgbDVeKZwCmMHzn58iFSgSZDTojRMM52arXnuNXto28R7mg";

const SENDER_MNEMONIC = process.env.SENDER_MNEMONIC;
const hasMnemonic = !!SENDER_MNEMONIC;

function getSignerAndAddress() {
  const entropy = mnemonicToEntropy(SENDER_MNEMONIC!);
  const miniSecret = entropyToMiniSecret(entropy);
  const derive = sr25519CreateDerive(miniSecret);
  const keyPair = derive("");
  const signer = getPolkadotSigner(
    keyPair.publicKey,
    "Sr25519",
    keyPair.sign,
  );
  return { signer, address: ss58Address(keyPair.publicKey) };
}

const senderAddress = hasMnemonic
  ? getSignerAndAddress().address
  : FALLBACK_ADDRESS;
const recipientAddress = senderAddress;

function buildBase() {
  return Builder()
    .from("AssetHubPaseo")
    .to("PeoplePaseo")
    .currency({ symbol: "PAS", amount: AMOUNT })
    .recipient(recipientAddress);
}

// ---------------------------------------------------------------------------
// 1. ParaSpell — Build Transfer Transaction
// ---------------------------------------------------------------------------

describe("1. ParaSpell — Build Transfer Transaction", () => {
  it("should build an XCM transfer from AssetHubPaseo to PeoplePaseo", async () => {
    const tx = await buildBase().sender(senderAddress).build();
    expect(tx).toBeDefined();
    console.log("ParaSpell: Transaction built");
  });
});

// ---------------------------------------------------------------------------
// 2. ParaSpell — Dry Run
// ---------------------------------------------------------------------------

describe("2. ParaSpell — Dry Run Transfer", () => {
  it("should dry-run the transfer (if supported on origin)", async () => {
    if (!hasDryRunSupport("AssetHubPaseo")) {
      console.log("Dry run is not supported on AssetHubPaseo.");
      return;
    }

    const result = await buildBase().sender(senderAddress).dryRun();
    console.log("ParaSpell dry run result:", result);
    expect(result).toBeDefined();
  });
});

// ---------------------------------------------------------------------------
// 3. ParaSpell — Verify Existential Deposit
// ---------------------------------------------------------------------------

describe("3. ParaSpell — Verify Existential Deposit", () => {
  it("should check ED requirement on destination", async () => {
    const isValid = await buildBase()
      .sender(senderAddress)
      .verifyEdOnDestination();
    console.log(`ParaSpell: ED verification ${isValid ? "passed" : "failed"}.`);
    expect(typeof isValid).toBe("boolean");
  });
});

// ---------------------------------------------------------------------------
// 4. ParaSpell — Get Transfer Info
// ---------------------------------------------------------------------------

describe("4. ParaSpell — Get Transfer Info", () => {
  it("should return fee estimates and balance info", async () => {
    const info = await buildBase()
      .sender(senderAddress)
      .getTransferInfo();
    console.log("ParaSpell transfer info:", info);
    expect(info).toBeDefined();
  });
});

// ---------------------------------------------------------------------------
// 5. ParaSpell — Sign and Submit Transfer
// ---------------------------------------------------------------------------

describe("5. ParaSpell — Sign and Submit Transfer", () => {
  it.skipIf(!hasMnemonic)(
    "should sign and submit the XCM transfer",
    async () => {
      const { signer } = getSignerAndAddress();
      const tx = await buildBase().build();
      const result = await tx.signAndSubmit(signer);
      console.log("ParaSpell submit result:", result);
      expect(result).toBeDefined();
    },
    180000,
  );
});
