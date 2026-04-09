---
name: release
description: Cut a new versioned release. Analyzes changes since last tag, determines semver bump, generates release notes and manifest, updates Cargo.toml and CHANGELOG.md, and opens a draft PR that triggers publish-release.yml on merge.
---

# Release

Create a versioned release of the Polkadot Cookbook.

**Contract with CI:** The `publish-release.yml` workflow triggers on merge to `master` when `.github/releases/v*/manifest.yml` changes. It builds CLI binaries, creates a git tag, and publishes the GitHub Release. This skill's job is to produce the artifacts that workflow expects.

---

## Phase 1: Analyze Changes

1. Get the last release tag:

   ```bash
   git fetch --tags --force
   git describe --tags --abbrev=0 --match "v*.*.*"
   ```

   If no tag exists, treat all commits on `master` as new.

2. Get all commits since that tag:

   ```bash
   git log {tag}..HEAD --pretty=format:"%H %s" --no-merges
   ```

3. For each commit, read the diff (`git show --stat {sha}`) to understand what actually changed. Categorize changes into:

   | Category | Criteria |
   |----------|----------|
   | **CLI/SDK** | Files under `dot/cli/` or `dot/sdk/` |
   | **New recipe** | New directory under `recipes/{pathway}/` |
   | **New docs test** | New directory under `polkadot-docs/` |
   | **CI/Config** | `.github/workflows/`, `.claude/`, `versions.yml` |
   | **Docs** | `*.md` files, `docs/` |
   | **Chore** | Everything else |

4. If there are zero commits since the last tag, report "Nothing to release" and **stop**.

---

## Phase 2: Determine Version Bump

Analyze the categorized changes to determine the semver bump. Use **both** your understanding of the actual changes and conventional commit signals.

### Breaking Change Detection

Scan every commit for breaking change signals (any match = breaking):
- Conventional commit footer: `BREAKING CHANGE:` or `BREAKING-CHANGE:` in the commit body (`git log --format="%B"`)
- Exclamation mark convention: `feat!:`, `fix!:`, `refactor!:` etc. in the subject line
- Public API surface changes: if CLI/SDK files changed, diff `dot/sdk/src/lib.rs` exports and CLI help output for removed/renamed items

### Version Bump Rules (alpha v0.x.x)

- Breaking changes to CLI/SDK public API → **minor** (becomes major post-v1.0)
- New recipes, new docs test harnesses, new features (`feat:`) → **minor**
- Bug fixes, CI changes, config, docs, chores → **patch**

**Highest wins:** If any change is minor-worthy, the whole release is minor.

---

## Phase 3: Generate Release Artifacts

### 3a. Gather metadata

Collect this information before generating any files:

- **Contributors:** `git log {tag}..HEAD --format="%aN" | sort -u` — deduplicate, look up GitHub usernames where possible
- **Stats:** `git diff --shortstat {tag}..HEAD` — extract files changed, insertions, deletions
- **Diff link:** `https://github.com/polkadot-developers/polkadot-cookbook/compare/{tag}...v{new}`

### 3b. Release directory and manifest

Create `.github/releases/vX.Y.Z/` with `manifest.yml`:

```yaml
release: vX.Y.Z
previous_release: vA.B.C          # ← the tag from Phase 1
release_date: 2026-04-09T00:00:00Z
status: alpha

tooling:
  rust: "1.91.0"       # from: rustc --version
  node: "v24.7.0"      # from: node --version
```

Get actual tool versions from the local environment.

### 3c. Release notes

Generate `.github/releases/vX.Y.Z/RELEASE_NOTES.md`. Study existing releases in `.github/releases/` for the established format, then **enhance** with these sections:

```
# Release vX.Y.Z

Released: YYYY-MM-DD

> One-sentence release summary highlighting the most impactful change.

## Breaking Changes          ← only if breaking changes exist; omit otherwise
- Description of what broke and migration steps

## What's New               ← group by category, omit empty categories
### Recipes / Documentation Tests / CLI & SDK / Infrastructure

## Migration Notes           ← only if versions.yml changed or deps bumped
- What downstream consumers need to update

## Commits                   ← ordered by type: feat → fix → chore → docs
- feat: ... (#N)
- fix: ... (#N)

## Contributors
- @username1, @username2, ...

## Stats
**N commits, N contributors, +X / -Y lines**
**Full Changelog:** https://github.com/polkadot-developers/polkadot-cookbook/compare/vA.B.C...vX.Y.Z

## Compatibility
Tested with:
- Rust: 1.91.0
- Node.js: v24.7.0

---
**Status:** Alpha (v0.x.x)
```

### 3d. Update CHANGELOG.md

Prepend the new release to `CHANGELOG.md` at the repository root (create the file if it doesn't exist). Follow the [Keep a Changelog](https://keepachangelog.com/) format:

```markdown
## [X.Y.Z] - YYYY-MM-DD

### Added
- ...

### Changed
- ...

### Fixed
- ...

### Breaking
- ...
```

At the bottom of the file, maintain a link reference section:
```
[X.Y.Z]: https://github.com/polkadot-developers/polkadot-cookbook/compare/vA.B.C...vX.Y.Z
```

If `CHANGELOG.md` doesn't exist yet, create it with a header and backfill the current release only — don't attempt to reconstruct past releases.

### 3e. Update Cargo.toml and lockfile

- Edit `Cargo.toml` `[workspace.package]` → `version = "X.Y.Z"` (strip `v` prefix)
- Run `cargo update --workspace`

---

## Phase 4: Create Release PR

1. Create a release branch: `git checkout -b release/vX.Y.Z`

2. Stage and commit:
   ```bash
   git add .github/releases/vX.Y.Z/ Cargo.toml Cargo.lock CHANGELOG.md
   git commit -m "chore(release): vX.Y.Z"
   ```

3. Push and create a **draft PR**:
   ```bash
   git push -u origin release/vX.Y.Z
   gh pr create --draft --title "Release vX.Y.Z" --label "release" --body "..."
   ```

   The PR body should include:
   - The full release notes content from Phase 3
   - A "Next Steps" section explaining that merging triggers `publish-release.yml` (tag creation, binary builds, GitHub Release)

4. Report the PR URL.

---

## Phase 5: Self-Improvement

After completing the pipeline, reflect on what happened during this run:

1. Did the version bump logic handle all commit patterns correctly?
2. Were the release notes accurate and well-categorized?
3. Did the `publish-release.yml` contract hold?
4. Were any instructions ambiguous?

If you identified concrete improvements, create a **draft PR** on a separate branch (`chore/improve-release-skill`) with changes to this skill file. Follow the best practices in the skill directory's supporting docs.
