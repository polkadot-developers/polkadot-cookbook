---
layout: doc
title: "Pre-Commit Hooks"
---

# Pre-Commit Hooks

Automated git hooks for code quality checks using `cargo-husky`.

## Overview

This project uses [cargo-husky](https://github.com/rhysd/cargo-husky) to automatically install and manage git hooks. Unlike Python-based pre-commit frameworks, cargo-husky is Rust-native and requires **zero manual setup**.

**Hooks are automatically installed when you run:**
- `cargo build`
- `cargo test`
- Any other cargo command

No Python, no manual installation required!

## What Gets Checked

### Pre-Commit Hook

Runs before each commit to ensure code quality:

1. **Rust Formatting** (`cargo fmt --check`)
   - Ensures code follows Rust formatting standards
   - **Blocking**: commit fails if formatting issues found
   - Fix: `cargo fmt --all`

2. **Clippy Lints** (`cargo clippy`)
   - Catches common mistakes and code smells
   - **Blocking**: commit fails if warnings found
   - Fix: Address clippy suggestions

### Commit Message Hook

Validates commit message format (**non-blocking**):

1. **Conventional Commits**
   - Recommends format: `<type>(<scope>): <description>`
   - Valid types: `feat`, `fix`, `docs`, `test`, `refactor`, `chore`, `ci`, `build`, `perf`, `style`
   - Shows warning if format doesn't match
   - **Non-blocking**: commit always proceeds

2. **Auto-Skips**
   - Merge commits
   - Revert commits

For complete conventional commits documentation, see: [Commit Conventions Guide](../contributors/commit-conventions.md)

## Hook Scripts

Hooks are stored in `.cargo-husky/hooks/`:
- `pre-commit` - Formatting and linting checks
- `commit-msg` - Commit message validation

These scripts are plain shell scripts and can be modified as needed.

## Manual Hook Installation

If you need to reinstall hooks manually:

```bash
# Hooks are automatically installed on next cargo command
cargo build

# Or if cargo-husky binary is installed:
cargo husky install
```

## Running Checks Manually

### Format code

```bash
cargo fmt --all
```

### Check formatting

```bash
cargo fmt --all -- --check
```

### Run clippy

```bash
cargo clippy --all-targets --all-features -- -D warnings
```

## Skipping Hooks

Sometimes you need to bypass hooks (use sparingly):

```bash
# Skip all hooks for this commit
git commit --no-verify -m "emergency fix"
```

Common valid reasons to skip:
- WIP commits on feature branches
- Emergency hotfixes (but should still pass checks eventually)
- Documented exceptions

## Disabling Hooks

If you want to disable automatic hook installation:

```bash
# Set environment variable before running cargo
CARGO_HUSKY_DONT_INSTALL_HOOKS=1 cargo build
```

Or remove `cargo-husky` from `Cargo.toml`.

## Troubleshooting

### Hooks not running

**Symptom**: Commits go through without checks

**Solutions**:
1. Verify hooks are installed: `ls -la .git/hooks/`
2. Run `cargo build` to reinstall hooks
3. Check hook files are executable: `ls -l .cargo-husky/hooks/`

### Formatting check fails

**Symptom**: `cargo fmt --check` fails in hook

**Solution**:
```bash
# Format all code
cargo fmt --all

# Verify formatting
cargo fmt --all -- --check

# Commit again
git commit
```

### Clippy fails

**Symptom**: `cargo clippy` finds warnings

**Solution**:
```bash
# See all clippy suggestions
cargo clippy --all-targets --all-features

# Fix issues and commit again
git add .
git commit
```

### Hook runs but doesn't block

**Symptom**: Hook runs but commit proceeds anyway

**Cause**: Hook script has incorrect exit code or is the non-blocking commit-msg hook

**Solution**:
- Check hook scripts exit with non-zero on failure
- Remember: commit-msg hook is intentionally non-blocking

## Comparison to Python Pre-Commit

**Old approach (Python pre-commit):**
- ❌ Requires Python installation
- ❌ Manual setup required
- ❌ Extra dependency for Rust project
- ✅ Large ecosystem of hooks

**New approach (cargo-husky):**
- ✅ Rust-native, no Python needed
- ✅ **Automatic** installation on `cargo build`
- ✅ Simpler for Rust projects
- ✅ Hooks are plain shell scripts
- ✅ Zero external dependencies
- ✅ No setup script needed

## CI Integration

These same checks run in CI via GitHub Actions workflows. Even if you skip hooks locally, CI will catch issues before merge.

## Contributing

When adding new hooks:

1. Create script in `.cargo-husky/hooks/`
2. Make it executable: `chmod +x .cargo-husky/hooks/new-hook`
3. Test it: Run the hook script manually
4. Document it in this file

Example hook structure:

```bash
#!/bin/sh
# Description of what this hook does

set -e  # Exit on error

echo "Running my-hook..."

# Your checks here
if ! your-check-command; then
    echo "❌ Check failed"
    exit 1  # Non-zero exit blocks commit
fi

echo "✅ Check passed"
exit 0
```

## References

- [cargo-husky GitHub](https://github.com/rhysd/cargo-husky)
- [Conventional Commits](https://www.conventionalcommits.org/)
- [Git Hooks Documentation](https://git-scm.com/docs/githooks)
- [Rust Clippy](https://github.com/rust-lang/rust-clippy)
