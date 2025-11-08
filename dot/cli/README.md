# Polkadot Cookbook CLI

Command-line tool for creating and managing Polkadot Cookbook recipes.

## Installation

### From Source

```bash
git clone https://github.com/polkadot-developers/polkadot-cookbook.git
cd polkadot-cookbook
cargo build --release
```

The binary will be available at `target/release/dot`.

> **Note:** The workspace is configured with `cli` as the default member, so `cargo build` automatically builds the CLI.

### Add to PATH (Optional)

```bash
# Add to your shell profile (.bashrc, .zshrc, etc.)
export PATH="$PATH:/path/to/polkadot-cookbook/target/release"
```

## Quick Start

### Create a Recipe (Interactive)

```bash
dot recipe create
```

This launches an interactive prompt that guides you through recipe creation.

### Create a Recipe (Non-Interactive)

```bash
dot recipe create --title "My Awesome Recipe" --non-interactive
```

## Commands

### `recipe create`

Create a new recipe with scaffolded structure.

**Usage:**
```bash
dot recipe create [OPTIONS]
```

**Options:**
- `--title <TITLE>` - Recipe title (required in non-interactive mode)
- `--pathway <PATHWAY>` - Recipe pathway: runtime, contracts, basic-interaction, xcm, testing
- `--difficulty <DIFFICULTY>` - Difficulty level: beginner, intermediate, advanced
- `--content-type <TYPE>` - Content type: tutorial, guide
- `--skip-install` - Skip npm dependency installation
- `--no-git` - Skip git branch creation
- `--non-interactive` - Non-interactive mode (requires --title)

**Examples:**
```bash
# Interactive mode (recommended)
dot recipe create

# Non-interactive with title (slug auto-generated)
dot recipe create --title "Custom Pallet Recipe"

# Skip installation for faster creation
dot recipe create --title "My Recipe" --skip-install --non-interactive

# CI/CD mode with full options
dot recipe create --title "My Recipe" --pathway runtime --difficulty beginner --non-interactive --skip-install
```

**What it creates:**
```
recipes/my-recipe/
├── README.md              # Recipe content
├── recipe.config.yml    # Metadata
├── package.json           # npm dependencies (TypeScript recipes)
├── tsconfig.json          # TypeScript config
├── vitest.config.ts       # Test config
├── src/                   # Implementation code
├── tests/                 # Test files
└── scripts/               # Deployment scripts (Solidity recipes)
```
Note: Structure varies by recipe type (Runtime, Solidity, XCM, etc.).

## Common Workflows

### Contributing a Recipe

```bash
# 1. Create recipe structure
dot recipe create --title "My Awesome Recipe"

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

## Configuration

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
dot recipe create --title "My Recipe"
```

### "Title argument is required"

**Problem:** Non-interactive mode without title

**Solution:** Provide --title argument

```bash
dot recipe create --title "My Recipe" --non-interactive
```

## Development

To contribute to the CLI itself, see the [Contributing Guide](../CONTRIBUTING.md) and [Development Guide](../docs/architecture.md).

## License

MIT OR Apache-2.0
