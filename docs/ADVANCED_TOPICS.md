# Advanced Topics

This guide covers advanced configuration and workflows for Polkadot Cookbook tutorials.

## Table of Contents

- [Tutorial Configuration](#tutorial-configuration)
- [Justfiles and Scripts](#justfiles-and-scripts)
- [CI/CD Pipeline](#cicd-pipeline)

## Tutorial Configuration

The `tutorial.config.yml` file controls how your tutorial is built and tested.

### Basic Configuration

```yaml
name: My Tutorial Title
slug: my-tutorial
category: polkadot-sdk-cookbook
needs_node: true
description: Learn how to build a custom pallet
type: sdk
```

### Advanced Configuration with Manifest

For tutorials with runtime/parachain code:

```yaml
name: My Tutorial Title
slug: my-tutorial
category: polkadot-sdk-cookbook
needs_node: true
description: Build a custom parachain
type: sdk

manifest:
  build:
    project_dir: src
    commands:
      - cargo build --release

  runtime:
    wasm_path: ./target/release/wbuild/my-runtime/my_runtime.compact.compressed.wasm

  network:
    relay_chain: paseo
    para_id: 1000

  tests:
    framework: vitest
    files:
      - tests/my-tutorial-e2e.test.ts

scripts_dir: scripts
zombienet_dir: zombienet
```

### When to Update

Update `tutorial.config.yml` when:
- Changing build process or commands
- Modifying runtime WASM output paths
- Updating network configuration
- Changing test file locations
- Toggling `needs_node` flag

## Justfiles and Scripts

### Tutorial Justfiles

Each tutorial can have a `justfile` for common development tasks:

```justfile
# List available commands
default:
  @just --list

# Setup Rust toolchain
setup-rust:
  rustup install 1.86.0
  rustup default 1.86.0

# Build the runtime
build:
  cd src && cargo build --release

# Run tests
test:
  npm test

# Start local node
start-node:
  ./scripts/start-node.sh
```

**Usage**:
```bash
cd tutorials/my-tutorial
just          # List commands
just build    # Run build
just test     # Run tests
```

### Global vs Tutorial Scripts

**Global scripts** (`/scripts/`):
- Shared across multiple tutorials
- Common setup procedures
- Reusable utilities

**Tutorial scripts** (`/tutorials/<slug>/scripts/`):
- Tutorial-specific workflows
- Auto-generated versioned scripts (post-merge)
- Custom setup unique to this tutorial

**When to use which**:
- Use global scripts for common patterns
- Use tutorial scripts for unique workflows
- Propose migration to global if script is reused 3+ times

## CI/CD Pipeline

### Automated Checks on PR

When you submit a PR, the following checks run automatically:

1. **Tutorial Tests** (`.github/workflows/test-tutorials.yml`)
   - Runs if new tutorial folder is added
   - Installs dependencies
   - Runs `npm test` for affected tutorials
   - Tests must pass or skip gracefully

2. **Build Verification**
   - Validates `tutorial.config.yml` syntax
   - Checks for required files
   - Verifies test files exist

### Post-Merge Workflow

After your PR is merged, maintainers will:

1. **Generate versioned scripts** via `/generate-scripts` command
   - Creates pinned setup scripts in `tutorials/<slug>/scripts/`
   - Commits scripts to repository

2. **Create tutorial tag** in format `tutorial/<slug>/vYYYYMMDD-HHMMSS`
   - Enables stable snippet extraction for documentation

3. **Optionally create GitHub release** for major tutorials

### Version Management

Tutorial versions are managed in `versions.yml`:

```yaml
versions:
  rust: "1.86.0"
  chain_spec_builder: "0.20.0"
  polkadot_omni_node: "0.4.1"

# Tutorial-specific overrides (optional)
my_tutorial:
  rust: "1.85.0"
```

Maintainers handle version updates. If your tutorial requires specific versions, note this in your proposal.
