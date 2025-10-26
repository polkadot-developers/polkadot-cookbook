use crate::version::types::{
    GlobalVersionConfig, RecipeVersionConfig, ResolvedVersions, VersionSource,
};

/// Resolves and merges version configurations
pub struct VersionResolver;

impl VersionResolver {
    /// Merge global and recipe-specific versions
    ///
    /// Recipe versions override global versions on a per-key basis.
    ///
    /// # Arguments
    /// * `global` - Global version configuration
    /// * `recipe` - Recipe-specific version configuration
    ///
    /// # Returns
    /// A `ResolvedVersions` containing the merged version set with source tracking
    ///
    /// # Example
    /// ```
    /// use polkadot_cookbook_core::version::{VersionResolver, GlobalVersionConfig, RecipeVersionConfig};
    /// use std::collections::HashMap;
    ///
    /// let mut global_versions = HashMap::new();
    /// global_versions.insert("rust".to_string(), "1.86".to_string());
    /// global_versions.insert("polkadot_omni_node".to_string(), "0.5.0".to_string());
    ///
    /// let global = GlobalVersionConfig {
    ///     versions: global_versions,
    ///     metadata: None,
    /// };
    ///
    /// let mut recipe_versions = HashMap::new();
    /// recipe_versions.insert("polkadot_omni_node".to_string(), "0.6.0".to_string());
    ///
    /// let recipe = RecipeVersionConfig {
    ///     versions: recipe_versions,
    ///     metadata: None,
    /// };
    ///
    /// let resolved = VersionResolver::merge(&global, &recipe);
    ///
    /// // rust comes from global
    /// assert_eq!(resolved.get("rust"), Some(&"1.86".to_string()));
    /// // polkadot_omni_node is overridden by recipe
    /// assert_eq!(resolved.get("polkadot_omni_node"), Some(&"0.6.0".to_string()));
    /// ```
    pub fn merge(
        global: &GlobalVersionConfig,
        recipe: &RecipeVersionConfig,
    ) -> ResolvedVersions {
        let mut resolved = ResolvedVersions::new();

        // First, add all global versions
        for (key, value) in &global.versions {
            resolved.versions.insert(key.clone(), value.clone());
            resolved.sources.insert(key.clone(), VersionSource::Global);
        }

        // Then, override with recipe-specific versions
        for (key, value) in &recipe.versions {
            resolved.versions.insert(key.clone(), value.clone());
            resolved
                .sources
                .insert(key.clone(), VersionSource::Recipe);
        }

        resolved
    }

    /// Merge global and optional recipe versions
    ///
    /// Convenience method that handles the case where recipe config might be None
    pub fn merge_optional(
        global: &GlobalVersionConfig,
        recipe: Option<&RecipeVersionConfig>,
    ) -> ResolvedVersions {
        match recipe {
            Some(t) => Self::merge(global, t),
            None => {
                let mut resolved = ResolvedVersions::new();
                for (key, value) in &global.versions {
                    resolved.versions.insert(key.clone(), value.clone());
                    resolved.sources.insert(key.clone(), VersionSource::Global);
                }
                resolved
            }
        }
    }

    /// Get a specific version from resolved versions
    ///
    /// Returns the version string if the dependency exists
    pub fn get_version<'a>(resolved: &'a ResolvedVersions, dependency: &str) -> Option<&'a String> {
        resolved.get(dependency)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn create_global_config() -> GlobalVersionConfig {
        let mut versions = HashMap::new();
        versions.insert("rust".to_string(), "1.86".to_string());
        versions.insert("polkadot_omni_node".to_string(), "0.5.0".to_string());
        versions.insert("chain_spec_builder".to_string(), "10.0.0".to_string());
        versions.insert("frame_omni_bencher".to_string(), "0.13.0".to_string());

        GlobalVersionConfig {
            versions,
            metadata: None,
        }
    }

    fn create_recipe_config() -> RecipeVersionConfig {
        let mut versions = HashMap::new();
        versions.insert("polkadot_omni_node".to_string(), "0.6.0".to_string());
        versions.insert("chain_spec_builder".to_string(), "11.0.0".to_string());

        RecipeVersionConfig {
            versions,
            metadata: None,
        }
    }

    #[test]
    fn test_merge_overrides_recipe_versions() {
        let global = create_global_config();
        let recipe =create_recipe_config();

        let resolved = VersionResolver::merge(&global, &recipe);

        // Rust should come from global
        assert_eq!(resolved.get("rust"), Some(&"1.86".to_string()));
        assert_eq!(resolved.get_source("rust"), Some(&VersionSource::Global));

        // polkadot_omni_node should be overridden by recipe
        assert_eq!(
            resolved.get("polkadot_omni_node"),
            Some(&"0.6.0".to_string())
        );
        assert_eq!(
            resolved.get_source("polkadot_omni_node"),
            Some(&VersionSource::Recipe)
        );

        // chain_spec_builder should be overridden by recipe
        assert_eq!(
            resolved.get("chain_spec_builder"),
            Some(&"11.0.0".to_string())
        );
        assert_eq!(
            resolved.get_source("chain_spec_builder"),
            Some(&VersionSource::Recipe)
        );

        // frame_omni_bencher should come from global (not in recipe)
        assert_eq!(
            resolved.get("frame_omni_bencher"),
            Some(&"0.13.0".to_string())
        );
        assert_eq!(
            resolved.get_source("frame_omni_bencher"),
            Some(&VersionSource::Global)
        );
    }

    #[test]
    fn test_merge_with_empty_recipe() {
        let global = create_global_config();
        let recipe =RecipeVersionConfig {
            versions: HashMap::new(),
            metadata: None,
        };

        let resolved = VersionResolver::merge(&global, &recipe);

        // All versions should come from global
        assert_eq!(resolved.get("rust"), Some(&"1.86".to_string()));
        assert_eq!(
            resolved.get("polkadot_omni_node"),
            Some(&"0.5.0".to_string())
        );
        assert_eq!(resolved.versions.len(), 4);

        // All sources should be global
        for source in resolved.sources.values() {
            assert_eq!(source, &VersionSource::Global);
        }
    }

    #[test]
    fn test_merge_optional_with_some() {
        let global = create_global_config();
        let recipe =create_recipe_config();

        let resolved = VersionResolver::merge_optional(&global, Some(&recipe));

        assert_eq!(
            resolved.get("polkadot_omni_node"),
            Some(&"0.6.0".to_string())
        );
    }

    #[test]
    fn test_merge_optional_with_none() {
        let global = create_global_config();

        let resolved = VersionResolver::merge_optional(&global, None);

        // All versions should come from global
        assert_eq!(resolved.get("rust"), Some(&"1.86".to_string()));
        assert_eq!(resolved.versions.len(), 4);

        // All sources should be global
        for source in resolved.sources.values() {
            assert_eq!(source, &VersionSource::Global);
        }
    }

    #[test]
    fn test_get_version() {
        let global = create_global_config();
        let recipe =create_recipe_config();
        let resolved = VersionResolver::merge(&global, &recipe);

        assert_eq!(
            VersionResolver::get_version(&resolved, "rust"),
            Some(&"1.86".to_string())
        );
        assert_eq!(VersionResolver::get_version(&resolved, "nonexistent"), None);
    }

    #[test]
    fn test_resolved_versions_methods() {
        let global = create_global_config();
        let recipe =create_recipe_config();
        let resolved = VersionResolver::merge(&global, &recipe);

        // Test contains
        assert!(resolved.contains("rust"));
        assert!(!resolved.contains("nonexistent"));

        // Test dependencies
        let deps = resolved.dependencies();
        assert_eq!(deps.len(), 4);
        assert!(deps.contains(&&"rust".to_string()));
    }

    #[test]
    fn test_recipe_can_add_new_dependencies() {
        let global = create_global_config();

        let mut recipe_versions = HashMap::new();
        recipe_versions.insert("custom_tool".to_string(), "1.0.0".to_string());

        let recipe =RecipeVersionConfig {
            versions: recipe_versions,
            metadata: None,
        };

        let resolved = VersionResolver::merge(&global, &recipe);

        // Should have global dependencies + new recipe dependency
        assert_eq!(resolved.versions.len(), 5);
        assert_eq!(resolved.get("custom_tool"), Some(&"1.0.0".to_string()));
        assert_eq!(
            resolved.get_source("custom_tool"),
            Some(&VersionSource::Recipe)
        );
    }
}
