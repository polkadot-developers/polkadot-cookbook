# Polkadot Cookbook SDK

SDK library for programmatic recipe creation and management.

## Overview

`sdk` is a Rust library that provides the business logic for creating and managing Polkadot Cookbook recipes. It can be used programmatically by other tools, CLIs, or IDE extensions.

**Key Features:**
- Recipe scaffolding with templates
- Dependency version management
- Git operations (branch creation, commits)
- npm/Node.js setup and installation
- Test environment configuration
- Async-first API using Tokio

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
sdk = { path = "../sdk" }
tokio = { version = "1", features = ["full"] }
```

## Quick Start

### Create a Recipe

```rust
use polkadot_cookbook_sdk::{config::ProjectConfig, Scaffold};
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Configure the recipe
    let config = ProjectConfig::new("my-recipe")
        .with_destination(PathBuf::from("./recipes"))
        .with_git_init(true)
        .with_skip_install(false);

    // Create the recipe
    let scaffold = Scaffold::new();
    let project_info = scaffold.create_project(config).await?;

    println!("Created: {}", project_info.project_path.display());
    println!("Git initialized: {}", project_info.git_initialized);

    Ok(())
}
```

## API Overview

### Modules

- **`config`** - Type-safe project and recipe configuration
- **`error`** - Comprehensive error types with serialization support
- **`git`** - Async git operations using git2
- **`templates`** - Template generation for scaffolding
- **`scaffold`** - Project creation and directory structure
- **`bootstrap`** - Test environment setup (npm, dependencies, config files)
- **`metadata`** - Metadata extraction and project type detection
- **`dependencies`** - Dependency checking for project pathways

### Key Types

#### `Scaffold`

Main entry point for creating recipes.

```rust
pub struct Scaffold { /* ... */ }

impl Scaffold {
    pub fn new() -> Self;
    pub async fn create_project(&self, config: ProjectConfig) -> Result<ProjectInfo>;
}
```

#### `ProjectConfig`

Configuration for recipe creation.

```rust
pub struct ProjectConfig {
    pub slug: String,
    pub destination: PathBuf,
    pub git_init: bool,
    pub skip_install: bool,
}

impl ProjectConfig {
    pub fn new(slug: &str) -> Self;
    pub fn with_destination(self, path: PathBuf) -> Self;
    pub fn with_git_init(self, enabled: bool) -> Self;
    pub fn with_skip_install(self, skip: bool) -> Self;
}
```

#### `ProjectInfo`

Information about the created project.

```rust
pub struct ProjectInfo {
    pub slug: String,
    pub title: String,
    pub project_path: PathBuf,
    pub git_initialized: bool,
}
```

## Examples

Run examples to see the SDK in action:

```bash
# Recipe creation example (coming soon)
cargo run --package sdk --example create_recipe
```

## Template Architecture

The SDK uses a **clone + overlay** approach for the Polkadot SDK template:

1. **Clone** the upstream `polkadot-sdk-parachain-template` at a pinned git tag
2. **Replace** upstream names with the user's project slug
3. **Overlay** SDK-specific files (tests, scripts, READMEs, etc.)

The upstream template is cached at `~/.dot/templates/` after the first clone.

Key constants:
- `UPSTREAM_TEMPLATE_TAG` - Pinned upstream version (e.g., `v0.0.5`)
- `RUST_VERSION` - Rust toolchain version for generated projects (e.g., `1.88`)

Updating to a new upstream template version requires changing these two constants.

## Architecture

### Design Principles

1. **Async-first** - All I/O operations are async using Tokio
2. **No UI dependencies** - Pure library, no terminal output
3. **Structured logging** - Uses `tracing` for observability
4. **Comprehensive errors** - Serializable error types for tool integration
5. **Testable** - High test coverage, isolated unit tests

### Module Structure

```
sdk/
├── src/
│   ├── lib.rs              # Public API
│   ├── constants.rs        # Library constants
│   ├── config/             # Configuration types
│   ├── error.rs            # Error types
│   ├── git/                # Git operations
│   ├── templates/          # File templates
│   ├── scaffold/           # Scaffolding logic
│   │   ├── mod.rs         # Main scaffold (clone + overlay)
│   │   └── bootstrap.rs   # npm/test setup
│   ├── metadata/           # Project type detection
│   └── dependencies/       # Dependency checking
├── templates/              # Embedded template overlays
└── tests/                  # Integration tests
```

## Testing

```bash
# Run all tests
cargo test --package sdk

# Run with logging
RUST_LOG=debug cargo test --package sdk

# Run specific test
cargo test --package sdk version::
```

## Error Handling

The SDK uses a comprehensive error type:

```rust
use polkadot_cookbook_sdk::error::CookbookError;

match scaffold.create_project(config).await {
    Ok(info) => println!("Created: {}", info.project_path.display()),
    Err(CookbookError::FileSystemError { message, path }) => {
        eprintln!("File error: {} ({:?})", message, path);
    }
    Err(CookbookError::ConfigError(msg)) => {
        eprintln!("Config error: {}", msg);
    }
    Err(e) => eprintln!("Error: {}", e),
}
```

## Contributing

To contribute to the SDK:

1. Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
2. Add documentation comments (`///`) for public APIs
3. Write tests for new functionality
4. Run `cargo fmt` and `cargo clippy` before committing

See the [Contributing Guide](../CONTRIBUTING.md) for details.

## Documentation

- **API Docs** - Run `cargo doc --package sdk --open`
- **Version Management** - See [Release Process - Dependency Version Management](../docs/RELEASE_PROCESS.md#dependency-version-management)
- **Examples** - Check `examples/` directory

## License

MIT OR Apache-2.0
