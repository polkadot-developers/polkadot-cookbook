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
    /// Solidity smart contracts (pallet-revive)
    Solidity,
    /// XCM cross-chain interactions (Chopsticks)
    Xcm,
}

/// Recipe metadata from recipe.config.yml
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecipeConfig {
    /// Recipe name/title
    pub name: String,

    /// Recipe slug
    pub slug: String,

    /// Recipe category
    pub category: String,

    /// Whether recipe needs a running node
    pub needs_node: bool,

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
            category: "polkadot-sdk-cookbook".to_string(),
            needs_node: true,
            description: "Replace with a short description.".to_string(),
            recipe_type,
        }
    }

    /// Load recipe config from YAML file
    pub fn from_file(path: &PathBuf) -> crate::error::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: RecipeConfig = serde_yaml::from_str(&content)?;
        Ok(config)
    }

    /// Save recipe config to YAML file
    pub fn to_file(&self, path: &PathBuf) -> crate::error::Result<()> {
        let yaml = serde_yaml::to_string(self)?;
        std::fs::write(path, yaml)?;
        Ok(())
    }

    /// Generate YAML content as string
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
        assert!(config.needs_node);
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
    }

    #[test]
    fn test_recipe_config_to_yaml() {
        let config = RecipeConfig::new("Test", "test", RecipeType::PolkadotSdk);
        let yaml = config.to_yaml().unwrap();
        assert!(yaml.contains("name: Test"));
        assert!(yaml.contains("slug: test"));
        assert!(yaml.contains("type: polkadot-sdk"));
    }
}
