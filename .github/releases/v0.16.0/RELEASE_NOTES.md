<div align="center">
  <img src="https://raw.githubusercontent.com/polkadot-developers/polkadot-cookbook/v0.16.0/.github/releases/v0.16.0/cover.svg" alt="Release v0.16.0" width="100%" />
</div>

# Release v0.16.0

Released: 2026-04-19

## Summary

This release delivers a new documentation test harness for **Uniswap V3 Core with Hardhat**, giving developers end-to-end verification that 187 Uniswap contracts compile and pass all 187 Hardhat tests on Polkadot's local network. It also ships a product-grade v2 brand palette with the new Index Mark and re-cut hero/social surfaces, plus a ParaSpell SDK bump to v13.2.2 that aligns the transfer-assets-parachains harness with upstream PAPI v2.

## What's New

### Documentation Tests

- Added Uniswap V3 Core with Hardhat test harness — compiles 187 contracts, runs 187 Hardhat tests on the local network, and soft-fails testnet deployment when credentials are unavailable, so developers can verify the full Uniswap V3 integration works on Polkadot before shipping (#272)
- Bumped ParaSpell SDK to **v13.2.2** and aligned transfer-assets-parachains with upstream rename (`.address` → `.recipient`, `.senderAddress` → `.sender`) — keeps the harness in lockstep with [polkadot-developers/polkadot-docs#1639](https://github.com/polkadot-developers/polkadot-docs/pull/1639) and the rest of the cookbook's PAPI v2 surface (#273)

### Tooling & Brand

- Evolved the brand system to a **v2 product palette**: near-black canvas `#0A0A0B` and warm paper `#F6F5F2` replace pure black/white, an 8-value grey ramp covers surfaces and muted text, and JetBrains Mono becomes the primary typeface (#274)
- Introduced the **Index Mark** (page-of-recipes glyph) as the new brand mark, replacing the orbital network mark; re-cut the hero image to a 1200×400 two-panel layout and updated release cover templates with the v2 palette and font stack (#274)

## Commits

- feat(brand): v2 product palette, Index Mark, and re-cut surfaces (#274)
- feat: add Uniswap V3 Core with Hardhat CI harness (#272)
- chore(paraspell): bump to v13.2.2 and align transfer-assets-parachains with upstream (#273)

## Stats

**3 commits, +3,469 / -1,516 lines**

**Full Changelog:** https://github.com/polkadot-developers/polkadot-cookbook/compare/v0.15.1...v0.16.0

## Compatibility

Tested with:
- Rust: 1.91.0
- Node.js: v24.7.0

## Next Steps

Merging the release PR triggers [`publish-release.yml`](../../../.github/workflows/publish-release.yml), which:
1. Creates the `v0.16.0` git tag
2. Builds the `dot` CLI binaries for Linux, macOS (Intel + ARM), and Windows
3. Publishes the GitHub Release with cover art, manifest, and binaries attached

---

**Status:** Alpha (v0.x.x)

---

<div align="center">
  <img src="https://raw.githubusercontent.com/polkadot-developers/polkadot-cookbook/v0.16.0/.github/releases/v0.16.0/cover-chain.svg" alt="Polkadot network state at v0.16.0 release" width="100%" />
</div>
