# /release --dry-run

Load this file when `/release` is invoked with `--dry-run`. **Do not** follow the real-run pipeline in `SKILL.md`; follow this file instead. The real-run pipeline is referenced only for its templates and data contracts, which are shared.

## Invocation

```
/release --dry-run [patch|minor|major]
```

Previews every rendered release artifact in a scratch directory. **No git mutations. No GitHub mutations.** Safe to run any time.

## What this does

- Computes all scalar tokens + marker fills from git + RPC exactly as a real run would.
- Renders every template (`cover.svg`, `cover-chain.svg`, `RELEASE_NOTES.md`, `manifest.yml`, PR body).
- Writes outputs to `/tmp/release-dry-run/v{VERSION}/`.
- Runs `xmllint --noout` on both SVGs.
- Rewrites cover-image `src` attributes in `RELEASE_NOTES.md` and `pr-body.md` to relative paths (`./cover.svg`, `./cover-chain.svg`) so the scratch-dir Markdown previews render locally in VS Code / any previewer.
- Prints a summary: scratch-dir path, per-file size/validity, unfilled token/marker counts.

## What this does NOT do

- No `git checkout -b release/vX.Y.Z`
- No edits to `Cargo.toml`, `Cargo.lock`, `CHANGELOG.md` (proposed diff written to `proposed-changes.diff` instead)
- No `git commit`, no `git push`
- No `gh pr create`
- No `gh release edit` on any existing release

Chain-state RPC capture **does** run against live endpoints — it's cheap and is the only way to verify the endpoint walk + fallback logic. If all endpoints fail, the footer cover is marked `SKIPPED` in the summary.

## Pipeline

Each step below mirrors the real-run phase by the same number in `SKILL.md`. Only differences from real-run are called out — read the real-run phase for the shared logic.

### Phase 1: Analyze changes

Run exactly as in `SKILL.md` Phase 1 — read-only, no difference.

### Phase 2: Bump logic

Same as real-run. Determine `{{VERSION}}` from the bump arg + current `Cargo.toml`.

### Phase 3 setup: scratch dir

```bash
SCRATCH=/tmp/release-dry-run/v{VERSION}
mkdir -p $SCRATCH
```

All subsequent writes target `$SCRATCH` instead of `.github/releases/v{VERSION}/`.

### Phase 3a: Gather metadata

Exactly as real-run — commands are read-only.

### Phase 3b: Release directory

**Skip.** Real-run creates `.github/releases/v{VERSION}/` and commits a placeholder. In dry-run, the scratch dir replaces this.

### Phase 3c: Top cover

Render `$SCRATCH/cover.svg` from [`covers/cover.svg.template`](covers/cover.svg.template) per [`covers/cover.data.md`](covers/cover.data.md). Run `xmllint --noout` on output.

### Phase 3c.2: Footer cover

Render `$SCRATCH/cover-chain.svg` from [`covers/cover-chain.svg.template`](covers/cover-chain.svg.template) per [`covers/cover-chain.data.md`](covers/cover-chain.data.md). Walk the RPC endpoint list (live). If all fail: do not write the file, create `$SCRATCH/cover-chain.SKIPPED` with a one-line reason. Run `xmllint --noout` on output if written.

### Phase 3d: Release notes

Render `$SCRATCH/RELEASE_NOTES.md` from [`RELEASE_NOTES.template.md`](RELEASE_NOTES.template.md). **After substitution**, rewrite the two cover-image `<img src="...">` URLs:

- `https://raw.githubusercontent.com/.../v{VERSION}/.github/releases/v{VERSION}/cover.svg` → `./cover.svg`
- `https://raw.githubusercontent.com/.../v{VERSION}/.github/releases/v{VERSION}/cover-chain.svg` → `./cover-chain.svg`

This makes the preview work in the scratch dir (the real `v{VERSION}` tag doesn't exist yet). Real-run keeps the absolute URLs.

Fill marker sections (`@@SUMMARY`, `@@BREAKING`, `@@WHATS_NEW`, `@@COMMITS`) following `SKILL.md` Phase 3d narrative rules.

### Phase 3e: CHANGELOG.md

**Do not edit.** Compute the proposed new file content, diff against the current `CHANGELOG.md`:

```bash
diff -u CHANGELOG.md /tmp/proposed-changelog.md > $SCRATCH/proposed-changes.diff
```

Append the Cargo.toml version-bump one-liner to the same diff file.

### Phase 3f: Cargo.toml

**Do not edit.** Append the proposed `version = "X.Y.Z"` change to `$SCRATCH/proposed-changes.diff`. Skip `cargo update`.

### Phase 3g: Manifest

Render `$SCRATCH/manifest.yml` from [`MANIFEST.template.yml`](MANIFEST.template.yml).

### Phase 4: PR body (no creation)

Render `$SCRATCH/pr-body.md` from [`RELEASE_PR_BODY.template.md`](RELEASE_PR_BODY.template.md). Apply the same `./cover.svg` relative-path rewrite as Phase 3d. **Do not** create a branch, commit, push, or open a PR. Print what the `gh pr create` invocation would be.

## Template rendering convention

Every template file starts with a **documentation header block** that explains tokens and markers for humans reading the template source. The doc text often *names* the tokens/markers (`{{VERSION}}`, `<!-- @@COMMIT_LIST -->`) as examples — if you substitute into the doc text, you inject content where it doesn't belong, or (for SVGs) corrupt the XML comment structure with `--` characters from commit subjects.

Every template ends its doc header with an unambiguous sentinel line:

- **Markdown / SVG templates**: `<!-- TEMPLATE_HEADER_END -->`
- **YAML templates**: `# TEMPLATE_HEADER_END`

**Strip rule:** delete all lines from line 1 up to **and including** the sentinel line, plus any immediately-following blank line. Use a simple line scan — do not use a regex against the comment delimiters, because doc headers may contain nested-looking `<!-- @@MARKER -->` references.

```python
lines = template.splitlines(keepends=True)
body_start = None
for i, line in enumerate(lines):
    if "TEMPLATE_HEADER_END" in line:
        body_start = i + 1
        # consume one trailing blank line if present
        if body_start < len(lines) and lines[body_start].strip() == "":
            body_start += 1
        break
if body_start is None:
    fail("template is missing TEMPLATE_HEADER_END sentinel")
template = "".join(lines[body_start:])
```

Then perform scalar substitution and marker fills on the remaining content. If a template is missing the sentinel, abort — do not attempt ad-hoc stripping.

## Exit criteria

A clean dry-run must satisfy all of:

- [ ] All six artifacts exist in `$SCRATCH`: `cover.svg`, `cover-chain.svg` OR `cover-chain.SKIPPED`, `RELEASE_NOTES.md`, `manifest.yml`, `pr-body.md`, `proposed-changes.diff`
- [ ] `xmllint --noout` passes on both SVGs
- [ ] Zero remaining `{{TOKEN}}` patterns in any rendered text file
- [ ] Zero remaining `<!-- @@MARKER -->` comments (except `@@BREAKING` if intentionally omitted for no-breaking-change releases)
- [ ] Sentinel checks — first non-empty line of each file must match:
  - `RELEASE_NOTES.md` → `<div align="center">`
  - `manifest.yml` → `release: v`
  - `pr-body.md` → `<div align="center">`
  - If any starts with `<!--` or `#`, the header-strip failed.

If any check fails, report the failure and stop. A failing dry-run is a real-run bug caught early.

## Summary output format

Print at the end:

```
======================================================
DRY RUN SUMMARY — v{VERSION}
======================================================
scratch dir: /tmp/release-dry-run/v{VERSION}/

  cover.svg                 NN,NNN bytes  ✓ xmllint passes
  cover-chain.svg           NN,NNN bytes  ✓ xmllint passes
  RELEASE_NOTES.md          NN,NNN bytes  0 unfilled tokens / 0 unfilled markers
  manifest.yml                 NNN bytes
  pr-body.md                NN,NNN bytes
  proposed-changes.diff      NN,NNN bytes  (CHANGELOG.md + Cargo.toml deltas)

Preview:
  open /tmp/release-dry-run/v{VERSION}/RELEASE_NOTES.md   # covers render via relative paths

Would do on real run:
  git checkout -b release/v{VERSION}
  git add .github/releases/v{VERSION}/ Cargo.toml Cargo.lock CHANGELOG.md
  git commit -m "Release v{VERSION}"
  git push -u origin release/v{VERSION}
  gh pr create --draft --title "Release v{VERSION}" --label release --body-file pr-body.md

No mutations were performed. Open the scratch dir to review artifacts.
======================================================
```

## Reset

To clear a prior dry-run output: `rm -rf /tmp/release-dry-run/v{VERSION}/`
