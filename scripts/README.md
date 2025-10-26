# Scripts

Utility scripts for Polkadot Cookbook development and CI/CD.

## Setup Scripts

### `setup-pre-commit.sh`

Installs and configures pre-commit hooks for automated code quality checks.

**Usage:**
```bash
./scripts/setup-pre-commit.sh
```

**What it does:**
- Installs `pre-commit` via pip (if not already installed)
- Configures git hooks for pre-commit and commit-msg stages
- Enables automated checks for Rust formatting, linting, and more

**See:** [docs/pre-commit-hooks.md](../docs/pre-commit-hooks.md)

### `check-commit-msg.sh`

Validates commit message format against conventional commit standards (warning only).

**Usage:**
```bash
# Automatically called by pre-commit hook
# Can also test manually:
./scripts/check-commit-msg.sh .git/COMMIT_EDITMSG
```

**What it does:**
- Checks if commit follows conventional commit format
- Shows a helpful warning with examples if format doesn't match
- **Non-blocking** - always allows commit to proceed
- Skips merge commits and reverts

## Tool Installation Scripts

### `setup-rust.sh`

Installs Rust toolchain via rustup.

**Usage:**
```bash
./scripts/setup-rust.sh
```

### `install-omni-node.sh`

Downloads and installs `polkadot-omni-node` binary.

**Usage:**
```bash
./scripts/install-omni-node.sh
```

**Environment Variables:**
- `VERSION` - Specific version to install (default: latest from versions.yml)

### `install-chain-spec-builder.sh`

Downloads and installs `chain-spec-builder` binary.

**Usage:**
```bash
./scripts/install-chain-spec-builder.sh
```

**Environment Variables:**
- `VERSION` - Specific version to install (default: latest from versions.yml)

## Chain Spec Scripts

### `generate-chain-spec.sh`

Generates chain specification files for parachains.

**Usage:**
```bash
./scripts/generate-chain-spec.sh
```

**Requirements:**
- `chain-spec-builder` must be installed
- Run from parachain directory or specify path

## CI/CD Integration

These scripts are used by GitHub Actions workflows. See:
- `.github/workflows/test-sdk.yml`
- `.github/workflows/test-recipes.yml`
- `.github/actions/setup-*/action.yml`

## Contributing

When adding new scripts:

1. Make them executable: `chmod +x scripts/new-script.sh`
2. Add usage documentation to this README
3. Include error handling and helpful messages
4. Test in both local and CI environments
