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
[![Kitchensink Parachain](https://github.com/polkadot-developers/polkadot-docs-tests/actions/workflows/build-kitchensink-parachain.yml/badge.svg)](https://github.com/polkadot-developers/polkadot-docs-tests/actions/workflows/build-kitchensink-parachain.yml)
[![Rust](https://img.shields.io/badge/rust-1.86%2B-orange.svg)](https://www.rust-lang.org/)
[![Node](https://img.shields.io/badge/node-20%2B-green.svg)](https://nodejs.org/)

</div>

---

## 🍽️ Recipes

| Recipe | Description | Difficulty |
|----------|-------------|------------|
| [**Zero to Hero**](recipes/zero-to-hero) | Build and deploy your first parachain | 🟢 Beginner |

> 💡 **Want to share your knowledge?** See [Contributing a Recipe](CONTRIBUTING.md)

---

## 🚀 Quick Start

### Run a Recipe

Each recipe is self-contained with working code and tests:

```bash
git clone https://github.com/polkadot-developers/polkadot-cookbook.git
cd polkadot-cookbook/recipes/zero-to-hero
npm install
npm test
```

### Contribute a Recipe

```bash
# Build the CLI tool
cargo build --package cli --release

# Create your recipe
./target/release/dot my-awesome-recipe

# Write, test, and submit
cd recipes/my-awesome-recipe
# ... edit README.md, implement code, write tests ...
npm test
git push
```

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
