---
layout: doc
title: "Contributors Guide"
---

# Contributors Guide

This section contains everything you need to know about contributing recipes to the Polkadot Cookbook.

> **How recipes work:** Recipe source code lives in **your own GitHub repository**. The cookbook's `recipes/` directory contains only **test harnesses** that clone, build, and verify each external recipe repo. Contributing a recipe means adding a test harness that points to your repo.

## Documentation

### Essential Guides

- **[Workflow Guide](workflow.md)** - Git workflow and contribution process
- **[Recipe Guidelines](recipe-guidelines.md)** - Quality standards and requirements
- **[Commit Conventions](commit-conventions.md)** - How to write conventional commits

### Development Guides

- **[Recipe Development](recipe-development.md)** - Patterns for different recipe types
- **[Testing Recipes](testing-recipes.md)** - Testing patterns and best practices

### Getting Help

- **[Troubleshooting](troubleshooting.md)** - FAQ and common issues

## Quick Start

If you're new to contributing, follow this path:

1. **Install the CLI** - See [Getting Started](../getting-started/installation.md)
2. **Read the Workflow** - Understand the [contribution workflow](workflow.md)
3. **Check Guidelines** - Review [recipe guidelines](recipe-guidelines.md)
4. **Develop Your Recipe** - Build your project in your own repository (use `dot create` to scaffold)
5. **Add a Test Harness** - Add a test harness to `recipes/` in the cookbook
6. **Follow Standards** - Use [conventional commits](commit-conventions.md)
7. **Open a PR** - Follow the [workflow guide](workflow.md)

## Recipe Types

The Cookbook supports different types of recipes:

- **Polkadot SDK (Rust)** - Pallets, runtime configurations, node setups
- **Solidity (EVM)** - Smart contracts for Polkadot EVM chains
- **XCM (Cross-chain)** - Cross-chain messaging examples

Each type has specific development patterns - see [Recipe Development](recipe-development.md).

## Community

- **Code of Conduct** - Be respectful and constructive
- **Getting Help** - Use [troubleshooting guide](troubleshooting.md) or open an issue
- **Discussions** - Engage with the community

---

[← Back to Documentation Hub](../)
