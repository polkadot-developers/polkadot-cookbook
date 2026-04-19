# Polkadot Cookbook Brand System

Single source of truth for every visual artifact this repo produces. Strict 3-color palette (pink / black / white), monospace typography, Mondrian-grid composition derived from the original Polkadot brand.

```
                          .github/brand/
                                 │
        ┌────────────────────────┼────────────────────────┐
        ▼                        ▼                        ▼
    tokens.yml              DESIGN.md                voice.md
  (palette/type/         (principles +          (writing style —
   motion/space)          composition            terse, fact-bound,
                          vocabulary)            monospace-friendly)
        │
        │ read by
        ▼
  .claude/skills/branding/generate.py
        │
        │ substitutes tokens into templates/*.svg.template
        ▼
  .github/media/  +  docs/
  hero-{dark,light}.svg · divider · social-preview · og-image
  pathway-*-dark.svg · contributing-hero-{dark,light}.svg · favicon.svg
```

## Read these in order

| File               | Purpose                                                          |
| ------------------ | ---------------------------------------------------------------- |
| `README.md`        | ← you are here. Entry point + quickstart.                        |
| `DESIGN.md`        | Visual principles. What the brand *is*.                          |
| `ARCHITECTURE.md`  | How the system *works*. Pipeline, files, CI, adding surfaces.    |
| `tokens.yml`       | Machine-readable palette + type + motion + space + surface roles.|
| `voice.md`         | Writing style (pairs with the visual system).                    |
| `CHANGELOG.md`     | Dated token / system changes. Bump alongside `tokens.yml` edits. |
| `scripts/*.sh`     | CI lint, drift, a11y, release-cover-verify scripts.              |

## The palette (from `tokens.yml`)

Three colors. That's it.

| Token                        | Hex       | Use                                           |
| ---------------------------- | --------- | --------------------------------------------- |
| `color.primary.pink`         | `#E6007A` | Polkadot brand pink. Slab fill, network mark, accent stroke, success. |
| `color.base.black`           | `#000000` | Canvas / accent-panel (dark mode); text on pink slab and on light mode. |
| `color.base.white`           | `#FFFFFF` | Canvas (light mode); text on dark surfaces.   |

Status differentiation (e.g. `fix` vs `feat` in release covers) is achieved with **opacity** and **composition**, never a secondary hue.

Mode-specific surface roles (`color.mode.{dark,light}`) flip `accent-panel` and `footer-surface` between black and white so every SVG ships both a `*-dark.svg` and `*-light.svg` from one template.

## The two project marks

### 1. Cookbook mark (primary — the project's logo)

Central pink circle + 8 orbital pink dots + 8 faint pink connection lines on the accent panel. Represents the network of tested recipes. Appears in the right panel of the hero and as the favicon.

```svg
<circle r="45" fill="#E6007A"/>               <!-- central node -->
<circle cx="0" cy="-78" r="10" fill="#E6007A"/>  <!-- 8 orbitals on an octagon -->
<!-- ...7 more orbital dots at 45° increments... -->
<g stroke="#E6007A" stroke-width="1.5" opacity="0.3">
  <!-- 8 thin connection lines from center to each orbital -->
</g>
```

### 2. dot CLI mark (secondary — subsidiary footnote)

Small pink outer circle + contrasting inner dot. Used only below the wordmark as a footnote referring to the included `dot` CLI scaffolder. The CLI is a small part of the Cookbook, so its mark is subordinate in size and position.

```svg
<circle cx="8" cy="8" r="8" fill="#000000"/>   <!-- outer -->
<circle cx="8" cy="8" r="3" fill="#E6007A"/>   <!-- inner -->
<text>DOT CLI · scaffolder included</text>
```

## Composition vocabulary (variant-05)

Every hero-class surface uses the same 3-block partition:

```
┌────────────────────────────────────┬──────────────────┐
│                                    │                  │
│   B1  PINK SLAB                    │   B2  ACCENT-    │
│       wordmark +                   │       PANEL      │
│       tagline +                    │                  │
│       dot CLI footnote             │  (Cookbook mark) │
│                                    │                  │
├────────────────────────────────────┴──────────────────┤
│   B3  FOOTER SURFACE                                  │
│       pathway nav · URL · license                     │
└───────────────────────────────────────────────────────┘
  0                                 800                1200
                       ↓ 540
                       ↓ 630
```

B1 is **always pink**. B2 flips: black (dark mode) / white (light mode). B3 inverts B2 for contrast.

## Common tasks

### Change a palette value

```bash
$EDITOR .github/brand/tokens.yml       # e.g. change pink hex
$EDITOR .github/brand/CHANGELOG.md     # add entry under [Unreleased] (CI enforces)
bash .claude/skills/branding/generate.sh   # regenerate every asset
git add .github/brand/ .github/media/ docs/
git commit -m "chore(brand): adjust pink shade"
```

### Add a recipe (counts update automatically)

```bash
# add recipes/<pathway>/<new-recipe>/
bash .claude/skills/branding/generate.sh   # pathway-*.svg banner counts update
git add .github/media/pathway-*.svg
```

### Add a new surface

1. Drop a new template under `.claude/skills/branding/templates/<name>.svg.template`
2. Use tokens: `{{PINK}}`, `{{CANVAS}}`, `{{ACCENT_PANEL}}`, `{{FOOTER_SURFACE}}`, `{{FG}}`, `{{FG_ON_FOOTER}}`, `{{MONO}}`, etc. See `ARCHITECTURE.md` for the full list.
3. Wire it into `.claude/skills/branding/generate.py`'s render block.
4. Add a row to the "Surfaces" table in `DESIGN.md`.
5. Run `bash .claude/skills/branding/generate.sh` and commit the template + generated outputs together.

### Modify an existing surface

Edit the `*.svg.template`, re-run the generator, commit template + regenerated outputs together. **Never hand-edit `.github/media/*.svg`** — CI drift check will fail.

## CI enforcement

`.github/workflows/brand-lint.yml` runs four checks on every PR that touches brand-relevant paths:

| Check                        | Script                                        | Fails when                                                 |
| ---------------------------- | --------------------------------------------- | ---------------------------------------------------------- |
| Palette lint                 | `scripts/lint-palette.sh`                     | A hex outside `tokens.yml` appears in tracked source.      |
| Release-cover palette verify | `scripts/verify-release-cover.sh`             | Release skill cover templates use off-palette hex.         |
| Drift check                  | `scripts/check-drift.sh`                      | Committed `.github/media/*.svg` differs from what `generate.py --dry-run` produces. |
| Accessibility check          | `scripts/check-a11y.sh`                       | `<animate>` element without a `prefers-reduced-motion` guard, or SVG missing `role` attribute. |
| Changelog guard              | (inline in workflow)                          | `tokens.yml` changed in PR but `CHANGELOG.md` did not.     |

`.github/workflows/brand-regenerate.yml` is a safety net: if `tokens.yml` or a template changes on `master` without accompanying regenerated assets, it opens a draft PR with the regeneration.

## Local setup

```bash
# python generator + validator
pip3 install pyyaml    # already present on most systems
brew install libxml2   # xmllint (macOS); apt: libxml2-utils

# optional — PNG rasterization
brew install librsvg   # rsvg-convert; apt: librsvg2-bin
# or: pip3 install cairosvg --break-system-packages
```

Then: `bash .claude/skills/branding/generate.sh` (or `/branding` inside Claude Code).

## Why this system

- **Strict 3-color** — derived from the original Polkadot brand (polkadot.network/brand-assets). No invented blue, cream, teal. Status differentiation uses opacity and composition, not hue inflation. Keeps the cookbook unmistakably Polkadot.
- **Monospace** — reinforces the "dense, fact-bound, terminal" character of a cookbook-for-builders. Every artifact reads like a log, not a brochure.
- **Tokens over duplication** — if pink ever needs to shift, one edit regenerates every surface. No hunting through templates.
- **Dark + light from one template** — mode swap via surface-role tokens (`color.mode.{dark,light}`). Single source, two outputs.
- **Fact-bound counts where it matters** — pathway banners pull live recipe counts; hero stays evergreen so it doesn't churn on every release.
- **Generated, not hand-maintained** — hand-editing a committed SVG is a CI failure. Forces everyone through the templates.
- **Deliberately narrow scope** — the brand skill produces visual artifacts only. Release covers stay in their own skill; CLI colors stay in the Rust crate; each surface pulls its palette from `tokens.yml` but owns its own production.
