# DESIGN.md — Polkadot Cookbook Brand System

This file is the human + LLM reference for the project's brand. If you're an AI coding agent asked to produce a visual artifact for this repo, **read this file first and resolve all values against `tokens.yml`.** No hex codes belong outside `tokens.yml`.

## One-line identity

A dense, monospace, fact-bound Cookbook — product-grade palette: pink accent + near-black canvas + warm paper + grey ramp.

## Principles

1. **Product palette.** Pink `#E6007A` accent, near-black canvas `#0A0A0B`, warm paper `#F6F5F2`, plus an 8-value grey ramp for hierarchy (see `tokens.yml` `color.grey.*`). No secondary hues. No invented blues or teals. Hierarchy comes from the grey ramp, typography, scale, and composition.
2. **Fact-bound over decorative.** Every number on every surface must trace to `git`, `Cargo.toml`, `versions.yml`, `recipes/`, `polkadot-docs/`, or a chain RPC.
3. **Typography does color's job.** Monospace, size, weight, and opacity carry hierarchy. Pink is reserved for the Index Mark, accent strokes, and success/emphasis — never as a wholesale fill on non-brand surfaces.
4. **Grid, not canvas.** Compositions are block-based (canvas + surface panels + paper footer). No gradients (except the divider). No drop shadows. No rounded corners except terminal-window chrome.
5. **Motion is meaning.** Animations reveal structure, not decoration. Always honor `prefers-reduced-motion: reduce`.
6. **Dark-first, light-parity.** Every asset ships `*-dark.svg` + `*-light.svg`, swapped via GitHub's `#gh-dark-mode-only` / `#gh-light-mode-only` or `<picture>` with `prefers-color-scheme`.

## Palette (resolved from `tokens.yml`)

| Role        | Token                             | Use                                            |
| ----------- | --------------------------------- | ---------------------------------------------- |
| Pink        | `color.primary.pink`              | accent stroke, Index Mark, success/emphasis     |
| Canvas      | `color.base.canvas`               | near-black background `#0A0A0B`                |
| Paper       | `color.base.paper`                | warm paper background `#F6F5F2`                |
| Grey ramp   | `color.grey.*`                    | 8-value ramp for surfaces, lines, muted text   |

Mode-specific surface roles in `color.mode.{dark,light}`:

| Role            | Dark         | Light        |
| --------------- | ------------ | ------------ |
| canvas          | #0A0A0B      | #F6F5F2      |
| surface         | #111114      | #ECEAE4      |
| surface-2       | #1A1A1F      | #E3E0D8      |
| line            | #2A2A30      | #D7D4CC      |
| fg              | #F6F5F2      | #0A0A0B      |
| fg-muted        | #9A9AA3      | #51504B      |
| fg-dim          | #6B6B74      | #8A867B      |

## Composition vocabulary (v2 two-panel layout)

The hero, social preview, and OG image share the same two-panel partition:

| Block | Region                           | Fill           | Content                                 |
| ----- | -------------------------------- | -------------- | --------------------------------------- |
| B1    | left text panel (0–700 × 0–360) | **canvas**     | wordmark, tagline, stats, `dot_` prompt |
| B2    | right mark panel (700–1200 × 0–360) | **surface** | Index Mark centerpiece                  |
| B3    | footer bar (0–1200 × 360–400)   | **paper**      | pathway nav + URL + license             |

## Project marks

- **Index Mark** (primary): page-of-recipes glyph — a rounded rectangle with an accent-pink top band, a right-edge index tab, and faint horizontal content rules. Represents the Cookbook as a reference you thumb through. Rendered in a 200x200 viewBox. Canonical: `.github/media/dot-logo-{dark,light}.svg` (existing), also generated inline in hero/social templates.
- **dot CLI mark** (secondary): `▸ dot_` prompt treatment — a monospace string with the play-triangle in accent pink and the cursor underscore blinking. Used as a footnote under the wordmark to reference the included `dot` CLI scaffolder — the CLI is a small part of the Cookbook, so its mark is subordinate.

## Typography

- Stack: JetBrains Mono (`type.mono`). No sans-serif anywhere in our artifacts.
- Headings (`kpi-xl`, `h1`) use `weight.bold` with `tracking.tight`.
- KPI numerals (`kpi-lg`..`kpi-md`) always `weight.bold`; labels use `tracking.wide` uppercase.
- Body monospace (`mono-sm`, `mono-xs`) uses `weight.regular`, opacity 0.55–0.9.

## Motion

| Token                        | Use                                             |
| ---------------------------- | ----------------------------------------------- |
| `motion.reveal-dur`          | fade-in on block first paint                    |
| `motion.gradient-flow-dur`   | divider gradient animation period               |

Every `<animate>` must be paired with a `prefers-reduced-motion` opt-out (static SVG must still be visually complete when animation is disabled).

## Surfaces (where the brand appears)

| Surface                          | Asset                                              | Owner        |
| -------------------------------- | -------------------------------------------------- | ------------ |
| README hero (1200x400)           | `.github/media/hero-{dark,light}.svg`              | `/branding`  |
| Wordmark                         | `.github/media/wordmark-{dark,light}.svg`          | `/branding`  |
| README section dividers          | `.github/media/divider-{dark,light}.svg`           | `/branding`  |
| GitHub social preview            | `.github/media/social-preview.{svg,png}`           | `/branding`  |
| Open Graph                       | `.github/media/og-image.{svg,png}`                 | `/branding`  |
| docs/ OG (GitHub Pages)          | `docs/og-image.png`                                | `/branding`  |
| Per-pathway section banner       | `.github/media/pathway-{name}-dark.svg`            | `/branding`  |
| CONTRIBUTING hero                | `.github/media/contributing-hero-{dark,light}.svg` | `/branding`  |
| Issue / PR templates             | `.github/ISSUE_TEMPLATE/*`, `PULL_REQUEST_TEMPLATE.md` | manual  |
| docs PWA favicon                 | `docs/favicon.svg`                                 | `/branding`  |
| Release cover (per-release)      | `.github/releases/vX.Y.Z/cover.svg`                | `/release`   |
| Release chain cover              | `.github/releases/vX.Y.Z/cover-chain.svg`          | `/release`   |
| CLI banner                       | `dot/cli/src/main.rs` — colors via `colored`       | manual       |

## Adding a new surface

1. Start by reading this file and `tokens.yml`.
2. Compose from v2 block vocabulary (B1 canvas text panel · B2 surface mark panel · B3 paper footer bar). Don't invent new primitives.
3. Add a template under `.claude/skills/branding/templates/` + data contract.
4. Wire into `generate.py`. Run `/branding`. Commit both template and generated output.
5. Bump `CHANGELOG.md` if tokens changed.

## Voice coupling

See `voice.md`. Terse, monospace-friendly, fact-bound, no marketing adjectives.

## Anti-patterns

- Any color not in the `tokens.yml` palette. (CI `brand-lint.yml` catches this.)
- Gradients (except the divider).
- Drop shadows.
- Emojis in titles/headlines.
- Hand-edited files under `.github/media/` (drift check catches this).
- Sans-serif (DM Sans, Inter, etc.) in our generated artifacts.
- Secondary teal/blue/green for "status differentiation" — use opacity + composition instead.
