# Contributing to the Polkadot Cookbook

This guide is for external contributors.

 **Visual Guide**: See the [Tutorial Creation Workflow](docs/TUTORIAL_WORKFLOW.md) diagram for a complete overview of the process.

## Quick Start

1. Propose your tutorial via [GitHub issue](https://github.com/polkadot-developers/polkadot-cookbook/issues/new?template=01-tutorial-proposal.md)
2. Wait for approval and a tutorial slug (e.g. `my-tutorial`)
3. Fork and clone the repo and `cd polkadot-cookbook`
4. **First time only:** Build the Rust CLI tool: `cd tools/create-tutorial && cargo build --release && cd ../..`
5. Run `npm run create-tutorial my-tutorial`
6. Write content, add code, write tests
7. Open a Pull Request

## 1) Propose your tutorial (required)

- Open an issue using the template: `Tutorial Proposal`.
- Include: learning objectives, audience, prerequisites, tools/versions.
- Wait for approval and a tutorial slug (e.g. `my-tutorial`).

## 2) Create your tutorial using the CLI tool

First, fork and clone this repository:

```bash
git clone https://github.com/YOUR_USERNAME/polkadot-cookbook.git
cd polkadot-cookbook
```

### Build the CLI tool (first time only)

The tutorial creator is a Rust CLI tool. Before first use, build it:

```bash
cd tools/create-tutorial
cargo build --release
cd ../..
```

**Prerequisites:**

- Rust `1.86+` ([install via rustup](https://rustup.rs))
- Cargo (comes with Rust)

### Create your tutorial

Run the unified tutorial creator from the repository root:

```bash
npm run create-tutorial my-tutorial
```

This single command will:

- ✅ Create a git branch (`feat/tutorial-my-tutorial`)
- ✅ Scaffold the complete folder structure
- ✅ Bootstrap the test environment
- ✅ Install all necessary dependencies
- ✅ Show you clear next steps

The created structure:

```text
tutorials/my-tutorial/
  tutorial.config.yml    # metadata and configuration
  justfile               # optional just commands
  README.md              # your written tutorial (required)
  src/                   # your project code (contracts or SDK)
  tests/                 # vitest e2e tests
  package.json           # npm dependencies
  vitest.config.ts       # test configuration
  tsconfig.json          # TypeScript configuration
```

## 3) Build the tutorial content

- Write the tutorial in `tutorials/my-tutorial/README.md` (required).
- Add code under `tutorials/my-tutorial/src/`.
- Add at least one e2e test under `tutorials/my-tutorial/tests/` using `@polkadot/api`.
  - Tests must skip fast when no local node is running.

## 4) Run tests locally

```bash
cd tutorials/my-tutorial
npm run test
```

## 5) Open a Pull Request

```bash
git add -A && git commit -m "feat(tutorial): add my-tutorial"
git push origin feat/tutorial-my-tutorial
```

Open the PR. The PR template will guide your checklist.

## 6) What CI runs on your PR (automatic)

- PR Tutorial Tests: `.github/workflows/ci-test.yml`
  - If your PR ADDS a new tutorial folder under `tutorials/<slug>/`, CI runs tests only for that new tutorial.
  - It installs deps and runs `vitest` for the selected tutorials.
  - Tests that require a node should "skip fast" if no endpoint is available.

## 7) After merge (maintainers do this)

- Generate finalized scripts (with concrete versions) for your tutorial:
  - Workflow: `.github/workflows/generate-scripts.yml` (manual trigger or on `versions.yml` changes)
  - Output: `tutorials/<slug>/scripts/` (committed to the repo for docs consumption)
- Tag and Release: the workflow will create a tutorial-specific tag `tutorial/<slug>/vYYYYMMDD-HHMMSS[-<shortsha>]`. If `create_release` is true on manual runs, it also creates a GitHub Release with resolved versions.

Docs publishing and snippet stability:

- This repo is the code source for `polkadot-developers/polkadot-docs` powering `docs.polkadot.com`.
- The per-tutorial tags let the docs fetch stable snippets without cross-tutorial conflicts. Keep your tutorial self-contained under `tutorials/<slug>/`.
- If your tutorial needs specific anchors/regions for snippet extraction, add clear comment anchors in code (ask maintainers for the current convention) and reference those in your README.

## Notes and tips

- Keep PRs focused: one tutorial per PR.
- If your SDK tutorial needs runtime changes, describe them in `README.md`. We can help you apply an overlay-based approach later to avoid clashes.
- If anything is unclear, open an issue using `Custom Blank Issue`.

## Understanding the Justfile

### What is a justfile?

A `justfile` is a command runner configuration file (similar to `Makefile`) that simplifies running common development tasks. Each tutorial has its own `justfile` located at `tutorials/<slug>/justfile`.

### How to use justfile commands

```bash
cd tutorials/my-tutorial

# List all available commands
just

# Run a specific command
just setup-rust
just run-zombienet
just run-tests
```

### Common justfile commands

The scaffolded justfile typically includes:
- `setup-rust` - Install and configure the required Rust toolchain
- `install-chain-spec-builder` - Install the chain spec builder tool
- `install-omni-node` - Install polkadot-omni-node
- `generate-chain-spec` - Generate the chain specification
- `start-node` - Start the local development node
- `run-zombienet` - Launch a zombienet test network
- `run-tests` - Execute the tutorial's test suite

### When to add new commands

Add new commands to your tutorial's `justfile` when you have:
- Multi-step setup procedures that you run frequently
- Complex commands with many flags or options
- Tasks that other tutorials might benefit from (consider moving to global `scripts/` instead)
- Build or test commands specific to your tutorial

### Reusable commands

If you identify a command pattern that multiple tutorials could benefit from:
1. First implement it in your tutorial's `justfile`
2. Test it thoroughly
3. Propose moving it to the global `scripts/` directory in your PR or a follow-up issue
4. Include documentation on why it's broadly useful

## Tutorial Configuration (tutorial.config.yml)

### Purpose

The `tutorial.config.yml` file defines metadata and runtime configuration for your tutorial. It's used by:
- CI/CD workflows to determine build and test requirements
- The documentation system to categorize and present tutorials
- Automated script generation workflows

### Configuration Options

```yaml
name: My Tutorial Title           # Display name
slug: my-tutorial                # URL-friendly identifier (must match folder name)
category: polkadot-sdk-cookbook  # Category: polkadot-sdk-cookbook or contracts-cookbook
needs_node: true                 # Whether tutorial requires a running node for tests
description: Short description   # Brief description of what the tutorial teaches
type: sdk                        # Type: sdk or contracts

# Optional: manifest for tutorials with runtime/parachain code
manifest:
  build:
    project_dir: src             # Directory containing Cargo.toml
    commands:
      - cargo build --release
  runtime:
    wasm_path: ./target/release/wbuild/...  # Path to compiled WASM runtime
  network:
    relay_chain: paseo           # Relay chain to use
    para_id: 1000               # Parachain ID
  tests:
    framework: vitest           # Test framework (vitest)
    files:
      - tests/my-tutorial-e2e.test.ts

scripts_dir: scripts            # Optional: custom scripts directory
zombienet_dir: zombienet        # Optional: zombienet config directory
```

### When to modify tutorial.config.yml

Update your `tutorial.config.yml` when:
- Changing the tutorial's build process
- Adding or modifying runtime WASM output paths
- Updating network configuration (relay chain, para ID)
- Changing test file locations
- Modifying whether a node is required for tests

### Validation

The workflow system validates that:
- `slug` matches the directory name
- Required fields (`name`, `slug`, `category`) are present
- Referenced paths (like `project_dir`, `wasm_path`) exist after build
- Test files specified in `manifest.tests.files` exist

## Scripts Organization

The repository uses a two-tier script system to balance reusability and tutorial-specific needs:

### Global scripts (`scripts/`)

**Location**: `/scripts/` (at repository root)

**Purpose**: Reusable scripts that multiple tutorials can use

**Examples**:
- Common setup procedures (Rust installation, tool setup)
- Shared build utilities
- Network management scripts used across tutorials

**When to add scripts here**:
- The script solves a problem common to multiple tutorials
- It encapsulates standard setup/configuration procedures
- It reduces duplication across tutorial-specific scripts

**How to use from tutorials**:
```bash
# From a tutorial's justfile
setup-common:
  ../../scripts/setup-rust.sh
```

### Tutorial-specific scripts (`tutorials/<slug>/scripts/`)

**Location**: `/tutorials/<slug>/scripts/`

**Purpose**: Scripts unique to a specific tutorial's workflow

**Generated by workflows**: After merging, the `generate-scripts.yml` workflow creates versioned scripts here:
- `setup-rust.sh` - Pinned Rust version for this tutorial
- `install-chain-spec-builder.sh` - Pinned chain-spec-builder version
- `install-omni-node.sh` - Pinned omni-node version
- `generate-chain-spec.sh` - Chain spec generation
- `start-node.sh` - Node startup

**When to add scripts here**:
- Tutorial-specific build or deployment procedures
- Custom test setup unique to this tutorial
- Integration scripts that combine multiple steps in a tutorial-specific way

### Script migration workflow

If you find yourself duplicating a tutorial script across multiple tutorials:

1. **Identify the pattern**: Note which tutorials need the same script
2. **Generalize the script**: Remove tutorial-specific hardcoded values, add parameters
3. **Propose the migration**: In your PR or a follow-up issue, suggest moving it to `/scripts/`
4. **Document parameters**: Ensure the global script is well-documented
5. **Update tutorials**: Update affected tutorials to use the global version

## Testing Guidelines

### Test Framework: Vitest + Polkadot API

All tutorials must use **Vitest** with **@polkadot/api** for end-to-end tests.

**Why this stack?**
- **Vitest**: Fast, modern test framework with excellent TypeScript support
- **@polkadot/api**: Official JavaScript API for interacting with Polkadot nodes
- **Consistency**: Uniform testing approach across all tutorials

### Test Structure

```typescript
// tests/my-tutorial-e2e.test.ts
import { describe, it, expect, beforeAll } from 'vitest';
import { ApiPromise, WsProvider } from '@polkadot/api';

describe('My Tutorial E2E Tests', () => {
  let api: ApiPromise;

  beforeAll(async () => {
    const provider = new WsProvider('ws://127.0.0.1:9944');
    api = await ApiPromise.create({ provider });
  }, 30000); // 30s timeout for node connection

  it('should connect to the node', async () => {
    expect(api.isConnected).toBe(true);
  });

  it('should verify runtime version', async () => {
    const version = await api.rpc.state.getRuntimeVersion();
    expect(version.specName.toString()).toBeDefined();
  });

  // Add your tutorial-specific tests here
});
```

### Fast Skip Pattern (REQUIRED)

**All tests must skip gracefully when no node is running**. This ensures CI can run tests for tutorials without building/starting nodes for every tutorial.

```typescript
import { describe, it, expect, beforeAll } from 'vitest';
import { ApiPromise, WsProvider } from '@polkadot/api';

describe('My Tutorial E2E Tests', () => {
  let api: ApiPromise | null = null;

  beforeAll(async () => {
    try {
      const provider = new WsProvider('ws://127.0.0.1:9944');
      // Short timeout for fast skip
      const promise = ApiPromise.create({ provider });
      api = await Promise.race([
        promise,
        new Promise((_, reject) =>
          setTimeout(() => reject(new Error('timeout')), 2000)
        )
      ]) as ApiPromise;
    } catch (e) {
      console.log('⏭️  Skipping tests - no node running');
      api = null;
    }
  }, 5000);

  it('should connect to node', () => {
    if (!api) return; // Fast skip
    expect(api.isConnected).toBe(true);
  });

  it('should perform tutorial operation', async () => {
    if (!api) return; // Fast skip

    // Your test logic here
    const result = await api.query.system.account('...');
    expect(result).toBeDefined();
  });
});
```

### Testing Best Practices

1. **Fast skip for missing nodes**: Always check for node availability and skip gracefully
2. **Meaningful assertions**: Test actual tutorial outcomes, not just API connectivity
3. **Clean up resources**: Disconnect APIs, close connections in `afterAll`
4. **Descriptive test names**: Use clear `describe` and `it` descriptions
5. **Timeouts**: Set appropriate timeouts for blockchain operations (usually 10-30s)
6. **Idempotent tests**: Tests should be runnable multiple times without side effects

### Running Tests

```bash
# Local development (with node running)
cd tutorials/my-tutorial
npm run test

# CI (may skip if node not started)
npm run test --silent
```

### Test Examples

See existing tutorials for reference:
- `tutorials/zero-to-hero/tests/zero-to-hero-e2e.test.ts`

Thank you for contributing! 🎉
