---
name: check-internal-docs
description: Verify that internal documentation (READMEs, docs/ site, badges) stays consistent with the codebase. Catches version mismatches, broken links, and stale references.
---

# Check Internal Docs

Scan all documentation files in the repo and verify they are consistent with the current codebase state.

---

## Phase 1: Version Consistency

1. Read the workspace version from `Cargo.toml` (`[workspace.package]` → `version`).

2. Check every location that references the CLI version:
   - `README.md` — CLI badge (`dot%20v{VERSION}`)
   - `docs/getting-started/installation.md` — expected `dot --version` output
   - Any other `.md` file referencing `dot {VERSION}` or `dot v{VERSION}`

3. Flag any file where the referenced version doesn't match `Cargo.toml`.

---

## Phase 2: Internal Link Validation

1. Find all relative markdown links (`[text](path)`) in:
   - `docs/**/*.md`
   - `dot/sdk/README.md`
   - `dot/cli/README.md`
   - `CONTRIBUTING.md`
   - `README.md`

2. For each relative link, resolve the path from the file's directory and verify the target file exists.

3. Flag broken links with the source file, line number, and broken target.

---

## Phase 3: Workflow References

1. Collect all actual workflow filenames from `.github/workflows/*.yml`.

2. Search `docs/**/*.md` for any workflow filename references (patterns like `*.yml` that look like workflow names).

3. Flag any referenced workflow that doesn't exist in `.github/workflows/`.

---

## Phase 4: Badge URL Validation

1. Find all badge image URLs in `README.md` files across the repo (patterns matching `shields.io` or `github.com/...actions/workflows/`).

2. For GitHub Actions badges, verify the referenced workflow file exists in `.github/workflows/`.

3. Flag badges pointing to nonexistent workflows.

---

## Phase 5: Report & Fix

Present a summary table:

| File | Issue | Details |
|------|-------|---------|
| ... | Version mismatch | Says `0.5.0`, should be `0.11.6` |
| ... | Broken link | `../docs/RELEASE_PROCESS.md` → file not found |
| ... | Stale workflow ref | `test-polkadot-sdk-recipes.yml` doesn't exist |

Then act autonomously:

- **Auto-fix** version mismatches and broken links where the correct target is unambiguous.
- **Report only** for issues requiring judgment (e.g., a referenced workflow was removed — unclear what to replace it with).
- Commit fixes on the current branch. If on `master`, create a new branch `chore/fix-internal-docs` first.
- Create a **draft PR** if fixes were made.
- If all checks pass, report "All internal docs are consistent" and stop.
