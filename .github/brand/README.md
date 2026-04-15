# .github/brand/

Single source of truth for Polkadot Cookbook visual identity.

## Contents

| File                  | Purpose                                                         |
| --------------------- | --------------------------------------------------------------- |
| `tokens.yml`          | Machine-readable palette, type, space, motion tokens.           |
| `DESIGN.md`           | Human + LLM reference. Read first when producing visual assets. |
| `voice.md`            | Writing style (terse, fact-bound, monospace-friendly).          |
| `CHANGELOG.md`        | Dated changes to tokens. Bump alongside any `tokens.yml` edit.  |
| `components/`         | Reusable SVG fragment templates.                                |
| `scripts/`            | Brand lint, drift check, a11y check, release-cover verify.      |

## Where assets are generated

The `/branding` skill (`.claude/skills/branding/`) reads `tokens.yml` + live repo facts (`Cargo.toml`, `recipes/*/*/`, `.github/workflows/*.yml`) and writes to `.github/media/`. Generated assets must not be hand-edited (CI drift check enforces).

## Changing a token

1. Edit `tokens.yml`.
2. Bump `CHANGELOG.md` with an `### Added` / `### Changed` entry under `[Unreleased]`.
3. Run `bash .claude/skills/branding/generate.sh` (or `/branding`).
4. Commit `tokens.yml`, `CHANGELOG.md`, and every regenerated file in `.github/media/` together.

## CI enforcement

- `brand-lint.yml` fails PRs that introduce hex codes outside `tokens.yml.allowlist`, or that commit generated assets out-of-sync with the templates.
- `brand-regenerate.yml` opens an auto-PR on `master` pushes that modify `tokens.yml` but don't include the regenerated assets (safety net).

## See also

- `.github/media/BRANDING.md` — legacy; superseded by `DESIGN.md`.
- `.claude/skills/release/covers/cover.data.md` — release cover data contract.
