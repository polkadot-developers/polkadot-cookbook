//! # Polkadot Cookbook SDK
//!
//! SDK library for Polkadot Cookbook - programmatic access to recipe scaffolding,
//! configuration management, and testing utilities.
//!
//! ## Lint Configuration
#![warn(missing_docs)]
#![deny(unsafe_code)]
//!
//! ## Features
//!
//! - **Async-first API**: All I/O operations are async using Tokio
//! - **Structured Error Handling**: Comprehensive error types with serialization support
//! - **Configuration Management**: Type-safe project and tutorial configuration
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
//! use polkadot_cookbook_sdk::config::ProjectConfig;
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
//! use polkadot_cookbook_sdk::config::{ProjectConfig, validate_slug};
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
//! use polkadot_cookbook_sdk::error::CookbookError;
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

/// Constants used throughout the library
pub mod constants;

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

/// Metadata extraction and recipe detection
pub mod metadata;

/// Dependency checking for recipe pathways
pub mod dependencies;

// Internal prelude for convenience
pub(crate) mod prelude {}

// Future features (not yet implemented - kept private)
// These modules are placeholders for planned features and are not exposed
// in the public API until implementation is complete.
#[cfg(feature = "fs")]
mod fs;

#[cfg(feature = "test_runner")]
mod test_runner;

#[cfg(feature = "query")]
mod query;
