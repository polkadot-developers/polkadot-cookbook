# Polkadot Cookbook Core

Core library for Polkadot Cookbook - programmatic access to tutorial scaffolding, configuration management, and testing utilities.

## Overview

`polkadot-cookbook-core` is a Rust library that provides the business logic for the Polkadot Cookbook project. It enables programmatic interaction with tutorial creation, testing, and management functionality.

## Features

- **Async-first API**: All I/O operations are async using Tokio
- **Structured Error Handling**: Comprehensive error types with serialization support
- **Configuration Management**: Type-safe project and tutorial configuration
- **Template Generation**: Reusable templates for project scaffolding
- **Git Integration**: Automated git operations for project workflows
- **Validation**: Input validation and project configuration checks
- **Observability**: Structured logging with the `tracing` crate

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
polkadot-cookbook-core = "0.1.0"
tokio = { version = "1.43", features = ["full"] }
```

## Usage

### Creating a New Project

```rust
use polkadot_cookbook_core::{
    config::ProjectConfig,
    scaffold::Scaffold,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create project configuration
    let config = ProjectConfig::new("my-tutorial")
        .with_destination(PathBuf::from("./tutorials"))
        .with_git_init(true);

    // Scaffold the project
    let scaffold = Scaffold::new();
    let project_info = scaffold.create_project(config).await?;

    println!("Created project: {}", project_info.slug);
    println!("  Path: {}", project_info.project_path.display());

    Ok(())
}
```

### Validating Configuration

```rust
use polkadot_cookbook_core::config::{ProjectConfig, validate_project_config};

let config = ProjectConfig::new("my-tutorial");
match validate_project_config(&config) {
    Ok(warnings) => {
        for warning in warnings {
            println!("Warning: {}", warning);
        }
    }
    Err(e) => {
        eprintln!("Invalid configuration: {}", e);
    }
}
```

### Querying Available Templates

```rust
use polkadot_cookbook_core::templates::list_available_templates;

let templates = list_available_templates();
for template in templates {
    println!("{}: {}", template.name, template.description);
}
```

### Error Handling

```rust
use polkadot_cookbook_core::error::CookbookError;

match some_operation().await {
    Ok(result) => println!("Success: {:?}", result),
    Err(CookbookError::ValidationError(msg)) => {
        eprintln!("Validation failed: {}", msg);
    }
    Err(CookbookError::GitError(msg)) => {
        eprintln!("Git operation failed: {}", msg);
    }
    Err(e) => {
        eprintln!("Error: {}", e);
    }
}
```

## Architecture

The library is organized into the following modules:

- **config**: Project and tutorial configuration management
- **error**: Structured error types with serialization support
- **git**: Git operations wrapper
- **templates**: Template generation for scaffolding
- **scaffold**: Project scaffolding logic
- **fs**: File system operations
- **test_runner**: Test execution engine
- **query**: Discovery and inspection APIs

## Error Types

All errors implement `Serialize` and `Deserialize` for easy integration with external tools:

```rust
pub enum CookbookError {
    GitError(String),
    ConfigError(String),
    ScaffoldError(String),
    TestError(String),
    FileSystemError { message: String, path: Option<PathBuf> },
    ValidationError(String),
    // ... and more
}
```

## Logging

The library uses the `tracing` crate for structured logging. Configure logging in your application:

```rust
use tracing_subscriber;

tracing_subscriber::fmt()
    .with_env_filter("polkadot_cookbook_core=debug")
    .init();
```

## Contributing

See [CONTRIBUTING.md](../CONTRIBUTING.md) for development guidelines.

## License

MIT OR Apache-2.0
