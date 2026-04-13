# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/), and this project adheres to [Semantic Versioning](https://semver.org/).

## [Unreleased]

## [0.14.0] - 2026-04-13

### Added
- Test harness for **Foundry dev environment** polkadot-docs guide
- Test harness for **NFT Hardhat** polkadot-docs guide
- Test harness for **Zero-to-Hero dApp** polkadot-docs guide
- Test harness for **Send Transactions** polkadot-docs guide with Rust subxt binary
- Mondrian cover art, wordmark, and release notes polish in /release skill
- Every release-notes commit now carries a `(#N)` PR link

### Fixed
- `publish-release.yml` duplicate tag check no longer fails on re-runs

## [0.13.0] - 2026-04-09

### Added
- Test harness for **Pay Fees with a Different Token** guide with Rust subxt binary
- Test harness for **Calculate Transaction Fees** guide
- Test harness for **Uniswap V2 Periphery with Hardhat** guide
- Claude Code devcontainer for one-click cloud development environments
- Version sync and run metrics to check-docs-drift skill
- Chopsticks-based guide pattern to add-polkadot-docs-test skill
- nhussein11 as code owner for smart contracts pathway
- Shared configs: `shared/tsconfig.base.json` and `shared/vitest.shared.ts`

### Changed
- Overhauled /release skill with CHANGELOG.md generation, contributor credits, breaking change detection, diff links, migration notes, bump override, and squash merge handling
- Synced dependency versions from polkadot-docs PR #1606
- Replaced legacy `sync-versions.yml` and `check-docs-drift.sh` with Claude Code skills

### Fixed
- Bumped pinned commit SHAs for 3 polkadot-docs test harnesses (query-rest, runtime-api-calls, run-a-parachain-network)
- Weekly maintainer tasks issue body formatting
- Uniswap V2 Periphery harness alignment with polkadot-docs conventions
- Source URLs after upstream docs restructured periphery page
- CI cache key to reference `docs.test.ts` after test file rename

[Unreleased]: https://github.com/polkadot-developers/polkadot-cookbook/compare/v0.14.0...HEAD
[0.14.0]: https://github.com/polkadot-developers/polkadot-cookbook/compare/v0.13.0...v0.14.0
[0.13.0]: https://github.com/polkadot-developers/polkadot-cookbook/compare/v0.12.0...v0.13.0
