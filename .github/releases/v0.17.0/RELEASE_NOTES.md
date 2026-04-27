
<div align="center">
  <img src="https://raw.githubusercontent.com/polkadot-developers/polkadot-cookbook/v0.17.0/.github/releases/v0.17.0/cover.svg" alt="Release v0.17.0" width="100%" />
</div>

# Release v0.17.0

Released: 2026-04-22

## Summary

This release lands five new `polkadot-docs` test harnesses covering the asset-management and XCM tutorials — register foreign asset, register local asset, convert assets, estimate XCM fees, and debug & preview XCMs — so every one of those guides is now continuously verified against the pinned toolchain. It also tightens the release skill's commit-list and Rust-crates accounting, and fixes mobile rendering of the README hero block.

## What's New

### Documentation Tests

- Added test harness for **Estimate XCM Fees** guide — gives developers a runnable end-to-end verification of the fee-estimation flow across Paseo Asset Hub and Paseo People Chain before they port the pattern into their own apps (#279)
- Added test harness for **Debug and Preview XCMs** guide — the guide now has CI coverage that the dry-run, trace, and preview flows work against the pinned Polkadot Hub build, so readers aren't the first to hit a regression (#281)
- Added test harness for **Register a Local Asset** guide — the local-asset registration steps are now continuously verified against the pinned Asset Hub runtime, catching upstream drift before it hits the published docs (#280)
- Added test harness for **Convert Assets** guide — readers get confidence that the swap / liquidity flows compile and execute against the current Asset Hub interface, not a stale snapshot (#267)
- Added test harness for **Register a Foreign Asset** guide — foreign-asset registration is now continuously tested end-to-end, closing one of the last gaps in the asset-management tutorial coverage (#268)

### Documentation

- Fixed README mobile rendering — collapsed the dual icons into a single responsive asset and centered the hero block so first-time visitors on phones see the intended landing page instead of a broken layout (#277)

### Tooling

- Tightened the release skill's Rust-crates counter and commit-list truncation — cover art now reflects the actual crate count and long histories no longer spill off the page (#276)

## Commits

- feat: add estimate-xcm-fees polkadot-docs test harness (#279)
- feat: add debug and preview xcms guide (#281)
- feat: add register-local-asset polkadot-docs test harness (#280)
- feat: add convert-assets polkadot-docs test harness (#267)
- feat: add register foreign asset test harness (#268)
- docs(readme): fix mobile rendering — collapse dual icons, center hero block (#277)
- chore(release-skill): tighten Rust-crates count and commit-list truncation (#276)

## Stats

**7 commits, +37,454 / -58 lines**

**Full Changelog:** https://github.com/polkadot-developers/polkadot-cookbook/compare/v0.16.0...v0.17.0

## Compatibility

Tested with:
- Rust: 1.91.0
- Node.js: v24.7.0

## Next Steps

Merging the release PR triggers [`publish-release.yml`](../../../.github/workflows/publish-release.yml), which:
1. Creates the `v0.17.0` git tag
2. Builds the `dot` CLI binaries for Linux, macOS (Intel + ARM), and Windows
3. Publishes the GitHub Release with cover art, manifest, and binaries attached

---

**Status:** Alpha (v0.x.x)

---

<div align="center">
  <img src="https://raw.githubusercontent.com/polkadot-developers/polkadot-cookbook/v0.17.0/.github/releases/v0.17.0/cover-chain.svg" alt="Polkadot network state at v0.17.0 release" width="100%" />
</div>
