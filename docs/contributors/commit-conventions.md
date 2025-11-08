# Commit Message Conventions

The Polkadot Cookbook uses [Conventional Commits](https://www.conventionalcommits.org/) for all commit messages. This enables automated semantic versioning and changelog generation.

## Table of Contents

- [Why Conventional Commits?](#why-conventional-commits)
- [Format](#format)
- [Commit Types](#commit-types)
- [Scopes](#scopes)
- [Breaking Changes](#breaking-changes)
- [Examples](#examples)
- [Validation](#validation)
- [Best Practices](#best-practices)
- [Troubleshooting](#troubleshooting)

## Why Conventional Commits?

Using conventional commits enables:

✅ **Automated versioning** - Determines MAJOR/MINOR/PATCH bumps automatically
✅ **Auto-generated changelogs** - Creates release notes from commit messages
✅ **Semantic labels** - PRs automatically labeled with `semantic:major/minor/patch`
✅ **Release automation** - Triggers appropriate release workflows
✅ **Clear history** - Makes it easy to understand what changed

## Format

All commits must follow this format:

```
<type>(<scope>): <description>

[optional body]

[optional footer]
```

### Components

- **type** (required): The kind of change (see [Commit Types](#commit-types))
- **scope** (optional): What part of the codebase changed (see [Scopes](#scopes))
- **description** (required): Short summary of the change
- **body** (optional): Longer explanation of the change
- **footer** (optional): Breaking changes, issue references

### Rules

- Description must be lowercase
- No period at the end of description
- Use imperative mood ("add" not "added" or "adds")
- Keep description under 72 characters
- Wrap body at 72 characters

## Commit Types

| Type | Description | Version Impact | When to Use |
|------|-------------|----------------|-------------|
| `feat` | New feature | **MINOR** bump | Adding new functionality |
| `fix` | Bug fix | **PATCH** bump | Fixing a bug |
| `perf` | Performance improvement | **PATCH** bump | Improving performance |
| `docs` | Documentation only | **No bump** | Updating documentation |
| `test` | Adding tests | **No bump** | Adding or updating tests |
| `refactor` | Code refactoring | **No bump** | Refactoring without changing behavior |
| `chore` | Maintenance tasks | **No bump** | Dependency updates, tooling |
| `ci` | CI/CD changes | **No bump** | Workflow updates |
| `build` | Build system changes | **No bump** | Build configuration changes |
| `style` | Code style/formatting | **No bump** | Formatting, missing semicolons |

### Version Impact Explained

**During Alpha (v0.x.x):**
- **Breaking changes** → MINOR bump (v0.1.0 → v0.2.0)
- **Features** → MINOR bump (v0.1.0 → v0.2.0)
- **Fixes** → PATCH bump (v0.1.0 → v0.1.1)

**After v1.0.0 (Stable):**
- **Breaking changes** → MAJOR bump (v1.0.0 → v2.0.0)
- **Features** → MINOR bump (v1.0.0 → v1.1.0)
- **Fixes** → PATCH bump (v1.0.0 → v1.0.1)

## Scopes

Common scopes in the cookbook:

- `recipe` - Recipe-related changes
- `cli` - CLI tool changes
- `sdk` / `core` - SDK/core library changes
- `ci` - CI/CD workflow changes
- `docs` - Documentation changes
- `deps` - Dependency updates

**Scope is optional** - if the change doesn't fit a specific scope, omit it.

## Breaking Changes

Breaking changes trigger a **MAJOR** version bump (or MINOR in alpha).

### Two Ways to Mark Breaking Changes

**Option 1: Use `!` after type/scope**

```
feat(sdk)!: redesign error handling API
```

**Option 2: Use `BREAKING CHANGE:` footer**

```
feat(recipe): redesign config format

BREAKING CHANGE: recipe.config.yml now requires version field.
All existing recipes must add a version field.
```

### When is it Breaking?

A change is breaking if it:
- Removes or renames public APIs
- Changes function signatures
- Requires user action to upgrade
- Changes file formats in incompatible ways
- Removes CLI commands or flags

## Examples

### Features (MINOR bump)

```bash
# New recipe
git commit -m "feat(recipe): add teleport assets example"

# New CLI feature
git commit -m "feat(cli): add recipe type selection in interactive mode"

# New SDK feature
git commit -m "feat(sdk): add version source tracking"
```

### Bug Fixes (PATCH bump)

```bash
# Recipe bug fix
git commit -m "fix(recipe): correct storage operations in basic-pallet"

# CLI bug fix

# SDK bug fix
git commit -m "fix(core): validate YAML syntax before parsing"
```

### Breaking Changes (MAJOR bump)

```bash
# With ! notation
git commit -m "feat(sdk)!: redesign error handling API"

# With footer (includes explanation)
git commit -m "$(cat <<'EOF'
feat(cli)!: redesign validation API

Replace boolean return with Result<ValidationReport>
for better error reporting.

BREAKING CHANGE: Validation functions now return Result.
Update code calling validate() to handle Result type.
EOF
)"
```

### Documentation (No bump)

```bash
git commit -m "docs: update contributing guidelines"
git commit -m "docs(recipe): add usage examples to README"
git commit -m "docs: fix typo in architecture.md"
```

### Other Types (No bump)

```bash
# Chore
git commit -m "chore(deps): update polkadot-sdk to 1.15.0"
git commit -m "chore: update .gitignore"

# CI
git commit -m "ci: add coverage threshold check"
git commit -m "ci(workflow): optimize cache strategy"

# Refactor
git commit -m "refactor(sdk): simplify version resolution logic"

# Test
git commit -m "test(core): add integration tests for scaffold"

# Style
git commit -m "style: run cargo fmt"
```

### Multi-line Commits

For complex changes, use body and footer:

```bash
git commit -m "$(cat <<'EOF'
feat(recipe): add staking pallet example

Implement a complete staking pallet with:
- Validator registration
- Nominator delegation
- Reward distribution
- Slashing logic

Includes comprehensive tests and documentation.

Closes #123
EOF
)"
```

## Validation

### Pre-commit Hook

The repository includes a pre-commit hook that validates commit messages:

```bash
# Hook validates commit format
git commit -m "add new feature"  # ❌ Invalid format

git commit -m "feat: add new feature"  # ✅ Valid format
```

**Note:** The commit-msg hook is **non-blocking** - it shows warnings but allows commits.

Valid types checked:
- `feat`, `fix`, `docs`, `test`, `refactor`
- `chore`, `ci`, `build`, `perf`, `style`

### Auto-skips

The hook automatically skips validation for:
- Merge commits
- Revert commits

### PR Validation

When you create a PR, the `auto-label-semantic` workflow:

1. **Analyzes all commits** in the PR
2. **Detects version impact** from commit types
3. **Applies semantic label** (`semantic:major/minor/patch/none`)
4. **Posts comment** explaining the analysis

**Example comment:**
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

### Manual Override

If the automatic label is wrong, you can manually change it:

- `semantic:major` 🔴 - Breaking changes
- `semantic:minor` 🟡 - New features
- `semantic:patch` 🟢 - Bug fixes
- `semantic:none` ⚪ - No version bump

## Best Practices

### DO ✅

**Write clear, descriptive commits:**
```bash
✅ feat(cli): add --skip-install flag to recipe create
✅ fix(recipe): prevent integer overflow in calculation
✅ docs(api): add examples for version resolution
```

**Use imperative mood:**
```bash
✅ feat: add feature
✅ fix: correct bug
✅ docs: update guide
```

**Keep descriptions concise:**
```bash
✅ feat(cli): add recipe type selection
✅ fix(core): validate YAML syntax
```

**Add context in body when needed:**
```bash
✅ feat(recipe): add staking pallet

Implement complete staking system with validator
registration, nomination, and reward distribution.
```

### DON'T ❌

**Avoid vague messages:**
```bash
❌ fix: fix bug
❌ chore: update stuff
❌ feat: improvements
```

**Don't use past tense:**
```bash
❌ feat: added feature
❌ fix: fixed bug
❌ docs: updated guide
```

**Don't exceed character limits:**
```bash
❌ feat(cli): add a very long description that exceeds seventy-two characters and should be split
```

**Don't include periods:**
```bash
❌ feat: add feature.
❌ fix: correct bug.
```

## Troubleshooting

### Commit Hook Warning

**Symptom:** Hook shows warning about invalid format

**Cause:** Commit message doesn't follow conventional format

**Solution:**
```bash
# Fix and amend commit
git commit --amend -m "feat: proper format"

# Or skip hook (not recommended)
git commit --no-verify -m "invalid format"
```

### Wrong Semantic Label on PR

**Symptom:** PR has incorrect `semantic:*` label

**Causes:**
- Commits don't follow conventional format
- Mixed commit types in PR
- Breaking change not properly marked

**Solution:**
1. Review PR commits
2. Manually change label if needed
3. Or fix commit messages:
   ```bash
   git rebase -i HEAD~3
   git commit --amend
   git push --force-with-lease
   ```

### Release Didn't Trigger

**Symptom:** Expected release didn't happen

**Causes:**
- All commits were `docs`, `chore`, etc. (no version bump)
- Commits don't follow conventional format
- No changes since last release

**Solution:**
- Check commit messages follow format
- Ensure at least one `feat` or `fix` commit
- Review release workflow logs

### Breaking Change Not Detected

**Symptom:** Breaking change didn't trigger major release

**Causes:**
- Missing `!` or `BREAKING CHANGE:` marker
- Not detected by semantic analysis

**Solution:**
```bash
# Use ! notation
git commit -m "feat(sdk)!: breaking change"

# Or use footer
git commit -m "$(cat <<'EOF'
feat(sdk): breaking change

BREAKING CHANGE: Description of what broke
EOF
)"
```

## Related Documentation

- [Conventional Commits Spec](https://www.conventionalcommits.org/)
- [Workflow Guide](workflow.md) - How to create PRs
- [Release Process](../maintainers/release-process.md) - How commits trigger releases
- [Pre-commit Hooks](../automation/pre-commit-hooks.md) - Automated validation

---

[← Back to Contributors Guide](README.md)
