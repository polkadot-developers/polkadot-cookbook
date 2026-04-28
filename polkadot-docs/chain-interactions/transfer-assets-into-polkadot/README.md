# Transfer Assets from Ethereum into Polkadot — Test Harness

Verifies the [Transfer Assets from Ethereum into Polkadot](https://docs.polkadot.com/chain-interactions/send-transactions/interoperability/transfer-assets-into-polkadot/) guide.

## What Is Tested

| # | Test | Live Network Required |
|---|------|-----------------------|
| 1 | `getSupportedAssets` returns non-empty list for Ethereum → AssetHubPolkadot | No |
| 1 | WETH is present in the supported asset list | No |
| 1 | All returned assets have a `symbol` field | No |
| 2 | `getSupportedAssets` returns assets for Ethereum → Hydration route | No |
| 3 | `EXTERNAL_CHAINS` constant includes `"Ethereum"` | No |
| 3 | Both AssetHubPolkadot and Hydration destinations return supported assets | No |
| 4 | `EvmBuilder` can be constructed for Ethereum → AssetHubPolkadot | No |
| 4 | `EvmBuilder` can be constructed for Ethereum → Hydration | No |
| 5 | Live bridge transfer sign + submit | **Yes** (skipped without `ETH_PRIVATE_KEY`) |

## Running Locally

```bash
npm ci
npm test
```

## Optional: Live Bridge Transfer Test

Set `ETH_PRIVATE_KEY` to an Ethereum wallet with WETH on mainnet (plus ETH for gas) to run the live bridge submission test:

```bash
ETH_PRIVATE_KEY=0x... ETH_RPC_URL=https://eth.llamarpc.com npm test
```

> **Warning:** The live test bridges real WETH from Ethereum to Polkadot Hub. Use a test wallet with minimal funds.

## Dependencies

| Package | Version |
|---------|---------|
| `@paraspell/sdk-pjs` | `13.2.2` |
| `@polkadot/api` | `16.5.6` |
| `ethers` | `^6.16.0` |

Versions follow `versions.yml` at the repository root.
