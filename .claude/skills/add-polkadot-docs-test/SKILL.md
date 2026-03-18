---
name: add-polkadot-docs-test
description: Automated pipeline to create a polkadot-docs test harness from a tutorial URL. Analyzes the guide, generates all files, runs tests, debugs failures, and creates PRs.
argument-hint: "<tutorial-url>"
---

# Add Polkadot Docs Test Harness

Automated two-phase pipeline that creates a test harness under `polkadot-docs/{category}/{guide-name}/` verifying a docs.polkadot.com guide works as documented.

## Execution Model

### Phase 1: Analyze + Generate (can run as background subagent)

Steps 0–4 below can optionally be dispatched as a single background subagent via the Agent tool when the user wants to continue other work while generation happens. The subagent should:
1. Create a feature branch
2. Perform Steps 0–4 (analyze, scaffold, write test, create CI workflow, install deps)
3. Return the branch name and generated file list

Default path: run Steps 0–4 inline in the main session.

### Phase 2: Test + Debug + PR (main session, iterative)

Steps 5–6 must stay in the main session because test-debug-fix is iterative — a one-shot subagent can't course-correct across multiple test runs.

**Human checkpoints:**
- After Phase 1: user reviews generated files before testing
- Before PR creation: user approves

---

## Step 0: Analyze Tutorial

Accept the URL argument from `/add-polkadot-docs-test <url>`.

### 0a. Resolve URL to Source Markdown

Map the docs URL to its source file in the `polkadot-developers/polkadot-docs` repo:
- `https://docs.polkadot.com/{path}/` → `{path}/index.md` (most guides)
- `https://docs.polkadot.com/{path}/` → `{path}.md` (some pages)

Fetch the raw source via GitHub API:
```bash
gh api repos/polkadot-developers/polkadot-docs/contents/{resolved-path} --jq '.content' | base64 -d
```

Also get the latest commit SHA for this file (used as `docs_commit` in the README):
```bash
gh api "repos/polkadot-developers/polkadot-docs/commits?path={resolved-path}&per_page=1" --jq '.[0].sha'
```

If the path doesn't resolve, try alternate paths (with/without `index.md`, different nesting).

### 0b. Extract Structured Information

From the raw markdown, extract:
- **Title**: from frontmatter `title:` field or first `#` heading
- **Description**: from frontmatter `description:` field
- **Category path**: derive from the URL structure (e.g., `smart-contracts`, `chain-interactions`)
- **Guide name**: the URL slug (e.g., `deploy-a-basic-contract-with-hardhat` → `basic-hardhat` or similar short name matching existing conventions)
- **Code snippets**: all fenced code blocks with their language tags
- **External repo URLs**: any GitHub repo references that tests need to clone
- **Prerequisite tools**: mentioned tools like Hardhat, Python, Rust, subxt, etc.
- **Required secrets**: private keys, API keys, RPC URLs referenced in the guide
- **versions.yml dependencies**: any references to pinned SDK versions, toolchain versions, etc.

### 0c. Classify the Guide

Use this decision tree to determine which CI template variant and test patterns to use:

| Signal | Action |
|--------|--------|
| Tutorial references `versions.yml` keys (SDK version, tool versions) | Add guard job + `versions.yml` path trigger (see CI Variant B) |
| Deploys Solidity to a local node | Add `setup-revive-dev-node` composite action |
| Uses custom precompiles or needs full network | Add `setup-zombienet-eth-rpc` composite action |
| Builds Rust code (cargo build) | Add Rust toolchain setup + cargo caching in CI |
| Uses Python | Add `actions/setup-python` step |
| Needs secrets (private keys, API keys, RPC URLs) | Add `env:` block with `${{ secrets.* }}`, use `it.skipIf` for local runs without secrets |
| Clones an external repo | Use pinned commit SHA pattern, add npm/cargo cache for cloned deps |
| Simple Node.js-only guide (no special infra) | Use simple CI template (Variant A) |

---

## Step 1: Scaffold the Directory

Create these files in `polkadot-docs/{category}/{guide-name}/`:

**package.json:**
```json
{
  "name": "{guide-name}",
  "version": "1.0.0",
  "description": "Test verification for the {Guide Title} guide",
  "type": "module",
  "scripts": {
    "test": "vitest run"
  },
  "devDependencies": {
    "@types/node": "^22.10.5",
    "typescript": "^5.7.2",
    "vitest": "^2.1.9"
  }
}
```

**tsconfig.json:**
```json
{
  "compilerOptions": {
    "target": "ES2022",
    "module": "ESNext",
    "moduleResolution": "bundler",
    "esModuleInterop": true,
    "strict": true,
    "skipLibCheck": true,
    "resolveJsonModule": true,
    "isolatedModules": true,
    "noEmit": true,
    "types": ["node"]
  },
  "include": ["tests/**/*.ts"],
  "exclude": ["node_modules"]
}
```

**vitest.config.ts:**
```typescript
import { defineConfig } from "vitest/config";

export default defineConfig({
  test: {
    fileParallelism: false,
    sequence: { shuffle: false },
    testTimeout: 30000,      // Adjust: 30s simple, 360000 contracts, 1800000 builds
    hookTimeout: 10000,
    reporters: ["verbose"],
    pool: "forks",
    poolOptions: { forks: { singleFork: true } },
    include: ["tests/recipe.test.ts"],
  },
});
```

If the test needs pinned versions from `versions.yml`, import the shared loader:
```typescript
import { loadVariables } from "../../shared/load-variables";
const vars = loadVariables();

export default defineConfig({
  test: {
    env: {
      POLKADOT_SDK_VERSION: vars.POLKADOT_SDK_VERSION,
      // ... other vars as needed
    },
    // ... rest of config
  },
});
```

**.gitignore:**
```
node_modules/
.test-workspace/
dist/
*.log
```

**README.md** — use this frontmatter format:
```markdown
---
title: "{Guide Title}"
description: "Verify the {Guide Title} guide from docs.polkadot.com"
source_url: "https://docs.polkadot.com/{path/to/guide}/"
source_github: "https://github.com/polkadot-developers/polkadot-docs/blob/master/{path/to/guide}.md"
docs_commit: "{commit SHA from Step 0a}"
---

# {Guide Title}

[![{Guide Title}](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-{guide-name}.yml/badge.svg?event=push)](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-{guide-name}.yml)

This project verifies the [{Guide Title}]({source_url}) guide from docs.polkadot.com.

## Running Tests

\```bash
npm ci
npm test
\```
```

---

## Step 2: Write the Test

Create `tests/recipe.test.ts` with numbered phases:

```typescript
import { describe, it, expect } from "vitest";

describe("{Guide Title}", () => {
  describe("1. Prerequisites", () => {
    // Check required tools
  });

  describe("2. Setup", () => {
    // Clone repo or set up environment
  });

  describe("3. Verify", () => {
    // Run the guide's steps and assert correctness
  });
});
```

**Pinning strategy:** polkadot-docs tests pin external repos by **commit SHA** (not tags).

**Cloning pattern** for tests that clone repos:
```typescript
const WORKSPACE_DIR = join(process.cwd(), ".test-workspace");
const REPO_URL = "https://github.com/...";
const PINNED_COMMIT = "abc123...";
```

**Secret-dependent tests:** When a test requires secrets (private keys, API keys), use `it.skipIf` to gracefully skip in local environments:
```typescript
const HAS_PRIVATE_KEY = !!process.env.PRIVATE_KEY;

it.skipIf(!HAS_PRIVATE_KEY)("deploys contract to testnet", async () => {
  // test that requires PRIVATE_KEY
});
```

---

## Step 3: Create the CI Workflow

Create `.github/workflows/polkadot-docs-{guide-name}.yml` using the appropriate variant.

### Variant A: Simple (no versions.yml dependency)

Use when: guide is Node.js-only, no pinned SDK versions, no special infrastructure.

Reference: `.github/workflows/polkadot-docs-basic-hardhat.yml`

```yaml
name: {Readable Guide Name}

on:
  push:
    branches: [master]
    paths:
      - 'polkadot-docs/{category}/{guide-name}/**'
      - '!polkadot-docs/{category}/{guide-name}/README.md'
  pull_request:
    paths:
      - 'polkadot-docs/{category}/{guide-name}/**'
      - '!polkadot-docs/{category}/{guide-name}/README.md'
  workflow_dispatch:

jobs:
  test:
    if: github.repository == 'polkadot-developers/polkadot-cookbook'
    runs-on: ubuntu-latest

    steps:
      - uses: actions/checkout@v4

      - uses: actions/setup-node@v4
        with:
          node-version: '22'

      - name: Install npm dependencies
        run: |
          cd polkadot-docs/{category}/{guide-name}
          npm ci

      - name: Run tests
        run: |
          cd polkadot-docs/{category}/{guide-name}
          npm test
        timeout-minutes: 10

  post-test:
    needs: test
    if: github.repository == 'polkadot-developers/polkadot-cookbook' && always() && github.ref == 'refs/heads/master'
    uses: ./.github/workflows/post-cleanup.yml
    with:
      test_result: ${{ needs.test.result }}
      readme_path: 'polkadot-docs/{category}/{guide-name}/README.md'
      workflow_name: '{Readable Guide Name}'
    permissions:
      contents: read
      issues: write
```

**Add to Variant A as needed:**
- **Secrets:** Add `env:` block to the "Run tests" step (see `.github/workflows/polkadot-docs-basic-hardhat.yml` for PRIVATE_KEY pattern)
- **npm cache for cloned repos:** Add `actions/cache@v4` step keyed on test file hash (see `.github/workflows/polkadot-docs-basic-hardhat.yml`)

### Variant B: Guard Job (depends on versions.yml)

Use when: guide references pinned versions from `versions.yml` (SDK version, tool versions, etc.).

Reference: `.github/workflows/polkadot-docs-query-accounts.yml`

Key differences from Variant A:
1. Add `versions.yml` to both push and pull_request path triggers
2. Add `guard` job using `check-version-keys` action
3. `test` job gets `needs: guard` and `if: needs.guard.outputs.should-run == 'true'`
4. Add "Load versions" step that reads keys from `versions.yml` via `yq`
5. `post-test` job needs both `[guard, test]` and checks `needs.guard.outputs.should-run == 'true'`

```yaml
name: {Readable Guide Name}

on:
  push:
    branches: [master]
    paths:
      - 'polkadot-docs/{category}/{guide-name}/**'
      - '!polkadot-docs/{category}/{guide-name}/README.md'
      - 'versions.yml'
  pull_request:
    paths:
      - 'polkadot-docs/{category}/{guide-name}/**'
      - '!polkadot-docs/{category}/{guide-name}/README.md'
      - 'versions.yml'
  workflow_dispatch:

jobs:
  guard:
    if: github.repository == 'polkadot-developers/polkadot-cookbook'
    runs-on: ubuntu-latest
    outputs:
      should-run: ${{ steps.check.outputs.should-run }}
    steps:
      - uses: actions/checkout@v4
      - id: check
        uses: ./.github/actions/check-version-keys
        with:
          event-name: ${{ github.event_name }}
          base-sha: ${{ github.event_name == 'pull_request' && github.event.pull_request.base.sha || github.event.before }}
          head-sha: ${{ github.sha }}

  test:
    needs: guard
    if: needs.guard.outputs.should-run == 'true'
    runs-on: ubuntu-latest

    steps:
      - uses: actions/checkout@v4

      - name: Load versions
        id: versions
        run: |
          echo "KEY_NAME=$(yq '.path.to.key' versions.yml)" >> $GITHUB_OUTPUT

      - uses: actions/setup-node@v4
        with:
          node-version: '22'

      # ... additional setup steps based on guide classification ...

      - name: Install npm dependencies
        run: |
          cd polkadot-docs/{category}/{guide-name}
          npm ci

      - name: Run tests
        run: |
          cd polkadot-docs/{category}/{guide-name}
          npm test
        timeout-minutes: 10

  post-test:
    needs: [guard, test]
    if: >
      github.repository == 'polkadot-developers/polkadot-cookbook' &&
      needs.guard.outputs.should-run == 'true' &&
      always() &&
      github.ref == 'refs/heads/master'
    uses: ./.github/workflows/post-cleanup.yml
    with:
      test_result: ${{ needs.test.result }}
      readme_path: 'polkadot-docs/{category}/{guide-name}/README.md'
      workflow_name: '{Readable Guide Name}'
    permissions:
      contents: read
      issues: write
```

**The guard auto-detects keys:** The `check-version-keys` action parses `yq '...' versions.yml` calls from the workflow file to determine which keys to check. No separate key list to maintain — just add `yq` calls in the "Load versions" step.

### Additional CI Setup Blocks (add to either variant as needed)

**Rust toolchain** (for guides that build Rust code):
Reference: `.github/workflows/polkadot-docs-local-dev-node.yml`
```yaml
      - name: Set up Rust stable toolchain
        run: |
          rustup show
          rustup target add wasm32-unknown-unknown
          rustup component add rust-src

      - name: Install system build dependencies
        run: |
          sudo apt-get update
          sudo apt-get install -y protobuf-compiler clang libclang-dev make
```

**Python** (for guides using Python):
Reference: `.github/workflows/polkadot-docs-query-accounts.yml`
```yaml
      - uses: actions/setup-python@v5
        with:
          python-version: '3.x'
```

**Cargo caching** (for Rust builds):
```yaml
      - name: Cache Cargo registry
        uses: actions/cache@v4
        with:
          path: |
            ~/.cargo/registry
            ~/.cargo/git
          key: ${{ runner.os }}-cargo-registry-${{ steps.versions.outputs.SOME_VERSION }}
          restore-keys: |
            ${{ runner.os }}-cargo-registry-
```

**Smart contract setup — standard contracts** (no custom precompiles):
Use `setup-revive-dev-node` composite action. Reference: `.github/workflows/recipe-contracts-example.yml`
```yaml
      - name: Set up revive dev node
        uses: ./.github/actions/setup-revive-dev-node
```

**Smart contract setup — custom precompiles**:
Use `setup-zombienet-eth-rpc` composite action. Reference: `.github/workflows/recipe-contracts-precompile-example.yml`
```yaml
      - name: Set up zombienet with eth-rpc
        uses: ./.github/actions/setup-zombienet-eth-rpc
```

---

## Step 4: Install Dependencies and Generate Lock File

```bash
cd polkadot-docs/{category}/{guide-name}
npm install   # generates package-lock.json (commit this)
```

---

## Step 5: Test + Debug Loop

Run tests locally and iterate on failures:

```bash
cd polkadot-docs/{category}/{guide-name}
npm test
```

### Failure Classification

When tests fail, classify the failure and respond accordingly:

**Category A — Code errors in generated test:**
- Syntax errors, wrong imports, incorrect assertions, missing await
- **Action:** Fix the test code directly, re-run
- Iterate up to 3-5 times until green

**Category B — Infrastructure/network failures:**
- RPC endpoint down, rate limiting, network timeouts, flaky connections
- **Action:** Add resilience wrappers:
  - `it.skipIf` for optional external dependencies
  - try/catch with meaningful warnings for intermittent failures
  - Retry logic for known-flaky network calls
- Re-run to confirm the wrapper works

**Category C — Missing prerequisites:**
- Funded testnet account required but no key available
- Binary not installed and can't be installed locally
- External service requires authentication not available
- **Action:** Report to user and stop. Do not attempt workarounds that would make the test meaningless.

### Debug Loop Protocol

1. Run `npm test`
2. Read error output carefully
3. Classify failure (A, B, or C)
4. If A: fix code, go to 1 (max 5 iterations)
5. If B: add resilience wrapper, go to 1
6. If C: report to user, stop
7. If green: proceed to Step 6

---

## Step 6: Create PRs

### 6a. Cookbook PR

Create a PR in this repository with all generated files:

```bash
gh pr create --title "feat: add {guide-name} polkadot-docs test harness" --body "$(cat <<'EOF'
## Summary
- Add test harness for the [{Guide Title}]({source_url}) documentation guide
- Verifies guide steps work as documented at commit {docs_commit}

## Files
- `polkadot-docs/{category}/{guide-name}/` — test harness
- `.github/workflows/polkadot-docs-{guide-name}.yml` — CI workflow

## Test plan
- [ ] `npm test` passes locally
- [ ] CI workflow triggers correctly on PR
- [ ] Badge renders in README
EOF
)"
```

### 6b. Companion PR (after cookbook PR is merged)

Open a companion PR in [`polkadot-developers/polkadot-docs`](https://github.com/polkadot-developers/polkadot-docs) to add CI badges to the documentation page. Add badges at both top and bottom of the guide:

```markdown
[![{Guide Title}](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-{guide-name}.yml/badge.svg?event=push)](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-{guide-name}.yml)
```

Reference the cookbook PR in the companion PR body: `Companion to polkadot-developers/polkadot-cookbook#{PR-number}`

---

## Reference Files

When generating files, study these existing examples for patterns and conventions:

| Pattern | Reference File |
|---------|---------------|
| Simple CI workflow | `.github/workflows/polkadot-docs-basic-hardhat.yml` |
| CI with guard job | `.github/workflows/polkadot-docs-query-accounts.yml` |
| CI with Rust + guard | `.github/workflows/polkadot-docs-local-dev-node.yml` |
| CI with secrets | `.github/workflows/polkadot-docs-erc20-hardhat.yml` |
| Guard action | `.github/actions/check-version-keys/action.yml` |
| Complex test (build + process mgmt) | `polkadot-docs/smart-contracts/local-dev-node/tests/recipe.test.ts` |
| Shared version loader | `polkadot-docs/shared/load-variables.ts` |
