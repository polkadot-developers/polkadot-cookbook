---
title: "Paseo Local Network with Asset Hub"
description: "Spin up a local Paseo relay chain with Asset Hub parachain using Zombienet"
source_repo: "https://github.com/brunopgalvao/recipe-paseo-local-network"
---

# Paseo Local Network with Asset Hub

[![Paseo Local Network with Asset Hub](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/recipe-paseo-local-network-example.yml/badge.svg?event=push)](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/recipe-paseo-local-network-example.yml)

[Paseo Local Network with Asset Hub](https://github.com/brunopgalvao/recipe-paseo-local-network)

## Running Tests

```bash
npm ci
npm test
```

## What This Test Verifies

1. Clones the [recipe-paseo-local-network](https://github.com/brunopgalvao/recipe-paseo-local-network) repository at a pinned version
2. Installs dependencies
3. Runs the recipe's test suite which:
   - Downloads Polkadot and polkadot-parachain binaries
   - Downloads Paseo and Asset Hub runtime WASMs
   - Generates local chain specs using `chain-spec-builder`
   - Spawns a Zombienet network (2 relay validators + 1 Asset Hub collator)
   - Verifies block production on both relay chain and parachain
   - Verifies dev accounts (Alice, Bob) on both chains

## Source Repository

[brunopgalvao/recipe-paseo-local-network](https://github.com/brunopgalvao/recipe-paseo-local-network)
