# Release v0.13.0

Released: 2026-04-09

> Three new documentation test harnesses, a Claude Code devcontainer, and a professionally overhauled release pipeline.

## What's New

### Documentation Tests
- Added test harness for **Pay Fees with a Different Token** guide (chain-interactions pathway), including a Rust subxt binary for fee payment transactions
- Added test harness for **Calculate Transaction Fees** guide (chain-interactions pathway)
- Added test harness for **Uniswap V2 Periphery with Hardhat** guide (smart-contracts pathway)

### Infrastructure
- Added **Claude Code devcontainer** for one-click cloud development environments with pre-installed Rust, Node.js, and project tooling
- Overhauled **/release skill** with CHANGELOG.md generation, contributor credits, breaking change detection, full diff links, migration notes, bump override support, and squash merge handling
- Enhanced **/check-docs-drift skill** with version sync and run metrics
- Added Chopsticks-based guide pattern to add-polkadot-docs-test skill references
- Removed legacy `sync-versions.yml` and `check-docs-drift.sh` in favor of Claude Code skills
- Added @nhussein11 as code owner for smart contracts pathway

### Bug Fixes
- Bumped pinned commit SHAs for 3 polkadot-docs test harnesses to track upstream cosmetic changes (query-rest, runtime-api-calls, run-a-parachain-network)
- Fixed weekly maintainer tasks issue body formatting (YAML indentation and consistent structure)
- Aligned Uniswap V2 Periphery harness with polkadot-docs conventions (shared configs, renamed test file to `docs.test.ts`)
- Updated source URLs after upstream docs restructured the periphery page
- Fixed CI cache key to reference `docs.test.ts` after test file rename

## Migration Notes
- **versions.yml** updated with dependency versions synced from polkadot-docs PR #1606 — downstream harnesses using `@polkadot/api`, `@polkadot/util`, or chopsticks should verify compatibility
- **Shared configs** added: `shared/tsconfig.base.json` and `shared/vitest.shared.ts` — new harnesses should extend these instead of inlining configs

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
- chore: enhance /release skill with professional open-source standards (#248)
- chore: fix 5 issues in /release skill from dry run (#250)
- docs: add Uniswap V2 Core entry to polkadot-docs README table
- docs: add Uniswap V2 Periphery entry to polkadot-docs README table

## Contributors

@brunopgalvao, @nhussein11, @sekisamu

## Stats

**23 commits, 3 contributors, +23,753 / -1,399 lines**

**Full Changelog:** https://github.com/polkadot-developers/polkadot-cookbook/compare/v0.12.0...v0.13.0

## Compatibility

Tested with:
- Rust: 1.91.0
- Node.js: v24.7.0

---

**Status:** Alpha (v0.x.x)
