# Release v0.11.0

Released: 2026-02-12

## Changes Since v0.10.1

**Version Bump:** MINOR

### Highlights

- **SDK: clone + overlay architecture** — `dot create` now clones the upstream `polkadot-sdk-parachain-template` at a pinned git tag instead of embedding ~40 template files at compile time. Templates are cached at `~/.dot/templates/` after first clone. Updating to a new upstream version is now a 2-constant change. (#96)

### Changes

- feat(sdk): replace embedded template with clone + overlay (#96) (31f56ea)
- ci: add auto PR description workflow (#100) (fbfc19d)
- feat: add uniswap v2 core revm migration (#92) (98b53aa)
- refactor: restructure recipes to test-only harnesses with external repos (#91) (748da67)
- ci: change polkadot-docs schedule to weekly on Sundays (#88) (8386714)

## Compatibility

This release was tested with:
- Rust: 1.91.0
- Node.js: v24.7.0

## Testing

All recipes have passed CI tests.

Full manifest: [manifest.yml](./manifest.yml)

---

**Status:** Alpha (v0.x.x)
