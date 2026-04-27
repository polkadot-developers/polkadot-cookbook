# XCM Fee Estimation

[![XCM Fee Estimation](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-estimate-xcm-fees.yml/badge.svg?event=push)](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-estimate-xcm-fees.yml)

Verification tests for the [XCM Fee Estimation](https://docs.polkadot.com/chain-interactions/send-transactions/interoperability/estimate-xcm-fees/) guide.

## What This Tests

Uses two local Chopsticks forks — Paseo Asset Hub (port 8001) and Paseo People Chain (port 8000) — and queries fee estimation APIs via `@polkadot/api` runtime calls:

1. **XCM weight on Polkadot Hub** — calls `XcmPaymentApi.queryXcmWeight` with a V4 teleport message and asserts `refTime > 0`
2. **Fee conversion on Polkadot Hub** — converts the weight to PAS via `XcmPaymentApi.queryWeightToAssetFee` and asserts `0 < fee < 10 PAS`
3. **Dry-run on Polkadot Hub** — calls `DryRunApi.dryRunXcm` with an Alice origin and asserts the execution result is `Ok`
4. **Remote XCM weight on People Chain** — repeats the weight query on Paseo People Chain for the receive-side message and asserts `refTime > 0`
5. **Remote fee conversion on People Chain** — converts the remote weight to PAS and asserts `0 < fee < 10 PAS`

## Running Locally

```bash
# Install dependencies
npm install

# Run tests (Chopsticks forks are started automatically)
npm test
```

## Guide

Source: [docs.polkadot.com](https://docs.polkadot.com/chain-interactions/send-transactions/interoperability/estimate-xcm-fees/)
