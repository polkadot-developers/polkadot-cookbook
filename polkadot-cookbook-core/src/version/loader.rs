use crate::error::{CookbookError, Result};
use crate::version::types::{GlobalVersionConfig, TutorialVersionConfig};
use std::path::Path;
use tokio::fs;

/// Loads version configuration files
pub struct VersionLoader;

impl VersionLoader {
    /// Load global version configuration from versions.yml
    ///
    /// # Arguments
    /// * `path` - Path to the versions.yml file (typically at repo root)
    ///
    /// # Example
    /// ```no_run
    /// use polkadot_cookbook_core::version::VersionLoader;
    /// use std::path::Path;
    ///
    /// # async fn example() -> polkadot_cookbook_core::Result<()> {
    /// let global = VersionLoader::load_global(Path::new("versions.yml")).await?;
    /// println!("Rust version: {:?}", global.versions.get("rust"));
    /// # Ok(())
    /// # }
    /// ```
    pub async fn load_global(path: &Path) -> Result<GlobalVersionConfig> {
        if !path.exists() {
            return Err(CookbookError::FileSystemError {
                message: format!("Global versions file not found: {}", path.display()),
                path: Some(path.to_path_buf()),
            });
        }

        let contents =
            fs::read_to_string(path)
                .await
                .map_err(|e| CookbookError::FileSystemError {
                    message: format!("Failed to read global versions file: {}", e),
                    path: Some(path.to_path_buf()),
                })?;

        Self::parse_global(&contents)
    }

    /// Load tutorial-specific version configuration
    ///
    /// # Arguments
    /// * `path` - Path to the tutorial's versions.yml file
    ///
    /// # Example
    /// ```no_run
    /// use polkadot_cookbook_core::version::VersionLoader;
    /// use std::path::Path;
    ///
    /// # async fn example() -> polkadot_cookbook_core::Result<()> {
    /// let tutorial = VersionLoader::load_tutorial(
    ///     Path::new("recipes/my-recipe/versions.yml")
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn load_tutorial(path: &Path) -> Result<TutorialVersionConfig> {
        if !path.exists() {
            return Ok(TutorialVersionConfig {
                versions: Default::default(),
                metadata: None,
            });
        }

        let contents =
            fs::read_to_string(path)
                .await
                .map_err(|e| CookbookError::FileSystemError {
                    message: format!("Failed to read tutorial versions file: {}", e),
                    path: Some(path.to_path_buf()),
                })?;

        Self::parse_tutorial(&contents)
    }

    /// Parse global version configuration from YAML string
    fn parse_global(yaml: &str) -> Result<GlobalVersionConfig> {
        serde_yaml::from_str(yaml).map_err(|e| {
            CookbookError::ConfigError(format!("Failed to parse global versions YAML: {}", e))
        })
    }

    /// Parse tutorial version configuration from YAML string
    fn parse_tutorial(yaml: &str) -> Result<TutorialVersionConfig> {
        serde_yaml::from_str(yaml).map_err(|e| {
            CookbookError::ConfigError(format!("Failed to parse tutorial versions YAML: {}", e))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_global() {
        let yaml = r#"
versions:
  rust: "1.86"
  polkadot_omni_node: "0.5.0"
  chain_spec_builder: "10.0.0"
  frame_omni_bencher: "0.13.0"

metadata:
  schema_version: "1.0"
"#;

        let config = VersionLoader::parse_global(yaml).unwrap();
        assert_eq!(config.versions.get("rust"), Some(&"1.86".to_string()));
        assert_eq!(
            config.versions.get("polkadot_omni_node"),
            Some(&"0.5.0".to_string())
        );
        assert_eq!(config.versions.len(), 4);
    }

    #[test]
    fn test_parse_tutorial() {
        let yaml = r#"
versions:
  polkadot_omni_node: "0.6.0"
  chain_spec_builder: "10.0.0"

metadata:
  schema_version: "1.0"
"#;

        let config = VersionLoader::parse_tutorial(yaml).unwrap();
        assert_eq!(
            config.versions.get("polkadot_omni_node"),
            Some(&"0.6.0".to_string())
        );
        assert_eq!(config.versions.len(), 2);
    }

    #[test]
    fn test_parse_global_minimal() {
        let yaml = r#"
versions:
  rust: "1.86"
"#;

        let config = VersionLoader::parse_global(yaml).unwrap();
        assert_eq!(config.versions.get("rust"), Some(&"1.86".to_string()));
        assert!(config.metadata.is_none());
    }

    #[test]
    fn test_parse_invalid_yaml() {
        let yaml = "invalid: yaml: content:";
        let result = VersionLoader::parse_global(yaml);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_missing_versions_key() {
        let yaml = r#"
metadata:
  schema_version: "1.0"
"#;
        let result = VersionLoader::parse_global(yaml);
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_load_nonexistent_global_file() {
        let result = VersionLoader::load_global(Path::new("/nonexistent/versions.yml")).await;
        assert!(result.is_err());
        match result.unwrap_err() {
            CookbookError::FileSystemError { message, .. } => {
                assert!(message.contains("not found"));
            }
            _ => panic!("Expected FileSystemError"),
        }
    }

    #[tokio::test]
    async fn test_load_nonexistent_tutorial_returns_empty() {
        let result = VersionLoader::load_tutorial(Path::new("/nonexistent/versions.yml")).await;
        assert!(result.is_ok());
        let config = result.unwrap();
        assert!(config.versions.is_empty());
    }
}
