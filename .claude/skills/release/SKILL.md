---
name: release
description: Cut a new versioned release. Analyzes changes since last tag, determines semver bump, generates release notes and manifest, updates Cargo.toml, and opens a draft PR that triggers publish-release.yml on merge.
disable-model-invocation: true
---

# Release

Create a versioned release of the Polkadot Cookbook. This skill replaces the old `release-weekly.yml` and `release-on-breaking-change.yml` workflows.

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

Analyze the categorized changes to determine the semver bump. Use your understanding of the actual changes, not just commit prefixes.

**Rules (alpha v0.x.x):**
- Breaking changes to CLI/SDK public API → **minor** (becomes major post-v1.0)
- New recipes, new docs test harnesses, new features (`feat:`) → **minor**
- Bug fixes, CI changes, config, docs, chores → **patch**

**Highest wins:** If any change is minor-worthy, the whole release is minor.

Calculate the new version:
```
Current: v0.11.6
Patch:   v0.11.7
Minor:   v0.12.0
```

---

## Phase 3: Generate Release Artifacts

1. **Create the release directory:**

   ```
   .github/releases/vX.Y.Z/
   ```

2. **Generate `manifest.yml`:**

   ```yaml
   release: vX.Y.Z
   release_date: 2026-04-03T00:00:00Z   # current UTC time
   status: alpha

   tooling:
     rust: "1.91.1"       # from: rustc --version
     node: "v20.20.1"     # from: node --version
   ```

   Get actual tool versions from the local environment.

3. **Generate `RELEASE_NOTES.md`:**

   Write meaningful, human-readable release notes. Group changes by category, not just a raw commit list. Example structure:

   ```markdown
   # Release vX.Y.Z

   Released: YYYY-MM-DD

   ## What's New

   ### Recipes
   - Added **Uniswap V2 Core with Hardhat** recipe (contracts pathway)

   ### Documentation Tests
   - Added test harness for **Query On-Chain State with Sidecar REST API** guide

   ### CLI & SDK
   - (if applicable)

   ### Infrastructure
   - Standardized test harness configuration across all harnesses

   ## Commits

   - feat: add query-rest polkadot-docs test harness (#210)
   - chore: standardize test harness configuration (#206)

   ## Compatibility

   Tested with:
   - Rust: 1.91.1
   - Node.js: v20.20.1

   ---

   **Status:** Alpha (v0.x.x)
   ```

   Omit empty categories. The "What's New" section should summarize in plain language; the "Commits" section is the raw list for reference.

4. **Update `Cargo.toml`** workspace version (strip `v` prefix):

   Edit `Cargo.toml` `[workspace.package]` → `version = "X.Y.Z"`

5. **Update `Cargo.lock`:**

   ```bash
   cargo update --workspace
   ```

---

## Phase 4: Create Release PR

1. Create a release branch:

   ```bash
   git checkout -b release/vX.Y.Z
   ```

2. Stage and commit:

   ```bash
   git add .github/releases/vX.Y.Z/ Cargo.toml Cargo.lock
   git commit -m "chore(release): vX.Y.Z"
   ```

3. Push and create a **draft PR**:

   ```bash
   git push -u origin release/vX.Y.Z
   gh pr create --draft \
     --title "Release vX.Y.Z" \
     --label "release" \
     --body "..."
   ```

   The PR body should include:
   - The full release notes content from Phase 3
   - A "Next Steps" section explaining that merging triggers `publish-release.yml` (tag creation, binary builds, GitHub Release)

4. Report the PR URL.

---

## Phase 5: Self-Improvement

After completing the pipeline, reflect on what happened during this run:

1. **What edge cases were hit?** Did the version bump logic handle all commit patterns correctly? Were there commits that were hard to categorize (e.g., a `chore:` that actually added a feature)?
2. **Were the release notes accurate?** Did the categorization match reality? Were any changes miscategorized or missed entirely?
3. **Did the `publish-release.yml` contract hold?** Was the manifest format correct? Did the file paths match what the workflow expects?
4. **What was unclear?** Were any instructions in this skill ambiguous or missing for the scenario encountered?

If you identified concrete improvements, create a **draft PR** on a separate branch (`chore/improve-release-skill`) with changes to this skill file.

**When writing improvements, follow the [Claude Code skills documentation](https://code.claude.com/docs/en/skills.md) and these best practices:**
- Keep `SKILL.md` **directive, not prescriptive** — say "study this reference file and adapt" instead of embedding full code templates. Inline code goes stale; real files in the repo stay current.
- Keep the skill **concise** (under ~200 lines). If detailed reference material is needed, split it into supporting files in this skill's directory and link from `SKILL.md`.
- **Reference existing releases** (e.g., `.github/releases/v0.11.6/`) rather than duplicating their patterns.
- **Keep the skill autonomous** — never add steps that prompt or wait for user input.
- Always create **draft PRs** (`gh pr create --draft`).

Include in the PR description:
- What triggered the improvement (the specific failure or gap encountered)
- What changed and why

This ensures the skill evolves with each use rather than accumulating blind spots.
