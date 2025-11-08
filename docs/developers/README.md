# Developers Guide

This section is for developers building tools or extensions using the Polkadot Cookbook SDK.

## Documentation

### Architecture & Design

- **[Architecture](architecture.md)** - System architecture and design principles

### SDK Usage

- **[SDK Guide](sdk-guide.md)** - Using the SDK programmatically
- **[API Reference](api-reference.md)** - Complete API documentation

### CLI Development

- **[CLI Reference](cli-reference.md)** - Complete CLI command reference

## Quick Start

### Using the SDK

The Polkadot Cookbook SDK is a Rust library for programmatically creating and managing recipes.

```rust
use polkadot_cookbook_core::{config::ProjectConfig, Scaffold};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ProjectConfig::new("my-recipe")
        .with_destination(PathBuf::from("./recipes"))
        .with_git_init(true);

    let scaffold = Scaffold::new();
    let project_info = scaffold.create_project(config).await?;

    println!("Created: {}", project_info.project_path.display());
    Ok(())
}
```

[→ Complete SDK Guide](sdk-guide.md)

### Using the CLI

The `dot` CLI provides commands for creating and managing recipes:

```bash
# Create a new recipe
dot recipe create --title "My Recipe"

# View versions

# Submit recipe for review
dot recipe submit
```

[→ Complete CLI Reference](cli-reference.md)

## Architecture Overview

The Polkadot Cookbook consists of two main components:

### Core Library (`core`)
- Async-first Rust library
- No terminal dependencies
- Structured logging
- 80%+ test coverage

### CLI Tool (`cli`)
- Interactive terminal interface
- Built on top of core library
- User-friendly prompts
- Git integration

[→ Full Architecture Documentation](architecture.md)

## Integration Examples

### Build a Custom CLI

```rust
// Use the SDK to build your own CLI
use polkadot_cookbook_core::Scaffold;

// Your custom CLI logic here
```

### IDE Extension

```rust
// Use the SDK to build an IDE extension
use polkadot_cookbook_core::version::resolve_recipe_versions;

// Extension logic here
```

### Automation Tool

```rust
// Use the SDK for automation scripts
use polkadot_cookbook_core::config::ProjectConfig;

// Automation logic here
```

## API Documentation

For complete API documentation, run:

```bash
cargo doc --package core --open
```

Or see [api-reference.md](api-reference.md) for curated examples.

---

[← Back to Documentation Hub](../README.md)
