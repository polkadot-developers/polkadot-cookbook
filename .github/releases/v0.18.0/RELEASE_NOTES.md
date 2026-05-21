
<div align="center">
  <img src="https://raw.githubusercontent.com/polkadot-developers/polkadot-cookbook/v0.18.0/.github/releases/v0.18.0/cover.svg" alt="Release v0.18.0" width="100%" />
</div>

# Release v0.18.0

Released: 2026-05-21

## Summary

This release adds two new documentation test harnesses — **Transfer Assets into Polkadot** (XCM) and **Uniswap V3 Periphery with Hardhat** (smart contracts) — extending coverage of the cross-chain and EVM tutorial surface. A new self-healing CI workflow now tracks JavaScript dependency drift in `versions.yml` against an automatically-managed `versions-drift` issue, and several brand templates were tightened to remove misleading captions and live counts.

## What's New

### Documentation Tests

- Added test harness for **Transfer Assets into Polkadot** — verifies the XCM flow that moves assets from a remote consumer parachain into Polkadot, so changes to the guide or upstream SDK can no longer silently break this cross-chain pattern (#288)
- Added test harness for **Uniswap V3 Periphery with Hardhat** — exercises the full Uniswap V3 deployment pipeline against pallet-revive, ensuring the EVM smart-contract tutorial keeps working as Hardhat and the dev node evolve (#286)

### Infrastructure

- Added a self-healing CI workflow that detects when JavaScript harnesses fall below the `versions.yml` floor and opens/updates a single tracked `versions-drift` issue, eliminating manual auditing of pinned JS versions across dozens of harnesses (#282)

### Tooling

- Resolved a conflict between the release skill's commit step and the local commit-msg hook so future releases no longer require manual intervention to bypass the hook (#285)
- Removed the misleading `RECIPES` caption from pathway banners — the banners are used by both recipes and polkadot-docs, so the label was inaccurate (#289)
- Removed live counts from hero, contributing-hero, and pathway banner templates — the numbers drifted between local renders and CI, undermining the apple.com-level polish those surfaces aim for (#287)

## Commits

- feat: add transfer-assets-into-polkadot polkadot-docs test harness (#288)
- feat: add Uniswap V3 Periphery with Hardhat CI harness (#286)
- ci: track versions.yml JS drift via self-healing issue (#282)
- chore(brand): drop misleading 'RECIPES' caption from pathway banner (#289)
- chore(brand): remove live counts from hero/contributing-hero/pathway templates (#287)
- chore(release-skill): resolve branch commit vs. commit-msg hook conflict (#285)

## Stats

**6 commits, +6,909 / -7,689 lines**

**Full Changelog:** https://github.com/polkadot-developers/polkadot-cookbook/compare/v0.17.0...v0.18.0

## Compatibility

Tested with:
- Rust: 1.91.0
- Node.js: v24.7.0

## Next Steps

Merging the release PR triggers [`publish-release.yml`](../../../.github/workflows/publish-release.yml), which:
1. Creates the `v0.18.0` git tag
2. Builds the `dot` CLI binaries for Linux, macOS (Intel + ARM), and Windows
3. Publishes the GitHub Release with cover art, manifest, and binaries attached

---

**Status:** Alpha (v0.x.x)

---

<div align="center">
  <img src="https://raw.githubusercontent.com/polkadot-developers/polkadot-cookbook/v0.18.0/.github/releases/v0.18.0/cover-chain.svg" alt="Polkadot network state at v0.18.0 release" width="100%" />
</div>
