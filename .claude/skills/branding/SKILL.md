---
name: branding
description: Regenerate all brand assets under .github/media/ from .github/brand/tokens.yml. Run after editing tokens, templates, or bumping the workspace version.
---

# /branding

Generates every visual artifact in `.github/media/` from a single source of truth.

## When to invoke

- After editing `.github/brand/tokens.yml`.
- After bumping `[workspace.package].version` in `Cargo.toml` (the `/release` skill does this for you).
- After editing a template under `templates/`.
- When `.github/workflows/brand-lint.yml` fails with a "drift" error.
- Whenever onboarding the repo to a new branding surface (add a template + re-run).

Recipe / workflow / docs-harness counts are **not** baked into any template, so adding one of those does not require a regeneration.

## What it does

1. Reads `.github/brand/tokens.yml` (palette, type, space, motion).
2. Reads live repo facts:
   - `{{VERSION}}` ← `Cargo.toml` `[workspace.package].version`
3. Substitutes `{{TOKEN}}` scalars into every template under `templates/`.
4. Validates each output with `xmllint --noout`.
5. Writes to `.github/media/`.

## Outputs

See `.github/brand/README.md` for the canonical list. Summary:

- `hero-{dark,light}.svg` (1200×630)
- `divider-{dark,light}.svg` (1200×12)
- `og-image.svg` (1200×630)
- `social-preview.svg` (1280×640)
- `pathway-{pallets,contracts,transactions,xcm,networks}-dark.svg` (1200×200)
- `contributing-hero-{dark,light}.svg`
- `../docs/favicon.svg`

## How

```bash
bash .claude/skills/branding/generate.sh
# or
bash .claude/skills/branding/generate.sh --dry-run   # preview; no writes
```

Exit codes: `0` ok, `1` token/template error, `2` xmllint failure, `3` drift (in `--dry-run`).

## Conventions

- Templates live under `.claude/skills/branding/templates/`.
- Never hand-edit files under `.github/media/` — they're regenerated. CI enforces.
- Add a new surface: drop a `.svg.template` + (optional) `.data.md` contract, then wire into `generate.sh`.
- Always commit regenerated assets alongside template/token changes in the same PR.
