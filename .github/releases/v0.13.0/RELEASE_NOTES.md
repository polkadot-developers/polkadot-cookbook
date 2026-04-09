# Release v0.13.0

Released: 2026-04-09

## What's New

### Documentation Tests
- Added test harness for **Pay Fees with a Different Token** guide (chain-interactions pathway)
- Added test harness for **Calculate Transaction Fees** guide (chain-interactions pathway)
- Added test harness for **Uniswap V2 Periphery with Hardhat** guide (smart-contracts pathway)
- Added Chopsticks-based guide pattern to skill references

### Infrastructure
- Added Claude Code **devcontainer** for one-click cloud development environments
- Enhanced **check-docs-drift** skill with version sync and run metrics
- Removed legacy docs-drift CI workflow in favor of the Claude Code skill
- Synced dependency versions from upstream polkadot-docs PR #1606
- Added nhussein11 as code owner for smart contracts

### Bug Fixes
- Bumped pinned commit SHAs for 3 polkadot-docs test harnesses (query-rest, runtime-api-calls, run-a-parachain-network)
- Fixed weekly maintainer tasks issue body formatting
- Aligned Uniswap V2 Periphery harness with polkadot-docs conventions (shared configs, test file naming)
- Updated source URLs after upstream docs restructured periphery page
- Fixed CI cache key to reference `docs.test.ts` after test file rename

## Commits

- feat: add pay-fees-different-tokens polkadot-docs test harness (#237)
- feat: add calculate-transaction-fees polkadot-docs test harness (#234)
- feat: add Uniswap V2 Periphery with Hardhat cookbook guide
- feat: add Claude Code devcontainer (#220)
- feat: add version sync and run metrics to check-docs-drift skill
- fix: bump docs_commit for 3 cosmetic upstream drifts
- fix: update CI cache key to reference docs.test.ts after rename
- fix: align periphery harness with polkadot-docs conventions per review
- fix: properly format weekly issue body with consistent indentation
- fix: strip YAML indentation from weekly issue body
- fix: update source URLs after docs flattened periphery page
- fix: update source URLs, pin commit, remove TESTNET_URL
- chore: sync dependency versions from polkadot-docs PR #1606
- chore: simplify weekly tasks, add periodic section
- chore: add Chopsticks-based guide pattern to skill references (#238)
- chore: improve add-polkadot-docs-test skill (#235)
- chore: add nhussein11 as code owner for smart contracts (#233)
- chore: remove old docs-drift CI workflow
- chore: remove disable-model-invocation from release skill (#225)
- docs: add Uniswap V2 Core entry to polkadot-docs README table
- docs: add Uniswap V2 Periphery entry to polkadot-docs README table

## Compatibility

Tested with:
- Rust: 1.91.0
- Node.js: v24.7.0

---

**Status:** Alpha (v0.x.x)
