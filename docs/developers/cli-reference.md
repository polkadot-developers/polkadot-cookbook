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

### `create`

Create a new project with scaffolded structure.

**Usage:**
```bash
dot create [OPTIONS]
```

**Interactive Mode (Recommended):**
```bash
dot create
```

Launches an interactive prompt that guides you through:
1. Pathway selection (Parachain, Smart Contract, Chain Transactions, Cross-Chain Transactions, Polkadot Networks)
2. Project title

**Non-Interactive Mode:**
```bash
dot create --title "My Project" --non-interactive [OPTIONS]
```

**Options:**

| Flag | Description | Required | Default |
|------|-------------|----------|---------|
| `--title <TITLE>` | Project title | Yes (non-interactive) | - |
| `--pathway <PATHWAY>` | Project pathway | No | parachain |
| `--skip-install` | Skip npm dependency installation | No | false |
| `--no-git` | Skip git branch creation | No | false |
| `--pallet-only` | Pallet-only mode (no runtime, no PAPI) | No | false |
| `--non-interactive` | Non-interactive mode | No | false |

**Pathway Options:**
- `pallets` - Parachain: Build a full parachain with custom pallets and PAPI integration
- `contracts` - Smart Contract: Build, test, and run Solidity smart contracts
- `transactions` - Chain Transactions: Single-chain transactions and state queries with PAPI
- `xcm` - Cross-Chain Transactions: Cross-chain asset transfers and cross-chain calls with Chopsticks
- `networks` - Polkadot Networks: Run a Polkadot network locally with Zombienet or Chopsticks

**Examples:**

```bash
# Interactive mode (recommended)
dot create

# Non-interactive parachain project
dot create --title "My Parachain" --pathway parachain --non-interactive

# Pallet-only mode (advanced, no runtime)
dot create --title "My Pallet" --pathway parachain --pallet-only --non-interactive

# Smart contracts recipe
dot create --title "My Contract" --pathway contracts --non-interactive

# Skip npm install for faster creation
dot create --title "Quick Test" --pathway basic-interaction --skip-install --non-interactive

# CI/CD mode (skip git branch creation)
dot create \
  --title "My Project" \
  --pathway parachain \
  --skip-install \
  --no-git \
  --non-interactive
```

**What it Creates:**

**Parachain Project:**
```
my-parachain/
├── README.md              # Tutorial documentation
├── Cargo.toml             # Workspace configuration
├── rust-toolchain.toml    # Rust version specification
├── package.json           # PAPI dependencies
├── pallets/               # Custom FRAME pallets
│   └── template/          # Template pallet
├── runtime/               # Parachain runtime
├── tests/                 # PAPI integration tests
├── scripts/               # Node management scripts
├── dev_chain_spec.json    # Development chain specification
├── zombienet.toml         # Parachain node network config
└── zombienet-omni-node.toml  # Omni-node network config
```

**Pallet-Only Project:**
```
my-pallet/
├── README.md              # Pallet documentation
├── Cargo.toml             # Minimal workspace
├── rust-toolchain.toml    # Rust version
└── pallets/               # Custom pallet only
    └── template/          # Template pallet with mock runtime
```

**Contracts Project:**
```
my-contract/
├── README.md              # Contract documentation
├── package.json           # Hardhat dependencies
├── hardhat.config.ts      # Hardhat configuration
├── contracts/             # Solidity contracts
├── tests/                 # Contract tests
└── scripts/               # Deployment scripts
```

Note: Structure varies by project pathway. See pathway-specific READMEs for details.

**Exit Codes:**
- `0` - Success
- `1` - Error (invalid input, file system error, etc.)

---

### `test`

Run tests for a project.

**Usage:**
```bash
dot test [PATH] [OPTIONS]
```

**Arguments:**
- `PATH` - Project path to test (defaults to current directory)

**Options:**
- `--rust` - Run only Rust tests (cargo test)
- `--ts` - Run only TypeScript tests (npm test)

**What it Does:**
1. Detects project type from `Cargo.toml` and/or `package.json`
2. By default, runs both test suites if project has both
3. Reports results for each test suite

**Examples:**
```bash
# Test current directory (runs both Rust and TS tests if applicable)
dot test

# Test specific project path
dot test my-parachain

# Run only Rust tests
dot test --rust

# Run only TypeScript tests
dot test --ts

# Combine path and flags
dot test my-parachain --rust
```

**Exit Codes:**
- `0` - All tests passed
- `1` - One or more tests failed

---

## Environment Variables

The CLI respects these environment variables:

| Variable | Description | Default |
|----------|-------------|---------|
| `RUST_LOG` | Logging level (error, warn, info, debug, trace) | error |
| `NO_COLOR` | Disable colored output | false |

**Examples:**

```bash
# Enable debug logging
RUST_LOG=debug dot create
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

**rust-toolchain.toml** (repository root and per recipe)
- Rust version specification


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

## Related Documentation

- [Getting Started](../getting-started/) - Installation and first recipe
- [Contributors Guide](../contributors/) - Contributing recipes
- [SDK Guide](sdk-guide.md) - Using the SDK programmatically

---

[← Back to Developers Guide](README.md)
