//! Version management for Polkadot Cookbook tutorials
//!
//! This module provides functionality to manage and resolve version dependencies
//! for tutorials. It supports:
//! - Loading global version configurations from the repository root
//! - Loading tutorial-specific version overrides
//! - Merging configurations with tutorial versions taking precedence
//! - Tracking version sources for debugging and reporting
//!
//! # Architecture
//!
//! The version management system consists of three main components:
//!
//! - **Types** (`types.rs`): Core data structures for version configurations
//! - **Loader** (`loader.rs`): Functions to load and parse YAML configuration files
//! - **Resolver** (`resolver.rs`): Logic to merge global and tutorial-specific versions
//!
//! # Example
//!
//! ```no_run
//! use polkadot_cookbook_core::version::{VersionLoader, VersionResolver};
//! use std::path::Path;
//!
//! # async fn example() -> polkadot_cookbook_core::Result<()> {
//! // Load global versions
//! let global = VersionLoader::load_global(Path::new("versions.yml")).await?;
//!
//! // Load tutorial-specific versions (optional)
//! let tutorial = VersionLoader::load_tutorial(
//!     Path::new("tutorials/my-tutorial/versions.yml")
//! ).await?;
//!
//! // Merge configurations
//! let resolved = VersionResolver::merge(&global, &tutorial);
//!
//! // Access versions
//! if let Some(rust_version) = resolved.get("rust") {
//!     println!("Rust version: {}", rust_version);
//! }
//! # Ok(())
//! # }
//! ```

mod loader;
mod resolver;
mod types;

pub use loader::VersionLoader;
pub use resolver::VersionResolver;
pub use types::{
    GlobalVersionConfig, ResolvedVersions, TutorialVersionConfig, VersionMetadata, VersionSet,
    VersionSource, VersionSpec,
};

use crate::error::Result;
use std::path::Path;

/// High-level API for version resolution
///
/// This is the recommended way to resolve versions for a tutorial.
/// It handles loading both global and tutorial-specific configurations
/// and merging them appropriately.
///
/// # Arguments
/// * `repo_root` - Path to repository root (where global versions.yml lives)
/// * `tutorial_path` - Optional path to tutorial directory (looks for versions.yml inside)
///
/// # Returns
/// Resolved versions with source tracking
///
/// # Example
/// ```no_run
/// use polkadot_cookbook_core::version::resolve_versions;
/// use std::path::Path;
///
/// # async fn example() -> polkadot_cookbook_core::Result<()> {
/// let resolved = resolve_versions(
///     Path::new("."),
///     Some(Path::new("tutorials/my-tutorial"))
/// ).await?;
///
/// println!("Rust version: {:?}", resolved.get("rust"));
/// # Ok(())
/// # }
/// ```
pub async fn resolve_versions(
    repo_root: &Path,
    tutorial_path: Option<&Path>,
) -> Result<ResolvedVersions> {
    // Load global versions
    let global_versions_path = repo_root.join("versions.yml");
    let global = VersionLoader::load_global(&global_versions_path).await?;

    // Load tutorial versions if path provided
    let tutorial = if let Some(path) = tutorial_path {
        let tutorial_versions_path = path.join("versions.yml");
        Some(VersionLoader::load_tutorial(&tutorial_versions_path).await?)
    } else {
        None
    };

    // Merge and return
    Ok(VersionResolver::merge_optional(&global, tutorial.as_ref()))
}

/// Get versions for a specific tutorial by slug
///
/// Convenience function that constructs the tutorial path from a slug.
///
/// # Arguments
/// * `repo_root` - Path to repository root
/// * `tutorial_slug` - Tutorial slug (e.g., "my-tutorial")
///
/// # Example
/// ```no_run
/// use polkadot_cookbook_core::version::resolve_tutorial_versions;
/// use std::path::Path;
///
/// # async fn example() -> polkadot_cookbook_core::Result<()> {
/// let resolved = resolve_tutorial_versions(
///     Path::new("."),
///     "my-tutorial"
/// ).await?;
/// # Ok(())
/// # }
/// ```
pub async fn resolve_tutorial_versions(
    repo_root: &Path,
    tutorial_slug: &str,
) -> Result<ResolvedVersions> {
    let tutorial_path = repo_root.join("tutorials").join(tutorial_slug);
    resolve_versions(repo_root, Some(&tutorial_path)).await
}

/// Load only global versions without tutorial overrides
///
/// Useful when you only need the default versions.
///
/// # Arguments
/// * `repo_root` - Path to repository root
///
/// # Example
/// ```no_run
/// use polkadot_cookbook_core::version::load_global_versions;
/// use std::path::Path;
///
/// # async fn example() -> polkadot_cookbook_core::Result<()> {
/// let versions = load_global_versions(Path::new(".")).await?;
/// # Ok(())
/// # }
/// ```
pub async fn load_global_versions(repo_root: &Path) -> Result<ResolvedVersions> {
    resolve_versions(repo_root, None).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use tempfile::TempDir;
    use tokio::fs;

    async fn create_test_versions_yml(dir: &Path, content: &str) -> PathBuf {
        let path = dir.join("versions.yml");
        fs::write(&path, content).await.unwrap();
        path
    }

    #[tokio::test]
    async fn test_resolve_versions_with_tutorial() {
        let temp_dir = TempDir::new().unwrap();
        let temp_path = temp_dir.path();

        // Create global versions.yml
        let global_yaml = r#"
versions:
  rust: "1.86"
  polkadot_omni_node: "0.5.0"
  chain_spec_builder: "10.0.0"
"#;
        create_test_versions_yml(temp_path, global_yaml).await;

        // Create tutorial directory and versions.yml
        let tutorial_dir = temp_path.join("tutorials").join("my-tutorial");
        fs::create_dir_all(&tutorial_dir).await.unwrap();

        let tutorial_yaml = r#"
versions:
  polkadot_omni_node: "0.6.0"
"#;
        create_test_versions_yml(&tutorial_dir, tutorial_yaml).await;

        // Resolve versions
        let resolved = resolve_versions(temp_path, Some(&tutorial_dir))
            .await
            .unwrap();

        // Check merged results
        assert_eq!(resolved.get("rust"), Some(&"1.86".to_string()));
        assert_eq!(
            resolved.get("polkadot_omni_node"),
            Some(&"0.6.0".to_string())
        );
        assert_eq!(
            resolved.get("chain_spec_builder"),
            Some(&"10.0.0".to_string())
        );

        // Check sources
        assert_eq!(resolved.get_source("rust"), Some(&VersionSource::Global));
        assert_eq!(
            resolved.get_source("polkadot_omni_node"),
            Some(&VersionSource::Tutorial)
        );
    }

    #[tokio::test]
    async fn test_resolve_versions_without_tutorial() {
        let temp_dir = TempDir::new().unwrap();
        let temp_path = temp_dir.path();

        let global_yaml = r#"
versions:
  rust: "1.86"
  polkadot_omni_node: "0.5.0"
"#;
        create_test_versions_yml(temp_path, global_yaml).await;

        let resolved = resolve_versions(temp_path, None).await.unwrap();

        assert_eq!(resolved.get("rust"), Some(&"1.86".to_string()));
        assert_eq!(
            resolved.get("polkadot_omni_node"),
            Some(&"0.5.0".to_string())
        );
        assert_eq!(resolved.versions.len(), 2);
    }

    #[tokio::test]
    async fn test_resolve_tutorial_versions_by_slug() {
        let temp_dir = TempDir::new().unwrap();
        let temp_path = temp_dir.path();

        let global_yaml = r#"
versions:
  rust: "1.86"
"#;
        create_test_versions_yml(temp_path, global_yaml).await;

        let tutorial_dir = temp_path.join("tutorials").join("test-tutorial");
        fs::create_dir_all(&tutorial_dir).await.unwrap();

        let tutorial_yaml = r#"
versions:
  custom_dep: "2.0.0"
"#;
        create_test_versions_yml(&tutorial_dir, tutorial_yaml).await;

        let resolved = resolve_tutorial_versions(temp_path, "test-tutorial")
            .await
            .unwrap();

        assert_eq!(resolved.get("rust"), Some(&"1.86".to_string()));
        assert_eq!(resolved.get("custom_dep"), Some(&"2.0.0".to_string()));
    }

    #[tokio::test]
    async fn test_load_global_versions() {
        let temp_dir = TempDir::new().unwrap();
        let temp_path = temp_dir.path();

        let global_yaml = r#"
versions:
  rust: "1.86"
  polkadot_omni_node: "0.5.0"
"#;
        create_test_versions_yml(temp_path, global_yaml).await;

        let resolved = load_global_versions(temp_path).await.unwrap();

        assert_eq!(resolved.versions.len(), 2);
        assert_eq!(resolved.get("rust"), Some(&"1.86".to_string()));

        // All sources should be global
        for source in resolved.sources.values() {
            assert_eq!(source, &VersionSource::Global);
        }
    }

    #[tokio::test]
    async fn test_missing_global_versions_returns_error() {
        let temp_dir = TempDir::new().unwrap();
        let temp_path = temp_dir.path();

        let result = load_global_versions(temp_path).await;
        assert!(result.is_err());
    }
}
