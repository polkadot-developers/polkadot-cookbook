# SDK Guide

Guide to using the Polkadot Cookbook SDK programmatically.

## Overview

The Polkadot Cookbook SDK (`sdk`) is a Rust library that provides:

- **Project Configuration** - Type-safe project settings with builder pattern
- **Project Scaffolding** - Generate new projects from templates
- **Template Management** - Clone + overlay approach for upstream templates
- **Metadata Extraction** - Auto-detect project types from file structure
- **Error Handling** - Comprehensive, serializable error types

## Installation

### As a Workspace Dependency

Add to your `Cargo.toml`:

```toml
[dependencies]
sdk = { path = "../sdk" }
tokio = { version = "1", features = ["full"] }
```

### From Source

```bash
git clone https://github.com/polkadot-developers/polkadot-cookbook.git
cd polkadot-cookbook
cargo build --release -p sdk
```

---

## Quick Start

### Create a Project

```rust
use polkadot_cookbook_sdk::{Scaffold, ProjectConfig, ProjectType};
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ProjectConfig::new("my-parachain")
        .with_title("My Parachain")
        .with_description("A custom parachain project")
        .with_project_type(ProjectType::PolkadotSdk)
        .with_destination(PathBuf::from("."))
        .with_skip_install(true)
        .with_git_init(false);

    let scaffold = Scaffold::new();
    let project_info = scaffold.create_project(config).await?;

    println!("Created: {}", project_info.project_path.display());
    Ok(())
}
```

---

## Core Types

### ProjectConfig

Configuration for creating a new project. Uses a builder pattern.

```rust
pub struct ProjectConfig {
    pub slug: String,
    pub title: String,
    pub destination: PathBuf,
    pub git_init: bool,
    pub skip_install: bool,
    pub project_type: ProjectType,
    pub category: String,
    pub description: String,
    pub pathway: Option<ProjectPathway>,
    pub pallet_only: bool,
}
```

**Builder methods:**

```rust
let config = ProjectConfig::new("my-project")    // slug, auto-generates title
    .with_title("My Project")                     // override title
    .with_destination(PathBuf::from("./output"))  // output directory
    .with_project_type(ProjectType::PolkadotSdk)  // project type
    .with_pathway(ProjectPathway::Pallets)         // pathway classification
    .with_description("A short description")      // project description
    .with_category("polkadot-sdk-cookbook")        // category
    .with_git_init(true)                          // initialize git repo
    .with_skip_install(false);                    // run npm install

// Get full project path
let path = config.project_path(); // destination/{slug}
```

### ProjectType

Classification of project implementation type:

```rust
pub enum ProjectType {
    PolkadotSdk,   // Rust parachain/pallet (Polkadot SDK)
    Solidity,      // Solidity smart contracts (Hardhat)
    Xcm,           // Cross-chain messaging (Chopsticks)
    Transactions,  // Chain transactions (PAPI)
    Networks,      // Network infrastructure (Zombienet/Chopsticks)
}
```

### ProjectPathway

High-level pathway classification:

```rust
pub enum ProjectPathway {
    Pallets,       // Pallet development
    Contracts,     // Smart contracts
    Transactions,  // Chain transactions
    Xcm,           // Cross-chain messaging
    Networks,      // Network infrastructure
}
```

### ProjectInfo

Information returned after project creation:

```rust
pub struct ProjectInfo {
    pub slug: String,
    pub title: String,
    pub project_path: PathBuf,
    pub git_initialized: bool,
}
```

### ProjectMetadata

Metadata loaded from an existing project directory:

```rust
pub struct ProjectMetadata {
    pub name: String,
    pub slug: String,
    pub category: Option<String>,
    pub pathway: Option<ProjectPathway>,
    pub description: String,
    pub project_type: ProjectType,
}
```

**Loading from a project directory:**

```rust
use polkadot_cookbook_sdk::ProjectMetadata;

// Auto-detects project type from file structure
// Reads title/description from README.md frontmatter
let metadata = ProjectMetadata::from_project_directory("./my-project").await?;

println!("Name: {}", metadata.name);
println!("Type: {:?}", metadata.project_type);
println!("Pathway: {:?}", metadata.pathway);
```

---

## Scaffold

The `Scaffold` struct is the main entry point for creating projects.

```rust
use polkadot_cookbook_sdk::{Scaffold, ProjectConfig, ProjectType};

let config = ProjectConfig::new("my-parachain")
    .with_project_type(ProjectType::PolkadotSdk)
    .with_skip_install(true)
    .with_git_init(false);

let scaffold = Scaffold::new();
let info = scaffold.create_project(config).await?;
```

### How Scaffolding Works

The scaffold process varies by project type:

**PolkadotSdk (clone + overlay):**
1. Clone upstream `polkadot-sdk-parachain-template` at a pinned git tag (cached at `~/.dot/templates/`)
2. Copy the cached clone to the destination directory
3. String-replace upstream names with the user's project slug
4. If pallet-only mode: remove node/, runtime/, and non-pallet files
5. Overlay SDK-specific files (tests, scripts, READMEs, package.json)
6. Process template placeholders in overlay files

**Other types (Solidity, XCM, Transactions, Networks):**
1. Copy embedded template files from the SDK
2. Process template placeholders (`{{slug}}`, `{{title}}`, `{{description}}`, etc.)

### Pallet-Only Mode

For PolkadotSdk projects, pallet-only mode creates a minimal pallet workspace without runtime or node:

```rust
let mut config = ProjectConfig::new("my-pallet")
    .with_project_type(ProjectType::PolkadotSdk);
config.pallet_only = true;

let scaffold = Scaffold::new();
let info = scaffold.create_project(config).await?;
// Creates: Cargo.toml, pallets/template/, rust-toolchain.toml, README.md
// Excludes: node/, runtime/, package.json, tests/, scripts/
```

---

## Error Handling

The SDK uses `CookbookError` with structured, serializable variants:

```rust
use polkadot_cookbook_sdk::error::CookbookError;

pub enum CookbookError {
    GitError(String),
    ConfigError(String),
    ScaffoldError(String),
    TestError(String),
    FileSystemError { message: String, path: Option<PathBuf> },
    ValidationError(String),
    WorkingDirectoryError(String),
    ProjectExistsError(String),
    TemplateNotFoundError(String),
    BootstrapError(String),
    IoError(String),
    CommandError { command: String, message: String },
}
```

**Usage:**

```rust
use polkadot_cookbook_sdk::{Scaffold, ProjectConfig};
use polkadot_cookbook_sdk::error::CookbookError;

let scaffold = Scaffold::new();
match scaffold.create_project(config).await {
    Ok(info) => println!("Created: {}", info.project_path.display()),
    Err(CookbookError::ProjectExistsError(path)) => {
        eprintln!("Project already exists at: {}", path);
    }
    Err(CookbookError::FileSystemError { message, path }) => {
        eprintln!("File error: {} ({:?})", message, path);
    }
    Err(CookbookError::CommandError { command, message }) => {
        eprintln!("Command '{}' failed: {}", command, message);
    }
    Err(e) => eprintln!("Error: {}", e),
}
```

Errors are serializable with `serde` for tool integration:

```rust
let error = CookbookError::ValidationError("invalid slug".to_string());
let json = serde_json::to_string(&error)?;
// {"type":"ValidationError","details":"invalid slug"}
```

---

## Validation Utilities

The SDK provides validation functions for project configuration:

```rust
use polkadot_cookbook_sdk::config::{validate_slug, validate_title, title_to_slug, slug_to_title};

// Validate a slug
validate_slug("my-project")?;   // Ok
validate_slug("My Project")?;   // Err - must be lowercase

// Convert between title and slug
let slug = title_to_slug("My Custom Project");  // "my-custom-project"
let title = slug_to_title("my-custom-project"); // "My Custom Project"

// Validate a title
validate_title("My Project")?;  // Ok
```

---

## Testing

```bash
# Run all SDK tests
cargo test --package sdk

# Run with logging
RUST_LOG=debug cargo test --package sdk

# Run unit tests only
cargo test --package sdk --lib

# Run integration tests only
cargo test --package sdk --test integration_test

# Run doc tests
cargo test --package sdk --doc
```

---

## Related Documentation

- **[CLI Reference](cli-reference.md)** - Command-line interface
- **[Architecture](architecture.md)** - System design

---

[← Back to Developers Guide](README.md)
