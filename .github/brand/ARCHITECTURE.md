# Brand System Architecture

How the brand system is built and how it runs. If `DESIGN.md` is *what* the brand is, this is *how* it's produced.

## Pipeline

```
┌─────────────────────┐     ┌──────────────────────────────┐     ┌─────────────────────┐
│  .github/brand/     │     │  .claude/skills/branding/    │     │  .github/media/     │
│                     │     │                              │     │  docs/              │
│  tokens.yml  ───────┼────►│  generate.py                 │────►│                     │
│                     │     │    ├── reads tokens          │     │  hero-{dark,light}  │
│  DESIGN.md          │     │    ├── reads live repo facts │     │  divider-*          │
│                     │     │    │   (Cargo.toml, recipes/,│     │  social-preview     │
│  voice.md           │     │    │    workflows/,          │     │  og-image + .png    │
│                     │     │    │    polkadot-docs/)      │     │  pathway-*-dark     │
│  scripts/           │     │    ├── substitutes {{…}}     │     │  contributing-hero  │
│    lint-palette.sh  │     │    │   into templates/*.svg  │     │  favicon.svg        │
│    check-drift.sh   │     │    ├── xmllint --noout       │     │                     │
│    check-a11y.sh    │     │    └── rsvg-convert → PNG    │     │                     │
│    verify-release-  │     │                              │     │                     │
│      cover.sh       │     │  templates/                  │     │                     │
│                     │     │    hero.svg.template         │     │                     │
│  CHANGELOG.md       │     │    wordmark.svg.template     │     │                     │
│                     │     │    social-preview.svg.t…     │     │                     │
└─────────────────────┘     │    divider.svg.template      │     └─────────────────────┘
          │                 │    pathway-banner.svg.t…     │               │
          │                 │    contributing-hero.svg.t…  │               │
          │                 │    favicon.svg.template      │               │
          │                 └──────────────────────────────┘               │
          │                                                                │
          │                         CI enforcement                         │
          │  ┌────────────────────────────────────────────────────────┐    │
          │  │  .github/workflows/brand-lint.yml                      │    │
          └─►│    palette lint · release-cover verify ·               │◄───┘
             │    drift check · a11y check · changelog guard          │
             │                                                        │
             │  .github/workflows/brand-regenerate.yml                │
             │    (auto-PR regenerated assets when tokens change on   │
             │     master without matching regeneration)              │
             └────────────────────────────────────────────────────────┘
```

## File-by-file

### `.github/brand/tokens.yml`

Single source of truth. YAML scalars, no expressions, no imports. Structure:

```
color:
  primary:      { pink }              # brand accent
  base:         { canvas, paper }     # near-black + warm paper
  grey:         { 50, 100, 200, 300, 400, 500, 600, 700, 800, 900 }  # 8-value ramp
  semantic:     { success, emphasis, ink, paper }
  mode:
    dark:       { canvas, surface, surface-2, line, fg, fg-muted, fg-dim }
    light:      { canvas, surface, surface-2, line, fg, fg-muted, fg-dim }
type:           { mono, weight, size, tracking }
space:          { grid-gutter, canvas: { og, social, divider, pathway-banner, contributing-hero } }
motion:         { reveal-dur, cascade-stagger, float-heading-dur, gradient-flow-dur, ease-out, honor-reduced-motion }
logo:           { clear-space-ratio, min-size-px, hero-size-px }
a11y:           { min-contrast-body, min-contrast-large }
allowlist:      []                    # additional hex codes permitted outside tokens.yml
```

Everything a template or skill might want is addressable by dotted path (`color.primary.pink`, `color.mode.dark.canvas`, etc.).

### `.claude/skills/branding/generate.py`

One file. Reads `tokens.yml`, computes live facts, renders every template twice (dark + light where applicable), validates with `xmllint`, rasterizes PNG where needed.

Key functions:

- `mode_subs(mode)` — returns the substitution dict for a given mode. Per-mode values: `CANVAS`, `SURFACE`, `SURFACE_2`, `LINE`, `FG`, `FG_MUTED`, `FG_DIM`. Mode-independent: `PINK`, `INK`, `PAPER`, `MONO`, plus the live `VERSION` fact. Per-pathway banner extras (`PATHWAY_NAME`, `PATHWAY_LABEL`, `PATHWAY_TAGLINE`, `PATHWAY_NUMBER`, `PATHWAY_GLYPH`) are passed via the `extra` arg in `render()`.
- `render(template, out_path, mode, extra=None)` — substitutes `{{TOKEN}}` into template body, writes output, validates.
- `rasterize(svg, png, width)` — `rsvg-convert` → fallback to `cairosvg` → warn if neither available.

The entire generator is ~170 lines. Straightforward string substitution.

### `.claude/skills/branding/templates/*.svg.template`

Plain SVG with `{{TOKEN}}` placeholders. Token names must appear in the substitution dict from `generate.py` or generation fails with "unresolved tokens".

Conventions:
- Every template has `role="img" aria-label="…"` or `role="presentation" aria-hidden="true"`.
- Animated elements paired with `@media (prefers-reduced-motion: reduce)` CSS.
- No hex codes — everything through tokens.
- `shape-rendering="crispEdges"` for block-heavy designs; `"geometricPrecision"` for curves.

### `.github/brand/scripts/`

| Script                       | What it does                                                            | Exit code on fail |
| ---------------------------- | ----------------------------------------------------------------------- | ----------------- |
| `lint-palette.sh`            | Walks tracked files (.svg/.css/.rs/.ts/.md/.json/.yml/.toml/.html), greps for `#RRGGBB` not in `tokens.yml` or `allowlist`. Skips `docs/` legacy site, `.github/releases/` archive, `.github/badges/`, legacy logos. | 1 |
| `verify-release-cover.sh`    | Same check, restricted to `.claude/skills/release/covers/*.template`.   | 1                 |
| `check-drift.sh`             | Runs `generate.py --dry-run`, diffs result against committed `.github/media/`. | 3               |
| `check-a11y.sh`              | Every generated SVG must have `role` attribute; every SVG with `<animate>` must reference `prefers-reduced-motion`. | 1 |

All scripts use Python + `pyyaml` for palette parsing (not `yq`) to avoid a dependency that isn't installed by default on macOS.

### `.github/workflows/brand-lint.yml`

Runs on PRs affecting brand-relevant paths. Installs `python3-yaml` + `libxml2-utils`, runs the four scripts, plus an inline check that `tokens.yml` changes include a `CHANGELOG.md` bump.

### `.github/workflows/brand-regenerate.yml`

Runs on `master` pushes that touch `tokens.yml` or `templates/`. Regenerates assets, uses `peter-evans/create-pull-request` to open a draft PR if `.github/media/` diverges. Safety net — the normal path is "regenerate locally, commit regenerated files in the same PR."

## Substitution token reference

Tokens available inside every template:

| Token                      | Source                                  | Mode-dependent? |
| -------------------------- | --------------------------------------- | --------------- |
| `{{PINK}}`                 | `color.primary.pink`                    | no              |
| `{{INK}}`                  | `color.base.canvas` (near-black `#0A0A0B`) | no          |
| `{{PAPER}}`                | `color.base.paper` (warm paper `#F6F5F2`)  | no          |
| `{{CANVAS}}`               | `color.mode.{mode}.canvas`              | **yes**         |
| `{{SURFACE}}`              | `color.mode.{mode}.surface`             | **yes**         |
| `{{SURFACE_2}}`            | `color.mode.{mode}.surface-2`           | **yes**         |
| `{{LINE}}`                 | `color.mode.{mode}.line`                | **yes**         |
| `{{FG}}`                   | `color.mode.{mode}.fg`                  | **yes**         |
| `{{FG_MUTED}}`             | `color.mode.{mode}.fg-muted`            | **yes**         |
| `{{FG_DIM}}`               | `color.mode.{mode}.fg-dim`              | **yes**         |
| `{{MONO}}`                 | `type.mono`                             | no              |
| `{{REVEAL_DUR}}`           | `motion.reveal-dur`                     | no              |
| `{{CASCADE_STAGGER}}`      | `motion.cascade-stagger`                | no              |
| `{{EASE_OUT}}`             | `motion.ease-out`                       | no              |
| `{{GRADIENT_FLOW_DUR}}`    | `motion.gradient-flow-dur`              | no              |
| `{{VERSION}}`              | `Cargo.toml` `[workspace.package].version` | no           |

Per-pathway-banner extras (passed via the `extra` dict):
`{{PATHWAY_NAME}}`, `{{PATHWAY_LABEL}}`, `{{PATHWAY_TAGLINE}}`, `{{PATHWAY_NUMBER}}` (`01`–`05`), `{{PATHWAY_GLYPH}}`.

Counts of recipes, pathways, workflows, and docs harnesses are deliberately **not** baked into any template — keeping the asset set independent of repo growth means adding a recipe / workflow / harness never requires regenerating brand assets. The release skill handles `VERSION` bumps as part of `/release`.

## Data flow for one run

```
1. generate.sh  →  python3 generate.py
2. generate.py  →  open .github/brand/tokens.yml                    (yaml.safe_load)
3.              →  git rev-parse, Cargo.toml read, recipes/ walk    (live facts)
4.              →  for each template:
                     mode = "dark" | "light"
                     subs = mode_subs(mode) + extra
                     body = template.read()
                     for k,v in subs: body = body.replace("{{"+k+"}}", v)
                     assert no unresolved {{TOKEN}}
                     out.write(body)
                     xmllint --noout out
5.              →  rsvg-convert social-preview.svg → .png           (if installed)
                   cp og-image.png → docs/og-image.png              (GitHub Pages)
6. (outside)    →  git add .github/media/ docs/; git commit
7. CI           →  brand-lint.yml runs palette + drift + a11y + changelog guard
```

## Why Python, not Bash?

Earlier prototype used bash with `yq`. Abandoned because:
- `yq` isn't on default macOS.
- Bash string substitution with values containing spaces and quotes (`type.mono` = `"ui-monospace, Menlo, Consolas, 'Courier New', monospace"`) is hostile.
- A 170-line Python script is easier to read than 170 lines of `sed`.

`pyyaml` is a universal dependency (comes with most Python distributions; one `apt install` in CI).

## Rasterization

- **Preferred:** `rsvg-convert` (librsvg). Fast, faithful text rendering, well-supported on Linux CI via `apt install librsvg2-bin`. macOS: `brew install librsvg`.
- **Fallback:** `cairosvg` Python lib. Requires PEP-668-override (`pip3 install --break-system-packages`) on modern macOS system Python.
- **Absent:** script warns and skips PNGs. SVG assets still produced. CI always has `rsvg-convert`, so committed PNGs are reproducible.

## Extension points

Common changes and where they land:

| You want to…                             | Edit                                                    | Regenerate? |
| ---------------------------------------- | ------------------------------------------------------- | ----------- |
| Tweak pink shade                         | `tokens.yml` (+ `CHANGELOG.md`)                         | yes         |
| Add a mode (high-contrast, etc.)         | `tokens.yml` `color.mode.<newmode>`, `generate.py` loop | yes         |
| Add a new surface (e.g., contributor card) | New template, add render block in `generate.py`, surface row in `DESIGN.md` | yes |
| Change hero layout                       | `templates/hero.svg.template`                           | yes         |
| Add a new live-data token                | `generate.py` `mode_subs` dict + data contract comment  | yes         |
| Relax palette (allow a new hex)          | `tokens.yml` `allowlist:` array                         | rerun lint  |
| Skip a legacy file from palette lint     | `scripts/lint-palette.sh` `skip_paths`                  | rerun lint  |

## Non-goals

- Not a replacement for the release skill. Release covers are fact-bound per-release and have their own templates + data contracts.
- Not a runtime theming system for the docs site. `docs/` has its own legacy palette (allowlisted); a future migration is tracked as a follow-up.
- Not a general asset pipeline. If you need to process images / fonts / icons, reach for a dedicated tool.
