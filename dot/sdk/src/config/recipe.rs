/// Recipe configuration types
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Recipe type classification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RecipeType {
    /// Polkadot SDK recipe (Runtime pallets with Rust)
    #[serde(rename = "polkadot-sdk")]
    PolkadotSdk,
    /// Solidity smart contracts
    Solidity,
    /// XCM cross-chain interactions (Chopsticks)
    Xcm,
    /// Basic chain interactions (PAPI, single-chain transactions)
    #[serde(rename = "basic-interaction")]
    BasicInteraction,
    /// Testing infrastructure (Zombienet, Chopsticks configs)
    Testing,
}

/// Recipe pathway classification (high-level categorization)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RecipePathway {
    /// Parachain development pathway
    #[serde(rename = "parachain")]
    Parachain,
    /// Smart contracts pathway
    #[serde(rename = "contracts")]
    Contracts,
    /// Basic chain interactions pathway
    #[serde(rename = "basic-interaction")]
    BasicInteraction,
    /// Cross-chain messaging pathway
    #[serde(rename = "xcm")]
    Xcm,
    /// Testing infrastructure pathway
    #[serde(rename = "testing")]
    Testing,
    /// Request a new template (CLI-only, triggers GitHub issue flow)
    #[serde(rename = "request-new")]
    RequestNew,
}

impl RecipePathway {
    /// Convert pathway to folder name for organizing recipes
    pub fn to_folder_name(&self) -> &'static str {
        match self {
            RecipePathway::Parachain => "parachain",
            RecipePathway::Contracts => "contracts",
            RecipePathway::BasicInteraction => "basic-interaction",
            RecipePathway::Xcm => "xcm",
            RecipePathway::Testing => "testing",
            RecipePathway::RequestNew => "request-new",
        }
    }
}

/// Recipe metadata from recipe.config.yml
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecipeConfig {
    /// Recipe name/title
    pub name: String,

    /// Recipe slug
    pub slug: String,

    /// Recipe category (optional, legacy field)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,

    /// Recipe pathway (high-level categorization)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pathway: Option<RecipePathway>,

    /// Recipe description
    pub description: String,

    /// Recipe type
    #[serde(rename = "type")]
    pub recipe_type: RecipeType,
}

impl RecipeConfig {
    /// Create a new recipe configuration
    pub fn new(name: impl Into<String>, slug: impl Into<String>, recipe_type: RecipeType) -> Self {
        Self {
            name: name.into(),
            slug: slug.into(),
            category: None,
            pathway: None,
            description: "Replace with a short description.".to_string(),
            recipe_type,
        }
    }

    /// Load recipe config from a recipe directory
    ///
    /// This method:
    /// - Reads frontmatter from README.md for title and description
    /// - Auto-detects recipe type from file presence
    /// - Derives slug from directory name
    ///
    /// # Arguments
    ///
    /// * `recipe_path` - Path to the recipe directory
    ///
    /// # Returns
    ///
    /// RecipeConfig loaded from the recipe directory
    pub async fn from_recipe_directory(
        recipe_path: impl AsRef<std::path::Path>,
    ) -> crate::error::Result<Self> {
        use crate::metadata::{detect_recipe_type, parse_frontmatter_from_file};

        let path = recipe_path.as_ref();

        // Get slug from directory name
        let slug = path
            .file_name()
            .ok_or_else(|| {
                crate::CookbookError::ValidationError("Invalid recipe path".to_string())
            })?
            .to_str()
            .ok_or_else(|| {
                crate::CookbookError::ValidationError("Invalid UTF-8 in path".to_string())
            })?
            .to_string();

        // Auto-detect recipe type
        let recipe_type = detect_recipe_type(path).await.map_err(|e| {
            crate::CookbookError::ValidationError(format!("Failed to detect recipe type: {}", e))
        })?;

        // Read frontmatter from README.md
        let readme_path = path.join("README.md");
        let frontmatter = parse_frontmatter_from_file(&readme_path)
            .await
            .map_err(|e| {
                crate::CookbookError::ValidationError(format!(
                    "Failed to parse README frontmatter: {}",
                    e
                ))
            })?;

        // Map recipe type to pathway
        let pathway = Some(match recipe_type {
            RecipeType::PolkadotSdk => RecipePathway::Parachain,
            RecipeType::Solidity => RecipePathway::Contracts,
            RecipeType::Xcm => RecipePathway::Xcm,
            RecipeType::BasicInteraction => RecipePathway::BasicInteraction,
            RecipeType::Testing => RecipePathway::Testing,
        });

        Ok(Self {
            name: frontmatter.title,
            slug,
            category: None, // Category field is deprecated
            pathway,
            description: frontmatter.description,
            recipe_type,
        })
    }

    /// Load recipe config from YAML file (deprecated, kept for backward compatibility)
    #[deprecated(note = "Use from_recipe_directory instead")]
    pub fn from_file(path: &PathBuf) -> crate::error::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: RecipeConfig = serde_yaml::from_str(&content)?;
        Ok(config)
    }

    /// Save recipe config to YAML file (deprecated)
    #[deprecated(note = "recipe.config.yml is deprecated, use README frontmatter instead")]
    pub fn to_file(&self, path: &PathBuf) -> crate::error::Result<()> {
        let yaml = serde_yaml::to_string(self)?;
        std::fs::write(path, yaml)?;
        Ok(())
    }

    /// Generate YAML content as string (deprecated)
    #[deprecated(note = "recipe.config.yml is deprecated, use README frontmatter instead")]
    pub fn to_yaml(&self) -> crate::error::Result<String> {
        Ok(serde_yaml::to_string(self)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recipe_config_new() {
        let config = RecipeConfig::new("My Recipe", "my-recipe", RecipeType::PolkadotSdk);
        assert_eq!(config.name, "My Recipe");
        assert_eq!(config.slug, "my-recipe");
        assert_eq!(config.recipe_type, RecipeType::PolkadotSdk);
        assert_eq!(config.pathway, None);
    }

    #[test]
    fn test_recipe_type_serialization() {
        let sdk = RecipeType::PolkadotSdk;
        let json = serde_json::to_string(&sdk).unwrap();
        assert_eq!(json, "\"polkadot-sdk\"");

        let solidity = RecipeType::Solidity;
        let json = serde_json::to_string(&solidity).unwrap();
        assert_eq!(json, "\"solidity\"");

        let xcm = RecipeType::Xcm;
        let json = serde_json::to_string(&xcm).unwrap();
        assert_eq!(json, "\"xcm\"");

        let basic = RecipeType::BasicInteraction;
        let json = serde_json::to_string(&basic).unwrap();
        assert_eq!(json, "\"basic-interaction\"");

        let testing = RecipeType::Testing;
        let json = serde_json::to_string(&testing).unwrap();
        assert_eq!(json, "\"testing\"");
    }

    #[test]
    fn test_pathway_serialization() {
        let parachain = RecipePathway::Parachain;
        let json = serde_json::to_string(&parachain).unwrap();
        assert_eq!(json, "\"parachain\"");

        let contracts = RecipePathway::Contracts;
        let json = serde_json::to_string(&contracts).unwrap();
        assert_eq!(json, "\"contracts\"");

        let basic = RecipePathway::BasicInteraction;
        let json = serde_json::to_string(&basic).unwrap();
        assert_eq!(json, "\"basic-interaction\"");

        let xcm = RecipePathway::Xcm;
        let json = serde_json::to_string(&xcm).unwrap();
        assert_eq!(json, "\"xcm\"");

        let testing = RecipePathway::Testing;
        let json = serde_json::to_string(&testing).unwrap();
        assert_eq!(json, "\"testing\"");
    }

    #[test]
    fn test_recipe_type_clone() {
        let original = RecipeType::PolkadotSdk;
        let cloned = original.clone();
        assert_eq!(original, cloned);
    }

    #[test]
    fn test_recipe_pathway_clone() {
        let original = RecipePathway::Parachain;
        let cloned = original.clone();
        assert_eq!(original, cloned);
    }

    #[test]
    fn test_recipe_type_copy() {
        let original = RecipeType::PolkadotSdk;
        let copied = original;
        assert_eq!(original, copied);
    }

    #[test]
    fn test_recipe_pathway_copy() {
        let original = RecipePathway::Parachain;
        let copied = original;
        assert_eq!(original, copied);
    }

    #[test]
    fn test_recipe_type_debug() {
        let sdk = RecipeType::PolkadotSdk;
        let debug_str = format!("{:?}", sdk);
        assert!(debug_str.contains("PolkadotSdk"));
    }

    #[test]
    fn test_recipe_pathway_debug() {
        let parachain = RecipePathway::Parachain;
        let debug_str = format!("{:?}", parachain);
        assert!(debug_str.contains("Parachain"));
    }

    #[test]
    fn test_recipe_config_with_pathway() {
        let mut config = RecipeConfig::new("My Recipe", "my-recipe", RecipeType::PolkadotSdk);
        config.pathway = Some(RecipePathway::Parachain);

        assert_eq!(config.pathway, Some(RecipePathway::Parachain));
    }

    #[test]
    fn test_recipe_config_with_description() {
        let mut config = RecipeConfig::new("My Recipe", "my-recipe", RecipeType::PolkadotSdk);
        config.description = "Test description".to_string();

        assert_eq!(config.description, "Test description");
    }

    #[test]
    fn test_recipe_config_yaml_serialization() {
        let config = RecipeConfig::new("My Recipe", "my-recipe", RecipeType::PolkadotSdk);

        #[allow(deprecated)]
        let yaml = config.to_yaml().unwrap();

        assert!(yaml.contains("name: My Recipe"));
        assert!(yaml.contains("slug: my-recipe"));
        assert!(yaml.contains("type: polkadot-sdk"));
    }

    #[tokio::test]
    async fn test_from_recipe_directory() {
        use std::fs;
        use tempfile::TempDir;

        // Create a temporary directory
        let temp_dir = TempDir::new().unwrap();
        let recipe_path = temp_dir.path().join("test-recipe");
        fs::create_dir(&recipe_path).unwrap();

        // Create a README.md with frontmatter
        let readme_content = r#"---
title: Test Recipe
description: A test recipe for testing
---

# Test Recipe

This is a test recipe.
"#;
        fs::write(recipe_path.join("README.md"), readme_content).unwrap();

        // Create a Cargo.toml to indicate this is a Polkadot SDK recipe
        fs::write(recipe_path.join("Cargo.toml"), "[package]\nname = \"test\"").unwrap();
        fs::create_dir(recipe_path.join("pallets")).unwrap();

        // Test loading the recipe config
        let config = RecipeConfig::from_recipe_directory(&recipe_path)
            .await
            .unwrap();

        assert_eq!(config.name, "Test Recipe");
        assert_eq!(config.slug, "test-recipe");
        assert_eq!(config.description, "A test recipe for testing");
        assert_eq!(config.recipe_type, RecipeType::PolkadotSdk);
        assert_eq!(config.pathway, Some(RecipePathway::Parachain));
    }

    #[tokio::test]
    async fn test_from_recipe_directory_solidity() {
        use std::fs;
        use tempfile::TempDir;

        // Create a temporary directory for Solidity recipe
        let temp_dir = TempDir::new().unwrap();
        let recipe_path = temp_dir.path().join("solidity-recipe");
        fs::create_dir(&recipe_path).unwrap();

        // Create a README.md with frontmatter
        let readme_content = r#"---
title: Solidity Contract
description: A Solidity smart contract recipe
---

# Solidity Contract
"#;
        fs::write(recipe_path.join("README.md"), readme_content).unwrap();

        // Create package.json with hardhat dependency to indicate Solidity recipe
        let package_json = r#"{
  "name": "solidity-recipe",
  "dependencies": {
    "hardhat": "^2.0.0"
  }
}"#;
        fs::write(recipe_path.join("package.json"), package_json).unwrap();

        // Test loading the recipe config
        let config = RecipeConfig::from_recipe_directory(&recipe_path)
            .await
            .unwrap();

        assert_eq!(config.name, "Solidity Contract");
        assert_eq!(config.slug, "solidity-recipe");
        assert_eq!(config.description, "A Solidity smart contract recipe");
        assert_eq!(config.recipe_type, RecipeType::Solidity);
        assert_eq!(config.pathway, Some(RecipePathway::Contracts));
    }
}
