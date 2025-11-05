# Deploy ERC-20 Token

Deploy an ERC-20 token on Polkadot Hub using Hardhat and OpenZeppelin contracts.

## Introduction

ERC-20 tokens are fungible tokens commonly used for creating cryptocurrencies, governance tokens, and staking mechanisms. This recipe demonstrates how to deploy a secure ERC-20 token contract on Polkadot Hub using Hardhat and OpenZeppelin's battle-tested contracts.

The token includes:
- Standard ERC-20 functionality (transfer, approve, allowance)
- Owner-only minting capability
- OpenZeppelin's Ownable pattern for access control

## Prerequisites

- Node.js 18+ installed
- MetaMask or another EVM-compatible wallet
- Basic understanding of Solidity
- PAS tokens for gas fees (get from [Polkadot Faucet](https://faucet.polkadot.io/?parachain=1111))

## Installation

Install dependencies:

```bash
npm install
```

This installs:
- Hardhat - Development environment
- Ethers.js v6 - Blockchain interaction library
- OpenZeppelin Contracts - Secure, audited contract libraries
- TypeScript - Type-safe development

## Contract Overview

The `MyToken` contract (`contracts/MyToken.sol`) implements:

```solidity
contract MyToken is ERC20, Ownable {
    constructor(address initialOwner)
        ERC20("MyToken", "MTK")
        Ownable(initialOwner)
    {}

    function mint(address to, uint256 amount) public onlyOwner {
        _mint(to, amount);
    }
}
```

**Key features:**
- **ERC20** - Standard fungible token interface
- **Ownable** - Only the owner can mint new tokens
- **MTK symbol** - Ticker symbol for the token
- **18 decimals** - Standard token precision

## Usage

### 1. Compile the Contract

```bash
npm run compile
```

This compiles the Solidity contracts and generates TypeScript types in `typechain-types/`.

### 2. Test the Contract

Run the test suite:

```bash
npm test
```

Tests verify:
- Deployment with correct owner
- Minting functionality
- Transfer operations
- Approval mechanisms
- Access control (only owner can mint)

### 3. Deploy to Local Network

Start a local Hardhat node:

```bash
npm run node
```

In another terminal, deploy:

```bash
npm run deploy:local
```

### 4. Deploy to Polkadot Hub

**Set up your private key:**

```bash
export PRIVATE_KEY="your-private-key-here"
```

!!! warning
    Never commit your private key to version control. Use environment variables or `.env` files (added to `.gitignore`).

**Deploy to Polkadot Hub TestNet:**

```bash
npm run deploy:polkadot
```

**Expected output:**

```
Deploying MyToken ERC-20 contract...
Deploying with account: 0x1234...5678
MyToken deployed to: 0xabcd...ef01
Initial owner: 0x1234...5678

Minting initial tokens...
Minted 1000000.0 MTK tokens to: 0x1234...5678
Current balance: 1000000.0 MTK

Token Information:
- Name: MyToken
- Symbol: MTK
- Decimals: 18
- Total Supply: 1000000.0 MTK
```

## Interacting with Your Token

After deployment, you can interact with your token using Hardhat console:

```bash
npx hardhat console --network polkadotHub
```

```javascript
// Get contract instance
const MyToken = await ethers.getContractFactory("MyToken");
const token = MyToken.attach("YOUR_CONTRACT_ADDRESS");

// Check balance
const balance = await token.balanceOf("0xYourAddress");
console.log("Balance:", ethers.formatEther(balance), "MTK");

// Transfer tokens
await token.transfer("0xRecipientAddress", ethers.parseEther("100"));

// Approve spending
await token.approve("0xSpenderAddress", ethers.parseEther("50"));

// Check allowance
const allowance = await token.allowance("0xOwner", "0xSpender");
console.log("Allowance:", ethers.formatEther(allowance), "MTK");

// Mint more tokens (owner only)
await token.mint("0xRecipientAddress", ethers.parseEther("10000"));
```

## Configuration

### Network Settings

Edit `hardhat.config.ts` to add more networks:

```typescript
networks: {
  polkadotHub: {
    url: "https://rpc-polkadot-hub-1111.parity-testnet.parity.io",
    chainId: 1111,
    accounts: process.env.PRIVATE_KEY ? [process.env.PRIVATE_KEY] : [],
  },
}
```

### Solidity Version

The contract uses Solidity 0.8.22 to match OpenZeppelin Contracts v5.

### Gas Optimization

The compiler is configured with optimization enabled (200 runs):

```typescript
solidity: {
  version: "0.8.22",
  settings: {
    optimizer: {
      enabled: true,
      runs: 200,
    },
  },
}
```

## Customization

### Change Token Name and Symbol

Edit `contracts/MyToken.sol`:

```solidity
constructor(address initialOwner)
    ERC20("YourTokenName", "SYMBOL")
    Ownable(initialOwner)
{}
```

### Add Initial Supply

Modify the constructor to mint tokens on deployment:

```solidity
constructor(address initialOwner)
    ERC20("MyToken", "MTK")
    Ownable(initialOwner)
{
    _mint(initialOwner, 1000000 * 10**decimals()); // 1 million tokens
}
```

### Add Burnable Functionality

Import and inherit from OpenZeppelin's ERC20Burnable:

```solidity
import {ERC20Burnable} from "@openzeppelin/contracts/token/ERC20/extensions/ERC20Burnable.sol";

contract MyToken is ERC20, ERC20Burnable, Ownable {
    // ...
}
```

### Add Pausable Functionality

Prevent transfers during emergencies:

```solidity
import {ERC20Pausable} from "@openzeppelin/contracts/token/ERC20/extensions/ERC20Pausable.sol";

contract MyToken is ERC20, ERC20Pausable, Ownable {
    function pause() public onlyOwner {
        _pause();
    }

    function unpause() public onlyOwner {
        _unpause();
    }
}
```

## Security Considerations

- **Owner Private Key** - The owner can mint unlimited tokens. Secure the private key.
- **Centralization** - Consider implementing timelocks or multi-sig ownership for production.
- **Audits** - OpenZeppelin contracts are audited, but always audit custom modifications.
- **Upgradability** - This contract is not upgradeable. Consider using proxy patterns for upgradeable tokens.

## Troubleshooting

### "Insufficient funds for gas"

Ensure your wallet has PAS tokens from the [faucet](https://faucet.polkadot.io/?parachain=1111).

### "Nonce too high"

Reset your MetaMask account:
Settings → Advanced → Clear activity tab data

### "Contract not verified"

Contract verification on block explorers may not be available on testnets. Use Hardhat's etherscan plugin for mainnet verification.

### Compilation errors

Ensure you're using the correct Solidity version (0.8.22) matching OpenZeppelin Contracts v5.

## Learn More

- [ERC-20 Token Standard](https://eips.ethereum.org/EIPS/eip-20)
- [OpenZeppelin ERC-20](https://docs.openzeppelin.com/contracts/5.x/erc20)
- [Hardhat Documentation](https://hardhat.org/docs)
- [Ethers.js v6 Documentation](https://docs.ethers.org/v6/)
- [Polkadot Hub Documentation](https://docs.polkadot.com)

## Related Recipes

- `deploy-nft` - Deploy an ERC-721 NFT collection
- `deploy-uniswap-v2` - Deploy a DEX on Polkadot Hub
- `hardhat-testing` - Advanced Hardhat testing patterns

## License

MIT OR Apache-2.0
