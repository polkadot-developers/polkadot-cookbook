import { ethers } from "hardhat";

async function main() {
  console.log("Deploying Counter contract...");

  const Counter = await ethers.getContractFactory("Counter");
  const counter = await Counter.deploy();

  await counter.waitForDeployment();

  const address = await counter.getAddress();
  console.log(`✅ Counter deployed to: ${address}`);

  // Verify deployment by checking initial count
  const initialCount = await counter.getCount();
  console.log(`Initial count: ${initialCount}`);
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
