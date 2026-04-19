# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/), and this project adheres to [Semantic Versioning](https://semver.org/).

## [Unreleased]

## [0.16.0] - 2026-04-19

### Added
- Test harness for **Uniswap V3 Core with Hardhat** polkadot-docs guide — clones the pinned commit, compiles 187 contracts, runs 187 Hardhat tests on the local network, and soft-fails testnet deployment when credentials are unavailable
- v2 product palette (`#0A0A0B` canvas, `#F6F5F2` paper, 8-value grey ramp) with JetBrains Mono as primary typeface
- Index Mark (page-of-recipes glyph) as the new brand mark, replacing the orbital network mark
- Wordmark template combining Index Mark with stacked text

### Changed
- Hero image resized from 1200×630 to 1200×400 with a two-panel layout
- Release cover templates (`cover.svg.template`, `cover-chain.svg.template`) updated to v2 palette and JetBrains Mono font stack
- Pathway banners now inject per-pathway SVG glyphs via `PATHWAY_GLYPH` token
- ParaSpell SDK bumped to v13.2.2 and `transfer-assets-parachains` aligned with upstream rename (`.address` → `.recipient`, `.senderAddress` → `.sender`), adding PAPI v2 compatibility

## [0.15.1] - 2026-04-16

### Changed
- `/release` real-run now hard-asserts zero unresolved template placeholders after every render step (parity with the dry-run exit criteria). Prevents a recurrence of the v0.15.0 cover-chain.svg incident where ~25 `{{TOKEN}}` placeholders shipped visibly to the draft PR because `xmllint` alone passes on unresolved braces.
- `/release` now requires a 1-2 sentence narration before every non-trivial tool call, so Claude Code permission prompts surface context alongside the raw command.
- `/release` no longer opens generated artifacts locally via the default application; the PR page is the canonical review surface.

## [0.15.0] - 2026-04-16

### Added
- Complete brand system at `.github/brand/` (tokens, DESIGN.md, ARCHITECTURE.md, voice.md, CHANGELOG.md, scripts) with strict 3-color palette derived from original Polkadot brand (`#E6007A` / `#000000` / `#FFFFFF`)
- `/branding` skill (`.claude/skills/branding/`) generates 15 SVGs + 2 PNGs in dark + light modes from one source of truth
- README hero + divider + CONTRIBUTING hero now generated; social-preview + OG image (1200×630) with rasterized PNGs
- Pathway banners (Pallets / Contracts / Transactions / XCM / Networks) fact-bound to live `recipes/*` counts
- Favicon at `docs/favicon.svg`
- Issue templates (`bug.yml`, `recipe-request.yml`, `docs.yml`) + PR template with required Test Plan checklist
- Brand Lint CI workflow: palette lint, release-cover-palette verify, drift check, a11y check, CHANGELOG-tokens guard
- Brand Regenerate CI workflow: auto-PR on master token changes
- `/release --dry-run` mode previews every artifact in a scratch dir without git or GitHub mutations
- Chain-state footer cover (`cover-chain.svg`) captured via JSON-RPC at release-cut time
- Template-driven, fact-bound cover art in the release skill
- Test harness for **Transfer Assets Across Parachains** polkadot-docs guide
- `TEMPLATE_HEADER_END` sentinel convention for unambiguous template-doc-header stripping
- `/release` argument-hint for `--dry-run`
- Frontmatter badge pattern documented in `/add-polkadot-docs-test` skill

### Changed
- Release skill cover templates (`cover.svg.template`, `cover-chain.svg.template`) migrated to strict pink/black/white palette; fix-commit bars differentiated by opacity, not a secondary hue
- polkadot-api harnesses pinned to v2.0.1 with refactored imports
- `run-a-parachain-network` workflow now wires `polkadot_sdk` version from `versions.yml`

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

[Unreleased]: https://github.com/polkadot-developers/polkadot-cookbook/compare/v0.16.0...HEAD
[0.16.0]: https://github.com/polkadot-developers/polkadot-cookbook/compare/v0.15.1...v0.16.0
[0.15.1]: https://github.com/polkadot-developers/polkadot-cookbook/compare/v0.15.0...v0.15.1
[0.15.0]: https://github.com/polkadot-developers/polkadot-cookbook/compare/v0.14.0...v0.15.0
[0.14.0]: https://github.com/polkadot-developers/polkadot-cookbook/compare/v0.13.0...v0.14.0
[0.13.0]: https://github.com/polkadot-developers/polkadot-cookbook/compare/v0.12.0...v0.13.0
