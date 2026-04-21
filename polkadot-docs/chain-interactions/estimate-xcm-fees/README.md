# XCM Fee Estimation

[![XCM Fee Estimation](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-estimate-xcm-fees.yml/badge.svg?event=push)](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-estimate-xcm-fees.yml)

Verification tests for the [XCM Fee Estimation](https://docs.polkadot.com/chain-interactions/send-transactions/interoperability/estimate-xcm-fees/) guide.

## What This Tests

This harness verifies that the XCM fee estimation workflow described in the guide works correctly:

1. **Local execution fees** — queries XCM weight on Polkadot Hub (Paseo Asset Hub) via `XcmPaymentApi.query_xcm_weight` and converts weight to PAS using `XcmPaymentApi.query_weight_to_asset_fee`
2. **Delivery fees** — dry-runs the XCM on Polkadot Hub via `DryRunApi.dry_run_xcm`, locates the forwarded message to People Chain, and queries delivery fees
3. **Remote execution fees** — connects to Paseo People Chain and repeats the weight/fee conversion for the forwarded XCM

Two Chopsticks forks run locally during the test suite:
- Paseo Asset Hub on `ws://localhost:8001`
- Paseo People Chain on `ws://localhost:8000`

## Running Locally

```bash
# Install dependencies and generate PAPI descriptors
npm install
npx papi add polkadotHub -n paseo_asset_hub
npx papi add paseoPeopleChain -w wss://people-paseo.rpc.amforc.com

# Run tests (Chopsticks forks are started automatically)
npm test
```

## Guide

Source: [docs.polkadot.com](https://docs.polkadot.com/chain-interactions/send-transactions/interoperability/estimate-xcm-fees/)
