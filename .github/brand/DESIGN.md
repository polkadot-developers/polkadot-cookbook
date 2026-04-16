# DESIGN.md — Polkadot Cookbook Brand System

This file is the human + LLM reference for the project's brand. If you're an AI coding agent asked to produce a visual artifact for this repo, **read this file first and resolve all values against `tokens.yml`.** No hex codes belong outside `tokens.yml`.

## One-line identity

A dense, monospace, fact-bound Cookbook — grounded in the original Polkadot brand (pink + black + white, nothing else).

## Principles

1. **Three colors only.** Pink `#E6007A`, black `#000000`, white `#FFFFFF`. No secondary hues. No invented blues or teals. Hierarchy comes from typography, scale, and composition — never from extra colors.
2. **Fact-bound over decorative.** Every number on every surface must trace to `git`, `Cargo.toml`, `versions.yml`, `recipes/`, `polkadot-docs/`, or a chain RPC.
3. **Typography does color's job.** Monospace, size, weight, and opacity carry hierarchy. Pink is reserved for the brand slab, the network mark, accent strokes, and success/emphasis — never as a wholesale fill on non-brand surfaces.
4. **Grid, not canvas.** Compositions are block-based (pink slab + black/white surfaces). No gradients (except the divider, which flows pink↔black↔white). No drop shadows. No rounded corners except terminal-window chrome.
5. **Motion is meaning.** Animations reveal structure, not decoration. Always honor `prefers-reduced-motion: reduce`.
6. **Dark-first, light-parity.** Every asset ships `*-dark.svg` + `*-light.svg`, swapped via GitHub's `#gh-dark-mode-only` / `#gh-light-mode-only` or `<picture>` with `prefers-color-scheme`.

## Palette (resolved from `tokens.yml`)

| Role        | Token                             | Use                                            |
| ----------- | --------------------------------- | ---------------------------------------------- |
| Pink        | `color.primary.pink`              | slab, network mark, accent stroke, success/emphasis |
| Black       | `color.base.black`                | canvas/accent-panel (dark mode), text on light |
| White       | `color.base.white`                | canvas (light mode), text on dark              |

Mode-specific surface roles in `color.mode.{dark,light}`:

| Role            | Dark         | Light        |
| --------------- | ------------ | ------------ |
| canvas          | #000000      | #FFFFFF      |
| accent-panel    | #000000      | #FFFFFF      |
| footer-surface  | #FFFFFF      | #000000      |
| fg              | #FFFFFF      | #000000      |

## Composition vocabulary (variant-05 layout)

The hero, social preview, and OG image share the same Mondrian partition:

| Block | Region                      | Fill           | Content                                 |
| ----- | --------------------------- | -------------- | --------------------------------------- |
| B1    | left slab (0–800 × 0–540)   | **pink**       | wordmark, tagline, dot-CLI footnote     |
| B2    | right panel (800–1200 × 0–540) | **accent-panel** | network mark (Cookbook logo) centerpiece |
| B3    | footer (0–1200 × 540–630)   | **footer-surface** | pathway nav + URL + license           |

## Project marks

- **Cookbook logo** (primary): central pink circle + 8 orbital dots + 8 faint connection lines. Represents the network of tested recipes. Rendered as pink on any surface (black panel in dark mode, white panel in light mode). Canonical: `.github/media/dot-logo-{dark,light}.svg` (existing), also generated inline in hero/social templates.
- **dot CLI mark** (secondary): small outer pink circle with a smaller inner dot (pink on pink or pink on inverse). Used as a footnote under the wordmark to reference the included `dot` CLI scaffolder — the CLI is a small part of the Cookbook, so its mark is subordinate.

## Typography

- Stack: `type.mono` only. No sans-serif anywhere in our artifacts.
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
| README hero                      | `.github/media/hero-{dark,light}.svg`              | `/branding`  |
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
2. Compose from variant-05 block vocabulary (B1 pink slab · B2 accent-panel · B3 footer-surface). Don't invent new primitives.
3. Add a template under `.claude/skills/branding/templates/` + data contract.
4. Wire into `generate.py`. Run `/branding`. Commit both template and generated output.
5. Bump `CHANGELOG.md` if tokens changed.

## Voice coupling

See `voice.md`. Terse, monospace-friendly, fact-bound, no marketing adjectives.

## Anti-patterns

- Any color not in `{#E6007A, #000000, #FFFFFF}`. (CI `brand-lint.yml` catches this.)
- Gradients (except the divider).
- Drop shadows.
- Emojis in titles/headlines.
- Hand-edited files under `.github/media/` (drift check catches this).
- Sans-serif (DM Sans, Inter, etc.) in our generated artifacts.
- Secondary teal/blue/green for "status differentiation" — use opacity + composition instead.
