<div style="margin-bottom: 20px;">
  <img height="24px" alt="Polkadot" src="https://github.com/paritytech/polkadot-sdk/raw/master/docs/images/Polkadot_Logo_Horizontal_Pink_White.png#gh-dark-mode-only" />
  <img height="24px" alt="Polkadot" src="https://github.com/paritytech/polkadot-sdk/raw/master/docs/images/Polkadot_Logo_Horizontal_Pink_Black.png#gh-light-mode-only" />
</div>

<div align="center">

<div style="display: flex; align-items: center; justify-content: center; gap: 20px;">
  <img src=".github/media/dot-logo-dark.svg#gh-dark-mode-only" alt="Dot CLI" width="80" height="80" />
  <img src=".github/media/dot-logo-light.svg#gh-light-mode-only" alt="Dot CLI" width="80" height="80" />
  <div>
    <h1 style="font-size: 2.5em; font-weight: bold; margin: 0; line-height: 1;">Polkadot Cookbook</h1>
  </div>
</div>

**Practical, tested recipes for building on Polkadot**

Build runtime logic, smart contracts, dApps, and cross-chain applications with working code examples.

[**Browse Recipes**](#recipes) • [**Contribute a Recipe**](CONTRIBUTING.md) • [**Documentation**](#documentation)

<br/>

[![License](https://img.shields.io/badge/License-MIT%20%2F%20Apache%202.0-11116B.svg)](LICENSE)
[![Polkadot Cookbook SDK](https://img.shields.io/github/actions/workflow/status/polkadot-developers/polkadot-cookbook/test-sdk.yml?label=Polkadot%20Cookbook%20SDK&color=E6007A)](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/test-sdk.yml)
[![CLI](https://img.shields.io/badge/CLI-dot%20v0.5.0-E6007A?logo=rust&logoColor=white)](dot/cli/)

</div>

<hr />

<a id="recipes"></a>

## <img src=".github/media/icons/recipes-dark.svg#gh-dark-mode-only" width="20" height="20" alt="" /> <img src=".github/media/icons/recipes-light.svg#gh-light-mode-only" width="20" height="20" alt="" /> Recipes

The Polkadot Cookbook provides recipes across 5 pathways of Polkadot development:

> **How it works:** Each recipe's source code lives in its own **external GitHub repository**. The `recipes/` directory here contains **test harnesses** that automatically clone, build, and verify each recipe.

### <img src=".github/media/icons/runtime-dark.svg#gh-dark-mode-only" width="20" height="20" alt="" /> <img src=".github/media/icons/runtime-light.svg#gh-light-mode-only" width="20" height="20" alt="" /> Pallets

Build custom parachains with FRAME pallets, runtime logic, and PAPI integration testing.

| Recipe | Description |
|----------|-------------|
| [**parachain-example**](recipes/parachains/parachain-example) | Verifies [recipe-parachain-example](https://github.com/brunopgalvao/recipe-parachain-example) -- full parachain with 12+ pallets, custom logic, and TypeScript tests |
| [**pallet-example**](recipes/pallets/pallet-example) | Verifies [recipe-pallet-example](https://github.com/brunopgalvao/recipe-pallet-example) -- pallet-only development (no runtime) for advanced users |

### <img src=".github/media/icons/contracts-dark.svg#gh-dark-mode-only" width="20" height="20" alt="" /> <img src=".github/media/icons/contracts-light.svg#gh-light-mode-only" width="20" height="20" alt="" /> Contracts

Deploy and interact with Solidity smart contracts on Polkadot parachains.

| Recipe | Description |
|----------|-------------|
| [**contracts-example**](recipes/contracts/contracts-example) | Verifies [recipe-contracts-example](https://github.com/brunopgalvao/recipe-contracts-example) -- Solidity contracts with Hardhat and deployment scripts |

### <img src=".github/media/icons/interactions-dark.svg#gh-dark-mode-only" width="20" height="20" alt="" /> <img src=".github/media/icons/interactions-light.svg#gh-light-mode-only" width="20" height="20" alt="" /> Transactions

Single-chain transaction submission and state queries using the Polkadot API.

| Recipe | Description |
|----------|-------------|
| [**transaction-example**](recipes/transactions/transaction-example) | Verifies [recipe-transaction-example](https://github.com/brunopgalvao/recipe-transaction-example) -- chain interactions with Polkadot API (TypeScript) |

### <img src=".github/media/icons/xcm-dark.svg#gh-dark-mode-only" width="20" height="20" alt="" /> <img src=".github/media/icons/xcm-light.svg#gh-light-mode-only" width="20" height="20" alt="" /> XCM

Cross-chain asset transfers and messaging between parachains.

| Recipe | Description |
|----------|-------------|
| [**cross-chain-transaction-example**](recipes/cross-chain-transactions/cross-chain-transaction-example) | Verifies [recipe-xcm-example](https://github.com/brunopgalvao/recipe-xcm-example) -- XCM messaging with Chopsticks local testing |

### <img src=".github/media/icons/testing-dark.svg#gh-dark-mode-only" width="20" height="20" alt="" /> <img src=".github/media/icons/testing-light.svg#gh-light-mode-only" width="20" height="20" alt="" /> Networks

Testing infrastructure with Zombienet and Chopsticks for local network development.

| Recipe | Description |
|----------|-------------|
| [**network-example**](recipes/networks/network-example) | Verifies [recipe-network-example](https://github.com/brunopgalvao/recipe-network-example) -- Zombienet and Chopsticks network configurations |

> <img src=".github/media/icons/idea-dark.svg#gh-dark-mode-only" width="18" height="18" alt="" /> <img src=".github/media/icons/idea-light.svg#gh-light-mode-only" width="18" height="18" alt="" /> **Want to share your knowledge?** See [Contributing a Recipe](CONTRIBUTING.md)

<hr />

<a id="quick-start"></a>

## <img src=".github/media/icons/rocket-dark.svg#gh-dark-mode-only" width="20" height="20" alt="" /> <img src=".github/media/icons/rocket-light.svg#gh-light-mode-only" width="20" height="20" alt="" /> Quick Start

### Run a Recipe

Each recipe directory under `recipes/` is a **test harness** that clones and verifies an external repository. To run any recipe:

```bash
cd recipes/{pathway}/{recipe-name}
npm ci          # Install test harness dependencies
npm test        # Clone external repo → install → build → test
```

For example:
```bash
cd recipes/contracts/contracts-example
npm ci && npm test
```

The test harness automatically:
1. Clones the recipe's external GitHub repository at a pinned version
2. Installs the recipe's dependencies
3. Builds the project
4. Runs the recipe's test suite

> **Tip:** Check each recipe's README.md for a link to the source repository and details on what the test verifies.

### Contribute a Recipe

Contributing a recipe is a two-part process: develop your recipe in your own repository, then add a test harness to the cookbook.

#### 1. Develop Your Recipe

Use the `dot` CLI to scaffold a new project locally:

```bash
# Install the CLI
curl -fsSL https://raw.githubusercontent.com/polkadot-developers/polkadot-cookbook/master/install.sh | bash

# Create a new project (interactive mode)
dot create
```

<a id="advanced-installation"></a>
<details>
<summary><b>Advanced Installation Options</b></summary>

<br/>

**Manual download (all platforms):**

Download pre-built binaries from [GitHub Releases](https://github.com/polkadot-developers/polkadot-cookbook/releases/latest):

```bash
# Linux (x86_64)
curl -L https://github.com/polkadot-developers/polkadot-cookbook/releases/latest/download/dot-linux-amd64.tar.gz | tar xz
sudo mv dot /usr/local/bin/

# macOS (Apple Silicon)
curl -L https://github.com/polkadot-developers/polkadot-cookbook/releases/latest/download/dot-macos-apple-silicon.tar.gz | tar xz
sudo mv dot /usr/local/bin/

# macOS (Intel)
curl -L https://github.com/polkadot-developers/polkadot-cookbook/releases/latest/download/dot-macos-intel.tar.gz | tar xz
sudo mv dot /usr/local/bin/

# Windows
# Download dot-windows-amd64.exe.zip from releases, extract, and add to PATH
```

**Build from source:**

Requires [Rust](https://rustup.rs/) installed.

```bash
git clone https://github.com/polkadot-developers/polkadot-cookbook.git
cd polkadot-cookbook
cargo build --release --bin dot
# Binary will be at ./target/release/dot (or dot.exe on Windows)
```

</details>

Develop and test your project, then push it to your own GitHub repository and tag a release (e.g., `v1.0.0`).

#### 2. Add a Test Harness

Fork the cookbook and add a test harness under `recipes/{pathway}/{your-recipe}/` that clones, builds, and tests your external repo. See [recipes/contracts/contracts-example/](recipes/contracts/contracts-example/) for a complete example.

#### 3. Open a PR

Run your test harness locally (`npm ci && npm test`), then open a pull request.

See [CONTRIBUTING.md](CONTRIBUTING.md) for the complete guide.

<hr />

<a id="documentation"></a>

## <img src=".github/media/icons/docs-dark.svg#gh-dark-mode-only" width="20" height="20" alt="" /> <img src=".github/media/icons/docs-light.svg#gh-light-mode-only" width="20" height="20" alt="" /> Documentation

### For Recipe Contributors
- [Contributing Guide](CONTRIBUTING.md) - How to create and submit recipes
- [CLI Tool](dot/cli/) - Command-line tool for creating projects

### For Developers
- [SDK Library](dot/sdk/) - Programmatic API for tool developers

### For Maintainers
- **Template Integration Tests** - Validate that `dot create` templates generate correctly:
  ```bash
  cargo test --package cli --test pathway_integration_tests -- --ignored
  ```
  These tests verify the CLI's project scaffolding, not the recipe test harnesses.
- Integration tests run automatically on:
  - Releases
  - Weekly schedule
  - PRs touching `rust-toolchain.toml` or templates

<hr />

<a id="contributing"></a>

## <img src=".github/media/icons/contributing-dark.svg#gh-dark-mode-only" width="20" height="20" alt="" /> <img src=".github/media/icons/contributing-light.svg#gh-light-mode-only" width="20" height="20" alt="" /> Contributing

We welcome all contributions:

- **<img src=".github/media/icons/book-dark.svg#gh-dark-mode-only" width="16" height="16" alt="" /> <img src=".github/media/icons/book-light.svg#gh-light-mode-only" width="16" height="16" alt="" /> Recipe** - Share your Polkadot knowledge (most welcome!)
- **<img src=".github/media/icons/bug-dark.svg#gh-dark-mode-only" width="16" height="16" alt="" /> <img src=".github/media/icons/bug-light.svg#gh-light-mode-only" width="16" height="16" alt="" /> Bug Report** - Help us improve
- **<img src=".github/media/icons/idea-dark.svg#gh-dark-mode-only" width="16" height="16" alt="" /> <img src=".github/media/icons/idea-light.svg#gh-light-mode-only" width="16" height="16" alt="" /> Feature** - Suggest tooling improvements
- **<img src=".github/media/icons/memo-dark.svg#gh-dark-mode-only" width="16" height="16" alt="" /> <img src=".github/media/icons/memo-light.svg#gh-light-mode-only" width="16" height="16" alt="" /> Documentation** - Make things clearer

See [CONTRIBUTING.md](CONTRIBUTING.md) to get started.

<hr />

<a id="license"></a>

## <img src=".github/media/icons/contracts-dark.svg#gh-dark-mode-only" width="20" height="20" alt="" /> <img src=".github/media/icons/contracts-light.svg#gh-light-mode-only" width="20" height="20" alt="" /> License

MIT OR Apache-2.0

<hr />

<div align="center">

<img src=".github/media/dot-logo-dark.svg#gh-dark-mode-only" alt="Dot CLI" width="40" height="40" />
<img src=".github/media/dot-logo-light.svg#gh-light-mode-only" alt="Dot CLI" width="40" height="40" />

<br/>

Built by [Polkadot Developers](https://github.com/polkadot-developers)

[Recipes](#recipes) • [Contributing](CONTRIBUTING.md) • [Issues](https://github.com/polkadot-developers/polkadot-cookbook/issues)

</div>
