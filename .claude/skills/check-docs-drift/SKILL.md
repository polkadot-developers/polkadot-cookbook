---
name: check-docs-drift
description: Detect upstream changes to polkadot-docs tutorials and classify drifts as cosmetic or substantive. Run on-demand to find which test harnesses need updating.
---

# Check Docs Drift

Scan all `polkadot-docs/` test harnesses, compare their pinned `docs_commit` against the latest upstream commit, and classify any drifts.

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

## Phase 6: Self-Improvement

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
