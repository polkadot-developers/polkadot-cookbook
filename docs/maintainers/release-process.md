
# Release Process

This document describes the automated release process for the Polkadot Cookbook.

## Overview

The Polkadot Cookbook uses an automated semantic versioning release system with three independent release streams:

1. **Recipe Releases** (`v0.x.x`) - Versioned collections of tested recipes
2. **CLI Releases** (`cli-v0.x.x`) - User-installable `dot` binary
3. **SDK Releases** (`sdk-v0.x.x`) - Core library for external tools

## Semantic Versioning

Releases follow [Semantic Versioning](https://semver.org/) based on conventional commits:

### During Alpha (v0.x.x)
- **Minor bump** (`v0.1.0` → `v0.2.0`): New features or breaking changes
- **Patch bump** (`v0.1.0` → `v0.1.1`): Bug fixes only

### After v1.0.0 (Stable)
- **Major bump** (`v1.0.0` → `v2.0.0`): Breaking changes
- **Minor bump** (`v1.0.0` → `v1.1.0`): New features, backward compatible
- **Patch bump** (`v1.0.0` → `v1.0.1`): Bug fixes, no API changes

## Conventional Commits

All commits must follow the [Conventional Commits](https://www.conventionalcommits.org/) format:

```
<type>(<scope>): <description>

[optional body]

[optional footer]
```

### Commit Types

| Type | Description | Version Impact |
|------|-------------|----------------|
| `feat` | New feature | MINOR bump |
| `fix` | Bug fix | PATCH bump |
| `perf` | Performance improvement | PATCH bump |
| `docs` | Documentation only | No bump |
| `test` | Adding tests | No bump |
| `refactor` | Code refactoring | No bump |
| `chore` | Maintenance tasks | No bump |
| `ci` | CI/CD changes | No bump |
| `build` | Build system changes | No bump |
| `style` | Code style/formatting | No bump |

### Breaking Changes

Add `!` after type/scope or include `BREAKING CHANGE:` in footer:

```
feat(recipe)!: redesign config format

BREAKING CHANGE: recipe.config.yml now requires version field
```

This triggers a **MAJOR** bump (or MINOR in alpha).

### Examples

```bash
# Feature (MINOR bump)
git commit -m "feat(recipe): add teleport assets example"

# Bug fix (PATCH bump)
git commit -m "fix(cli): correct validation logic"

# Breaking change (MAJOR bump)
git commit -m "feat(sdk)!: redesign error handling API"

# Documentation (no bump)
git commit -m "docs: update contributing guide"

# With scope
git commit -m "feat(cli): add recipe submit command"

# Without scope
git commit -m "docs: fix typo in README"
```

## Automated Release Triggers

### 1. Weekly Recipe Release

**Schedule:** Every Wednesday at 9:00 AM Bangkok time (02:00 UTC)

**Workflow:** `.github/workflows/release-weekly.yml`

**Process:**
1. Collects all merged PRs since last release
2. Determines version bump from semantic labels
3. Tests all recipes
4. Generates manifest with recipe inventory
5. Creates GitHub release with tag (e.g., `v0.2.0`)

**Skips release if:**
- No changes since last release
- Only non-version-bump commits (docs, chore, etc.)

### 2. Breaking Change Release

**Trigger:** CLI or SDK breaking change merged to master

**Workflow:** `.github/workflows/release-on-breaking-change.yml`

**Process:**
1. Detects `semantic:major` label on merged PR
2. Checks if CLI (`dot/cli/**`) or SDK (`dot/core/**`) changed
3. Releases new CLI/SDK version
4. Tests all recipes with new tooling
5. Creates immediate recipe release if tests pass

**Example flow:**
```
PR #123: feat(cli)!: redesign validation API
  → Merged with semantic:major label
  → CLI Release: cli-v0.3.0
  → Tests all recipes
  → Recipe Release: v0.2.0 (if tests pass)
```

### 3. Manual Release

**Trigger:** Manual `workflow_dispatch` on GitHub Actions

**Use cases:**
- Critical fixes between scheduled releases
- Coordinated releases with docs.polkadot.com
- Milestone releases (v1.0.0, v2.0.0)

**Process:**
1. Go to Actions → "Weekly Recipe Release"
2. Click "Run workflow"
3. Select branch and options
4. Click "Run workflow"

## Pull Request Workflow

### 1. Developer Creates PR

Developer creates branch and commits using conventional format:

```bash
git checkout -b feat/basic-pallet
git commit -m "feat(recipe): add pallet structure"
git commit -m "feat(recipe): add storage items"
git commit -m "test(recipe): add unit tests"
git push origin feat/basic-pallet
```

PR title can be human-friendly (no semantic syntax required):
```
"Add basic pallet recipe with storage and events"
```

### 2. Auto-Labeling

**Workflow:** `.github/workflows/auto-label-semantic.yml`

When PR is opened/updated:
1. Analyzes ALL commits in the PR
2. Determines highest semantic level:
   - Breaking change (!) → `semantic:major`
   - `feat` → `semantic:minor`
   - `fix`/`perf` → `semantic:patch`
   - Only docs/chore → `semantic:none`
3. Applies label automatically
4. Posts comment explaining decision

**Example comment:**
```
🤖 Semantic Version Analysis

Result: MINOR version bump

Commit Analysis
| Commit  | Type | Impact  | Message                              |
|---------|------|---------|--------------------------------------|
| abc123  | feat | 🟡 MINOR | feat(recipe): add pallet structure  |
| def456  | feat | 🟡 MINOR | feat(recipe): add storage items     |
| ghi789  | test | ⚪ none  | test(recipe): add unit tests        |

This PR will trigger a MINOR version bump when merged.
```

### 3. Manual Override

If auto-detection is wrong, manually add/change label:
- `semantic:major` 🔴 - Breaking changes
- `semantic:minor` 🟡 - New features
- `semantic:patch` 🟢 - Bug fixes
- `semantic:none` ⚪ - No version bump

Once manual label is added, automation respects it.

### 4. PR Merged

When PR is squash-merged:
1. Semantic label is preserved in commit message metadata
2. Next scheduled release (or breaking change trigger) uses this information
3. Version bump determined by highest semantic level across all merged PRs

## Release Artifacts

Each recipe release includes:

### Manifest (`.repo/releases/v0.x.x/manifest.yml`)

Machine-readable inventory of all recipes:

```yaml
release: v0.2.0
release_date: 2025-11-06T02:00:00Z
status: alpha

tooling:
  cli_version: cli-v0.2.0
  sdk_version: sdk-v0.1.0
  rust: "1.86"
  polkadot_sdk: "1.15.0"

recipes:
  basic-pallet:
    version: "0.1.0"
    path: "recipes/basic-pallet"
    tested: true          # Passed CI tests
    commit: "abc123def"   # Immutable reference
    pathway: "runtime"
    difficulty: "beginner"
    description: "Create a custom FRAME pallet"
```

### Release Notes (`.repo/releases/v0.x.x/RELEASE_NOTES.md`)

Human-readable changelog:

```markdown
# Polkadot Cookbook v0.2.0

Released: 2025-11-06

## Changes Since v0.1.0

**Version Bump:** MINOR

### Merged Pull Requests
- Add teleport assets recipe (#123)
- Fix storage bug in basic-pallet (#124)
- Update documentation (#125)

## Compatibility
- Rust: 1.86
- Polkadot SDK: 1.15.0

## Testing
All recipes have passed CI tests.
```

### GitHub Release

Created automatically with:
- Tag (e.g., `v0.2.0`)
- Release notes
- Manifest file attached

## Integration with docs.polkadot.com

Documentation can reference stable recipe code:

```markdown
View complete code:
https://github.com/polkadot-developers/polkadot-cookbook/tree/v0.2.0/recipes/basic-pallet

Code snippet:
https://github.com/polkadot-developers/polkadot-cookbook/blob/v0.2.0/recipes/basic-pallet/pallets/template/src/lib.rs#L45-L52
```

### Manifest-Driven Integration

docs.polkadot.com can:
1. Fetch manifest from latest release
2. Browse available tested recipes
3. Generate links to tagged release or commit hashes
4. Check compatibility (Rust, Polkadot SDK versions)

### Workflow for Docs Authors

1. Check latest release: https://github.com/polkadot-developers/polkadot-cookbook/releases
2. Review manifest for available recipes
3. Reference specific release tag in tutorials
4. Use commit hashes for immutable references (optional)

## Recipe Configuration

Each recipe must include `version` in `recipe.config.yml`:

```yaml
name: Basic Pallet
slug: basic-pallet
version: 0.1.0        # Recipe semantic version
type: polkadot-sdk
difficulty: beginner
# ... other fields
```

### Version Management

**When to bump recipe version:**

| Change Type | Example | Bump |
|-------------|---------|------|
| Breaking API change | Redesign pallet config | MAJOR |
| New feature | Add storage item | MINOR |
| Bug fix | Fix test failure | PATCH |
| Docs only | Update README | None |

**Who bumps version:**
- Contributors bump version in their PR when making changes
- Maintainers verify during review
- CLI tool can automate this (future enhancement)

## Dependency Version Management

Recipes manage dependencies using standard tooling files:

### Rust Recipes (Polkadot SDK)

**Rust version:**
- `rust-toolchain.toml` (per recipe) - Specifies Rust toolchain version
- Automatically respected by `rustup`, `cargo`, and IDEs

**Dependencies:**
- `Cargo.toml` - Specifies crate dependencies with exact versions

**Example `rust-toolchain.toml`:**
```toml
[toolchain]
channel = "1.86"
components = ["rustfmt", "clippy"]
profile = "minimal"
```

### TypeScript Recipes

**Node/npm versions:**
- `package.json` - Specifies npm package versions

**Example:**
```json
{
  "dependencies": {
    "polkadot-api": "^1.20.0",
    "vitest": "^2.0.0"
  }
}


## Troubleshooting

### Release Didn't Trigger

**Check:**
1. Are there changes since last release?
2. Do commits have valid semantic format?
3. Check GitHub Actions logs for errors

### Wrong Version Bump

**Cause:** Incorrect semantic labels

**Fix:**
1. Next PR can adjust if needed
2. Or trigger manual release with correct bump type

### Tests Failing

**Cause:** Recipe not compatible with latest CLI/SDK

**Fix:**
1. Update recipe code
2. Merge fix
3. Next release will include fix

### Manual Release Needed

**Process:**
1. Go to Actions → "Weekly Recipe Release"
2. Click "Run workflow"
3. Select options:
   - `version_bump`: Choose type or `auto`
   - `skip_tests`: Only for emergencies
4. Click "Run workflow"

## Best Practices

### For Contributors

✅ **DO:**
- Use conventional commits for all commits
- Test recipes locally before submitting
- Update recipe version in config file

❌ **DON'T:**
- Use non-conventional commit messages
- Submit untested recipes
- Forget to bump recipe version for changes

### For Maintainers

✅ **DO:**
- Review semantic labels on PRs
- Verify recipe versions are bumped correctly
- Monitor release workflow for failures

❌ **DON'T:**
- Skip release process for quick fixes
- Force push to master
- Bypass semantic labeling

### For docs.polkadot.com Authors

✅ **DO:**
- Reference specific release tags (not `latest`)
- Check manifest for compatible recipes
- Use commit hashes for extra stability
- Coordinate major cookbook updates with docs updates

❌ **DON'T:**
- Reference `master` branch directly (unstable)
- Ignore version compatibility information

## Future Enhancements

Planned improvements to the release process:

1. **Automated Recipe Versioning**
   - Detect recipe changes and auto-bump versions
   - Suggest version bumps in PR comments

3. **Release Previews**
   - Generate preview of next release
   - Show which recipes will be included

4. **Notification System**
   - Post to Discord/Slack on release
   - Create issue in docs.polkadot.com repo

5. **Release Channels**
   - Beta releases for testing
   - Stable releases for production

## Questions?

- See [.repo/releases/README.md](../.repo/releases/README.md) for manifest format details
- See [CONTRIBUTING.md](../CONTRIBUTING.md) for contribution workflow
- Open an issue for release process questions
