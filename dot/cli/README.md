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
dot create
```

This launches an interactive prompt that guides you through recipe creation.

### Create a Recipe (Non-Interactive)

```bash
dot create --title "My Awesome Recipe" --non-interactive
```

## Commands

### `create`

Create a new recipe with scaffolded structure.

**Usage:**
```bash
dot create [OPTIONS]
```

**Options:**
- `--title <TITLE>` - Recipe title (required in non-interactive mode)
- `--pathway <PATHWAY>` - Recipe pathway:
  - `runtime` - Polkadot SDK/FRAME pallet development
  - `contracts` - Solidity smart contracts
  - `basic-interaction` - Polkadot API interactions
  - `xcm` - Cross-chain messaging
  - `testing` - Network infrastructure (Zombienet/Chopsticks)
- `--difficulty <DIFFICULTY>` - Difficulty level: beginner, intermediate, advanced
- `--content-type <TYPE>` - Content type: tutorial, guide
- `--skip-install` - Skip npm dependency installation
- `--no-git` - Skip git branch creation
- `--non-interactive` - Non-interactive mode (requires --title)

**Examples:**
```bash
# Interactive mode (recommended)
dot create

# Non-interactive with title (slug auto-generated)
dot create --title "Custom Pallet Recipe"

# Skip installation for faster creation
dot create --title "My Recipe" --skip-install --non-interactive

# CI/CD mode with full options
dot create --title "My Recipe" --pathway runtime --difficulty beginner --non-interactive --skip-install
```

**What it creates:**

The structure varies by pathway:

**Runtime:**
```
recipes/my-recipe/
├── README.md              # Recipe documentation
├── Cargo.toml             # Rust workspace
├── rust-toolchain.toml    # Rust version pinning
└── pallets/               # FRAME pallets
    └── template/
        ├── Cargo.toml
        └── src/
            ├── lib.rs     # Pallet implementation
            ├── mock.rs    # Test runtime
            └── tests.rs   # Unit tests
```

**Contracts/Basic Interactions/XCM/Infrastructure:**
```
recipes/my-recipe/
├── README.md              # Recipe documentation
├── package.json           # npm dependencies
├── tsconfig.json          # TypeScript config
├── vitest.config.ts       # Test config (or hardhat.config.ts)
├── src/                   # Implementation code
├── tests/                 # Test files
└── scripts/               # Deployment scripts (Contracts only)
```

## Common Workflows

### Contributing a Recipe

**Runtime pathway (Rust):**
```bash
# 1. Create recipe structure
dot create --title "Custom Storage Pallet" --pathway runtime

# 2. Write content
cd recipes/custom-storage-pallet
code README.md

# 3. Implement pallet
code pallets/template/src/lib.rs

# 4. Test locally
cargo test

# 5. Commit and push
git add -A
git commit -m "feat(recipe): add custom storage pallet"
git push origin recipe/custom-storage-pallet
```

**TypeScript pathways (Contracts/Basic Interactions/XCM/Infrastructure):**
```bash
# 1. Create recipe structure
dot create --title "Token Transfer" --pathway basic-interaction

# 2. Write content
cd recipes/token-transfer
code README.md

# 3. Implement code
code src/transfer.ts

# 4. Write tests
code tests/transfer.test.ts

# 5. Test locally
npm test

# 6. Commit and push
git add -A
git commit -m "feat(recipe): add token transfer example"
git push origin recipe/token-transfer
```

## Troubleshooting

### "Invalid working directory"

**Problem:** Running command from wrong directory

**Solution:** Run from repository root

```bash
cd /path/to/polkadot-cookbook
dot create --title "My Recipe"
```

### "Title argument is required"

**Problem:** Non-interactive mode without title

**Solution:** Provide --title argument

```bash
dot create --title "My Recipe" --non-interactive
```

## Development

To contribute to the CLI itself, see the [Contributing Guide](../CONTRIBUTING.md) and [Development Guide](../docs/architecture.md).

## License

MIT OR Apache-2.0
