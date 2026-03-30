---
name: add-polkadot-docs-test
description: Automated pipeline to create a polkadot-docs test harness from a tutorial URL. Analyzes the guide, generates all files, runs tests, debugs failures, and creates PRs.
argument-hint: "<tutorial-url>"
---

# Add Polkadot Docs Test Harness

Automated pipeline that creates a test harness under `polkadot-docs/{category}/{guide-name}/` verifying a docs.polkadot.com guide works as documented.

Run all steps sequentially without prompting the user. The pipeline is fully autonomous.

---

## Step 0: Analyze Tutorial

Accept the URL from `/add-polkadot-docs-test <url>`.

### 0a. Resolve URL to Source Markdown

Map the docs URL to its source file in `polkadot-developers/polkadot-docs`:
- `https://docs.polkadot.com/{path}/` → try `{path}.md` then `{path}/index.md`

```bash
gh api repos/polkadot-developers/polkadot-docs/contents/{resolved-path} --jq '.content' | base64 -d
gh api "repos/polkadot-developers/polkadot-docs/commits?path={resolved-path}&per_page=1" --jq '.[0].sha'
```

Also fetch any referenced code snippets (`--8<-- "code/..."` includes) to see the actual implementation code.

### 0b. Extract Structured Information

From the raw markdown, extract: title, description, category path, guide name, code snippets, external repo URLs, prerequisite tools, required secrets, versions.yml dependencies, and whether the guide has tabbed SDK sections.

**Directory naming:** Flatten intermediate URL segments to match existing conventions. Study existing directories under `polkadot-docs/{category}/` — e.g., URL `chain-interactions/query-data/runtime-api-calls` maps to dir `chain-interactions/runtime-api-calls`.

### 0c. Classify the Guide

Determine which patterns to use:

| Signal | Action |
|--------|--------|
| Has tabbed SDK sections (PAPI, PJS, Dedot, Python, Subxt) | Use multi-SDK test pattern |
| Uses PAPI | Add PAPI descriptor generation (`npx papi add`) |
| Uses Python | Add Python helper script + setup in CI |
| Uses Subxt/Rust | Add Subxt subproject with versions from `versions.yml` |
| References `versions.yml` keys | CI Variant B (guard job) |
| Deploys Solidity locally | Add `setup-revive-dev-node` |
| Needs full network / custom precompiles | Add `setup-zombienet-eth-rpc` |
| Needs secrets | Add `env:` block with `${{ secrets.* }}`, use `it.skipIf` |
| Clones an external repo | Pin by commit SHA, add cache |
| Simple Node.js-only | CI Variant A (simple) |

---

## Step 1: Scaffold the Directory

Create `polkadot-docs/{category}/{guide-name}/` with: `package.json`, `tsconfig.json`, `vitest.config.ts`, `.gitignore`, `README.md`.

**Before writing any file**, study 2-3 existing harnesses under `polkadot-docs/` that are closest to the guide type. Read their files and follow the exact same structure, dependency versions, and conventions. Key references:

- Multi-SDK guide: `polkadot-docs/chain-interactions/query-accounts/`
- Clone-and-build guide: `polkadot-docs/smart-contracts/basic-hardhat/`
- Complex build guide: `polkadot-docs/networks/run-a-parachain-network/`

For dependency versions, always use `versions.yml` as the source of truth — not versions from the tutorial.

For guides with Python sections, create `tests/{script_name}.py`. For Subxt, create `tests/subxt-{guide-name}/` with `Cargo.toml` + `src/bin/*.rs`. Study the corresponding files in the reference harnesses.

---

## Step 2: Write the Test

Create `tests/docs.test.ts`. **Study the closest reference test file** and adapt its pattern:

| Guide Type | Reference Test |
|-----------|----------------|
| Multi-SDK (5 SDKs) | `polkadot-docs/chain-interactions/runtime-api-calls/tests/docs.test.ts` |
| Multi-SDK (queries) | `polkadot-docs/chain-interactions/query-accounts/tests/docs.test.ts` |
| Clone + build (Hardhat) | `polkadot-docs/smart-contracts/basic-hardhat/tests/docs.test.ts` |
| Complex build + process | `polkadot-docs/smart-contracts/local-dev-node/tests/docs.test.ts` |

Key conventions to follow from the reference files:
- Each SDK or phase gets a numbered `describe` block
- WebSocket clients must be disconnected in `afterAll` hooks
- PAPI uses dynamic imports (`await import(...)`)
- Python/Rust tests use `execSync` to run external scripts
- Subxt test timeout must match the `it()` timeout argument
- External repos are pinned by **commit SHA** (not tags)
- Secret-dependent tests use `it.skipIf`

---

## Step 3: Create the CI Workflow

Create `.github/workflows/polkadot-docs-{guide-name}.yml`. **Study the closest reference workflow** and adapt:

| CI Type | Reference Workflow |
|---------|-------------------|
| Simple (no versions.yml) | `.github/workflows/polkadot-docs-basic-hardhat.yml` |
| Guard job (versions.yml) | `.github/workflows/polkadot-docs-query-accounts.yml` |
| Multi-SDK (all 5 SDKs) | `.github/workflows/polkadot-docs-runtime-api-calls.yml` |
| Rust + guard | `.github/workflows/polkadot-docs-local-dev-node.yml` |
| Secrets (private keys) | `.github/workflows/polkadot-docs-erc20-hardhat.yml` |

The `check-version-keys` guard action auto-detects which `versions.yml` keys your workflow uses by parsing `yq` calls — just add the right `yq` lines in the "Load versions" step.

---

## Step 4: Install Dependencies and Generate Artifacts

```bash
cd polkadot-docs/{category}/{guide-name}
npm install   # generates package-lock.json (commit this)
```

Run any additional setup needed based on classification (PAPI descriptors, pip install, subxt metadata download, cargo build). Study how the reference CI workflow does these steps and replicate locally.

---

## Step 5: Test + Debug Loop

```bash
cd polkadot-docs/{category}/{guide-name}
npm test
```

Classify failures and respond:

- **Category A** (code errors): Fix and re-run (max 5 iterations). Common: API version mismatches (e.g., subxt 0.44 vs 0.50), wrong import paths, missing `await`.
- **Category B** (infrastructure): Add resilience wrappers (`it.skipIf`, retries, try/catch with warnings).
- **Category C** (missing prerequisites): Report to user and stop.

**Do NOT proceed to Step 6 until all tests pass.** If tests cannot be made green after 5 iterations, report the status to the user and stop — do not commit or create PRs with failing tests.

---

## Step 6: Create PRs

Only after `npm test` exits with all tests passing:

1. **Cookbook PR**: Commit all generated files + CI workflow. Create as a **draft PR** (`gh pr create --draft`). Use title `feat: add {guide-name} polkadot-docs test harness`.
2. **Companion PR**: Immediately after creating the cookbook PR, create a **draft PR** in `polkadot-developers/polkadot-docs` adding the CI badge to the guide page. Reference the cookbook PR in the body: `Companion to polkadot-developers/polkadot-cookbook#{PR-number}`. Do not wait for the cookbook PR to merge — both PRs are created in the same run.

---

## Step 7: Self-Improvement

After completing the pipeline, reflect on what happened during this run:

1. **What patterns were missing?** Did you encounter a guide type, SDK combination, or CI setup not covered by any existing reference? Did you have to improvise or guess?
2. **What failed and why?** Were there Category A failures that could have been avoided with better guidance? API version mismatches? Missing setup steps?
3. **What reference files helped most?** Which existing harnesses were the best templates? Should new ones be added to the reference table?
4. **What was unclear?** Were any instructions in this skill ambiguous or misleading?

If you identified concrete improvements, create a **draft PR** on a separate branch (`chore/improve-add-polkadot-docs-test-skill`) with changes to this skill file.

**When writing improvements, follow the [Claude Code skills documentation](https://code.claude.com/docs/en/skills.md) and these best practices:**
- Keep `SKILL.md` **directive, not prescriptive** — say "study this reference file and adapt" instead of embedding full code templates. Inline code goes stale; real files in the repo stay current.
- Keep the skill **concise** (under ~200 lines). If detailed reference material is needed, split it into supporting files in this skill's directory and link from `SKILL.md`.
- **Reference existing harnesses** rather than duplicating their patterns. Point to the file path and describe what to look for.
- **Avoid over-specifying** — the Polkadot SDK accommodates many creative use cases. Provide classification signals and decision trees, not exhaustive templates for every scenario.
- **Keep the skill autonomous** — never add steps that prompt or wait for user input.
- Update the reference tables (Steps 2 and 3) when a new harness becomes a better example for a particular guide type.

Include in the PR description:
- What triggered the improvement (the specific failure or gap encountered)
- What changed and why
- Whether the improvement is backward-compatible with other guide types

This ensures the skill evolves with each use rather than accumulating blind spots.
