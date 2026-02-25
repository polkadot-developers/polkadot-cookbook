import { describe, it, expect, beforeAll } from "vitest";
import { cryptoWaitReady, mnemonicGenerate } from "@polkadot/util-crypto";
import { Keyring } from "@polkadot/keyring";

describe("Create an Account Guide", () => {
  // ==================== CRYPTO INITIALIZATION ====================
  describe("1. Crypto Initialization", () => {
    it("should initialize WASM crypto", async () => {
      const ready = await cryptoWaitReady();
      expect(ready).toBe(true);
      console.log("Crypto WASM initialized successfully");
    });
  });

  // ==================== MNEMONIC GENERATION ====================
  describe("2. Mnemonic Generation", () => {
    it("should generate a 12-word mnemonic", () => {
      const mnemonic = mnemonicGenerate(12);
      const words = mnemonic.split(" ");
      expect(words).toHaveLength(12);
      console.log(`Generated mnemonic: ${words.slice(0, 3).join(" ")} ...`);
    });
  });

  // ==================== KEYRING CREATION ====================
  describe("3. Keyring Creation", () => {
    it("should create a keyring with sr25519 and ss58Format 0", () => {
      const keyring = new Keyring({ type: "sr25519", ss58Format: 0 });
      expect(keyring).toBeDefined();
      console.log("Keyring created (sr25519, ss58Format: 0)");
    });
  });

  // ==================== ACCOUNT FROM MNEMONIC ====================
  describe("4. Account from Mnemonic", () => {
    it("should derive an account with a valid SS58 address", () => {
      const keyring = new Keyring({ type: "sr25519", ss58Format: 0 });
      const mnemonic = mnemonicGenerate(12);
      const pair = keyring.addFromMnemonic(mnemonic);

      expect(pair.address).toBeDefined();
      expect(pair.address).toMatch(/^1/);
      console.log(`Address: ${pair.address}`);
    });
  });

  // ==================== ADDRESS DETERMINISM ====================
  describe("5. Address Determinism", () => {
    it("should produce the same address from the same mnemonic", () => {
      const mnemonic = mnemonicGenerate(12);

      const keyring1 = new Keyring({ type: "sr25519", ss58Format: 0 });
      const pair1 = keyring1.addFromMnemonic(mnemonic);

      const keyring2 = new Keyring({ type: "sr25519", ss58Format: 0 });
      const pair2 = keyring2.addFromMnemonic(mnemonic);

      expect(pair1.address).toBe(pair2.address);
      console.log(`Deterministic address: ${pair1.address}`);
    });
  });

  // ==================== MULTIPLE ACCOUNTS ====================
  describe("6. Multiple Accounts", () => {
    it("should produce different addresses from different mnemonics", () => {
      const keyring = new Keyring({ type: "sr25519", ss58Format: 0 });

      const mnemonic1 = mnemonicGenerate(12);
      const mnemonic2 = mnemonicGenerate(12);
      const pair1 = keyring.addFromMnemonic(mnemonic1);
      const pair2 = keyring.addFromMnemonic(mnemonic2);

      expect(pair1.address).not.toBe(pair2.address);
      console.log(`Account 1: ${pair1.address}`);
      console.log(`Account 2: ${pair2.address}`);
    });
  });
});
