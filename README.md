<style>
  @keyframes fadeInFull {
    0% { opacity: 0.7; }
    100% { opacity: 1; }
  }
  @keyframes fadeIn {
    0% { opacity: 0.9; }
    100% { opacity: 1; }
  }
  .polkadot-logo-fade {
    opacity: 0.7;
    animation: fadeInFull 60s ease-in 5s forwards;
  }
  .cookbook-fade-in {
    animation: fadeIn 30s ease-in forwards;
  }
  @keyframes gradientFlow {
    0% {
      background-position: 0% 50%;
    }
    50% {
      background-position: 100% 50%;
    }
    100% {
      background-position: 0% 50%;
    }
  }
  @keyframes textShine {
    0% {
      background-position: -200% center;
    }
    100% {
      background-position: 200% center;
    }
  }
  @keyframes breathe {
    0%, 100% {
      transform: scale(1);
      opacity: 1;
    }
    50% {
      transform: scale(1.05);
      opacity: 0.9;
    }
  }
  @keyframes float {
    0%, 100% {
      transform: translateY(0px);
    }
    50% {
      transform: translateY(-2px);
    }
  }
  .polkadot-divider {
    height: 3px;
    background: linear-gradient(90deg, #E6007A 0%, #11116B 25%, #E6007A 50%, #11116B 75%, #E6007A 100%);
    background-size: 200% 100%;
    border: none;
    margin: 40px 0;
    opacity: 0.6;
    animation: gradientFlow 8s ease-in-out infinite;
  }
  .text-shine {
    animation: breathe 8s ease-in-out infinite;
  }
  .heading-shine {
    animation: float 6s ease-in-out infinite;
  }
</style>

<div style="margin-bottom: 20px;">
  <img class="polkadot-logo-fade" height="24px" alt="Polkadot" src="https://github.com/paritytech/polkadot-sdk/raw/master/docs/images/Polkadot_Logo_Horizontal_Pink_White.png#gh-dark-mode-only" />
  <img class="polkadot-logo-fade" height="24px" alt="Polkadot" src="https://github.com/paritytech/polkadot-sdk/raw/master/docs/images/Polkadot_Logo_Horizontal_Pink_Black.png#gh-light-mode-only" />
</div>

<div align="center">

<div style="display: flex; align-items: center; justify-content: center; gap: 20px;">
  <img class="cookbook-fade-in" src=".media/dot-logo-dark.svg#gh-dark-mode-only" alt="Dot CLI" width="80" height="80" />
  <img class="cookbook-fade-in" src=".media/dot-logo-light.svg#gh-light-mode-only" alt="Dot CLI" width="80" height="80" />
  <div>
    <h1 class="cookbook-fade-in" style="font-size: 2.5em; font-weight: bold; margin: 0; line-height: 1;">Polkadot Cookbook</h1>
  </div>
</div>

**Practical, tested recipes for Polkadot SDK development**

[**Browse Recipes**](#-recipes) • [**Contribute a Recipe**](CONTRIBUTING.md) • [**Documentation**](#-documentation)

<br/>

[![License](https://img.shields.io/badge/License-MIT%20%2F%20Apache%202.0-11116B.svg)](LICENSE)
[![Polkadot Cookbook SDK](https://img.shields.io/github/actions/workflow/status/polkadot-developers/polkadot-cookbook/test-sdk.yml?label=Polkadot%20Cookbook%20SDK&color=E6007A)](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/test-sdk.yml)
[![CLI](https://img.shields.io/badge/CLI-dot%20v0.1.0-E6007A?logo=rust&logoColor=white)](cli/)
[![Rust](https://img.shields.io/badge/dynamic/yaml?url=https://raw.githubusercontent.com/polkadot-developers/polkadot-cookbook/master/versions.yml&query=$.versions.rust&prefix=v&label=rust&color=11116B)](https://www.rust-lang.org/)

</div>

<hr class="polkadot-divider" />

## <img src=".media/icons/recipes-dark.svg#gh-dark-mode-only" width="20" height="20" alt="" /> <img src=".media/icons/recipes-light.svg#gh-light-mode-only" width="20" height="20" alt="" /> <span class="heading-shine">Recipes</span>

The Polkadot Cookbook provides recipes across 5 pathways of Polkadot development:

### <img src=".media/icons/runtime-dark.svg#gh-dark-mode-only" width="20" height="20" alt="" /> <img src=".media/icons/runtime-light.svg#gh-light-mode-only" width="20" height="20" alt="" /> <span class="heading-shine">Runtime Development (Polkadot SDK)</span>

Build custom FRAME pallets and runtime logic with Rust.

| Recipe | Description | Difficulty |
|----------|-------------|------------|
| [**Basic Pallet**](recipes/basic-pallet) | Create a custom FRAME pallet with storage and events | <img src=".media/icons/beginner-dark.svg#gh-dark-mode-only" width="14" height="14" alt="" /> <img src=".media/icons/beginner-light.svg#gh-light-mode-only" width="14" height="14" alt="" /> Beginner |

### <img src=".media/icons/contracts-dark.svg#gh-dark-mode-only" width="20" height="20" alt="" /> <img src=".media/icons/contracts-light.svg#gh-light-mode-only" width="20" height="20" alt="" /> <span class="heading-shine">Smart Contracts (Solidity)</span>

Deploy and interact with Solidity contracts using pallet-revive.

| Recipe | Description | Difficulty |
|----------|-------------|------------|
| [**Simple Counter**](recipes/simple-counter) | A simple counter smart contract using pallet-revive | <img src=".media/icons/beginner-dark.svg#gh-dark-mode-only" width="14" height="14" alt="" /> <img src=".media/icons/beginner-light.svg#gh-light-mode-only" width="14" height="14" alt="" /> Beginner |

### <img src=".media/icons/interactions-dark.svg#gh-dark-mode-only" width="20" height="20" alt="" /> <img src=".media/icons/interactions-light.svg#gh-light-mode-only" width="20" height="20" alt="" /> <span class="heading-shine">Basic Interactions</span>

Single-chain transaction submission and state queries with PAPI.

| Recipe | Description | Difficulty |
|----------|-------------|------------|
| *Coming soon* | Balance transfers, batch operations, proxy calls | - |

### <img src=".media/icons/xcm-dark.svg#gh-dark-mode-only" width="20" height="20" alt="" /> <img src=".media/icons/xcm-light.svg#gh-light-mode-only" width="20" height="20" alt="" /> <span class="heading-shine">XCM (Cross-Chain Messaging)</span>

Asset transfers and cross-chain communication with Chopsticks.

| Recipe | Description | Difficulty |
|----------|-------------|------------|
| [**Teleport Assets**](recipes/teleport-assets) | Teleport assets between parachains using XCM v5 and PAPI | <img src=".media/icons/beginner-dark.svg#gh-dark-mode-only" width="14" height="14" alt="" /> <img src=".media/icons/beginner-light.svg#gh-light-mode-only" width="14" height="14" alt="" /> Beginner |

### <img src=".media/icons/testing-dark.svg#gh-dark-mode-only" width="20" height="20" alt="" /> <img src=".media/icons/testing-light.svg#gh-light-mode-only" width="20" height="20" alt="" /> <span class="heading-shine">Testing Infrastructure</span>

Zombienet and Chopsticks configurations for network testing.

| Recipe | Description | Difficulty |
|----------|-------------|------------|
| *Coming soon* | Parachain test network, fork testing setups | - |

> <img src=".media/icons/idea-dark.svg#gh-dark-mode-only" width="18" height="18" alt="" /> <img src=".media/icons/idea-light.svg#gh-light-mode-only" width="18" height="18" alt="" /> **Want to share your knowledge?** See [Contributing a Recipe](CONTRIBUTING.md)

<hr class="polkadot-divider" />

## <img src=".media/icons/rocket-dark.svg#gh-dark-mode-only" width="20" height="20" alt="" /> <img src=".media/icons/rocket-light.svg#gh-light-mode-only" width="20" height="20" alt="" /> <span class="heading-shine">Quick Start</span>

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

Use the CLI tool to create and manage recipes:

```bash
# Build the CLI tool
cargo build --release --bin dot

# Setup and verify your environment
./target/release/dot setup
./target/release/dot doctor

# Create a new recipe (interactive mode)
./target/release/dot

# Or create with a specific slug
./target/release/dot my-pallet

# Test your recipe
./target/release/dot recipe test my-pallet

# Validate recipe structure
./target/release/dot recipe validate my-pallet

# Run linting checks
./target/release/dot recipe lint my-pallet

# List all recipes
./target/release/dot recipe list

# Submit your recipe as a pull request
./target/release/dot recipe submit my-pallet
```

The CLI supports five recipe pathways:
- **Runtime Development** - Build custom FRAME pallets with Rust
- **Smart Contracts** - Deploy Solidity contracts with pallet-revive
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

<hr class="polkadot-divider" />

## <img src=".media/icons/docs-dark.svg#gh-dark-mode-only" width="20" height="20" alt="" /> <img src=".media/icons/docs-light.svg#gh-light-mode-only" width="20" height="20" alt="" /> <span class="heading-shine">Documentation</span>

### For Recipe Contributors
- [Contributing Guide](CONTRIBUTING.md) - How to create and submit recipes

### For Tool Users
- [CLI Tool](cli/) - Command-line tool for creating recipes
- [SDK Library](core/) - Programmatic API for tool developers

### For Maintainers
- [Architecture](docs/architecture.md) - System design and architecture
- [Testing](docs/testing.md) - Testing guide
- [Workflows](docs/workflows.md) - CI/CD and automation

<hr class="polkadot-divider" />

## <img src=".media/icons/contributing-dark.svg#gh-dark-mode-only" width="20" height="20" alt="" /> <img src=".media/icons/contributing-light.svg#gh-light-mode-only" width="20" height="20" alt="" /> <span class="heading-shine">Contributing</span>

We welcome all contributions:

- **<img src=".media/icons/book-dark.svg#gh-dark-mode-only" width="16" height="16" alt="" /> <img src=".media/icons/book-light.svg#gh-light-mode-only" width="16" height="16" alt="" /> Recipe** - Share your Polkadot knowledge (most welcome!)
- **<img src=".media/icons/bug-dark.svg#gh-dark-mode-only" width="16" height="16" alt="" /> <img src=".media/icons/bug-light.svg#gh-light-mode-only" width="16" height="16" alt="" /> Bug Report** - Help us improve
- **<img src=".media/icons/idea-dark.svg#gh-dark-mode-only" width="16" height="16" alt="" /> <img src=".media/icons/idea-light.svg#gh-light-mode-only" width="16" height="16" alt="" /> Feature** - Suggest tooling improvements
- **<img src=".media/icons/memo-dark.svg#gh-dark-mode-only" width="16" height="16" alt="" /> <img src=".media/icons/memo-light.svg#gh-light-mode-only" width="16" height="16" alt="" /> Documentation** - Make things clearer

See [CONTRIBUTING.md](CONTRIBUTING.md) to get started.

<hr class="polkadot-divider" />

## <img src=".media/icons/contracts-dark.svg#gh-dark-mode-only" width="20" height="20" alt="" /> <img src=".media/icons/contracts-light.svg#gh-light-mode-only" width="20" height="20" alt="" /> <span class="heading-shine">License</span>

MIT OR Apache-2.0

<hr class="polkadot-divider" />

<div align="center">

<img src=".media/dot-logo-dark.svg#gh-dark-mode-only" alt="Dot CLI" width="40" height="40" />
<img src=".media/dot-logo-light.svg#gh-light-mode-only" alt="Dot CLI" width="40" height="40" />

<br/>

Built by [Polkadot Developers](https://github.com/polkadot-developers)

[Recipes](#-recipes) • [Contributing](CONTRIBUTING.md) • [Issues](https://github.com/polkadot-developers/polkadot-cookbook/issues)

</div>
