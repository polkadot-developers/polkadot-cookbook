//! # Polkadot Cookbook Core
//!
//! Core library for Polkadot Cookbook - programmatic access to tutorial scaffolding,
//! configuration management, and testing utilities.
//!
//! ## Features
//!
//! - **Async-first API**: All I/O operations are async using Tokio
//! - **Structured Error Handling**: Comprehensive error types with serialization support
//! - **Configuration Management**: Type-safe project and tutorial configuration
//! - **Version Management**: Load and merge global and tutorial-specific version configurations
//! - **Template Generation**: Reusable templates for project scaffolding
//! - **Git Integration**: Automated git operations for project workflows
//! - **Validation**: Input validation and project configuration checks
//! - **Observability**: Structured logging with the `tracing` crate
//!
//! ## Usage
//!
//! ### Creating a New Project
//!
//! ```no_run
//! use polkadot_cookbook_core::config::ProjectConfig;
//! use std::path::PathBuf;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Create project configuration
//!     let config = ProjectConfig::new("my-tutorial")
//!         .with_destination(PathBuf::from("./tutorials"))
//!         .with_git_init(true);
//!
//!     println!("Project will be created at: {}", config.project_path().display());
//!
//!     Ok(())
//! }
//! ```
//!
//! ### Validating Configuration
//!
//! ```
//! use polkadot_cookbook_core::config::{ProjectConfig, validate_slug};
//!
//! let config = ProjectConfig::new("my-tutorial");
//! assert!(validate_slug(&config.slug).is_ok());
//! ```
//!
//! ### Error Handling
//!
//! All operations return `Result<T, CookbookError>` with structured error types:
//!
//! ```
//! use polkadot_cookbook_core::error::CookbookError;
//!
//! fn handle_error(error: CookbookError) {
//!     match error {
//!         CookbookError::ValidationError(msg) => {
//!             eprintln!("Validation failed: {}", msg);
//!         }
//!         CookbookError::GitError(msg) => {
//!             eprintln!("Git operation failed: {}", msg);
//!         }
//!         _ => {
//!             eprintln!("Error: {}", error);
//!         }
//!     }
//! }
//! ```

// Re-export commonly used types
pub use config::{ProjectConfig, ProjectInfo, RecipeConfig, RecipeType};
pub use error::{CookbookError, Result};
pub use scaffold::{Bootstrap, Scaffold};

/// Error types and result aliases
pub mod error;

/// Configuration management for recipes
pub mod config;

/// Git operations wrapper
pub mod git;

/// Template generation for scaffolding
pub mod templates;

/// Project scaffolding logic
pub mod scaffold;

/// Version management for dependencies
pub mod version;

/// File system operations (TODO: to be implemented)
#[cfg(feature = "fs")]
pub mod fs;

/// Test execution engine (TODO: to be implemented)
#[cfg(feature = "test_runner")]
pub mod test_runner;

/// Query and discovery APIs (TODO: to be implemented)
#[cfg(feature = "query")]
pub mod query;

// Internal prelude for convenience
pub(crate) mod prelude {}
