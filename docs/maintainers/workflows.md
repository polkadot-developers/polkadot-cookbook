---
layout: doc
title: "GitHub Actions Workflows"
---

# GitHub Actions Workflows

This document describes all GitHub Actions workflows in the Polkadot Cookbook repository.

## Overview

The repository uses GitHub Actions for:
- **Automated Testing** - Recipe and SDK tests on every PR
- **Release Publishing** - Binary builds and GitHub Releases on merge
- **Quality Control** - Code coverage, formatting, linting

Release creation (version bump, release notes, manifest) is handled by the `/release` Claude Code skill, not a workflow. See [Release Process](release-process.md) for details.

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

#### `/release` Skill - Release Creation

Release creation is handled by the `/release` Claude Code skill (`.claude/skills/release/SKILL.md`), not a GitHub Actions workflow.

**Process:**
1. Analyzes commits since last git tag (`v*.*.*`)
2. Determines version bump from semantic understanding of changes
3. Generates `manifest.yml` and `RELEASE_NOTES.md` under `.github/releases/vX.Y.Z/`
4. Updates `Cargo.toml` workspace version and `Cargo.lock`
5. Creates a draft release PR

**Invocation:**
```bash
# Run the skill in Claude Code
/release
```

The skill can also be scheduled via Claude Code triggers for automated periodic releases.

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

## Workflow Interactions

### Release Flow

```
1. Developer creates PR with conventional commits
   ↓
2. PR reviewed and merged to master
   ↓
3. Maintainer runs /release skill (on-demand or scheduled)
   ├─ Analyzes all commits since last tag
   ├─ Determines version bump from actual changes
   ├─ Generates release notes and manifest
   └─ Creates draft release PR
   ↓
4. Release PR reviewed and merged to master
   ↓
5. publish-release.yml triggers automatically
   ├─ Detects manifest files
   ├─ Builds CLI binaries for 5 platforms
   ├─ Creates Git tag
   └─ Publishes GitHub Release
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

### Create a Release

```bash
# Run the /release skill in Claude Code
/release
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

### Release Didn't Publish After PR Merge

**Check:**
- Does the release PR contain `.github/releases/v*/manifest.yml`?
- Was the PR merged to `master`?
- Check `publish-release.yml` workflow logs for errors

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

- Use conventional commits for all commits
- Wait for CI to pass before requesting review
- Test locally before pushing

### For Maintainers

- Review release PRs created by `/release` before merging
- Monitor `publish-release.yml` after merging release PRs
- Review breaking change releases carefully

## Related Documentation

- [Release Process](release-process.md) - Semantic versioning and release automation
- [Pre-commit Hooks](../automation/pre-commit-hooks.md) - Local quality checks
- [Contributing Guide](https://github.com/polkadot-developers/polkadot-cookbook/blob/master/CONTRIBUTING.md) - Development workflow
