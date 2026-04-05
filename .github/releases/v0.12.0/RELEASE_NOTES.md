# Release v0.12.0

Released: 2026-04-03

## What's New

### Infrastructure
- Replaced 3 brittle release workflows with a `/release` Claude Code skill that analyzes changes semantically and generates meaningful release notes
- Added weekly maintainer tasks workflow — creates a Monday morning issue with a checklist (cut release, check docs drift, review PRs, CI health)
- Updated all maintainer and contributor docs to reflect the new skill-driven release process

### Documentation Tests
- Bumped pinned commit SHAs for 5 polkadot-docs test harnesses to align with upstream tutorial changes (runtime-upgrades, run-a-parachain-network, set-up-parachain-template, benchmark-pallet, install-polkadot-sdk)

## Commits

- feat: add /release skill, replace brittle release workflows (#223)
- fix: bump docs_commit for 5 substantive upstream drifts (#216)

## Compatibility

Tested with:
- Rust: 1.91.0
- Node.js: v24.7.0

---

**Status:** Alpha (v0.x.x)
