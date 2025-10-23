<div align="center">

# 🍳 Polkadot Cookbook

*Master the art of building on Polkadot, one recipe at a time.*

[![License](https://img.shields.io/badge/License-MIT%20%2F%20Apache%202.0-blue.svg)](LICENSE)
[![Kitchensink Parachain](https://github.com/polkadot-developers/polkadot-docs-tests/actions/workflows/build-kitchensink-parachain.yml/badge.svg)](https://github.com/polkadot-developers/polkadot-docs-tests/actions/workflows/build-kitchensink-parachain.yml)
[![Rust](https://img.shields.io/badge/rust-1.86%2B-orange.svg)](https://www.rust-lang.org/)
[![Node](https://img.shields.io/badge/node-20%2B-green.svg)](https://nodejs.org/)

[**Tutorials**](#-tutorials) • [**Getting Started**](#-getting-started) • [**Contributing**](CONTRIBUTING.md)

</div>

---

## 🎯 What is Polkadot Cookbook?

A curated collection of practical, battle-tested tutorials for developers building with the Polkadot SDK.

Each recipe is crafted with care:

- **Working code** - Real implementations you can use
- **Tested daily** - Every tutorial is continuously verified
- **Clear guidance** - Step-by-step instructions
- **Best practices** - Patterns to help you succeed

---

## 🍽️ Tutorials

| Tutorial | Description | Difficulty |
|----------|-------------|------------|
| [**Zero to Hero**](tutorials/zero-to-hero) | Build and deploy your first parachain | 🟢 Beginner |

> 💡 More tutorials coming soon! Check [issues](https://github.com/polkadot-developers/polkadot-cookbook/issues) or [contribute your own](#-contributing).

---

## 🚀 Getting Started

### Prerequisites

- **Rust** `1.86+` - [Install via rustup](https://rustup.rs)
- **Node.js** `20+` - [Download](https://nodejs.org/)

### Quick Start

```bash
# Clone the repository
git clone https://github.com/polkadot-developers/polkadot-cookbook.git
cd polkadot-cookbook

# Build the SDK
cargo build --workspace --release

# Run a tutorial
cd tutorials/zero-to-hero
npm install
npm test
```

### Create a Tutorial

```bash
# Interactive mode
cargo run --package polkadot-cookbook-cli

# With slug
cargo run --package polkadot-cookbook-cli -- my-tutorial
```

---

## 🤝 Contributing

We welcome contributions! See our [Contributing Guide](CONTRIBUTING.md) for details.

**Tutorial ideas we'd love to see:**
- Building custom pallets and Polkadot SDK runtimes
- Cross-chain messaging with XCM
- Smart contracts leveraging Polkadot precompiles

---

## 📜 License

Licensed under **MIT OR Apache-2.0**.

---

<div align="center">

Built by [Polkadot Developers](https://github.com/polkadot-developers)

</div>
