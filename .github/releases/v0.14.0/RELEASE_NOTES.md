<div align="center">
  <img src="https://raw.githubusercontent.com/polkadot-developers/polkadot-cookbook/v0.14.0/.github/releases/v0.14.0/cover.svg" alt="Release v0.14.0" width="100%" />
</div>

# Release v0.14.0

Released: 2026-04-13

## Summary

This release adds four new polkadot-docs test harnesses covering smart contract development environments, NFT workflows, full-stack dApps, and chain-interaction transactions — giving developers automated verification that official tutorials stay working end-to-end. It also folds in post-v0.13.0 fixes to the release pipeline and release-notes tooling.

## What's New

### Documentation Tests
- Added test harness for **Foundry dev environment** guide — developers can now verify the Foundry smart contract workflow on Polkadot Hub works before following it (#230)
- Added test harness for the **NFT Hardhat** guide — the ERC-721 minting and deployment flow is now covered by CI, catching breaks in the NFT tutorial before users hit them (#228)
- Added test harness for the **Zero-to-Hero dApp** guide — the full-stack tutorial is now verified end-to-end, so learners starting from scratch get a path that's known-good (#229)
- Added test harness for the **Send Transactions** guide (chain-interactions pathway) with a Rust `subxt` binary, ensuring the transfer flow and metadata stay in sync with upstream (#232)

### Infrastructure
- Fixed `publish-release.yml` duplicate tag check so re-running the publish workflow no longer fails when the tag already exists (#254)
- Added Mondrian cover art, wordmark, and release notes polish to the `/release` skill so every release ships with unique generative branding (#253)
- Ensured every commit in release notes carries a `(#N)` PR link — makes rendered notes fully navigable on GitHub (#252)

## Commits

- feat: add send-transactions polkadot-docs test harness (#232)
- feat: add nft-hardhat polkadot-docs test harness (#228)
- feat: add zero-to-hero-dapp polkadot-docs test harness (#229)
- feat: add foundry polkadot-docs test harness (#230)
- fix: publish-release tag duplicate check (#254)
- chore: add Mondrian cover art, branding, and release notes polish (#253)
- chore: ensure all release note commits have PR links (#252)

## Stats

**7 commits, +16,595 / -14 lines**

**Full Changelog:** https://github.com/polkadot-developers/polkadot-cookbook/compare/v0.13.0...v0.14.0

## Compatibility

Tested with:
- Rust: 1.91.0
- Node.js: v24.7.0

---

**Status:** Alpha (v0.x.x)
