<div align="center">

<div style="display: flex; align-items: center; justify-content: center; gap: 20px;">
  <img src="assets/polkadot-chef.svg" alt="Polkadot Chef" width="80" height="80" />
  <div>
    <div style="font-size: 2.5em; font-weight: bold; margin: 0; line-height: 1;">
      <img height="50px" alt="Polkadot" src="https://github.com/paritytech/polkadot-sdk/raw/master/docs/images/Polkadot_Logo_Horizontal_Pink_White.png#gh-dark-mode-only" style="vertical-align: middle;" />
      <img height="50px" alt="Polkadot" src="https://github.com/paritytech/polkadot-sdk/raw/master/docs/images/Polkadot_Logo_Horizontal_Pink_Black.png#gh-light-mode-only" style="vertical-align: middle;" />
    </div>
    <div style="font-size: 2.5em; font-weight: bold; margin: 0; line-height: 1;">
      <h1>Polkadot Cookbook</h1>
    </div>
  </div>
</div>

**Practical, tested recipes for Polkadot SDK development**

[**Browse Recipes**](#-recipes) • [**Contribute a Recipe**](CONTRIBUTING.md) • [**Documentation**](#-documentation)

<br/>

[![License](https://img.shields.io/badge/License-MIT%20%2F%20Apache%202.0-blue.svg)](LICENSE)
[![Test SDK](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/test-sdk.yml/badge.svg)](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/test-sdk.yml)
[![Rust](https://img.shields.io/badge/rust-1.81%2B-orange.svg)](https://www.rust-lang.org/)

</div>

---

## 🍽️ Recipes

### Polkadot SDK

| Recipe | Description | Difficulty |
|----------|-------------|------------|
| [**Basic Pallet**](recipes/basic-pallet) | Create a custom FRAME pallet with storage and events | 🟢 Beginner |

### XCM

| Recipe | Description | Difficulty |
|----------|-------------|------------|
| [**Teleport Assets**](recipes/teleport-assets) | Teleport assets between parachains using XCM v5 and PAPI | 🟢 Beginner |

### Coming Soon

- **Solidity** - Smart contracts using pallet-revive

> 💡 **Want to share your knowledge?** See [Contributing a Recipe](CONTRIBUTING.md)

---

## 🚀 Quick Start

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
```

The CLI supports three recipe types:
- **Polkadot SDK** - Runtime pallets with Rust ✅
- **XCM** - Cross-chain interactions with Chopsticks ✅
- **Solidity** - Smart contracts with pallet-revive ✅

See [CONTRIBUTING.md](CONTRIBUTING.md) for the complete guide.

---

## 📚 Documentation

### For Recipe Contributors
- [Contributing Guide](CONTRIBUTING.md) - How to create and submit recipes

### For Tool Users
- [CLI Tool](cli/) - Command-line tool for creating recipes
- [SDK Library](core/) - Programmatic API for tool developers

### For Maintainers
- [Architecture](docs/architecture.md) - System design and architecture
- [Testing](docs/testing.md) - Testing guide
- [Workflows](docs/workflows.md) - CI/CD and automation

---

## 🤝 Contributing

We welcome all contributions:

- **📖 Recipe** - Share your Polkadot knowledge (most welcome!)
- **🐛 Bug Report** - Help us improve
- **💡 Feature** - Suggest tooling improvements
- **📝 Documentation** - Make things clearer

See [CONTRIBUTING.md](CONTRIBUTING.md) to get started.

---

## 📜 License

MIT OR Apache-2.0

---

<div align="center">

Built by [Polkadot Developers](https://github.com/polkadot-developers)

[Recipes](#-recipes) • [Contributing](CONTRIBUTING.md) • [Issues](https://github.com/polkadot-developers/polkadot-cookbook/issues)

</div>
