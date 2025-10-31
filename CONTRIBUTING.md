<div style="margin-bottom: 20px;">
  <img height="24px" alt="Polkadot" src="https://github.com/paritytech/polkadot-sdk/raw/master/docs/images/Polkadot_Logo_Horizontal_Pink_White.png#gh-dark-mode-only" />
  <img height="24px" alt="Polkadot" src="https://github.com/paritytech/polkadot-sdk/raw/master/docs/images/Polkadot_Logo_Horizontal_Pink_Black.png#gh-light-mode-only" />
</div>

<div align="center">

<img src=".media/dot-logo-dark.svg#gh-dark-mode-only" alt="Dot CLI" width="60" height="60" />
<img src=".media/dot-logo-light.svg#gh-light-mode-only" alt="Dot CLI" width="60" height="60" />

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

# macOS (Intel)
curl -L https://github.com/polkadot-developers/polkadot-cookbook/releases/latest/download/dot-macos-amd64.tar.gz | tar xz
sudo mv dot /usr/local/bin/

# macOS (Apple Silicon)
curl -L https://github.com/polkadot-developers/polkadot-cookbook/releases/latest/download/dot-macos-arm64.tar.gz | tar xz
sudo mv dot /usr/local/bin/
```

**Or build from source:**

```bash
git clone https://github.com/polkadot-developers/polkadot-cookbook.git
cd polkadot-cookbook
cargo build --release --bin dot
# Use ./target/release/dot
```

### 2. Setup Your Environment

```bash
# Fork the repository on GitHub first, then:
git clone https://github.com/YOUR_USERNAME/polkadot-cookbook.git
cd polkadot-cookbook
git remote add upstream https://github.com/polkadot-developers/polkadot-cookbook.git

# Verify your setup
dot setup
```

### 3. Create a Recipe

```bash
# Interactive mode (recommended)
dot

# Or with a slug
dot my-recipe-name
```

The CLI will guide you through:
- **Pathway** - Runtime Development, Smart Contracts, Basic Interactions, XCM, or Testing
- **Title** - Clear, descriptive name (e.g., "NFT Pallet with Minting")
- **Difficulty** - Beginner, Intermediate, or Advanced
- **Content Type** - Tutorial (comprehensive) or Guide (focused task)
- **Description** - Brief 1-2 sentence summary

The CLI automatically:
- Creates the recipe structure
- Sets up testing infrastructure
- Installs dependencies
- Creates a git branch

### 4. Write Your Recipe

Edit the generated files:
- `README.md` - Your recipe content (the CLI provides a template)
- `src/` or `pallets/` - Your implementation
- `tests/` - Test your code

### 5. Test Your Recipe

```bash
# Test your recipe
dot recipe test my-recipe-name

# Validate structure
dot recipe validate my-recipe-name

# Run linting
dot recipe lint my-recipe-name
```

### 6. Submit Your Recipe

```bash
# Submit as a pull request
dot recipe submit my-recipe-name
```

The CLI will:
- Validate your recipe
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
# Setup and diagnostics
dot setup          # Check and setup your environment
dot doctor         # Run comprehensive health checks

# Recipe management
dot                # Create recipe (interactive)
dot <slug>         # Create recipe with slug
dot recipe new     # Create recipe (explicit command)
dot recipe list    # List all recipes
dot recipe test    # Test a recipe
dot recipe validate # Validate recipe structure
dot recipe lint    # Run linting checks
dot recipe submit  # Submit as pull request

# Non-interactive mode
dot my-recipe \
  --non-interactive \
  --title "My Recipe Title" \
  --pathway runtime \
  --difficulty beginner \
  --content-type tutorial
```

<hr />

## Recipe Guidelines

### Title Naming

Use clear, descriptive titles:
- ✅ "NFT Pallet with Minting and Transfers"
- ✅ "Asset Transfer using XCM"
- ❌ "My NFT Pallet" (no personal pronouns)
- ❌ "Simple Counter" (no vague qualifiers)
- ❌ "NFT Tutorial" (don't include content type)

### Content Types

- **Tutorial** - Comprehensive learning journey from scratch
- **Guide** - Focused, actionable steps for a specific task

### Code Quality

- Test all functionality
- Follow language conventions (run `cargo fmt`, `cargo clippy`)
- Add clear comments for complex logic
- Include error handling

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

- [CLI Documentation](cli/README.md) - Detailed CLI reference
- [Core Library](core/README.md) - SDK API documentation
- [Architecture](docs/architecture.md) - System design
- [Testing Guide](docs/testing.md) - Testing workflows
- [Workflows](docs/workflows.md) - CI/CD and automation
- [Polkadot SDK Docs](https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/index.html)

<hr />

## Getting Help

- **Questions**: [Open an issue](https://github.com/polkadot-developers/polkadot-cookbook/issues)
- **Example Recipe**: See `recipes/basic-pallet/` for reference

<hr />

<div align="center">

<img src=".media/dot-logo-dark.svg#gh-dark-mode-only" alt="Dot CLI" width="40" height="40" />
<img src=".media/dot-logo-light.svg#gh-light-mode-only" alt="Dot CLI" width="40" height="40" />

<br/>

Thank you for contributing to Polkadot Cookbook!

[Back to Top](#contributing-to-polkadot-cookbook) • [README](README.md) • [Issues](https://github.com/polkadot-developers/polkadot-cookbook/issues)

</div>
