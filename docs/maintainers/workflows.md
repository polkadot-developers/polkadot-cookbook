---
layout: doc
title: "GitHub Actions Workflows"
---

# GitHub Actions Workflows

This document describes all GitHub Actions workflows in the Polkadot Cookbook repository.

## Overview

The repository uses GitHub Actions for:
- **Automated Testing** - Recipe and SDK tests on every PR
- **Semantic Versioning** - Automatic version detection from commits
- **Release Automation** - Weekly and breaking-change-triggered releases
- **Quality Control** - Code coverage, formatting, linting

## Workflow Categories

### 1. Testing Workflows

#### `test-sdk.yml` - Polkadot Cookbook SDK

Tests the core SDK library and CLI tool.

**Triggers:**
- Push to master (paths: `dot/sdk/**`, `dot/cli/**`, `Cargo.toml`, `Cargo.lock`)
- Pull requests (always runs for branch protection, but skips work if no SDK changes)

**Jobs:**
1. **check-changes** - Detects if SDK files changed
2. **test-sdk** - Tests SDK library
   - Formatting check (`cargo fmt`)
   - Clippy lints (`cargo clippy`)
   - Unit tests
   - Integration tests
   - Doc tests
   - Coverage report (80% threshold for SDK library)
   - Posts coverage comment on PRs
3. **test-cli** - Tests CLI tool
   - Valid recipe creation test
   - Error handling tests
   - Help command test
4. **build-workspace** - Verifies full workspace builds

**Coverage Threshold:** 80% for SDK library (CLI coverage tracked but not enforced)

**Files:** `.github/workflows/test-sdk.yml`

---

#### `recipe-*.yml` - Per-Recipe Test Workflows

Each recipe has its own dedicated workflow file (e.g., `recipe-parachain-example.yml`, `recipe-contracts-example.yml`). Workflows are path-filtered so only the affected recipe's tests run on a given PR.

**Current recipe workflows:**
- `recipe-contracts-example.yml`
- `recipe-contracts-precompile-example.yml`
- `recipe-network-example.yml`
- `recipe-pallet-example.yml`
- `recipe-parachain-example.yml`
- `recipe-paseo-local-network-example.yml`
- `recipe-transaction-example.yml`
- `recipe-xcm-example.yml`

**Triggers:**
- Push to master (paths: `recipes/{pathway}/{recipe-name}/**`)
- Pull requests (paths: `recipes/{pathway}/{recipe-name}/**`)
- Changes to `versions.yml` (filtered by a `guard` job that checks relevant keys)

**Jobs:**
1. **guard** - Skips the test job if a `versions.yml` change doesn't affect the workflow's keys
2. **test** - Installs dependencies (`npm ci`) and runs tests (`npm test`)

**Files:** `.github/workflows/recipe-*.yml`

---

### 2. Release Workflows

#### `release-weekly.yml` - Weekly Recipe Release

Automated weekly release of tested recipes.

**Triggers:**
- Schedule: Every Wednesday at 9:00 AM Bangkok Time (02:00 UTC)
- Manual dispatch (optional version bump and skip tests)

**Jobs:**
1. **check-changes** - Analyzes commits since last release
   - Gets last recipe release tag (`v*.*.*`)
   - Analyzes commits for conventional commit types
   - Determines version bump type (major/minor/patch)
   - Calculates new version (respects alpha versioning: `v0.x.x`)
   - Skips if no changes
2. **test-recipes** - Tests all recipes (unless `skip_tests` is true)
   - Rust recipes: `cargo test`
   - TypeScript recipes: `npm test` (with Chopsticks if needed)
3. **create-release** - Generates release artifacts
   - Generates `manifest.yml` with recipe inventory
   - Generates `RELEASE_NOTES.md`
   - Creates release branch
   - Creates PR with `semantic:patch` and `release` labels

**Version Bump Logic:**
- Breaking changes → MAJOR (MINOR in alpha)
- Features (`feat`) → MINOR
- Fixes (`fix`) → PATCH
- Docs/chore → No bump (skips release)

**Alpha Versioning:** In `v0.x.x`, major bumps become minor bumps

**Files:** `.github/workflows/release-weekly.yml`

**Artifacts:**
- `.github/releases/v<version>/manifest.yml`
- `.github/releases/v<version>/RELEASE_NOTES.md`

---

#### `release-on-breaking-change.yml` - Breaking Change Release

Triggers immediate release when breaking changes are merged to master.

**Triggers:**
- Push to master (paths: `dot/cli/**`, `dot/sdk/**`)

**Jobs:**
1. **detect-breaking-change** - Checks if merged PR has `semantic:major` label
   - Extracts PR number from merge commit
   - Checks PR labels
   - Determines if CLI or SDK changed
   - Skips if not a breaking change
2. **get-cli-version** - Calculates new CLI version (if CLI changed)
3. **release-cli** - Calls reusable `release-cli.yml` workflow
4. **release-sdk** - Creates SDK release (if SDK changed)
   - Builds and tests SDK
   - Creates `sdk-v*.*.*` tag and release
5. **test-and-release-recipes** - Tests all recipes with new tooling
   - Tests all recipes
   - Calculates new recipe version
   - Generates manifest and release notes
   - Creates release PR

**Version Bumps:**
- CLI: Bumps MAJOR version
- SDK: Bumps MAJOR version
- Recipes: Bumps MINOR in alpha (triggered by breaking change)

**Files:** `.github/workflows/release-on-breaking-change.yml`

---

#### `release-cli.yml` - CLI Binary Release

Reusable workflow for building and releasing CLI binaries across platforms.

**Triggers:**
- Called by other workflows (`workflow_call`)
- Manual dispatch with version input

**Inputs:**
- `version` (required): CLI version to release (e.g., `0.2.0`)
- `is_breaking` (optional): Whether this is a breaking change

**Jobs:**
1. **build-binaries** - Builds CLI for multiple platforms (matrix)
   - Linux (x86_64, arm64 - using cross-compilation)
   - macOS (Intel, Apple Silicon)
   - Windows (x86_64)
   - Strips binaries (except Windows and macOS ARM)
   - Creates archives (.tar.gz or .zip)
   - Uploads artifacts
2. **create-release** - Publishes GitHub Release
   - Creates `cli-v<version>` tag
   - Generates release notes with installation instructions
   - Attaches all platform binaries

**Files:** `.github/workflows/release-cli.yml`

**Release Artifacts:**
- `dot-linux-amd64.tar.gz`
- `dot-linux-arm64.tar.gz`
- `dot-macos-intel.tar.gz`
- `dot-macos-apple-silicon.tar.gz`
- `dot-windows-amd64.exe.zip`

---

#### `publish-release.yml` - Publish Release

Publishes final GitHub Release when manifest is merged to master.

**Triggers:**
- Push to master (paths: `.github/releases/v*/manifest.yml`, `.github/releases/v*/RELEASE_NOTES.md`)

**Jobs:**
1. **detect-version** - Extracts version from changed files
   - Checks for changes to manifest files
   - Skips if tag already exists
2. **build-cli-binaries** - Builds CLI binaries for all platforms
   - Same as `release-cli.yml` but inline
3. **publish-release** - Creates Git tag and GitHub Release
   - Creates `v<version>` tag
   - Attaches manifest file
   - Attaches all CLI binaries
   - Uses `RELEASE_NOTES.md` for release description

**Files:** `.github/workflows/publish-release.yml`

**Release Tags:** `v*.*.*` (recipe releases)

---

### 3. Automation Workflows

#### `auto-label-semantic.yml` - Auto-Label Semantic Version

Automatically labels PRs based on commit analysis and API breaking changes.

**Triggers:**
- Pull request opened, synchronized, or reopened

**Jobs:**
1. **analyze-commits** - Analyzes commits and applies semantic label
   - Runs `cargo-semver-checks` on SDK to detect API breaking changes
   - Analyzes all commits in PR for conventional commit format
   - Determines highest semantic level:
     - API breaking change (detected by cargo-semver-checks) → `semantic:major`
     - Breaking change (`!` or `BREAKING CHANGE:`) → `semantic:major`
     - Feature (`feat:`) → `semantic:minor`
     - Fix (`fix:`, `perf:`) → `semantic:patch`
     - Docs/chore only → `semantic:none`
   - Applies semantic label
   - Posts comment with analysis table
   - Respects manual labels (doesn't override if label already exists)

**Semantic Labels:**
- `semantic:major` - Breaking changes
- `semantic:minor` - New features
- `semantic:patch` - Bug fixes
- `semantic:none` - No version bump

**API Breaking Change Detection:**
- Uses `cargo-semver-checks` to detect breaking changes in SDK public API
- Only checks `dot/sdk/` crate (SDK library)
- Skips `dot/cli/` crate (binary-only, no public API)

**Files:** `.github/workflows/auto-label-semantic.yml`

**Comment Format:**
```
🤖 Semantic Version Analysis

Result: MINOR version bump

Commit Analysis
| Commit | Type | Impact | Message |
|--------|------|--------|---------|
| abc123 | feat | 🟡 MINOR | feat(recipe): add feature |
| def456 | fix  | 🟢 PATCH | fix(cli): bug fix |

This PR will trigger a MINOR version bump when merged.
```

---

## Workflow Interactions

### Release Flow

```
1. Developer creates PR with conventional commits
   ↓
2. auto-label-semantic.yml analyzes commits → applies semantic label
   ↓
3. PR merged to master
   ↓
4. Two possible paths:

   Path A: Breaking Change (semantic:major + CLI/SDK changes)
   ├─ release-on-breaking-change.yml detects breaking change
   ├─ Releases CLI or SDK immediately
   ├─ Tests all recipes with new tooling
   └─ Creates recipe release PR

   Path B: Regular Changes
   ├─ Changes accumulate until Wednesday
   ├─ release-weekly.yml runs on schedule
   ├─ Analyzes all commits since last release
   ├─ Tests all recipes
   └─ Creates release PR

5. Release PR merged to master
   ↓
6. publish-release.yml detects manifest files
   ↓
7. Creates Git tag and GitHub Release with binaries
```

### Testing Flow

```
1. Developer creates PR with recipe changes
   ↓
2. Per-recipe workflows run (only affected recipes):
   ├─ recipe-parachain-example.yml
   ├─ recipe-contracts-example.yml
   └─ ... (one workflow per recipe)
   ↓
3. SDK changes also trigger:
   └─ test-sdk.yml
      ├─ Tests SDK library (coverage threshold: 80%)
      └─ Tests CLI tool
```

## Manual Workflow Triggers

### Test a Specific Recipe

```bash
# Via GitHub UI: Actions → Select workflow → Run workflow
# Or via gh CLI:
gh workflow run recipe-pallet-example.yml
gh workflow run recipe-xcm-example.yml
```

### Trigger Weekly Release Manually

```bash
gh workflow run release-weekly.yml -f version_bump=minor -f skip_tests=false
```

### Release CLI Version

```bash
gh workflow run release-cli.yml -f version=0.3.0 -f is_breaking=true
```

## Troubleshooting

### Tests Failing in CI but Pass Locally

**Check:**
- Ensure all dependencies are committed
- Verify Rust toolchain compatibility (check recipe `rust-toolchain.toml`)
- Check for environment-specific issues (CI runs Ubuntu)

### Release Didn't Trigger

**Check:**
- Were there changes since last release?
- Do commits follow conventional commit format?
- Check workflow logs for errors

### Chopsticks Fails to Start

**Symptom:** XCM tests timeout waiting for Chopsticks

**Check:**
- Verify `chopsticks.yml` configuration
- Check Chopsticks logs in workflow artifacts
- Ensure recipe specifies `"chopsticks"` in `package.json`

### Coverage Below Threshold

**Symptom:** `test-sdk.yml` fails with "coverage below 80%"

**Solution:**
- Add tests to increase SDK library coverage
- Coverage requirement only applies to `dot/sdk/` package
- CLI coverage is tracked but not enforced

## Best Practices

### For Contributors

✅ **DO:**
- Use conventional commits for all commits
- Wait for CI to pass before requesting review
- Check semantic label on your PR (correct if wrong)
- Test locally before pushing

❌ **DON'T:**
- Skip CI checks with `[skip ci]`
- Force push to branches with open PRs
- Ignore failed CI checks

### For Maintainers

✅ **DO:**
- Verify semantic labels before merging
- Monitor release workflows for failures
- Review breaking change releases carefully

❌ **DON'T:**
- Merge PRs with failing CI
- Override semantic labels without reason
- Skip release process for "quick fixes"

## Related Documentation

- [Release Process](release-process.md) - Semantic versioning and release automation
- [Pre-commit Hooks](../automation/pre-commit-hooks.md) - Local quality checks
- [Contributing Guide](https://github.com/polkadot-developers/polkadot-cookbook/blob/master/CONTRIBUTING.md) - Development workflow
