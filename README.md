<div style="margin-bottom: 20px;">
  <img height="24px" alt="Polkadot" src="https://github.com/paritytech/polkadot-sdk/raw/master/docs/images/Polkadot_Logo_Horizontal_Pink_White.png#gh-dark-mode-only" />
  <img height="24px" alt="Polkadot" src="https://github.com/paritytech/polkadot-sdk/raw/master/docs/images/Polkadot_Logo_Horizontal_Pink_Black.png#gh-light-mode-only" />
</div>

<div align="center">

<div style="display: flex; align-items: center; justify-content: center; gap: 20px;">
  <img src=".media/dot-logo-dark.svg#gh-dark-mode-only" alt="Dot CLI" width="80" height="80" />
  <img src=".media/dot-logo-light.svg#gh-light-mode-only" alt="Dot CLI" width="80" height="80" />
  <div>
    <h1 style="font-size: 2.5em; font-weight: bold; margin: 0; line-height: 1;">Polkadot Cookbook</h1>
  </div>
</div>

**Practical, tested recipes for Polkadot SDK development**

[**Browse Recipes**](#recipes) • [**Contribute a Recipe**](CONTRIBUTING.md) • [**Documentation**](#documentation)

<br/>

[![License](https://img.shields.io/badge/License-MIT%20%2F%20Apache%202.0-11116B.svg)](LICENSE)
[![Polkadot Cookbook SDK](https://img.shields.io/github/actions/workflow/status/polkadot-developers/polkadot-cookbook/test-sdk.yml?label=Polkadot%20Cookbook%20SDK&color=E6007A)](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/test-sdk.yml)
[![CLI](https://img.shields.io/badge/CLI-dot%20v0.1.0-E6007A?logo=rust&logoColor=white)](cli/)
[![Rust](https://img.shields.io/badge/dynamic/yaml?url=https://raw.githubusercontent.com/polkadot-developers/polkadot-cookbook/master/versions.yml&query=$.versions.rust&prefix=v&label=rust&color=11116B)](https://www.rust-lang.org/)

</div>

<hr />

<a id="recipes"></a>

## <img src=".media/icons/recipes-dark.svg#gh-dark-mode-only" width="20" height="20" alt="" /> <img src=".media/icons/recipes-light.svg#gh-light-mode-only" width="20" height="20" alt="" /> Recipes

The Polkadot Cookbook provides recipes across 5 pathways of Polkadot development:

### <img src=".media/icons/runtime-dark.svg#gh-dark-mode-only" width="20" height="20" alt="" /> <img src=".media/icons/runtime-light.svg#gh-light-mode-only" width="20" height="20" alt="" /> Runtime Development (Polkadot SDK)

Build custom FRAME pallets and runtime logic with Rust.

| Recipe | Description | Difficulty |
|----------|-------------|------------|
| [**Basic Pallet**](recipes/basic-pallet) | Create a custom FRAME pallet with storage and events | <img src=".media/icons/beginner-dark.svg#gh-dark-mode-only" width="14" height="14" alt="" /> <img src=".media/icons/beginner-light.svg#gh-light-mode-only" width="14" height="14" alt="" /> Beginner |

### <img src=".media/icons/contracts-dark.svg#gh-dark-mode-only" width="20" height="20" alt="" /> <img src=".media/icons/contracts-light.svg#gh-light-mode-only" width="20" height="20" alt="" /> Smart Contracts (Solidity)

Deploy and interact with Solidity contracts.

| Recipe | Description | Difficulty |
|----------|-------------|------------|
| [**Simple Counter**](recipes/simple-counter) | A simple counter smart contract in Solidity | <img src=".media/icons/beginner-dark.svg#gh-dark-mode-only" width="14" height="14" alt="" /> <img src=".media/icons/beginner-light.svg#gh-light-mode-only" width="14" height="14" alt="" /> Beginner |

### <img src=".media/icons/interactions-dark.svg#gh-dark-mode-only" width="20" height="20" alt="" /> <img src=".media/icons/interactions-light.svg#gh-light-mode-only" width="20" height="20" alt="" /> Basic Interactions

Single-chain transaction submission and state queries with PAPI.

| Recipe | Description | Difficulty |
|----------|-------------|------------|
| *Coming soon* | Balance transfers, batch operations, proxy calls | - |

### <img src=".media/icons/xcm-dark.svg#gh-dark-mode-only" width="20" height="20" alt="" /> <img src=".media/icons/xcm-light.svg#gh-light-mode-only" width="20" height="20" alt="" /> XCM (Cross-Chain Messaging)

Asset transfers and cross-chain communication with Chopsticks.

| Recipe | Description | Difficulty |
|----------|-------------|------------|
| [**Teleport Assets**](recipes/teleport-assets) | Teleport assets between parachains using XCM v5 and PAPI | <img src=".media/icons/beginner-dark.svg#gh-dark-mode-only" width="14" height="14" alt="" /> <img src=".media/icons/beginner-light.svg#gh-light-mode-only" width="14" height="14" alt="" /> Beginner |

### <img src=".media/icons/testing-dark.svg#gh-dark-mode-only" width="20" height="20" alt="" /> <img src=".media/icons/testing-light.svg#gh-light-mode-only" width="20" height="20" alt="" /> Testing Infrastructure

Zombienet and Chopsticks configurations for network testing.

| Recipe | Description | Difficulty |
|----------|-------------|------------|
| *Coming soon* | Parachain test network, fork testing setups | - |

> <img src=".media/icons/idea-dark.svg#gh-dark-mode-only" width="18" height="18" alt="" /> <img src=".media/icons/idea-light.svg#gh-light-mode-only" width="18" height="18" alt="" /> **Want to share your knowledge?** See [Contributing a Recipe](CONTRIBUTING.md)

<hr />

<a id="quick-start"></a>

## <img src=".media/icons/rocket-dark.svg#gh-dark-mode-only" width="20" height="20" alt="" /> <img src=".media/icons/rocket-light.svg#gh-light-mode-only" width="20" height="20" alt="" /> Quick Start

### Run a Recipe

Each recipe is self-contained with working code and tests:

```bash
git clone https://github.com/polkadot-developers/polkadot-cookbook.git
cd polkadot-cookbook/recipes/basic-pallet

# Run tests
cargo test

# Build the pallet
cargo build --release
```

### Contribute a Recipe

#### Install the CLI Tool

**Download pre-built binary (Recommended):**

```bash
# Linux (x86_64)
curl -L https://github.com/polkadot-developers/polkadot-cookbook/releases/latest/download/dot-linux-amd64.tar.gz | tar xz
sudo mv dot /usr/local/bin/

# macOS (Apple Silicon)
curl -L https://github.com/polkadot-developers/polkadot-cookbook/releases/latest/download/dot-macos-apple-silicon.tar.gz | tar xz
sudo mv dot /usr/local/bin/
```

**Or build from source:**

```bash
cargo build --release --bin dot
# Binary will be at ./target/release/dot
```

#### Use the CLI

```bash
# Setup and verify your environment
dot setup
dot doctor

# Create a new recipe (interactive mode)
dot recipe create

# Test your recipe
dot recipe test my-pallet

# Validate recipe structure
dot recipe validate my-pallet

# Run linting checks
dot recipe lint my-pallet

# List all recipes
dot recipe list

# Submit your recipe as a pull request
dot recipe submit my-pallet
```

The CLI supports five recipe pathways:
- **Runtime Development** - Build custom FRAME pallets with Rust
- **Smart Contracts** - Deploy Solidity contracts
- **Basic Interactions** - Single-chain transactions with PAPI (TypeScript)
- **XCM** - Cross-chain messaging with Chopsticks (TypeScript)
- **Testing Infrastructure** - Zombienet and Chopsticks configurations

**Interactive CLI Features:**
- <img src=".media/icons/target-dark.svg#gh-dark-mode-only" width="16" height="16" alt="" /> <img src=".media/icons/target-light.svg#gh-light-mode-only" width="16" height="16" alt="" /> Title-first workflow (auto-generates slugs)
- <img src=".media/icons/chart-dark.svg#gh-dark-mode-only" width="16" height="16" alt="" /> <img src=".media/icons/chart-light.svg#gh-light-mode-only" width="16" height="16" alt="" /> Difficulty levels (Beginner/Intermediate/Advanced)
- <img src=".media/icons/docs-dark.svg#gh-dark-mode-only" width="16" height="16" alt="" /> <img src=".media/icons/docs-light.svg#gh-light-mode-only" width="16" height="16" alt="" /> Content types (Tutorial/Guide)
- <img src=".media/icons/refresh-dark.svg#gh-dark-mode-only" width="16" height="16" alt="" /> <img src=".media/icons/refresh-light.svg#gh-light-mode-only" width="16" height="16" alt="" /> Automatic git branch creation
- <img src=".media/icons/package-dark.svg#gh-dark-mode-only" width="16" height="16" alt="" /> <img src=".media/icons/package-light.svg#gh-light-mode-only" width="16" height="16" alt="" /> Automatic dependency installation

**New:** No proposal required! Submit recipes directly via PR using the `dot recipe submit` command.

See [CONTRIBUTING.md](CONTRIBUTING.md) for the complete guide.

<hr />

<a id="documentation"></a>

## <img src=".media/icons/docs-dark.svg#gh-dark-mode-only" width="20" height="20" alt="" /> <img src=".media/icons/docs-light.svg#gh-light-mode-only" width="20" height="20" alt="" /> Documentation

### For Recipe Contributors
- [Contributing Guide](CONTRIBUTING.md) - How to create and submit recipes

### For Tool Users
- [CLI Tool](cli/) - Command-line tool for creating recipes
- [SDK Library](core/) - Programmatic API for tool developers

### For Maintainers
- [Architecture](docs/architecture.md) - System design and architecture
- [Testing](docs/testing.md) - Testing guide
- [Workflows](docs/workflows.md) - CI/CD and automation

<hr />

<a id="contributing"></a>

## <img src=".media/icons/contributing-dark.svg#gh-dark-mode-only" width="20" height="20" alt="" /> <img src=".media/icons/contributing-light.svg#gh-light-mode-only" width="20" height="20" alt="" /> Contributing

We welcome all contributions:

- **<img src=".media/icons/book-dark.svg#gh-dark-mode-only" width="16" height="16" alt="" /> <img src=".media/icons/book-light.svg#gh-light-mode-only" width="16" height="16" alt="" /> Recipe** - Share your Polkadot knowledge (most welcome!)
- **<img src=".media/icons/bug-dark.svg#gh-dark-mode-only" width="16" height="16" alt="" /> <img src=".media/icons/bug-light.svg#gh-light-mode-only" width="16" height="16" alt="" /> Bug Report** - Help us improve
- **<img src=".media/icons/idea-dark.svg#gh-dark-mode-only" width="16" height="16" alt="" /> <img src=".media/icons/idea-light.svg#gh-light-mode-only" width="16" height="16" alt="" /> Feature** - Suggest tooling improvements
- **<img src=".media/icons/memo-dark.svg#gh-dark-mode-only" width="16" height="16" alt="" /> <img src=".media/icons/memo-light.svg#gh-light-mode-only" width="16" height="16" alt="" /> Documentation** - Make things clearer

See [CONTRIBUTING.md](CONTRIBUTING.md) to get started.

<hr />

<a id="license"></a>

## <img src=".media/icons/contracts-dark.svg#gh-dark-mode-only" width="20" height="20" alt="" /> <img src=".media/icons/contracts-light.svg#gh-light-mode-only" width="20" height="20" alt="" /> License

MIT OR Apache-2.0

<hr />

<div align="center">

<img src=".media/dot-logo-dark.svg#gh-dark-mode-only" alt="Dot CLI" width="40" height="40" />
<img src=".media/dot-logo-light.svg#gh-light-mode-only" alt="Dot CLI" width="40" height="40" />

<br/>

Built by [Polkadot Developers](https://github.com/polkadot-developers)

[Recipes](#recipes) • [Contributing](CONTRIBUTING.md) • [Issues](https://github.com/polkadot-developers/polkadot-cookbook/issues)

</div>
