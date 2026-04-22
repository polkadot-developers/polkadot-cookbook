# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build & Test Commands

### Rust (dot SDK & CLI)

```bash
cargo fmt --check --package sdk
cargo clippy --package sdk --locked -- -D warnings
cargo build --workspace --locked --verbose

# SDK tests (use --test-threads=1, tests share filesystem state)
cargo test --package sdk --lib --locked --verbose -- --test-threads=1
cargo test --package sdk --test integration_test --locked --verbose -- --test-threads=1
cargo test --package sdk --doc --locked --verbose

# CLI validation
cargo run --package cli --locked -- create --title "Test" --skip-install --no-git --non-interactive

# Pathway integration tests (slow, ~90 min cold, must use --release --ignored)
cargo test --package cli --test pathway_integration_tests --release -- --ignored --nocapture

# Coverage (80% threshold enforced for SDK)
cargo llvm-cov --package sdk --locked --summary-only -- --test-threads=1
```

### Node.js Test Harnesses (recipes, polkadot-docs, migration)

Each harness is a standalone npm project:

```bash
cd recipes/{pathway}/{recipe-name}   # or polkadot-docs/... or migration/...
npm ci && npm test
```

## Architecture

**Rust workspace** (`dot/`): The `dot` CLI wraps the SDK library. CLI uses `clap` + `cliclack` for interactive prompts; SDK is pure library with no UI. Default cargo member is `cli` — bare `cargo build` only builds the CLI; use `--package sdk` to target the SDK.

**Test harnesses** (`recipes/`, `polkadot-docs/`, `migration/`): Standalone Node.js/Vitest projects that clone external repos at pinned versions, install, build, and run their tests. They verify that external code and documentation guides actually work. Recipe tests pin by **git tag**; polkadot-docs tests pin by **commit SHA**.

**`versions.yml`**: Source of truth for pinned dependency versions (polkadot-sdk release tag, parachain template version, zombienet version, Rust crates under `crates.*`, and JS packages under `javascript_packages.*`). Referenced by CI workflows and `polkadot-docs/shared/load-variables.ts` (shared utility that parses versions at test runtime). For JS entries, `.github/workflows/check-js-versions.yml` treats each pinned version as a **minimum floor** — harnesses under `polkadot-docs/`, `recipes/`, and `migration/` may be ahead of the floor (and often are, to track upstream tutorials). The workflow never fails a check: on drift it opens or updates a single issue labeled `versions-drift`, and closes it automatically when every tracked dep is back at or above the floor. Raise below-floor specs with `node .github/scripts/check-js-versions.mjs --fix`. Changes to `versions.yml` trigger downstream CI runs.

**CI composite actions** (`.github/actions/`): Reusable actions like `setup-revive-dev-node` (builds/caches pallet-revive dev node + eth-rpc adapter), `setup-zombienet-eth-rpc`, and `check-version-keys` (guard that skips expensive test jobs when a `versions.yml` change doesn't affect the workflow's keys). Used by recipe, migration, and polkadot-docs workflows.

## Key Conventions

- Rust toolchain pinned to **1.91** (`rust-toolchain.toml`)
- Rust formatting: `max_width = 100`, `wrap_comments = true` (`rustfmt.toml`)
- SDK tests require `--test-threads=1` and use `#[serial]` from `serial_test` (shared filesystem state)
- Recipe source code lives in **external repos** (`brunopgalvao/recipe-*`); this repo only contains test harnesses
- Test file naming: recipes use `recipe.test.ts`, polkadot-docs use `docs.test.ts`, migrations use `migration.test.ts`
- Shared configs: `shared/tsconfig.base.json` and `shared/vitest.shared.ts` — all harnesses extend/import these
- CI workflows are **path-filtered** per component (e.g., `recipe-contracts-example.yml` only triggers on `recipes/contracts/contracts-example/**`)
- `versions.yml` changes also trigger downstream workflow runs — each workflow has a `guard` job that auto-detects which `versions.yml` keys it uses (by parsing `yq` calls in the workflow file) and skips the test job if none of those keys changed
- When adding a new `yq '...' versions.yml` line to a workflow's "Load versions" step, the guard picks it up automatically — no separate key list to maintain
- Commit both `Cargo.lock` and `package-lock.json` — locked dependencies are intentional
- No local git hooks — run `cargo fmt` and `cargo clippy` manually before pushing
- Workspace version: check `Cargo.toml` `[workspace.package]`

## Workflow

- Never work directly on the `master` branch — create feature branches
- Keep related changes on one branch; don't split iterative fixes across branches/PRs
- Conventional commits: `feat:`, `fix:`, `chore:`, etc.
- When adding a `polkadot-docs/` test harness, open a **companion PR** in [`polkadot-developers/polkadot-docs`](https://github.com/polkadot-developers/polkadot-docs) to add the CI badge to the corresponding documentation page
- PRs must include a **Test Plan** checklist — before marking a PR ready, execute every item in the checklist, verify it passes, then check it off (`- [x]`). Never leave test plan items unchecked
- Skill-generated PRs (from `/add-recipe`, `/add-polkadot-docs-test`, etc.) should be created as **drafts** (`--draft`)
- When changing `Cargo.toml` version, docs, workflow files, or badges, run `/check-internal-docs` before creating the PR to catch stale references
