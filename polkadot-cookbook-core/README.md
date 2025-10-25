# Polkadot Cookbook Core

SDK library for programmatic tutorial creation and management.

## Overview

`polkadot-cookbook-core` is a Rust library that provides the business logic for creating and managing Polkadot Cookbook tutorials. It can be used programmatically by other tools, CLIs, or IDE extensions.

**Key Features:**
- Tutorial scaffolding with templates
- Dependency version management
- Git operations (branch creation, commits)
- npm/Node.js setup and installation
- Test environment configuration
- Async-first API using Tokio

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
polkadot-cookbook-core = { path = "../polkadot-cookbook-core" }
tokio = { version = "1", features = ["full"] }
```

## Quick Start

### Create a Tutorial

```rust
use polkadot_cookbook_core::{config::ProjectConfig, Scaffold};
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Configure the tutorial
    let config = ProjectConfig::new("my-tutorial")
        .with_destination(PathBuf::from("./tutorials"))
        .with_git_init(true)
        .with_skip_install(false);

    // Create the tutorial
    let scaffold = Scaffold::new();
    let project_info = scaffold.create_project(config).await?;

    println!("Created: {}", project_info.project_path.display());
    println!("Branch: {}", project_info.git_branch.unwrap());

    Ok(())
}
```

### Manage Versions

```rust
use polkadot_cookbook_core::version::{
    load_global_versions,
    resolve_tutorial_versions,
    VersionSource,
};
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let repo_root = Path::new(".");

    // Load global versions
    let global = load_global_versions(repo_root).await?;
    for (name, version) in &global.versions {
        println!("{}: {}", name, version);
    }

    // Resolve tutorial-specific versions
    let resolved = resolve_tutorial_versions(repo_root, "zero-to-hero").await?;
    for (name, version) in &resolved.versions {
        let source = match resolved.get_source(name) {
            Some(VersionSource::Global) => "global",
            Some(VersionSource::Tutorial) => "tutorial",
            None => "unknown",
        };
        println!("{}: {} ({})", name, version, source);
    }

    Ok(())
}
```

## API Overview

### Modules

- **`config`** - Type-safe project and tutorial configuration
- **`error`** - Comprehensive error types with serialization support
- **`git`** - Async git operations using git2
- **`templates`** - Template generation for scaffolding
- **`scaffold`** - Project creation and directory structure
- **`bootstrap`** - Test environment setup (npm, dependencies, config files)
- **`version`** - Version management for tutorial dependencies

### Key Types

#### `Scaffold`

Main entry point for creating tutorials.

```rust
pub struct Scaffold { /* ... */ }

impl Scaffold {
    pub fn new() -> Self;
    pub async fn create_project(&self, config: ProjectConfig) -> Result<ProjectInfo>;
}
```

#### `ProjectConfig`

Configuration for tutorial creation.

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

Information about the created tutorial.

```rust
pub struct ProjectInfo {
    pub slug: String,
    pub title: String,
    pub project_path: PathBuf,
    pub git_branch: Option<String>,
}
```

### Version Management

#### Types

```rust
pub struct ResolvedVersions {
    pub versions: HashMap<String, String>,
    sources: HashMap<String, VersionSource>,
}

pub enum VersionSource {
    Global,    // From global versions.yml
    Tutorial,  // From tutorial versions.yml
}
```

#### Functions

```rust
// Load global versions
pub async fn load_global_versions(repo_root: &Path) -> Result<ResolvedVersions>;

// Resolve versions for a specific tutorial
pub async fn resolve_tutorial_versions(
    repo_root: &Path,
    tutorial_slug: &str
) -> Result<ResolvedVersions>;

// Get version source
impl ResolvedVersions {
    pub fn get(&self, name: &str) -> Option<&String>;
    pub fn get_source(&self, name: &str) -> Option<&VersionSource>;
}
```

## Examples

Run examples to see the SDK in action:

```bash
# Version resolution example
cargo run --package polkadot-cookbook-core --example version_resolution

# Tutorial creation example (coming soon)
cargo run --package polkadot-cookbook-core --example create_tutorial
```

## Version Management

The SDK provides a powerful version management system that allows tutorials to specify dependency versions while inheriting defaults from a global configuration.

### Global Versions

Defined in `versions.yml` at repository root:

```yaml
versions:
  rust: "1.86"
  polkadot_omni_node: "0.5.0"
  chain_spec_builder: "10.0.0"
  frame_omni_bencher: "0.13.0"

metadata:
  schema_version: "1.0"
```

### Tutorial Overrides

Each tutorial can override versions in `tutorials/<slug>/versions.yml`:

```yaml
versions:
  polkadot_omni_node: "0.6.0"  # Override global version

metadata:
  schema_version: "1.0"
```

### Resolution

The SDK merges global and tutorial versions, with tutorial versions taking precedence:

```rust
let resolved = resolve_tutorial_versions(repo_root, "my-tutorial").await?;

// Result:
// - rust: "1.86" (from global)
// - polkadot_omni_node: "0.6.0" (from tutorial)
// - chain_spec_builder: "10.0.0" (from global)
// - frame_omni_bencher: "0.13.0" (from global)
```

For complete version management documentation, see [VERSION_MANAGEMENT.md](VERSION_MANAGEMENT.md).

## Architecture

### Design Principles

1. **Async-first** - All I/O operations are async using Tokio
2. **No UI dependencies** - Pure library, no terminal output
3. **Structured logging** - Uses `tracing` for observability
4. **Comprehensive errors** - Serializable error types for tool integration
5. **Testable** - High test coverage, isolated unit tests

### Module Structure

```
polkadot-cookbook-core/
├── src/
│   ├── lib.rs              # Public API
│   ├── config/             # Configuration types
│   ├── error/              # Error types
│   ├── git/                # Git operations
│   ├── templates/          # File templates
│   ├── scaffold/           # Scaffolding logic
│   │   ├── mod.rs         # Main scaffold
│   │   └── bootstrap.rs   # npm/test setup
│   └── version/            # Version management
│       ├── mod.rs         # Public API
│       ├── types.rs       # Data structures
│       ├── loader.rs      # YAML loading
│       └── resolver.rs    # Version merging
├── examples/               # Usage examples
└── tests/                  # Integration tests
```

## Testing

```bash
# Run all tests
cargo test --package polkadot-cookbook-core

# Run with logging
RUST_LOG=debug cargo test --package polkadot-cookbook-core

# Run specific test
cargo test --package polkadot-cookbook-core version::
```

## Error Handling

The SDK uses a comprehensive error type:

```rust
use polkadot_cookbook_core::error::CookbookError;

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

- **API Docs** - Run `cargo doc --package polkadot-cookbook-core --open`
- **Version Management** - See [VERSION_MANAGEMENT.md](VERSION_MANAGEMENT.md)
- **Examples** - Check `examples/` directory

## License

MIT OR Apache-2.0
