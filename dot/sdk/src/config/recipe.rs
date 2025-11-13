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
}
