---
name: add-recipe
description: Create a new recipe test harness that verifies an external recipe repository. Use when adding a new recipe to the cookbook.
argument-hint: "[recipe-name]"
---

# Add Recipe Test Harness

Create a test harness under `recipes/{pathway}/{recipe-name}/` that clones an external repo, builds it, and runs its tests.

Recipe source code lives in **external repos** (e.g., `brunopgalvao/recipe-{name}`). This repo only contains the test harness.

## Step 1: Scaffold the directory

Create these files in `recipes/{pathway}/{recipe-name}/`:

**package.json:**
```json
{
  "name": "{recipe-name}-test",
  "version": "1.0.0",
  "description": "Verification tests for the {recipe-name} recipe",
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
    "declaration": true,
    "outDir": "dist",
    "rootDir": "."
  },
  "include": ["tests/**/*"],
  "exclude": ["node_modules", "dist"]
}
```

**vitest.config.ts:**
```typescript
import { defineConfig } from "vitest/config";

export default defineConfig({
  test: {
    fileParallelism: false,
    sequence: { shuffle: false },
    testTimeout: 300000,     // 5 min for Node.js recipes, 2700000 (45 min) for Rust
    hookTimeout: 60000,
    reporters: ["verbose"],
    pool: "forks",
    poolOptions: { forks: { singleFork: true } },
    include: ["tests/recipe.test.ts"],
  },
});
```

**.gitignore:**
```
node_modules/
recipe-{name}/
dist/
*.log
```

**README.md:**
```markdown
# {Recipe Title}

[![{Recipe Title}](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/recipe-{recipe-name}.yml/badge.svg?event=push)](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/recipe-{recipe-name}.yml)

[{Recipe Title}](https://github.com/brunopgalvao/recipe-{recipe-name})

## Running Tests

\```bash
npm ci
npm test
\```
```

## Step 2: Write the test

Create `tests/recipe.test.ts` with this lifecycle:

```typescript
import { describe, it, expect } from "vitest";
import { execSync } from "child_process";
import { existsSync } from "fs";
import { join } from "path";

const REPO_URL = "https://github.com/brunopgalvao/recipe-{recipe-name}";
const REPO_VERSION = "v1.0.0";  // Must be a git tag
const CLONE_DIR = join(process.cwd(), "recipe-{recipe-name}");

describe("{Recipe Title}", () => {
  describe("1. Prerequisites", () => {
    it("should have required tools", () => {
      // Check node/npm, git, cargo, etc.
      execSync("node --version", { encoding: "utf-8" });
      execSync("git --version", { encoding: "utf-8" });
    });
  });

  describe("2. Clone Repository", () => {
    it("should clone at pinned version", () => {
      if (!existsSync(CLONE_DIR)) {
        execSync(`git clone --branch ${REPO_VERSION} --depth 1 ${REPO_URL} ${CLONE_DIR}`, {
          encoding: "utf-8",
          stdio: "inherit",
        });
      }
      expect(existsSync(join(CLONE_DIR, "package.json"))).toBe(true);
    }, 120000);
  });

  describe("3. Install Dependencies", () => {
    it("should install", () => {
      execSync("npm ci", { cwd: CLONE_DIR, encoding: "utf-8", stdio: "inherit" });
    });
  });

  describe("4. Build", () => {
    it("should compile", () => {
      // e.g., execSync("npx hardhat compile", { cwd: CLONE_DIR, ... });
    });
  });

  describe("5. Test", () => {
    it("should pass tests", () => {
      // e.g., execSync("npx hardhat test", { cwd: CLONE_DIR, ... });
    });
  });
});
```

**Pinning strategy:** Recipe tests pin external repos by **git tag** (e.g., `v1.0.0`).

**Timeouts:** Node.js recipes ~5 min, Rust recipes ~45 min.

## Step 3: Create the CI workflow

Create `.github/workflows/recipe-{recipe-name}.yml`:

```yaml
name: {Readable Recipe Name}

on:
  push:
    branches: [master]
    paths:
      - 'recipes/{pathway}/{recipe-name}/**'
  pull_request:
    paths:
      - 'recipes/{pathway}/{recipe-name}/**'
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
          cd recipes/{pathway}/{recipe-name}
          npm ci

      - name: Run tests
        run: |
          cd recipes/{pathway}/{recipe-name}
          npm test
        timeout-minutes: 15
```

Add `versions.yml` to path triggers if the recipe depends on pinned versions.

**CI setup for smart contract recipes:**
- **Standard contracts** (no custom precompiles): use `setup-revive-dev-node` action — runs a vanilla pallet-revive dev node + eth-rpc adapter
- **Contracts with custom precompiles**: use `setup-zombienet-eth-rpc` action — requires a full network with a custom runtime that includes the precompile

For Rust recipes, add Rust toolchain setup and cargo caching.

## Step 4: Run locally and generate lock file

```bash
cd recipes/{pathway}/{recipe-name}
npm install   # generates package-lock.json (commit this)
npm test
```

## Pathways

Place the recipe in the correct pathway directory:
- `recipes/pallets/` — FRAME pallet development (Rust)
- `recipes/contracts/` — Solidity smart contracts (Hardhat)
- `recipes/transactions/` — Chain interactions (PAPI, TypeScript)
- `recipes/cross-chain-transactions/` — XCM messaging (Chopsticks)
- `recipes/networks/` — Network testing (Zombienet, Chopsticks)
