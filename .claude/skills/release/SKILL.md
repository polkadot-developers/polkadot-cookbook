---
name: release
description: Cut a new versioned release. Analyzes changes since last tag, determines semver bump, generates release notes and manifest, updates Cargo.toml and CHANGELOG.md, and opens a draft PR that triggers publish-release.yml on merge.
---

# Release

Create a versioned release of the Polkadot Cookbook. Accepts an optional bump override argument: `/release patch`, `/release minor`, or `/release major`. If no argument is given, the bump is determined automatically.

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

4. **Filter out release bookkeeping commits** — exclude commits matching `chore(release): v*` from the "What's New" and categorization. They are version bumps from prior releases, not user-facing changes.

5. If there are zero non-bookkeeping commits since the last tag, report "Nothing to release" and **stop**.

---

## Phase 2: Determine Version Bump

If a bump override was provided (`/release patch|minor|major`), use it directly and skip auto-detection. Otherwise, analyze commits.

### Breaking Change Detection

Scan every commit for breaking change signals (any match = breaking):
- **Footer convention:** `BREAKING CHANGE:` or `BREAKING-CHANGE:` appearing at the **start of a line** in the commit body (`git log --format="%B"`). Ignore free-text mentions of "breaking" that aren't footer-formatted — these are descriptions, not signals.
- **Exclamation mark convention:** `feat!:`, `fix!:`, `refactor!:` etc. in the subject line
- **Public API surface:** if CLI/SDK files changed, diff `dot/sdk/src/lib.rs` exports and CLI help output for removed/renamed items

### Squash merge handling

GitHub squash merges often strip conventional commit prefixes (e.g., "Add feature (#123)" instead of "feat: add feature"). When a commit has no `type:` prefix, fall back to **diff-based categorization** — the file paths changed determine the category, not the subject line.

### Version Bump Rules (alpha v0.x.x)

- Breaking changes to CLI/SDK public API → **minor** (becomes major post-v1.0)
- New recipes, new docs test harnesses, new features (`feat:`) → **minor**
- Bug fixes, CI changes, config, docs, chores → **patch**

**Highest wins:** If any change is minor-worthy, the whole release is minor.

---

## Phase 3: Generate Release Artifacts

### 3a. Gather metadata

- **PR numbers:** For each commit, if the subject doesn't already contain `(#N)`, look up the associated PR: `gh pr list --search "{sha}" --state merged --json number --jq '.[0].number'`. Every commit in the release notes **must** have a `(#N)` PR reference so GitHub renders clickable links.
- **Stats:** `git diff --shortstat {tag}..HEAD` and `git log --oneline {tag}..HEAD | wc -l` for commit count
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

### 3c. Cover art (template-driven, fact-bound)

**Do not hand-design.** Render `.github/releases/vX.Y.Z/cover.svg` from the canonical template at [`COVER_TEMPLATE.svg`](COVER_TEMPLATE.svg), filled with values from git queries per the contract in [`COVER_DATA.md`](COVER_DATA.md).

Pipeline:

1. Compute every scalar token (`{{VERSION}}`, `{{COMMIT_COUNT}}`, `{{INSERTIONS}}`, `{{HEAD_SHA}}`, `{{PR_LIST}}`, …) using the commands listed in `COVER_DATA.md`. A failed command aborts the release — never fabricate a fallback value.
2. Generate the six variable-count fragments (`@@COMMIT_LIST`, `@@DAILY_TIMELINE`, `@@CONTRIBUTOR_LIST`, `@@BAR_CHART`, `@@COMMIT_TYPES`, `@@REPO_STATE`). Apply the scaling rules in `COVER_DATA.md` for commit counts of 1, 14, 27, 200, etc.
3. Substitute scalars first, then markers, then write to `.github/releases/vX.Y.Z/cover.svg`.
4. Sanitize any injected commit subject / author name: `&` → `&amp;`, `<` → `&lt;`, `>` → `&gt;`.
5. Validate: `xmllint --noout .github/releases/vX.Y.Z/cover.svg` — on failure, fix the template/data issue and re-render. Do **not** ship an invalid SVG.

The template itself is frozen. If a release needs a new data point on the cover, add the token to `COVER_TEMPLATE.svg` and its source command to `COVER_DATA.md` — never inline a value directly.

### 3d. Release notes

Generate `.github/releases/vX.Y.Z/RELEASE_NOTES.md`. Study existing releases in `.github/releases/` for the established format, then **enhance** with these sections:

```
<div align="center">
  <img src="https://raw.githubusercontent.com/polkadot-developers/polkadot-cookbook/vX.Y.Z/.github/releases/vX.Y.Z/cover.svg" alt="Release vX.Y.Z" width="100%" />
</div>

# Release vX.Y.Z

Released: YYYY-MM-DD

## Summary
2-3 sentences: what this release delivers and why it matters. Lead with the most impactful change.

## Breaking Changes          ← only if breaking changes exist; omit otherwise
- Description of what broke and migration steps

## What's New               ← group by category, omit empty categories
### Recipes / Documentation Tests / CLI & SDK / Infrastructure
```

**What's New writing rules:**
- Every bullet **must** include a PR link `(#N)` — look up missing ones via `gh`
- After stating what changed, explain **why it matters** to users in the same bullet. Don't use "why is this important" — just naturally say what it enables, what problem it solves, or what's now possible. Keep it to one sentence.
- Example: "Added test harness for **Pay Fees with a Different Token** guide — developers can now verify cross-chain fee payment flows work end-to-end before deploying (#237)"

```
## Migration Notes           ← only if versions.yml changed or deps bumped

## Commits                   ← ordered by type: feat → fix → chore → docs
- feat: ... (#N)             ← every commit MUST have a PR link
- fix: ... (#N)

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

**Do NOT include a Contributors section** — GitHub auto-generates one with avatars at the bottom of every release. Adding a manual one creates duplicates.

### 3e. Update CHANGELOG.md

Prepend the new release to `CHANGELOG.md` at the repository root (create the file if it doesn't exist). Follow the [Keep a Changelog](https://keepachangelog.com/) format.

**Structure:** The file must always have an `## [Unreleased]` section at the top (below the header), followed by versioned entries. When cutting a release:
1. Move any content under `[Unreleased]` into the new version entry
2. Leave `[Unreleased]` empty (with no subsections) for future changes
3. Add the new version entry below `[Unreleased]`

```markdown
## [Unreleased]

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
[Unreleased]: https://github.com/polkadot-developers/polkadot-cookbook/compare/vX.Y.Z...HEAD
[X.Y.Z]: https://github.com/polkadot-developers/polkadot-cookbook/compare/vA.B.C...vX.Y.Z
```

If `CHANGELOG.md` doesn't exist yet, create it with the header, `[Unreleased]` section, and the current release only.

### 3f. Update Cargo.toml and lockfile

- Edit `Cargo.toml` `[workspace.package]` → `version = "X.Y.Z"` (strip `v` prefix)
- Run `cargo update --workspace`

---

## Phase 4: Create Release PR

1. Create a release branch: `git checkout -b release/vX.Y.Z`

2. Stage and commit:
   ```bash
   git add .github/releases/vX.Y.Z/ Cargo.toml Cargo.lock CHANGELOG.md  # includes cover.svg
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

   **Cover art in PR body:** `RELEASE_NOTES.md` is authored with an absolute tag-pinned raw URL so the **published release page** renders correctly (tags exist at publish time; relative paths on release pages resolve against repo root and 404). That URL will 404 in the PR body because the tag does not exist yet. For the PR body only, rewrite the `<img>` `src` to a raw URL pinned to the release branch's HEAD **commit SHA** — the SHA is immutable, so the image keeps rendering even after the release branch is deleted post-merge:

   ```
   https://raw.githubusercontent.com/polkadot-developers/polkadot-cookbook/{commit-sha}/.github/releases/vX.Y.Z/cover.svg
   ```

   Do **not** pin the PR body URL to the branch name (`release/vX.Y.Z`) — the branch is deleted on merge and the PR body image will break retroactively.

4. Report the PR URL.

---

## Phase 5: Self-Improvement

Reflect on this run: bump logic correctness, release notes accuracy, CI contract validity, instruction gaps. If concrete improvements exist, open a **draft PR** on `chore/improve-release-skill` with changes to this skill file.
