import { ethers } from "hardhat";

async function main() {
  console.log("Deploying MyToken ERC-20 contract...");

  // Get the deployer's address
  const [deployer] = await ethers.getSigners();
  console.log("Deploying with account:", deployer.address);

  // Get the contract factory
  const MyToken = await ethers.getContractFactory("MyToken");

  // Deploy the contract with the deployer as the initial owner
  const token = await MyToken.deploy(deployer.address);
  await token.waitForDeployment();

  const tokenAddress = await token.getAddress();
  console.log("MyToken deployed to:", tokenAddress);
  console.log("Initial owner:", deployer.address);

  // Mint some initial tokens (1 million tokens = 1,000,000 * 10^18)
  console.log("\nMinting initial tokens...");
  const mintAmount = ethers.parseEther("1000000"); // 1 million tokens
  const mintTx = await token.mint(deployer.address, mintAmount);
  await mintTx.wait();

  console.log("Minted", ethers.formatEther(mintAmount), "MTK tokens to:", deployer.address);

  // Check balance
  const balance = await token.balanceOf(deployer.address);
  console.log("Current balance:", ethers.formatEther(balance), "MTK");

  // Get token info
  const name = await token.name();
  const symbol = await token.symbol();
  const decimals = await token.decimals();
  const totalSupply = await token.totalSupply();

  console.log("\nToken Information:");
  console.log("- Name:", name);
  console.log("- Symbol:", symbol);
  console.log("- Decimals:", decimals);
  console.log("- Total Supply:", ethers.formatEther(totalSupply), symbol);
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
