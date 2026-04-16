---
name: release
description: Cut a new versioned release. Analyzes changes since last tag, determines semver bump, generates release notes and manifest, updates Cargo.toml and CHANGELOG.md, and opens a draft PR that triggers publish-release.yml on merge. Supports `--dry-run` to preview all artifacts in a scratch dir without any git or GitHub mutations.
argument-hint: "[--dry-run] [patch|minor|major]"
---

# Release

Create a versioned release of the Polkadot Cookbook. Accepts an optional bump override argument: `/release patch`, `/release minor`, or `/release major`. If no argument is given, the bump is determined automatically.

**Contract with CI:** The `publish-release.yml` workflow triggers on merge to `master` when `.github/releases/v*/manifest.yml` changes. It builds CLI binaries, creates a git tag, and publishes the GitHub Release. This skill's job is to produce the artifacts that workflow expects.

**If invoked with `--dry-run`, read [`DRY_RUN.md`](DRY_RUN.md) and follow that pipeline instead of the one below.** The real-run pipeline below makes git/GitHub mutations. Do not run it in dry-run mode.

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

### Rendering assertions (apply after every render step)

Each render step below ends with **two** hard assertions. Both must pass or the release aborts:

1. **`xmllint --noout` on SVG outputs** (or `yamllint`/YAML round-trip on manifest).
2. **Zero unresolved placeholders.** Run `grep -oE '\{\{[A-Z_]+\}\}' <output-file>` — any output = abort. Any output of `grep -oE '<!-- @@[A-Z_]+ -->' <output-file>` = abort (except `@@BREAKING` when omitted for no-breaking-change releases).

Why: `xmllint` alone passes on SVGs that still contain unresolved `{{TOKEN}}` text (they're valid XML character data). The visible failure — "giant `{{CHAIN_NAME_UPPER}}` rendered on the release cover" — only shows up in a browser preview or on the published release page. Past incident: v0.15.0 shipped with ~25 unresolved tokens in `cover-chain.svg` because the renderer's token set drifted from the data contract; caught visually after the PR was opened.

**Proactive guard (cheaper than discovering drift post-render):** before computing tokens, extract the canonical list from the template:

```bash
grep -oE '\{\{[A-Z_]+\}\}' covers/cover-chain.svg.template | sort -u
```

Make sure every entry maps to a value in your substitution table. If the template gained a token that the data contract / renderer hasn't caught up with, that mismatch becomes visible here — not on the published release page.

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

**Do not hand-design.** Render `.github/releases/vX.Y.Z/cover.svg` from the canonical template at [`covers/cover.svg.template`](covers/cover.svg.template), filled with values from git queries per the contract in [`covers/cover.data.md`](covers/cover.data.md).

Pipeline:

1. Compute every scalar token (`{{VERSION}}`, `{{COMMIT_COUNT}}`, `{{INSERTIONS}}`, `{{HEAD_SHA}}`, `{{PR_LIST}}`, …) using the commands listed in `covers/cover.data.md`. A failed command aborts the release — never fabricate a fallback value.
2. Generate the six variable-count fragments (`@@COMMIT_LIST`, `@@DAILY_TIMELINE`, `@@CONTRIBUTOR_LIST`, `@@BAR_CHART`, `@@COMMIT_TYPES`, `@@REPO_STATE`). Apply the scaling rules in `covers/cover.data.md` for commit counts of 1, 14, 27, 200, etc.
3. Substitute scalars first, then markers, then write to `.github/releases/vX.Y.Z/cover.svg`.
4. Sanitize any injected commit subject / author name: `&` → `&amp;`, `<` → `&lt;`, `>` → `&gt;`.
5. Validate per **Rendering assertions** above: `xmllint --noout` + zero unresolved `{{TOKEN}}` / `<!-- @@MARKER -->`. Abort on any failure.

The template itself is frozen. If a release needs a new data point on the cover, add the token to `covers/cover.svg.template` and its source command to `covers/cover.data.md` — never inline a value directly.

### 3c.2. Footer cover — chain-state reading

Each release also ships a second cover at `.github/releases/vX.Y.Z/cover-chain.svg`: a point-in-time reading of Polkadot mainnet as it was at release-cut time. Rendered from [`covers/cover-chain.svg.template`](covers/cover-chain.svg.template) using data pulled via JSON-RPC per the contract in [`covers/cover-chain.data.md`](covers/cover-chain.data.md).

Pipeline:

1. Walk the endpoint list (primary → fallbacks) until one answers within 5s total-budget 15s.
2. Run the one-shot capture sequence (`chain_getFinalizedHead` → `chain_getHeader` → `state_getRuntimeVersion` → `system_*` → `system_properties` → `chain_getBlockHash [0]`), record capture timestamp in UTC.
3. **If all endpoints fail: skip this cover entirely.** Do not write `cover-chain.svg`. Omit the footer embed from `RELEASE_NOTES.md`. Log the failure in the release PR body. Never fabricate or cache-reuse chain data.
4. If success: compute scalars per the table in `covers/cover-chain.data.md`, substitute into template, sanitize (`&<>`), write, then run the **Rendering assertions** above (`xmllint --noout` + zero unresolved `{{TOKEN}}`). The cover-chain template has the most tokens of any artifact (~30); missing even one results in visible `{{TOKEN_NAME}}` text in the rendered SVG. Abort on any failure.
5. Append the footer embed block to `RELEASE_NOTES.md` (after the "Next Steps" section, preceded by an `---` separator):

   ```html
   <div align="center">
     <img src="https://raw.githubusercontent.com/polkadot-developers/polkadot-cookbook/v{{VERSION}}/.github/releases/v{{VERSION}}/cover-chain.svg" alt="Polkadot network state at v{{VERSION}} release" width="100%" />
   </div>
   ```

The footer cover is deliberately text-dense and point-in-time; the B1 disclaimer badge is the single source of truth for its historical nature. Do not add redundant "at snapshot" / "at release-cut" qualifiers elsewhere in the template.

### 3d. Release notes (template-driven)

**Do not hand-author the scaffolding.** Render `.github/releases/vX.Y.Z/RELEASE_NOTES.md` from [`RELEASE_NOTES.template.md`](RELEASE_NOTES.template.md). Substitute scalar tokens (`{{VERSION}}`, `{{PREV_VERSION}}`, `{{RELEASE_DATE}}`, `{{RUST_VERSION}}`, `{{NODE_VERSION}}`, `{{COMMIT_COUNT}}`, `{{INSERTIONS}}`, `{{DELETIONS}}`) from the same git queries used by the top cover; then fill four LLM-authored marker sections:

| Marker | Content |
|---|---|
| `<!-- @@SUMMARY -->` | 2-3 sentences: what this release delivers and why. Lead with the most impactful change. |
| `<!-- @@BREAKING -->` | If no breaking changes, omit the entire `## Breaking Changes` block. If present, bullet list of what broke + migration steps. |
| `<!-- @@WHATS_NEW -->` | `### Category` subheadings (Recipes / Documentation Tests / CLI & SDK / Infrastructure / Tooling) and bulleted entries. Every bullet MUST include a PR link `(#N)` — look up missing ones via `gh`. After stating what changed, explain **why it matters** in the same bullet. Example: "Added test harness for **Pay Fees with a Different Token** guide — developers can now verify cross-chain fee payment flows work end-to-end before deploying (#237)" |
| `<!-- @@COMMITS -->` | Bulleted commit subjects with `(#N)` PR links, ordered by type (`feat` → `fix` → `chore` → `docs` → `ci`), newest first within each group. |

**Do NOT include a Contributors section** — GitHub auto-generates one with avatars at the bottom of every release. Adding a manual one creates duplicates.

**Do NOT** add the cover embeds, the `## Next Steps` block, the `---\n**Status:** Alpha` footer, or the footer-cover embed manually — those are part of the template scaffolding and are generated by substitution.

After rendering, run the **Rendering assertions** on `RELEASE_NOTES.md`: zero unresolved `{{TOKEN}}` and zero unresolved `<!-- @@MARKER -->` (except `@@BREAKING` when omitted for no-breaking-change releases). Abort on any failure.

<details>
<summary>Previous prose shape (kept here for reference — superseded by the template)</summary>

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

</details>

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

### 3g. Manifest (template-driven)

Render `.github/releases/vX.Y.Z/manifest.yml` from [`MANIFEST.template.yml`](MANIFEST.template.yml). Substitute `{{VERSION}}`, `{{PREV_VERSION}}`, `{{RELEASE_DATE}}` (ISO-8601 UTC), `{{STATUS}}` (`alpha` while major=0, `beta` during 1.0 RC, `stable` thereafter), `{{RUST_VERSION}}`, `{{NODE_VERSION}}`. No markers — manifest is scalar-only.

Run the **Rendering assertions** on `manifest.yml`: YAML-parseable (`python3 -c "import yaml; yaml.safe_load(open('...'))"` or equivalent) and zero unresolved `{{TOKEN}}`. Abort on any failure.

---

## Phase 4: Create Release PR

1. Create a release branch: `git checkout -b release/vX.Y.Z`

2. Stage and commit:
   ```bash
   git add .github/releases/vX.Y.Z/ Cargo.toml Cargo.lock CHANGELOG.md  # includes cover.svg
   git commit -m "chore(release): vX.Y.Z"
   ```

3. Push and create a **draft PR** whose body is rendered from [`RELEASE_PR_BODY.template.md`](RELEASE_PR_BODY.template.md):

   ```bash
   git push -u origin release/vX.Y.Z
   gh pr create --draft --title "Release vX.Y.Z" --label "release" --body-file <rendered-body>
   ```

   The template embeds the cover via a **commit-SHA-pinned** raw URL (not the branch name — the branch is deleted on merge and branch-based raw URLs break retroactively). Scalar tokens and marker sections are documented inline in the template file.

   Branch name, commit subject, and tag format follow [`COMMIT_CONVENTIONS.md`](COMMIT_CONVENTIONS.md) — do not vary from those.

4. Report the PR URL.

---

## Phase 5: Self-Improvement

Reflect on this run: bump logic correctness, release notes accuracy, CI contract validity, instruction gaps. If concrete improvements exist, open a **draft PR** on `chore/improve-release-skill` with changes to this skill file.
