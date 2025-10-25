# Polkadot Cookbook CLI

Command-line tool for creating and managing Polkadot Cookbook tutorials.

## Installation

### From Source

```bash
git clone https://github.com/polkadot-developers/polkadot-cookbook.git
cd polkadot-cookbook
cargo build --package polkadot-cookbook-cli --release
```

The binary will be available at `target/release/create-tutorial`.

### Add to PATH (Optional)

```bash
# Add to your shell profile (.bashrc, .zshrc, etc.)
export PATH="$PATH:/path/to/polkadot-cookbook/target/release"
```

## Quick Start

### Create a Tutorial (Interactive)

```bash
create-tutorial create
```

This launches an interactive prompt that guides you through tutorial creation.

### Create a Tutorial (Command Line)

```bash
create-tutorial create my-awesome-tutorial
```

### View Versions

```bash
# View global dependency versions
create-tutorial versions

# View tutorial-specific versions
create-tutorial versions my-tutorial

# Show where each version comes from
create-tutorial versions my-tutorial --show-source
```

## Commands

### `create`

Create a new tutorial with scaffolded structure.

**Usage:**
```bash
create-tutorial create [OPTIONS] [SLUG]
```

**Arguments:**
- `SLUG` - Tutorial slug (e.g., "my-tutorial"). Optional in interactive mode.

**Options:**
- `--skip-install` - Skip npm dependency installation
- `--no-git` - Skip git branch creation
- `--non-interactive` - Non-interactive mode (requires SLUG)

**Examples:**
```bash
# Interactive mode (recommended)
create-tutorial create

# With slug
create-tutorial create custom-pallet-tutorial

# Skip installation for faster creation
create-tutorial create my-tutorial --skip-install

# CI/CD mode
create-tutorial create my-tutorial --non-interactive --skip-install
```

**What it creates:**
```
tutorials/my-tutorial/
├── README.md              # Tutorial content
├── tutorial.config.yml    # Metadata
├── versions.yml           # Dependency versions
├── package.json           # npm dependencies
├── tsconfig.json          # TypeScript config
├── vitest.config.ts       # Test config
├── src/                   # Implementation code
├── tests/                 # Test files
└── scripts/               # Helper scripts
```

### `versions`

View and manage dependency versions for tutorials.

**Usage:**
```bash
create-tutorial versions [OPTIONS] [SLUG]
```

**Arguments:**
- `SLUG` - Tutorial slug. Omit to show global versions.

**Options:**
- `--ci` - Output in CI format (KEY=VALUE pairs)
- `--show-source` - Show version sources (global vs tutorial override)
- `--validate` - Validate version keys

**Examples:**
```bash
# View global versions
create-tutorial versions

# View tutorial versions
create-tutorial versions zero-to-hero

# Debug version resolution
create-tutorial versions my-tutorial --show-source

# CI usage
eval $(create-tutorial versions my-tutorial --ci)
echo "Using Rust $RUST"

# Validate configuration
create-tutorial versions my-tutorial --validate
```

**Version Resolution:**

The CLI merges global versions (`versions.yml` at repo root) with tutorial-specific versions (`tutorials/<slug>/versions.yml`). Tutorial versions override global versions on a per-key basis.

Example:
```yaml
# Global versions.yml
versions:
  rust: "1.86"
  polkadot_omni_node: "0.5.0"

# Tutorial versions.yml
versions:
  polkadot_omni_node: "0.6.0"  # Override

# Resolved: rust=1.86, polkadot_omni_node=0.6.0
```

## Common Workflows

### Contributing a Tutorial

```bash
# 1. Create tutorial structure
create-tutorial create my-awesome-tutorial

# 2. Write content
cd tutorials/my-awesome-tutorial
code README.md

# 3. Implement code
code src/lib.rs

# 4. Write tests
code tests/tutorial.test.ts

# 5. Test locally
npm test

# 6. Commit and push
git add -A
git commit -m "feat(tutorial): add my awesome tutorial"
git push origin tutorial/my-awesome-tutorial
```

### Testing with Custom Versions

```bash
# Create tutorial
create-tutorial create test-new-version

# Edit versions
cd tutorials/test-new-version
cat > versions.yml <<EOF
versions:
  polkadot_omni_node: "0.7.0"
metadata:
  schema_version: "1.0"
EOF

# Verify resolution
create-tutorial versions test-new-version --show-source

# Test
npm test
```

## Configuration

### Global Versions

Edit `versions.yml` at repository root to change default versions for all tutorials.

### Tutorial Versions

Each tutorial can override versions by editing its `tutorials/<slug>/versions.yml` file.

### Tutorial Metadata

Edit `tutorials/<slug>/tutorial.config.yml` to configure:
- Tutorial name and description
- Category (polkadot-sdk-cookbook or contracts-cookbook)
- Whether a node is required (`needs_node`)
- Build and runtime settings

## Troubleshooting

### "Invalid working directory"

**Problem:** Running command from wrong directory

**Solution:** Run from repository root

```bash
cd /path/to/polkadot-cookbook
create-tutorial create my-tutorial
```

### "Slug argument is required"

**Problem:** Non-interactive mode without slug

**Solution:** Provide slug argument

```bash
create-tutorial create my-tutorial --non-interactive
```

### "Failed to resolve versions"

**Problem:** Invalid YAML syntax in versions.yml

**Solution:** Validate YAML

```bash
# Check syntax
yq eval tutorials/my-tutorial/versions.yml

# Use validation flag
create-tutorial versions my-tutorial --validate
```

## Development

To contribute to the CLI itself, see the [Contributing Guide](../CONTRIBUTING.md) and [Development Guide](../docs/architecture.md).

## License

MIT OR Apache-2.0
