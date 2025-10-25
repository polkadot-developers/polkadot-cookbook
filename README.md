<div align="center">

# 🍳 Polkadot Cookbook

[![License](https://img.shields.io/badge/License-MIT%20%2F%20Apache%202.0-blue.svg)](LICENSE)
[![Kitchensink Parachain](https://github.com/polkadot-developers/polkadot-docs-tests/actions/workflows/build-kitchensink-parachain.yml/badge.svg)](https://github.com/polkadot-developers/polkadot-docs-tests/actions/workflows/build-kitchensink-parachain.yml)
[![Rust](https://img.shields.io/badge/rust-1.86%2B-orange.svg)](https://www.rust-lang.org/)
[![Node](https://img.shields.io/badge/node-20%2B-green.svg)](https://nodejs.org/)

**Practical, tested tutorials for Polkadot SDK development**

[**Browse Tutorials**](#-tutorials) • [**Contribute a Tutorial**](CONTRIBUTING.md) • [**Documentation**](#-documentation)

</div>

---

## 🍽️ Tutorials

| Tutorial | Description | Difficulty |
|----------|-------------|------------|
| [**Zero to Hero**](tutorials/zero-to-hero) | Build and deploy your first parachain | 🟢 Beginner |

> 💡 **Want to share your knowledge?** See [Contributing a Tutorial](CONTRIBUTING.md)

---

## 🚀 Quick Start

### Run a Tutorial

Each tutorial is self-contained with working code and tests:

```bash
git clone https://github.com/polkadot-developers/polkadot-cookbook.git
cd polkadot-cookbook/tutorials/zero-to-hero
npm install
npm test
```

### Contribute a Tutorial

```bash
# Build the CLI tool
cargo build --package polkadot-cookbook-cli --release

# Create your tutorial
./target/release/create-tutorial create my-awesome-tutorial

# Write, test, and submit
cd tutorials/my-awesome-tutorial
# ... edit README.md, implement code, write tests ...
npm test
git push
```

See [CONTRIBUTING.md](CONTRIBUTING.md) for the complete guide.

---

## 📚 Documentation

### For Tutorial Contributors
- [Contributing Guide](CONTRIBUTING.md) - How to create and submit tutorials

### For Tool Users
- [CLI Tool](polkadot-cookbook-cli/) - Command-line tool for creating tutorials
- [SDK Library](polkadot-cookbook-core/) - Programmatic API for tool developers

### For Maintainers
- [Architecture](docs/architecture.md) - System design and architecture
- [Testing](docs/testing.md) - Testing guide
- [Workflows](docs/workflows.md) - CI/CD and automation

---

## 🤝 Contributing

We welcome all contributions:

- **📖 Tutorial** - Share your Polkadot knowledge (most welcome!)
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

[Tutorials](#-tutorials) • [Contributing](CONTRIBUTING.md) • [Issues](https://github.com/polkadot-developers/polkadot-cookbook/issues)

</div>
