# Release v0.1.0

Released: 2026-02-12

## Changes Since v0.0.0

**Version Bump:** MINOR

### Changes

- docs: replace recipe tables with blockquote format (#104) (a1e6ac6)
- fix(ci): fetch tags in release workflow (#106) (4052541)
- feat(sdk): replace embedded template with clone + overlay (#96) (31f56ea)
- ci: add auto PR description workflow (#100) (fbfc19d)
- feat: add uniswap v2 core revm migration (#92) (98b53aa)
- refactor: restructure recipes to test-only harnesses with external repos (#91) (748da67)
- ci: change polkadot-docs schedule to weekly on Sundays (#88) (8386714)
- chore(release): trigger fresh v0.10.1 release build (#86) (0903f3f)
- chore: sync Cargo.lock with v0.10.1 version bump (#85) (344ef0b)
- Release v0.10.1 (#84) (6f60b03)
- Fix CLI version and automate version bumping (#83) (9547832)
- feat(polkadot-docs): add test for install-polkadot-sdk guide (#77) (30b7960)
- feat(polkadot-docs): add test for benchmark-pallet guide (#76) (7b0a1e3)
- feat(polkadot-docs): add test for pallet-testing guide (#75) (3a5c9f5)
- feat(polkadot-docs): add test for mock-runtime guide (#74) (b190741)
- feat(polkadot-docs): add test for add-pallet-instances guide (#71) (3180521)
- refactor(ci): split polkadot-docs tests into separate workflows (#70) (e8720bc)
- chore: retrigger v0.7.1 release with CI fix (#69) (cd7f524)
- fix(ci): pin cross version to 0.2.4 for Rust 1.91 compatibility (#68) (c6f5b24)
- docs: add status badges to individual recipe READMEs (#66) (9866313)
- feat: add test for 'Add Existing Pallets' guide (#65) (8c43cba)
- fix: move XCM example to cross-chain-transactions folder (#59) (30c3ec0)
- fix: move xcm-example to correct location matching documentation (#58) (3cd3663)
- refactor: standardize example naming and remove pathway column from README (#57) (9e85cd0)
- test: fix pathway integration and parallel test issues (#55) (ae1b916)
- refactor: major CLI simplification and comprehensive improvements (#53) (4ec1902)
- hotfix: fix CLI path in release workflows (#54) (db940fa)
- refactor: major CLI simplification and dependency checking (#51) (cbde66c)
- refactor: comprehensive CLI improvements and repository modernization (#50) (b949ecd)
- refactor: streamline documentation and improve CLI-first workflow (#49) (79e8a53)
- Release v0.3.0 (#48) (a8b258e)
- feat: add automatic API breaking change detection (#46) (3705667)
- Release v0.2.0 (#47) (41dda9e)
- fix: update weekly release workflow to detect squash-merged PRs (#45) (b10a8eb)
- refactor: restructure CLI command hierarchy and remove slug parameter (#44) (21c78f7)
- Rename macOS binaries for clarity (#43) (563b41f)
- Release v0.1.0 (#42) (273e776)
- Fix binary location for workspace builds (#41) (da6da5f)
- Release v0.1.0 (#40) (4268eba)
- Fix CLI binary preparation in release workflows (#39) (dd3478d)
- Release v0.1.0 (#38) (862c0af)
- Fix CLI binary builds for cross-platform releases (#37) (84a209d)
- Release v0.1.0 (#36) (91cca14)
- fix(workflows): add CLI binaries to releases and remove branding (#35) (6acbe9a)
- Revert "Release v0.1.0 (#34)" (457fb24)
- Release v0.1.0 (#34) (4c4ca54)
- fix(workflows): create release PRs instead of pushing directly to master (#33) (b8bf678)
- fix(workflows): start Chopsticks before running XCM recipe tests (#32) (54c8262)
- fix(workflows): multiple workflow fixes for release and testing (#31) (54cb812)
- fix(workflow): add explicit job-level condition for pull_request events (#30) (f50a939)
- fix(workflow): remove PR-specific ref from checkout step (#29) (fc60cbd)
- fix(workflow): add guard for non-pull-request events in semantic labeler (#28) (8602d0b)
- docs: simplify CONTRIBUTING.md and fix anchor links (#27) (48dc30b)
- feat(release): Automated Semantic Versioning and Multi-Platform CLI Releases (#26) (9f076d4)
- fix: remove automatic coverage badge commit to resolve branch protection conflict (#25) (e603df7)
- fix: remove CONTRIBUTING.md style tags and resolve workflow coverage badge commit failure (#24) (d87e09c)
- Streamline contributor flow with recipe templates and infrastructure improvements (#23) (0ef512f)


## Compatibility

This release was tested with:
- Rust: 1.91.1
- Node.js: v20.20.0

## Testing

All recipes have passed CI tests.

Full manifest: [manifest.yml](./manifest.yml)

---

**Status:** Alpha (v0.x.x)
