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

For each drifted file, spawn a **parallel subagent** (via the Agent tool) that:

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

## Phase 4: Offer Actions

After presenting results, offer the user these actions:

- **Cosmetic drifts**: "Want me to auto-update `docs_commit` in these READMEs to the latest SHA?" — If yes, edit the frontmatter in each affected README, commit, and push.
- **Missing `docs_commit`**: "Want me to add `docs_commit` to these READMEs with the current latest SHA?" — If yes, add the field after the last frontmatter line before `---`, commit, and push.
- **Substantive drifts**: "These guides have meaningful upstream changes. Review the diffs and update test harnesses as needed." — Do not auto-bump; the test code likely needs changes too.
