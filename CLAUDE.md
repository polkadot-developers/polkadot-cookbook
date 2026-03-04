# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**Polkadot Cookbook** is a Rust monorepo containing:
1. **`dot` CLI** — interactive scaffolding tool for Polkadot projects
2. **`dot` SDK** — library powering the CLI, usable programmatically
3. **`recipes/`** — Node.js/Vitest test harnesses that validate external recipe repos
4. **`polkadot-docs/`** — tutorial markdown content for docs.polkadot.com

Rust toolchain: `1.91` (pinned in `rust-toolchain.toml`)

## Commands

### Rust (SDK & CLI)

```bash
# Build
cargo build --release --bin dot

# Format check
cargo fmt --check --package sdk

# Lint
cargo clippy --package sdk --locked -- -D warnings

# SDK unit tests
cargo test --package sdk --lib

# SDK integration tests
cargo test --package sdk --test integration_test

# SDK doc tests
cargo test --package sdk --doc

# Template/pathway integration tests (slow, ~90 min cold)
cargo test --package cli --test pathway_integration_tests -- --ignored
```

### Recipe Test Harnesses (Node.js)

Each recipe under `recipes/{pathway}/{name}/` is independently runnable:

```bash
cd recipes/contracts/contracts-example
npm ci && npm test
```

These harnesses clone an external GitHub repo, build it, and run its tests.

## Architecture

### `dot/sdk/` — Library

| Module | Purpose |
|--------|---------|
| `config/` | `ProjectConfig`, `ProjectType` (enum of 5 pathways), validation |
| `scaffold/` | `Bootstrap` + `Scaffold` — project generation logic |
| `templates/` | Embedded project templates (README, test harness) per pathway |
| `git/` | `git2`-based operations (init, commit, push) |
| `metadata/` | Project detection and YAML front matter parsing |
| `dependencies/` | Checks for required tools per pathway (e.g. `cargo`, `node`) |
| `error.rs` | `CookbookError` enum — all operations return `Result<T, CookbookError>` |
| `constants.rs` | Shared constants (brand color `#E6007A`, repo URLs, etc.) |

Features `fs`, `test_runner`, `query` are gated and not yet in the public API.

### `dot/cli/` — Binary

Thin wrapper around SDK using `clap` for arg parsing and `cliclack` for interactive prompts. Subcommands: `create`, `contract`, `test`.

### `recipes/`

Each recipe is a standalone Node.js project:
- `vitest.config.ts` — test runner config
- `tests/recipe.test.ts` — clones external repo at pinned tag, installs, builds, tests
- `package.json` — `npm test` entry point

### 5 Development Pathways

`Parachains` | `Pallets` | `Contracts` | `Transactions` | `Networks` (XCM also covered)

### Testing Strategy

- **SDK unit tests**: in `#[cfg(test)]` blocks within each module
- **SDK integration tests**: `dot/sdk/tests/integration_test.rs`
- **Pathway integration tests**: `dot/cli/tests/pathway_integration_tests.rs` — runs `dot create` end-to-end (marked `#[ignore]`, run explicitly)
- **Recipe tests**: per-recipe `npm test` (CI runs these in parallel)
- **Coverage threshold**: 80% for SDK (`cargo-llvm-cov`)

Use `#[serial]` from `serial_test` when tests share filesystem state.
