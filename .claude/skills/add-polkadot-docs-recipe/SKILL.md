---
name: add-polkadot-docs-recipe
description: Create a new polkadot-docs test harness that verifies a Polkadot documentation guide. Use when adding a new test for a docs.polkadot.com guide.
argument-hint: "[guide-url]"
---

# Add Polkadot Docs Test Harness

Create a test harness under `polkadot-docs/{category}/{guide-name}/` that verifies a docs.polkadot.com guide works as documented.

## Step 1: Scaffold the directory

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
docs_commit: "{latest commit SHA from upstream}"
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

## Step 2: Write the test

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

## Step 3: Create the CI workflow

Create `.github/workflows/polkadot-docs-{guide-name}.yml`:

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

**CI setup for smart contract guides:**
- **Standard contracts** (no custom precompiles): add `setup-revive-dev-node` action — runs a vanilla pallet-revive dev node + eth-rpc adapter
- **Contracts with custom precompiles**: add `setup-zombienet-eth-rpc` action — requires a full network with a custom runtime that includes the precompile

## Step 4: Run locally and generate lock file

```bash
cd polkadot-docs/{category}/{guide-name}
npm install   # generates package-lock.json (commit this)
npm test
```

## Step 5: Open companion PR

After the cookbook PR is merged, open a **companion PR** in [`polkadot-developers/polkadot-docs`](https://github.com/polkadot-developers/polkadot-docs) to add CI badges to the documentation page. Add badges at both top and bottom of the guide:

```markdown
[![{Guide Title}](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-{guide-name}.yml/badge.svg?event=push)](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/polkadot-docs-{guide-name}.yml)
```

Reference the cookbook PR in the companion PR body: `Companion to polkadot-developers/polkadot-cookbook#{PR-number}`
