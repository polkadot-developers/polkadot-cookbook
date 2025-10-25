# Polkadot Cookbook CLI

Command-line tool for creating and managing Polkadot Cookbook recipes.

## Installation

### From Source

```bash
git clone https://github.com/polkadot-developers/polkadot-cookbook.git
cd polkadot-cookbook
cargo build --package polkadot-cookbook-cli --release
```

The binary will be available at `target/release/create-recipe`.

### Add to PATH (Optional)

```bash
# Add to your shell profile (.bashrc, .zshrc, etc.)
export PATH="$PATH:/path/to/polkadot-cookbook/target/release"
```

## Quick Start

### Create a Recipe (Interactive)

```bash
create-recipe create
```

This launches an interactive prompt that guides you through recipe creation.

### Create a Recipe (Command Line)

```bash
create-recipe create my-awesome-recipe
```

### View Versions

```bash
# View global dependency versions
create-recipe versions

# View recipe-specific versions
create-recipe versions my-recipe

# Show where each version comes from
create-recipe versions my-recipe --show-source
```

## Commands

### `create`

Create a new recipe with scaffolded structure.

**Usage:**
```bash
create-recipe create [OPTIONS] [SLUG]
```

**Arguments:**
- `SLUG` - Recipe slug (e.g., "my-recipe"). Optional in interactive mode.

**Options:**
- `--skip-install` - Skip npm dependency installation
- `--no-git` - Skip git branch creation
- `--non-interactive` - Non-interactive mode (requires SLUG)

**Examples:**
```bash
# Interactive mode (recommended)
create-recipe create

# With slug
create-recipe create custom-pallet-recipe

# Skip installation for faster creation
create-recipe create my-recipe --skip-install

# CI/CD mode
create-recipe create my-recipe --non-interactive --skip-install
```

**What it creates:**
```
recipes/my-recipe/
├── README.md              # Recipe content
├── recipe.config.yml    # Metadata
├── versions.yml           # Dependency versions
├── package.json           # npm dependencies
├── tsconfig.json          # TypeScript config
├── vitest.config.ts       # Test config
├── src/                   # Implementation code
├── tests/                 # Test files
└── scripts/               # Helper scripts
```

### `versions`

View and manage dependency versions for recipes.

**Usage:**
```bash
create-recipe versions [OPTIONS] [SLUG]
```

**Arguments:**
- `SLUG` - Recipe slug. Omit to show global versions.

**Options:**
- `--ci` - Output in CI format (KEY=VALUE pairs)
- `--show-source` - Show version sources (global vs recipe override)
- `--validate` - Validate version keys

**Examples:**
```bash
# View global versions
create-recipe versions

# View recipe versions
create-recipe versions zero-to-hero

# Debug version resolution
create-recipe versions my-recipe --show-source

# CI usage
eval $(create-recipe versions my-recipe --ci)
echo "Using Rust $RUST"

# Validate configuration
create-recipe versions my-recipe --validate
```

**Version Resolution:**

The CLI merges global versions (`versions.yml` at repo root) with recipe-specific versions (`recipes/<slug>/versions.yml`). Recipe versions override global versions on a per-key basis.

Example:
```yaml
# Global versions.yml
versions:
  rust: "1.86"
  polkadot_omni_node: "0.5.0"

# Recipe versions.yml
versions:
  polkadot_omni_node: "0.6.0"  # Override

# Resolved: rust=1.86, polkadot_omni_node=0.6.0
```

## Common Workflows

### Contributing a Recipe

```bash
# 1. Create recipe structure
create-recipe create my-awesome-recipe

# 2. Write content
cd recipes/my-awesome-recipe
code README.md

# 3. Implement code
code src/lib.rs

# 4. Write tests
code tests/recipe.test.ts

# 5. Test locally
npm test

# 6. Commit and push
git add -A
git commit -m "feat(recipe): add my awesome recipe"
git push origin recipe/my-awesome-recipe
```

### Testing with Custom Versions

```bash
# Create recipe
create-recipe create test-new-version

# Edit versions
cd recipes/test-new-version
cat > versions.yml <<EOF
versions:
  polkadot_omni_node: "0.7.0"
metadata:
  schema_version: "1.0"
EOF

# Verify resolution
create-recipe versions test-new-version --show-source

# Test
npm test
```

## Configuration

### Global Versions

Edit `versions.yml` at repository root to change default versions for all recipes.

### Recipe Versions

Each recipe can override versions by editing its `recipes/<slug>/versions.yml` file.

### Recipe Metadata

Edit `recipes/<slug>/recipe.config.yml` to configure:
- Recipe name and description
- Category (polkadot-sdk-cookbook or contracts-cookbook)
- Whether a node is required (`needs_node`)
- Build and runtime settings

## Troubleshooting

### "Invalid working directory"

**Problem:** Running command from wrong directory

**Solution:** Run from repository root

```bash
cd /path/to/polkadot-cookbook
create-recipe create my-recipe
```

### "Slug argument is required"

**Problem:** Non-interactive mode without slug

**Solution:** Provide slug argument

```bash
create-recipe create my-recipe --non-interactive
```

### "Failed to resolve versions"

**Problem:** Invalid YAML syntax in versions.yml

**Solution:** Validate YAML

```bash
# Check syntax
yq eval recipes/my-recipe/versions.yml

# Use validation flag
create-recipe versions my-recipe --validate
```

## Development

To contribute to the CLI itself, see the [Contributing Guide](../CONTRIBUTING.md) and [Development Guide](../docs/architecture.md).

## License

MIT OR Apache-2.0
