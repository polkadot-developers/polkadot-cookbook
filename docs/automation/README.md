---
layout: doc
title: "Automation & CI/CD"
---

# Automation & CI/CD

This section covers automation setup, pre-commit hooks, and continuous integration workflows.

## Documentation

### Local Development

- **[Pre-commit Hooks](pre-commit-hooks.md)** - Automated code quality checks

### CI/CD Workflows

For complete GitHub Actions documentation, see [../maintainers/workflows.md](../maintainers/workflows.md).

## Quick Start

### Setting Up Pre-commit Hooks

Pre-commit hooks are automatically installed when you run any cargo command:

```bash
cargo build
```

This installs hooks that check:
- **Formatting** - `cargo fmt --check`
- **Linting** - `cargo clippy`
- **Commit messages** - Conventional commits format

[→ Complete Pre-commit Hooks Guide](pre-commit-hooks.md)

### Running Checks Manually

```bash
# Format code
cargo fmt --all

# Run clippy
cargo clippy --all-targets --all-features

# Run tests
cargo test --package sdk
```

### Skipping Hooks

For emergency commits only:

```bash
git commit --no-verify -m "emergency fix"
```

## CI/CD Overview

The repository uses GitHub Actions for:

- **Automated Testing** - Recipe and SDK tests on every PR
- **Semantic Versioning** - Automatic version detection from commits
- **Release Automation** - Weekly and breaking-change-triggered releases
- **Quality Control** - Code coverage, formatting, linting

For complete CI/CD documentation, see [Maintainers Guide](../maintainers/).

---

[← Back to Documentation Hub](../)
