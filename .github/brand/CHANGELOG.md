# Brand Changelog

All notable changes to the Polkadot Cookbook brand system (tokens, components, generated assets). Format: [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

Bump this file **in the same commit** as any change to `tokens.yml`. External embedders (polkadot-docs CI badges, social cards) may cache on token URLs — entries here signal when to bust caches.

## [Unreleased]

## [0.1.0] — 2026-04-15

### Added
- Initial brand system: `.github/brand/{tokens.yml, DESIGN.md, voice.md, components/}`.
- `/branding` skill at `.claude/skills/branding/` generates hero, dividers, social preview, OG image, per-pathway banners, CONTRIBUTING hero, favicon from tokens.
- `.github/media/hero-{dark,light}.svg` (Mondrian `cover-echo` layout, fact-bound from `Cargo.toml` + `recipes/` + `.github/workflows/`).
- `.github/media/divider-{dark,light}.svg` (animated gradient bar, per existing BRANDING.md spec).
- `.github/media/social-preview.svg` (1280×640, GitHub Settings → Social preview).
- `.github/media/og-image.svg` (1200×630, Open Graph / Twitter card).
- `.github/media/pathway-{pallets,contracts,transactions,xcm,networks}-dark.svg`.
- `.github/media/contributing-hero-{dark,light}.svg`.
- `docs/favicon.svg` (PWA + browser tab icon).
- `.github/ISSUE_TEMPLATE/{bug,recipe-request,docs}.yml` + `PULL_REQUEST_TEMPLATE.md`.
- `.github/workflows/brand-lint.yml` (palette lint + drift check + a11y check).
- `.github/workflows/brand-regenerate.yml` (auto-PR regenerated assets when `tokens.yml` changes on master).
- `.github/brand/scripts/{lint-palette.sh, check-drift.sh, check-a11y.sh, verify-release-cover.sh}`.

### Changed
- `README.md` hero block + `<hr />` dividers now reference generated SVGs via `<picture>` tags.
- `CONTRIBUTING.md` gains a brand addendum pointing at `DESIGN.md`.

### Migrated from
- `.github/media/BRANDING.md` content superseded by `DESIGN.md` + `tokens.yml`. File retained for now with a pointer; scheduled for removal in next brand release.
- Previously duplicated hex codes across `.claude/skills/release/covers/*.template`, `dot/cli/src/main.rs`, `docs/manifest.json` now trace back to `tokens.yml` (enforced by `brand-lint.yml`).
