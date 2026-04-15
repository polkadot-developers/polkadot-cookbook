# DESIGN.md — Polkadot Cookbook Brand System

This file is the human + LLM reference for the project's brand. If you're an AI coding agent asked to produce a visual artifact for this repo, **read this file first and resolve all values against `tokens.yml`.** No hex codes belong outside `tokens.yml`.

## One-line identity

A Bloomberg-terminal for building on Polkadot: dense, monospace, fact-bound, animated, earnest.

## Principles

1. **Fact-bound over decorative.** Every number on every surface must be traceable to `git`, `Cargo.toml`, `versions.yml`, `recipes/`, or a chain RPC. No invented metrics.
2. **Typography does color's job.** Size and weight carry hierarchy. Color is reserved for state (pink = brand/success, teal = fix, mustard = warn).
3. **Grid, not canvas.** Every composition is a Mondrian grid of solid-color blocks (B1..Bn). No gradients except the divider. No shadows. No rounded corners except terminal-window chrome.
4. **Motion is meaning.** Animations reveal structure (staggered cascade = order of events), not decoration. Always honor `prefers-reduced-motion: reduce`.
5. **Dark-first, light-parity.** Every asset ships `*-dark.svg` + `*-light.svg`, swapped via GitHub's `#gh-dark-mode-only` / `#gh-light-mode-only`.

## Palette (from `tokens.yml`)

| Role      | Token                      | Use                                           |
| --------- | -------------------------- | --------------------------------------------- |
| Pink      | `color.primary.pink`       | wordmark slab, accents, success state         |
| Blue      | `color.primary.blue`       | data panels (charts, KPI strips), status rail |
| Canvas    | `color.surface.canvas`     | dark background, grid dividers                |
| Terminal  | `color.surface.terminal`   | monospace code / log panels                   |
| Cream     | `color.surface.cream`      | text on dark                                  |
| Fix teal  | `color.semantic.fix`       | `fix:` commits, secondary data series         |
| Warn      | `color.semantic.warn`      | deprecation, cautions (use sparingly)         |

## Block vocabulary (Mondrian grid)

Inherited from `.claude/skills/release/covers/cover.svg.template` (commit `b67f316`). Every hero-class composition uses this partitioning:

| Block | Role                  | Typical content                               |
| ----- | --------------------- | --------------------------------------------- |
| B1    | pink identity slab    | wordmark, title, tagline                      |
| B2    | terminal panel        | monospace list (recipes, commits, logs)       |
| B3    | deep-blue data panel  | horizontal bar chart, KPI breakdown           |
| B4    | pink accent strip     | version badge, CTA                            |
| B5    | deep-blue footer      | KPI row (totals, license, status)             |

**Divider width:** 14px (`space.grid-gutter`) — non-negotiable. Uses `color.surface.canvas`.

## Typography

- Stack: `type.mono` only. No sans-serif anywhere. (Exception: GitHub renders README body copy in its own sans — acceptable because it's outside our artifacts.)
- Headings (`type.size.h1`, `h2`) use `weight.bold` with `tracking.tight`.
- KPI numerals (`kpi-xl`..`kpi-sm`) always `weight.bold`; labels use `tracking.wide` uppercase.
- Monospace body (`mono-sm`, `mono-xs`) uses `weight.regular`, opacity 0.7–0.9.

## Motion

| Token                        | Use                                             |
| ---------------------------- | ----------------------------------------------- |
| `motion.reveal-dur`          | fade-in opacity 0→1 on block first paint        |
| `motion.cascade-stagger`     | delay between successive rows in terminal panel |
| `motion.float-heading-*`     | subtle float on 80px hero logo                  |
| `motion.gradient-flow-dur`   | divider gradient animation period               |

Every `<animate>` element MUST be wrapped — or paired — with a `prefers-reduced-motion` opt-out. The generator enforces this.

## Surfaces (where the brand appears)

| Surface                                | Asset                                            | Owner skill   |
| -------------------------------------- | ------------------------------------------------ | ------------- |
| README hero                            | `.github/media/hero-{dark,light}.svg`            | `/branding`   |
| README section dividers                | `.github/media/divider-{dark,light}.svg`         | `/branding`   |
| GitHub social preview                  | `.github/media/social-preview.{svg,png}`         | `/branding`   |
| Open Graph                             | `.github/media/og-image.{svg,png}`               | `/branding`   |
| Per-pathway section banner             | `.github/media/pathway-{name}-dark.svg`          | `/branding`   |
| CONTRIBUTING hero                      | `.github/media/contributing-hero-{dark,light}.svg` | `/branding` |
| Issue / PR templates                   | `.github/ISSUE_TEMPLATE/*`, `PULL_REQUEST_TEMPLATE.md` | manual (palette from DESIGN.md) |
| docs PWA favicon                       | `docs/favicon.svg`                               | `/branding`   |
| Release cover (per-release)            | `.github/releases/vX.Y.Z/cover.svg`              | `/release`    |
| Release chain cover (per-release)      | `.github/releases/vX.Y.Z/cover-chain.svg`        | `/release`    |
| CLI banner                             | `dot/cli/src/main.rs` — colors via `colored`     | manual (palette from tokens) |

## Adding a new surface

1. Start by reading this file and `tokens.yml`.
2. Compose from blocks B1..B5 and `.github/brand/components/` fragments. Don't invent new primitives.
3. Add a template under `.claude/skills/branding/templates/` + a data contract (see `hero.data.md`).
4. Wire into `generate.sh`. Run it. Commit both the template and the generated output.
5. Add a row to the "Surfaces" table above.
6. Bump `CHANGELOG.md` if tokens changed.

## Voice coupling

See `voice.md`. Short version: the writing matches the visuals — terse, monospace-friendly, fact-bound, no marketing adjectives, numbers over prose.

## Anti-patterns

- Gradients (except the one in `divider`).
- Drop shadows.
- Emojis in titles/headlines. (Existing icons are OK.)
- Hex codes outside `tokens.yml` (`brand-lint.yml` catches this).
- Hand-edited files under `.github/media/` (drift check catches this).
- "Powered by Polkadot" marketing stamps. Identity is through the palette + grid, not captions.
