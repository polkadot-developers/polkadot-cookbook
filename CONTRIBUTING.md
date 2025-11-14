<div style="margin-bottom: 20px;">
  <img height="24px" alt="Polkadot" src="https://github.com/paritytech/polkadot-sdk/raw/master/docs/images/Polkadot_Logo_Horizontal_Pink_White.png#gh-dark-mode-only" />
  <img height="24px" alt="Polkadot" src="https://github.com/paritytech/polkadot-sdk/raw/master/docs/images/Polkadot_Logo_Horizontal_Pink_Black.png#gh-light-mode-only" />
</div>

<div align="center">

<img src=".github/media/dot-logo-dark.svg#gh-dark-mode-only" alt="Dot CLI" width="60" height="60" />
<img src=".github/media/dot-logo-light.svg#gh-light-mode-only" alt="Dot CLI" width="60" height="60" />

<br/>

# Contributing to Polkadot Cookbook

Thank you for your interest in contributing! The easiest way to contribute a recipe is using the `dot` CLI tool.

</div>

<hr />

## Quick Start

### 1. Install the CLI

**Download pre-built binary:**

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
git clone https://github.com/polkadot-developers/polkadot-cookbook.git
cd polkadot-cookbook
cargo build --release --bin dot
# Use ./target/release/dot
```

### 2. Create a Project

```bash
# Interactive mode (recommended)
dot create

# Or specify a pathway directly
dot create --pathway pallets --title "My Custom Pallet"
```

The CLI will guide you through:
- **Pathway** - Pallets, Contracts, Transactions, XCM, or Networks
- **Title** - Clear, descriptive name (e.g., "NFT Pallet with Minting")
- **Description** - Brief 1-2 sentence summary

The CLI automatically:
- Creates the project structure
- Sets up testing infrastructure
- Installs dependencies
- Creates a git branch

### 3. Write Your Code

Edit the generated files:
- `README.md` - Your recipe content (the CLI provides a template)
- `src/` or `pallets/` - Your implementation
- `tests/` - Test your code

### 4. Test Your Project

```bash
# Test your project
dot test my-project-name
```

### 5. Submit as a Recipe

```bash
# Submit as a pull request to the cookbook
dot submit my-project-name
```

The CLI will:
- Run tests to validate your code
- Validate that required lock files are present (Cargo.lock and/or package-lock.json)
- Commit changes
- Push to your fork
- Create a pull request

Done! 🎉

<hr />

## Other Ways to Contribute

### Report Bugs

Found an issue? [Open an issue](https://github.com/polkadot-developers/polkadot-cookbook/issues/new) with:
- Steps to reproduce
- Expected vs actual behavior
- Environment details

### Suggest Enhancements

Have an idea? [Open an issue](https://github.com/polkadot-developers/polkadot-cookbook/issues/new) describing:
- The enhancement
- Use case and benefits
- Examples if applicable

### Improve Documentation

Documentation fixes are welcome! Submit changes via pull request.

<hr />

## CLI Commands Reference

```bash
# Project creation
dot create          # Create project (interactive)
dot create --title "My Project" --pathway pallets --non-interactive

# Shortcut commands
dot contract        # Create a contract project
dot parachain       # Create a parachain project

# Testing and submission
dot test            # Test a project
dot submit          # Submit as pull request to cookbook
```

<hr />

## Recipe Guidelines

### Title Naming

Use clear, descriptive titles:
- ✅ "NFT Pallet with Minting and Transfers"
- ✅ "Asset Transfer using XCM"
- ❌ "My NFT Pallet" (no personal pronouns)
- ❌ "Simple Counter" (no vague qualifiers)

### Code Quality

- Test all functionality
- Follow language conventions (run `cargo fmt`, `cargo clippy`)
- Add clear comments for complex logic
- Include error handling
- **Commit lock files** (Cargo.lock and/or package-lock.json) to ensure reproducible builds

### Commit Messages

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```
feat(recipe): add my-recipe
fix(recipe): correct storage operations in my-recipe
docs: update CONTRIBUTING.md
test(my-recipe): add integration tests
```

<hr />

## Additional Resources

### Documentation

- **[Documentation Hub](docs/README.md)** - Complete documentation organized by role
- **[Getting Started Guide](docs/getting-started/)** - Installation and first project tutorial
- **[Contributor Guide](docs/contributors/)** - Workflow, guidelines, and best practices
- **[CLI Reference](docs/developers/cli-reference.md)** - Complete CLI command reference
- **[SDK Guide](docs/developers/sdk-guide.md)** - Using the SDK programmatically

### External Resources

- **[Polkadot SDK Docs](https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/index.html)** - Official Polkadot SDK documentation
- **[Polkadot Wiki](https://wiki.polkadot.network/)** - Comprehensive Polkadot network guide

<hr />

## Getting Help

- **Questions**: [Open an issue](https://github.com/polkadot-developers/polkadot-cookbook/issues)
- **Example Projects**: See `recipes/` directory for reference implementations

<hr />

<div align="center">

<img src=".github/media/dot-logo-dark.svg#gh-dark-mode-only" alt="Dot CLI" width="40" height="40" />
<img src=".github/media/dot-logo-light.svg#gh-light-mode-only" alt="Dot CLI" width="40" height="40" />

<br/>

Thank you for contributing to Polkadot Cookbook!

[Back to Top](#contributing-to-polkadot-cookbook) • [README](README.md) • [Issues](https://github.com/polkadot-developers/polkadot-cookbook/issues)

</div>
