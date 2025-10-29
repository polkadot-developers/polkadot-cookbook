import { expect } from "chai";
import { ethers } from "hardhat";
import { Counter } from "../typechain-types";
import { SignerWithAddress } from "@nomicfoundation/hardhat-ethers/signers";

describe("Counter", function () {
  let counter: Counter;
  let owner: SignerWithAddress;
  let addr1: SignerWithAddress;

  beforeEach(async function () {
    [owner, addr1] = await ethers.getSigners();

    const Counter = await ethers.getContractFactory("Counter");
    counter = await Counter.deploy();
    await counter.waitForDeployment();
  });

  describe("Deployment", function () {
    it("Should set the right owner", async function () {
      expect(await counter.owner()).to.equal(owner.address);
    });

    it("Should initialize count to 0", async function () {
      expect(await counter.getCount()).to.equal(0);
    });
  });

  describe("Increment", function () {
    it("Should increment the counter", async function () {
      await counter.increment();
      expect(await counter.getCount()).to.equal(1);

      await counter.increment();
      expect(await counter.getCount()).to.equal(2);
    });

    it("Should emit CountChanged event", async function () {
      await expect(counter.increment())
        .to.emit(counter, "CountChanged")
        .withArgs(1, owner.address);
    });
  });

  describe("Decrement", function () {
    it("Should decrement the counter", async function () {
      await counter.increment();
      await counter.increment();
      await counter.decrement();
      expect(await counter.getCount()).to.equal(1);
    });

    it("Should revert when decrementing below zero", async function () {
      await expect(counter.decrement()).to.be.revertedWith(
        "Counter: cannot decrement below zero"
      );
    });

    it("Should emit CountChanged event", async function () {
      await counter.increment();
      await expect(counter.decrement())
        .to.emit(counter, "CountChanged")
        .withArgs(0, owner.address);
    });
  });

  describe("Reset", function () {
    it("Should reset the counter to 0", async function () {
      await counter.increment();
      await counter.increment();
      await counter.reset();
      expect(await counter.getCount()).to.equal(0);
    });

    it("Should only allow owner to reset", async function () {
      await expect(counter.connect(addr1).reset()).to.be.revertedWith(
        "Counter: only owner can reset"
      );
    });

    it("Should emit CountChanged event", async function () {
      await counter.increment();
      await expect(counter.reset())
        .to.emit(counter, "CountChanged")
        .withArgs(0, owner.address);
    });
  });
});
