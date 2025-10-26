# Pre-commit Hooks

This project uses [pre-commit](https://pre-commit.com/) to run automated checks before each commit, ensuring code quality and consistency.

## Installation

Run the setup script from the repository root:

```bash
./scripts/setup-pre-commit.sh
```

This will:
1. Install `pre-commit` if not already installed
2. Set up git hooks for pre-commit and commit-msg stages
3. Configure all checks

### Manual Installation

If you prefer to install manually:

```bash
# Install pre-commit (requires Python)
pip install pre-commit

# Install the git hooks
pre-commit install
pre-commit install --hook-type commit-msg
```

## What Gets Checked

### Rust Code Quality

- **`cargo fmt`** - Ensures consistent code formatting
- **`cargo clippy`** - Catches common mistakes and anti-patterns

### File Validation

- **YAML syntax** - Validates `.yml` and `.yaml` files
- **JSON syntax** - Validates `.json` files
- **TOML syntax** - Validates `Cargo.toml` and other TOML files

### Code Hygiene

- **End of file fixer** - Ensures files end with a newline
- **Trailing whitespace** - Removes trailing whitespace
- **Line endings** - Enforces LF line endings
- **Large files** - Prevents accidentally committing files >1MB
- **Merge conflicts** - Detects unresolved merge conflict markers

### Documentation

- **Markdown linting** - Ensures consistent markdown formatting
  - Rules configured in `.markdownlint.json`
  - Disabled rules: MD013 (line length), MD033 (inline HTML), MD041 (first line heading)

### Commit Messages

- **Conventional commits** - **Warning only (non-blocking)**
  - Shows a warning if commit doesn't follow conventional format
  - Commit proceeds regardless (warning only)
  - Recommended types: `feat`, `fix`, `docs`, `test`, `refactor`, `chore`, `ci`, `build`
  - Format: `type(scope): description`
  - Examples:
    - `feat(recipe): add zero-to-hero recipe`
    - `fix(cli): correct validation error message`
    - `docs: update CONTRIBUTING.md`
  - Why non-blocking? We encourage but don't enforce this format to keep the contribution barrier low

## Running Manually

### Check All Files

```bash
pre-commit run --all-files
```

### Check Specific Files

```bash
pre-commit run --files path/to/file.rs
```

### Check Specific Hook

```bash
pre-commit run fmt --all-files
pre-commit run clippy --all-files
```

## Skipping Hooks

**Use sparingly!** Only skip hooks when absolutely necessary:

```bash
git commit --no-verify
```

Common valid reasons to skip:
- WIP commits on feature branches
- Emergency hotfixes (but should still pass checks eventually)
- Commits that intentionally fail a check (with good reason)

## Troubleshooting

### Hook Fails but Changes Look Correct

Some hooks (like `fmt` and `markdownlint`) auto-fix issues. After they run:

1. Review the changes they made
2. Stage the auto-fixed files: `git add .`
3. Commit again

### Clippy Warnings

If clippy fails, fix the warnings:

```bash
cargo clippy --all -- -D warnings
```

Then commit again.

### Python/pip Not Found

Install Python:
- **macOS**: `brew install python`
- **Ubuntu**: `sudo apt install python3-pip`
- **Windows**: Download from [python.org](https://www.python.org/downloads/)

### Hooks Take Too Long

Pre-commit caches results. First run is slow, subsequent runs are fast. If hooks are consistently slow:

```bash
# Clear cache
pre-commit clean

# Update to latest hook versions
pre-commit autoupdate
```

## Configuration

### `.pre-commit-config.yaml`

Main configuration file defining all hooks and their settings.

### `.markdownlint.json`

Markdown linting rules. Customize as needed:

```json
{
  "default": true,
  "MD013": false,  // Line length (disabled)
  "MD033": false,  // Inline HTML (disabled)
  "MD041": false   // First line heading (disabled)
}
```

## Updating Hooks

Keep hooks up-to-date with the latest versions:

```bash
pre-commit autoupdate
```

This updates the `rev` fields in `.pre-commit-config.yaml`.

## Disabling Hooks

To temporarily disable pre-commit hooks:

```bash
pre-commit uninstall
```

To re-enable:

```bash
pre-commit install
pre-commit install --hook-type commit-msg
```

## CI Integration

Pre-commit hooks also run in CI via GitHub Actions. See `.github/workflows/` for details.

Even if you skip hooks locally, CI will catch issues before merge.

## Learn More

- [pre-commit documentation](https://pre-commit.com/)
- [Conventional Commits](https://www.conventionalcommits.org/)
- [Rust pre-commit hooks](https://github.com/doublify/pre-commit-rust)
