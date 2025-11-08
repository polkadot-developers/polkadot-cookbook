# CLI Reference

Complete reference for the `dot` command-line tool.

## Installation

See [Getting Started - Installation](../getting-started/installation.md) for installation instructions.

## Global Flags

Flags that work with all commands:

```bash
dot [GLOBAL_FLAGS] <COMMAND> [COMMAND_FLAGS]
```

- `--help, -h` - Show help information
- `--version, -V` - Show version information

## Commands

### `recipe create`

Create a new recipe with scaffolded structure.

**Usage:**
```bash
dot recipe create [OPTIONS]
```

**Interactive Mode (Recommended):**
```bash
dot recipe create
```

Launches an interactive prompt that guides you through:
1. Recipe title
2. Pathway selection (runtime, contracts, basic-interaction, xcm, testing)
3. Difficulty level (beginner, intermediate, advanced)
4. Content type (tutorial, guide)

**Non-Interactive Mode:**
```bash
dot recipe create --title "My Recipe" --non-interactive [OPTIONS]
```

**Options:**

| Flag | Description | Required | Default |
|------|-------------|----------|---------|
| `--title <TITLE>` | Recipe title | Yes (non-interactive) | - |
| `--pathway <PATHWAY>` | Recipe pathway | No | Interactive prompt |
| `--difficulty <DIFFICULTY>` | Difficulty level | No | Interactive prompt |
| `--content-type <TYPE>` | Content type | No | Interactive prompt |
| `--skip-install` | Skip npm dependency installation | No | false |
| `--no-git` | Skip git branch creation | No | false |
| `--non-interactive` | Non-interactive mode | No | false |

**Pathway Options:**
- `runtime` - Polkadot SDK runtime development (pallets, runtimes)
- `contracts` - Smart contract development (Solidity, ink!)
- `basic-interaction` - Basic blockchain interactions
- `xcm` - Cross-chain messaging
- `testing` - Testing strategies and patterns

**Difficulty Options:**
- `beginner` - Introductory recipes
- `intermediate` - Moderate complexity
- `advanced` - Complex, production-ready examples

**Content Type Options:**
- `tutorial` - Step-by-step learning content
- `guide` - Reference/how-to guides

**Examples:**

```bash
# Interactive mode (recommended for first-time users)
dot recipe create

# Non-interactive with title only (minimal)
dot recipe create --title "Custom Pallet Recipe" --non-interactive

# Full specification (CI/CD)
dot recipe create \
  --title "My Recipe" \
  --pathway runtime \
  --difficulty beginner \
  --content-type tutorial \
  --skip-install \
  --no-git \
  --non-interactive

# Quick creation without installation
dot recipe create --title "Quick Test" --skip-install --non-interactive
```

**What it Creates:**

```
recipes/my-recipe/
├── README.md              # Recipe content and documentation
├── recipe.config.yml      # Metadata and configuration
├── versions.yml           # Dependency version overrides
├── package.json           # npm dependencies (TypeScript recipes)
├── tsconfig.json          # TypeScript configuration
├── vitest.config.ts       # Test configuration
├── src/                   # Implementation code
├── tests/                 # Test files
└── scripts/               # Deployment scripts (Solidity recipes)
```

Note: Structure varies by recipe type (Runtime, Solidity, XCM, etc.)

**Exit Codes:**
- `0` - Success
- `1` - Error (invalid input, file system error, etc.)

---

### `versions`

View and manage dependency versions for recipes.

**Usage:**
```bash
dot versions [OPTIONS] [SLUG]
```

**Arguments:**

- `SLUG` - Recipe slug. Omit to show global versions.

**Options:**

| Flag | Description | Output Format |
|------|-------------|---------------|
| `--ci` | Output in CI format (KEY=VALUE pairs) | Shell variables |
| `--show-source` | Show version sources (global vs recipe) | Human-readable |
| `--validate` | Validate version keys | Validation report |

**Examples:**

```bash
# View global versions
dot versions

# View recipe-specific versions
dot versions zero-to-hero

# Debug version resolution (show sources)
dot versions my-recipe --show-source

# CI usage (export as environment variables)
eval $(dot versions my-recipe --ci)
echo "Using Rust $RUST"

# Validate configuration
dot versions my-recipe --validate
```

**Output Formats:**

**Default (human-readable):**
```
📦 Versions for recipe: my-recipe

  rust                1.86
  polkadot_omni_node  0.5.0
  chain_spec_builder  10.0.0
  frame_omni_bencher  0.13.0
```

**With `--show-source`:**
```
📦 Versions for recipe: my-recipe

  rust                1.86   (global)
  polkadot_omni_node  0.6.0  (recipe)
  chain_spec_builder  10.0.0 (global)
  frame_omni_bencher  0.13.0 (global)
```

**With `--ci` (shell variables):**
```
RUST=1.86
POLKADOT_OMNI_NODE=0.6.0
CHAIN_SPEC_BUILDER=10.0.0
FRAME_OMNI_BENCHER=0.13.0
```

**With `--validate`:**
```
✅ All version keys are valid!

Found 4 valid version keys:
  • rust
  • polkadot_omni_node
  • chain_spec_builder
  • frame_omni_bencher
```

**Version Resolution:**

The CLI merges global versions (`versions.yml` at repo root) with recipe-specific versions (`recipes/<slug>/versions.yml`). Recipe versions override global versions on a per-key basis.

See [Version Management Guide](../maintainers/version-management.md) for details.

**Exit Codes:**
- `0` - Success
- `1` - Error (file not found, invalid YAML, etc.)

---

### `recipe submit`

Submit recipe as a pull request (requires GitHub CLI).

**Usage:**
```bash
dot recipe submit
```

**Prerequisites:**
- GitHub CLI (`gh`) installed and authenticated
- Git configured with user.name and user.email
- Recipe exists in `recipes/` directory
- Changes committed to git

**What it Does:**
1. Validates recipe structure
2. Checks git repository state
3. Pushes to your fork (or creates one)
4. Creates pull request on GitHub

**Interactive Flow:**
1. Prompts for recipe slug (if not in recipe directory)
2. Validates recipe exists
3. Checks for uncommitted changes
4. Pushes to remote
5. Creates PR with template

**Exit Codes:**
- `0` - Success (PR created)
- `1` - Error (validation failed, git error, GitHub error)

**Troubleshooting:**
```bash
# Ensure gh CLI is authenticated
gh auth status

# Login if needed
gh auth login

# Configure git if needed
git config --global user.name "Your Name"
git config --global user.email "your.email@example.com"
```

---

### `recipe list`

List all recipes in the repository.

**Usage:**
```bash
dot recipe list
```

**Output:**
```
📚 Available recipes:

Runtime Recipes:
  • basic-pallet (beginner)
  • custom-runtime (intermediate)

Contract Recipes:
  • simple-contract (beginner)

XCM Recipes:
  • teleport-assets (intermediate)
```

**Exit Codes:**
- `0` - Success
- `1` - Error (repository not found)

---

### `recipe validate`

Validate recipe structure and configuration.

**Usage:**
```bash
dot recipe validate <SLUG>
```

**Arguments:**
- `SLUG` - Recipe slug to validate

**What it Checks:**
- `recipe.config.yml` exists and is valid
- Required files present (README.md, etc.)
- YAML syntax correctness
- Version keys validity

**Output:**
```
✅ Recipe validation passed!

Checks:
  ✓ recipe.config.yml exists
  ✓ README.md exists
  ✓ versions.yml is valid
  ✓ All version keys are known
```

**Exit Codes:**
- `0` - Validation passed
- `1` - Validation failed

---

### `recipe test`

Run tests for a recipe.

**Usage:**
```bash
dot recipe test <SLUG>
```

**Arguments:**
- `SLUG` - Recipe slug to test

**What it Does:**
1. Detects recipe type (Rust, TypeScript, etc.)
2. Runs appropriate test command
3. Reports results

**Examples:**
```bash
# Test specific recipe
dot recipe test basic-pallet

# Test current directory recipe
dot recipe test .
```

**Exit Codes:**
- `0` - Tests passed
- `1` - Tests failed

---

### `recipe lint`

Run linting checks on a recipe.

**Usage:**
```bash
dot recipe lint <SLUG>
```

**Arguments:**
- `SLUG` - Recipe slug to lint

**What it Checks:**
- Code formatting (Rust: `cargo fmt`, TypeScript: `prettier`)
- Code quality (Rust: `cargo clippy`, TypeScript: `eslint`)
- Documentation quality

**Exit Codes:**
- `0` - Lint checks passed
- `1` - Lint issues found

---

### `setup`

Check and setup your development environment.

**Usage:**
```bash
dot setup
```

**What it Checks:**
- Rust toolchain installed
- cargo available
- Node.js and npm installed
- Git configured
- GitHub CLI installed (optional)

**Output:**
```
🔍 Checking development environment...

✓ Rust toolchain: 1.86.0
✓ cargo: 1.86.0
✓ Node.js: 20.10.0
✓ npm: 10.2.3
✓ Git: 2.42.0
✓ GitHub CLI: 2.40.0

✅ Environment ready!
```

**Exit Codes:**
- `0` - Environment ready
- `1` - Missing dependencies

---

### `doctor`

Run comprehensive health checks.

**Usage:**
```bash
dot doctor
```

**What it Checks:**
- All `setup` checks
- Repository structure validity
- Configuration files syntax
- Dependencies up-to-date
- Common issues

**Output:**
```
🏥 Running health checks...

Repository:
  ✓ Valid git repository
  ✓ On branch: main
  ✓ No uncommitted changes

Configuration:
  ✓ versions.yml valid
  ✓ All recipe configs valid

Dependencies:
  ⚠ Rust 1.85.0 (1.86.0 available)
  ✓ Node.js up to date

✅ Overall health: Good
⚠ 1 warning
```

**Exit Codes:**
- `0` - All checks passed
- `1` - Critical issues found

---

## Environment Variables

The CLI respects these environment variables:

| Variable | Description | Default |
|----------|-------------|---------|
| `GITHUB_TOKEN` | GitHub authentication token | From `gh auth token` |
| `RUST_LOG` | Logging level (error, warn, info, debug, trace) | error |
| `NO_COLOR` | Disable colored output | false |

**Examples:**

```bash
# Enable debug logging
RUST_LOG=debug dot recipe create

# Use specific GitHub token
GITHUB_TOKEN=ghp_xxx dot recipe submit

# Disable colors
NO_COLOR=1 dot versions
```

---

## Configuration Files

### Global Configuration

Location: `~/.config/polkadot-cookbook/config.toml` (TODO: Not yet implemented)

Future configuration options:
- Default pathway
- Default difficulty
- Editor preferences
- GitHub username/org

### Repository Configuration

**versions.yml** (repository root)
- Global dependency versions
- See [Version Management](../maintainers/version-management.md)

**recipe.config.yml** (per recipe)
- Recipe metadata
- See [Recipe Config Schema](../reference/recipe-config-schema.md)

---

## Troubleshooting

### Command Not Found

**Symptom:** `dot: command not found`

**Solution:**
```bash
# Add to PATH
export PATH="$PATH:/path/to/polkadot-cookbook/target/release"

# Or install globally (future)
cargo install --path ./cli
```

### Permission Denied

**Symptom:** `Permission denied` when running `dot`

**Solution:**
```bash
# Make executable
chmod +x target/release/dot
```

### GitHub Authentication Failed

**Symptom:** `dot recipe submit` fails with auth error

**Solution:**
```bash
# Check auth status
gh auth status

# Login
gh auth login

# Verify token
gh auth token
```

### Invalid Recipe Structure

**Symptom:** `dot recipe validate` fails

**Solution:**
```bash
# Check what's wrong
dot recipe validate my-recipe

# Fix issues reported
# Re-validate
dot recipe validate my-recipe
```

---

## Related Documentation

- [Getting Started](../getting-started/) - Installation and first recipe
- [Contributors Guide](../contributors/) - Contributing recipes
- [Version Management](../maintainers/version-management.md) - Managing versions
- [SDK Guide](sdk-guide.md) - Using the SDK programmatically

---

[← Back to Developers Guide](README.md)
