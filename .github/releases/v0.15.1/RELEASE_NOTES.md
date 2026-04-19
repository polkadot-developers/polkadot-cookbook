<div align="center">
  <img src="https://raw.githubusercontent.com/polkadot-developers/polkadot-cookbook/v0.15.1/.github/releases/v0.15.1/cover.svg" alt="Release v0.15.1" width="100%" />
</div>

# Release v0.15.1

Released: 2026-04-16

## Summary

v0.15.1 is a patch release hardening the /release skill itself. Real-run now hard-asserts zero unresolved template placeholders after every render (parity with the dry-run exit criteria), requires 1-2 sentence narration before every non-trivial tool call so permission prompts show context alongside the command, and stops opening local previews of generated artifacts (the PR page is the canonical review surface). No functional or CLI-surface changes.



## What's New

### Release Skill

- docs(release-skill): zero-token assertion, narration cadence, no local preview — real-run now asserts zero unresolved template placeholders (parity with DRY_RUN.md) and each tool call gets a 1-2 sentence narration; v0.15.0 shipped with 25 visible unresolved tokens before this was in place (#270)

## Commits

- docs(release-skill): zero-token assertion, narration cadence, no local preview (#270)

## Stats

**1 commits, +36 / -2 lines**

**Full Changelog:** https://github.com/polkadot-developers/polkadot-cookbook/compare/v0.15.0...v0.15.1

## Compatibility

Tested with:
- Rust: 1.91.0
- Node.js: v24.7.0

## Next Steps

Merging the release PR triggers [`publish-release.yml`](../../../.github/workflows/publish-release.yml), which:
1. Creates the `v0.15.1` git tag
2. Builds the `dot` CLI binaries for Linux, macOS (Intel + ARM), and Windows
3. Publishes the GitHub Release with cover art, manifest, and binaries attached

---

**Status:** Alpha (v0.x.x)

---

<div align="center">
  <img src="https://raw.githubusercontent.com/polkadot-developers/polkadot-cookbook/v0.15.1/.github/releases/v0.15.1/cover-chain.svg" alt="Polkadot network state at v0.15.1 release" width="100%" />
</div>
