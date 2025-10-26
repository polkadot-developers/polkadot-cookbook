use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Version specification for a single dependency
pub type VersionSpec = String;

/// Set of version specifications keyed by dependency name
pub type VersionSet = HashMap<String, VersionSpec>;

/// Metadata for version configuration files
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VersionMetadata {
    /// Schema version for the configuration format
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_version: Option<String>,
}

impl Default for VersionMetadata {
    fn default() -> Self {
        Self {
            schema_version: Some("1.0".to_string()),
        }
    }
}

/// Global version configuration loaded from root versions.yml
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GlobalVersionConfig {
    /// Global default versions for all dependencies
    pub versions: VersionSet,

    /// Optional metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<VersionMetadata>,
}

/// Recipe-specific version configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RecipeVersionConfig {
    /// Recipe-specific version overrides
    pub versions: VersionSet,

    /// Optional metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<VersionMetadata>,
}

impl RecipeVersionConfig {
    /// Create a new recipe version config with default metadata
    pub fn new(versions: VersionSet) -> Self {
        Self {
            versions,
            metadata: Some(VersionMetadata::default()),
        }
    }
}

/// Resolved version configuration after merging global and recipe-specific versions
#[derive(Debug, Clone, PartialEq)]
pub struct ResolvedVersions {
    /// The merged version set (global + recipe overrides)
    pub versions: VersionSet,

    /// Source of each version (for debugging/reporting)
    pub sources: HashMap<String, VersionSource>,
}

/// Source of a version specification
#[derive(Debug, Clone, PartialEq)]
pub enum VersionSource {
    /// Version came from global versions.yml
    Global,
    /// Version came from recipe-specific versions.yml
    Recipe,
}

impl ResolvedVersions {
    /// Create a new resolved versions set
    pub fn new() -> Self {
        Self {
            versions: HashMap::new(),
            sources: HashMap::new(),
        }
    }

    /// Get a version by dependency name
    pub fn get(&self, name: &str) -> Option<&VersionSpec> {
        self.versions.get(name)
    }

    /// Get the source of a version
    pub fn get_source(&self, name: &str) -> Option<&VersionSource> {
        self.sources.get(name)
    }

    /// Check if a dependency version exists
    pub fn contains(&self, name: &str) -> bool {
        self.versions.contains_key(name)
    }

    /// Get all dependency names
    pub fn dependencies(&self) -> Vec<&String> {
        self.versions.keys().collect()
    }
}

impl Default for ResolvedVersions {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_metadata_default() {
        let metadata = VersionMetadata::default();
        assert_eq!(metadata.schema_version, Some("1.0".to_string()));
    }

    #[test]
    fn test_recipe_version_config_new() {
        let mut versions = HashMap::new();
        versions.insert("rust".to_string(), "1.86".to_string());

        let config = RecipeVersionConfig::new(versions.clone());
        assert_eq!(config.versions, versions);
        assert!(config.metadata.is_some());
    }

    #[test]
    fn test_resolved_versions() {
        let mut resolved = ResolvedVersions::new();

        assert!(!resolved.contains("rust"));
        assert_eq!(resolved.dependencies().len(), 0);

        resolved
            .versions
            .insert("rust".to_string(), "1.86".to_string());
        resolved
            .sources
            .insert("rust".to_string(), VersionSource::Global);

        assert!(resolved.contains("rust"));
        assert_eq!(resolved.get("rust"), Some(&"1.86".to_string()));
        assert_eq!(resolved.get_source("rust"), Some(&VersionSource::Global));
    }
}
