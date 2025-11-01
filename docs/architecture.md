# SDK Architecture

The Polkadot Cookbook uses a modular SDK architecture consisting of two main components.

## Table of Contents

- [Polkadot Cookbook Core](#core)
- [Polkadot Cookbook CLI](#cli)
- [Why This Architecture?](#why-this-architecture)
- [Contributing to the SDK](#contributing-to-the-sdk)

## Polkadot Cookbook Core

**Package**: `core`

The core library provides the business logic for recipe creation and management. It can be used programmatically by other tools.

### Key Modules

- `config` - Type-safe project and recipe configuration
- `error` - Comprehensive error types with serialization support
- `git` - Async git operations
- `templates` - Template generation for scaffolding
- `scaffold` - Project creation and directory structure
- `bootstrap` - Test environment setup (npm, dependencies, config files)
- `version` - Version management for recipe dependencies (see [VERSION_MANAGEMENT.md](../core/VERSION_MANAGEMENT.md))

### Features

- Async-first API using Tokio
- Structured logging with `tracing`
- Serializable errors for tooling integration
- Comprehensive test coverage (80%+)
- No terminal dependencies (pure library)

### Example Programmatic Usage

```rust
use polkadot_cookbook_core::{config::ProjectConfig, Scaffold};
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ProjectConfig::new("my-recipe")
        .with_destination(PathBuf::from("./recipes"))
        .with_git_init(true)
        .with_skip_install(false);

    let scaffold = Scaffold::new();
    let project_info = scaffold.create_project(config).await?;

    println!("Created: {}", project_info.project_path.display());
    Ok(())
}
```

For more information, see [`core/README.md`](../core/README.md).

## Polkadot Cookbook CLI

**Package**: `cli`

A thin CLI wrapper around the core library that provides a command-line interface with interactive prompts.

### Commands

- `recipe create` - Create a new recipe with interactive prompts
- `recipe test` - Test a recipe
- `recipe validate` - Validate recipe structure
- `recipe lint` - Run linting checks
- `recipe list` - List all recipes
- `recipe submit` - Submit recipe as pull request
- `setup` - Setup development environment
- `doctor` - Check environment and diagnose issues
- `versions` - View and manage dependency versions

### Features

- Interactive prompts with validation (using cliclack)
- Beautiful colored output and spinners
- Progress indicators
- Error handling with helpful messages
- Command-line flags for customization
- Non-interactive mode for CI/CD
- CI-friendly output formats

### Usage

```bash
# Create recipe - Interactive mode (prompts for options)
dot recipe create

# Non-interactive mode with title
dot recipe create --title "My Recipe"

# With options
dot recipe create --title "My Recipe" --skip-install --no-git --non-interactive

# Non-interactive mode for CI/CD
dot recipe create --title "My Recipe" --non-interactive

# View global dependency versions
dot versions

# View recipe-specific versions
dot versions my-recipe

# Show version sources (global vs recipe override)
dot versions my-recipe --show-source

# CI-friendly output (KEY=VALUE format)
dot versions my-recipe --ci

# Validate version keys
dot versions --validate

# Show help
dot --help
```

## Why This Architecture?

The SDK architecture provides several benefits:

### 1. Separation of Concerns
- Core library has zero UI/terminal dependencies
- CLI is a thin presentation layer
- Business logic is testable and reusable

### 2. Programmatic Access
- Other tools can use the core library directly
- IDE extensions can integrate the functionality
- CI/CD pipelines can automate recipe creation

### 3. Better Testing
- Unit tests for business logic
- Integration tests for workflows
- CLI can be tested separately

### 4. Easier Maintenance
- Clear module boundaries
- Async-first for better performance
- Structured logging for observability

## Contributing to the SDK

If you want to contribute to the SDK itself (not just recipes):

### Core Library Changes

Changes go in `core/`:
- Add features to appropriate modules
- Write comprehensive tests
- Use structured logging (`tracing`)
- Ensure no terminal dependencies

### CLI Changes

Changes go in `cli/`:
- Keep it thin (mostly UI/formatting)
- Delegate logic to core library
- Use interactive prompts for better UX

### Testing

```bash
# Test core library
cargo test --package core

# Test CLI
cargo run --package cli -- test-project --skip-install --no-git

# Test entire workspace
cargo test --workspace
```

### Quality Checks

```bash
# Format code
cargo fmt --check

# Run clippy
cargo clippy --workspace -- -D warnings
```
