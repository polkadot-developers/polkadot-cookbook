---
name: check-docs-drift
description: Detect upstream changes to polkadot-docs tutorials, classify drifts, and sync dependency versions from upstream variables.yml.
---

# Check Docs Drift

Scan all `polkadot-docs/` test harnesses, compare their pinned `docs_commit` against the latest upstream commit, classify any drifts, and sync dependency versions from the upstream `variables.yml`.

---

## Metrics Tracking

Throughout the entire run, maintain running counters to produce a Run Report at the end:

- **Wall-clock time**: Record `date +%s` at the start and end of each phase to measure duration.
- **Counts per phase**: READMEs scanned, drifts found, subagents spawned, diff lines fetched, `gh api` calls made, files read/modified, `npm install` runs, versions bumped.

Track these as you go — do not retroactively estimate. Report them in Phase 8.

---

## Phase 1: Scan & Detect Drift

1. Find all `polkadot-docs/**/README.md` files. Exclude `node_modules`, `.test-workspace`, and the top-level `polkadot-docs/README.md`.

2. For each README, parse the YAML frontmatter and extract:
   - `source_github` or `source_repo` (both field names are used — check both)
   - `docs_commit`
   - `title`

3. Categorize each README:
   - **Trackable**: has a source URL (`source_github` or `source_repo`) AND `docs_commit`
   - **Missing pin**: has a source URL but no `docs_commit`
   - **Untracked**: no source URL — skip silently (these are cloned template READMEs, etc.)

4. For each trackable README, extract the file path from the GitHub URL (strip everything up to and including `/blob/master/`) and fetch the latest commit SHA:

   ```bash
   gh api "repos/polkadot-developers/polkadot-docs/commits?path={FILE_PATH}&per_page=1&sha=master" --jq '.[0].sha'
   ```

5. Compare the fetched SHA against `docs_commit`. Build a list of drifted entries (old SHA, new SHA, file path, title, README path).

---

## Phase 2: Analyze Drifts

**Optimization:** Before spawning subagents, group drifted files by their `(old_sha, new_sha)` pair. Files sharing the same commit range can be analyzed in a single subagent with one API call, avoiding redundant fetches. This is common when upstream makes a bulk change (e.g., badge updates) that touches many files at once.

For each drift (or group of drifts sharing a commit range), spawn a **parallel subagent** (via the Agent tool) that:

1. Fetches the patch:
   ```bash
   gh api "repos/polkadot-developers/polkadot-docs/compare/{old_sha}...{new_sha}" --jq '.files[] | select(.filename == "{FILE_PATH}") | .patch'
   ```
   If the patch is empty or truncated, fetch the raw file at both SHAs and diff locally.

2. Classifies the diff as **COSMETIC** or **SUBSTANTIVE**:

   **COSMETIC** (safe to auto-bump `docs_commit`):
   - Whitespace, indentation, blank lines
   - Markdown formatting changes (bold/italic syntax, heading levels without content change)
   - Typo corrections in prose (NOT in code blocks)
   - Link URL updates where destination content is unchanged
   - Badge/shield URL updates
   - Comment rewording with no semantic change
   - Content reordering without additions/removals

   **SUBSTANTIVE** (requires test harness review):
   - Any change inside fenced code blocks (commands, code, config)
   - New or removed numbered steps
   - Changed dependency names or versions
   - Changed CLI commands, flags, or arguments
   - New prerequisites or tool requirements
   - Changed file paths or directory structures
   - New or removed sections
   - Changed expected output or behavior descriptions

3. Returns: verdict (cosmetic/substantive), one-line summary, list of key changes.

---

## Phase 3: Present Results

Display a summary table grouped into sections:

### 1. Substantive Drifts (action needed)

| Guide | Drift | Summary |
|-------|-------|---------|
| {title} | [view diff]({compare_url}) | {one-line summary} |

### 2. Cosmetic Drifts (safe to bump)

| Guide | Drift | Summary |
|-------|-------|---------|
| {title} | [view diff]({compare_url}) | {one-line summary} |

### 3. Up to Date

List titles of guides where `docs_commit` matches the latest SHA.

### 4. Missing `docs_commit`

List guides that have a source URL but no `docs_commit` field — suggest adding it.

---

## Phase 4: Auto-Bump & Report

Act autonomously — do not prompt the user for confirmation:

- **Cosmetic drifts**: Automatically update `docs_commit` in each affected README to the latest SHA, commit, and push.
- **Substantive drifts**: Do NOT auto-bump. Report them in the results and the GitHub issue for manual review.
- **Missing `docs_commit`**: Report them in the results. Do not add the field automatically — this is an informational note for the user.

---

## Phase 5: Create GitHub Issue

After presenting results and performing any user-approved actions, create a GitHub issue summarizing the findings so a developer can resolve the substantive drifts later.

- **Title**: `[Docs Drift] {N} tutorial(s) updated upstream`
- **Label**: `docs-drift`
- Check for an existing open issue with the `docs-drift` label first. If one exists, comment on it instead of creating a new one.
- **Body** should include:
  - The full results table from Phase 3 (substantive drifts, cosmetic drifts, up to date, missing pins)
  - For substantive drifts: diff links and summaries of what changed
  - For cosmetic drifts: note whether they were auto-bumped or still pending
  - A call to action: "Review the substantive drifts and update test harnesses as needed."

```bash
# Check for existing issue
gh issue list --state open --label "docs-drift" --json number --jq '.[0].number // empty'

# Create or comment
gh issue create --title "{title}" --body "{body}" --label "docs-drift"
# OR
gh issue comment {number} --body "{body}"
```

---

## Phase 6: Sync Dependency Versions

Compare our `versions.yml` against the upstream [polkadot-docs `variables.yml`](https://github.com/polkadot-developers/polkadot-docs/blob/master/variables.yml).

1. Fetch the upstream file:
   ```bash
   gh api repos/polkadot-developers/polkadot-docs/contents/variables.yml --jq '.content' | base64 -d
   ```

2. For each entry in our `versions.yml`, find the matching entry in upstream `variables.yml` and compare versions. The structures differ — map between them:
   - `polkadot_sdk.release_tag` ↔ `dependencies.repositories.polkadot_sdk.version`
   - `parachain_template.crates.*` ↔ `dependencies.repositories.polkadot_sdk_parachain_template.subdependencies.*` and `dependencies.crates.*`
   - `zombienet.version` ↔ `dependencies.repositories.zombienet.version`
   - `crates.*` ↔ `dependencies.crates.*`
   - `javascript_packages.*` ↔ `dependencies.javascript_packages.*`

3. Build a list of version mismatches (our version vs upstream version).

4. For each outdated entry:
   - Update `versions.yml`
   - Find all `package.json` files (in `polkadot-docs/`, `recipes/`, `migration/`, and `dot/sdk/templates/`) that reference the same package at the old version and update them
   - Run `npm install --package-lock-only` in each modified harness directory to regenerate lockfiles

5. If any entries exist in upstream but not in our `versions.yml`, check whether that package is used in any harness `package.json`. If so, add it to `versions.yml` and report it.

6. Present a summary table of version changes made, and any new entries added.

---

## Phase 7: Self-Improvement

After completing the pipeline, reflect on what happened during this run:

1. **Were there API or parsing issues?** Did any README use a frontmatter field name not covered by the skill? Did the GitHub API return unexpected results?
2. **Was the batching effective?** Did grouping by commit range save API calls, or were there edge cases?
3. **Were the classifications accurate?** Did any diff get misclassified as cosmetic when it was substantive, or vice versa?
4. **Were there missing categories?** Did the rubric fail to cover a type of change encountered in the diffs?

If you identified concrete improvements, create a **draft PR** on a separate branch (`chore/improve-check-docs-drift-skill`) with changes to this skill file.

**When writing improvements, follow the [Claude Code skills documentation](https://code.claude.com/docs/en/skills.md) and these best practices:**
- Keep `SKILL.md` **directive, not prescriptive** — say "study this reference file and adapt" instead of embedding full code templates.
- Keep the skill **concise** (under ~200 lines).
- **Reference existing implementations** rather than duplicating patterns.
- **Keep the skill autonomous** — never add steps that prompt or wait for user input.
- **Always create draft PRs** (`gh pr create --draft`) — never create ready-to-merge PRs from skill runs.

Include in the PR description:
- What triggered the improvement (the specific failure or gap encountered)
- What changed and why

---

## Phase 8: Run Report

Display a final metrics summary so the user can see where time and tokens were spent:

```
## Run Report

| Phase | Duration | Key stats |
|-------|----------|-----------|
| 1. Scan & Detect | 12s | 15 READMEs scanned, 4 drifted |
| 2. Analyze Drifts | 38s | 3 subagents, 2 batches, 847 diff lines |
| 3. Present Results | 2s | — |
| 4. Auto-Bump | 5s | 2 cosmetic bumps committed |
| 5. GitHub Issue | 3s | Created #248 |
| 6. Version Sync | 22s | 4 bumped, 10 package.json, 8 lockfiles |
| 7. Self-Improvement | 8s | No improvements identified |
| **Total** | **90s** | **3 subagents, 847 diff lines, 15 API calls** |

### Bottleneck notes
- Phase 2 consumed most time due to 1 large diff (623 lines)
```

**Guidelines:**
- Always populate with real numbers from the run — never estimate or omit.
- In **Bottleneck notes**, call out the single largest time/token consumer and suggest whether it could be optimized (e.g., better batching, skipping unchanged sections, splitting large diffs).
- If any phase took 0s or had no work, show it as `—` to confirm it was reached.
