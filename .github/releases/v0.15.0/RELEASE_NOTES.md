<div align="center">
  <img src="https://raw.githubusercontent.com/polkadot-developers/polkadot-cookbook/v0.15.0/.github/releases/v0.15.0/cover.svg" alt="Release v0.15.0" width="100%" />
</div>

# Release v0.15.0

Released: 2026-04-16

## Summary

v0.15.0 delivers a complete brand system refactor: every visual surface — README hero, dividers, social preview, OG image, per-pathway banners, CONTRIBUTING hero, favicon, and the release skill's cover art — now derives from a strict 3-color Polkadot palette (#E6007A pink, black, white) via tokens in `.github/brand/`, with CI-enforced palette discipline, drift prevention, and a11y checks. Also ships the `/release --dry-run` mode, a new chain-state footer cover pulled live via JSON-RPC at release-cut time, and one new polkadot-docs test harness (Transfer Assets Across Parachains).



## What's New

### Brand System

- feat(brand): mondrian brand system, /branding skill, and CI enforcement — continuous refinement of the Cookbook's visual identity (#266)

### Release Skill

- fix(release-skill): TEMPLATE_HEADER_END sentinels for unambiguous stripping (#264)
- chore(release-skill): add argument-hint for --dry-run — preview every rendered release artifact without any git or GitHub mutations (#263)
- feat(release-skill): add --dry-run mode — preview every rendered release artifact without any git or GitHub mutations (#262)
- feat(release-skill): footer cover-chain with Polkadot network reading — every release now captures a point-in-time reading of Polkadot mainnet (block number, runtime spec version, node version) via JSON-RPC (#261)
- feat(release-skill): template-driven fact-bound cover art — cover art is now fact-bound to git facts — no hand-designed per-release variations (b67f316)

### Documentation Tests

- feat: add transfer-assets-parachains polkadot-docs test harness — developers can verify cross-chain asset transfer flows work end-to-end before touching production (#258)

### Dependencies

- chore(polkadot-api): bump to v2.0.1 and refactor harnesses — harnesses now pin polkadot-api v2.0.1 with cleaner imports (#265)

### CI / Infrastructure

- fix(ci): wire polkadot_sdk version into run-a-parachain-network workflow — CI now drives the parachain network test from the canonical polkadot_sdk version in versions.yml (#260)

### Documentation

- chore: document frontmatter badge pattern in add-polkadot-docs-test skill — the /add-polkadot-docs-test skill now documents how to add the polkadot-docs CI-badge frontmatter (#259)

## Commits

- feat(release-skill): add --dry-run mode (#262)
- feat: add transfer-assets-parachains polkadot-docs test harness (#258)
- feat(brand): mondrian brand system, /branding skill, and CI enforcement (#266)
- feat(release-skill): template-driven fact-bound cover art (b67f316)
- feat(release-skill): footer cover-chain with Polkadot network reading (#261)
- fix(release-skill): TEMPLATE_HEADER_END sentinels for unambiguous stripping (#264)
- fix(ci): wire polkadot_sdk version into run-a-parachain-network workflow (#260)
- chore(polkadot-api): bump to v2.0.1 and refactor harnesses (#265)
- chore(release-skill): add argument-hint for --dry-run (#263)
- chore: document frontmatter badge pattern in add-polkadot-docs-test skill (#259)

## Stats

**10 commits, +11,944 / -8,420 lines**

**Full Changelog:** https://github.com/polkadot-developers/polkadot-cookbook/compare/v0.14.0...v0.15.0

## Compatibility

Tested with:
- Rust: 1.91.0
- Node.js: v24.7.0

## Next Steps

Merging the release PR triggers [`publish-release.yml`](../../../.github/workflows/publish-release.yml), which:
1. Creates the `v0.15.0` git tag
2. Builds the `dot` CLI binaries for Linux, macOS (Intel + ARM), and Windows
3. Publishes the GitHub Release with cover art, manifest, and binaries attached

---

**Status:** Alpha (v0.x.x)

---

<div align="center">
  <img src="https://raw.githubusercontent.com/polkadot-developers/polkadot-cookbook/v0.15.0/.github/releases/v0.15.0/cover-chain.svg" alt="Polkadot network state at v0.15.0 release" width="100%" />
</div>
