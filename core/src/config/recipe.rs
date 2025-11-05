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
    /// Runtime development pathway
    #[serde(rename = "runtime")]
    Runtime,
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

/// Content type classification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ContentType {
    /// Tutorial - Complete journey from zero to working solution
    Tutorial,
    /// Guide - Focused, actionable steps for specific tasks
    Guide,
}

/// Difficulty level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Difficulty {
    /// Beginner level
    Beginner,
    /// Intermediate level
    Intermediate,
    /// Advanced level
    Advanced,
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

    /// Content type (tutorial or guide)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<ContentType>,

    /// Difficulty level
    #[serde(skip_serializing_if = "Option::is_none")]
    pub difficulty: Option<Difficulty>,

    /// Whether recipe needs a running node
    #[serde(default)]
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
            category: None,
            pathway: None,
            content_type: None,
            difficulty: None,
            needs_node: false,
            description: "Replace with a short description.".to_string(),
            recipe_type,
        }
    }

    /// Create a new recipe configuration with full details
    pub fn new_with_details(
        name: impl Into<String>,
        slug: impl Into<String>,
        recipe_type: RecipeType,
        pathway: RecipePathway,
        content_type: ContentType,
        difficulty: Difficulty,
        description: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            slug: slug.into(),
            category: None,
            pathway: Some(pathway),
            content_type: Some(content_type),
            difficulty: Some(difficulty),
            needs_node: false,
            description: description.into(),
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
        assert_eq!(config.pathway, None);
        assert_eq!(config.content_type, None);
        assert_eq!(config.difficulty, None);
    }

    #[test]
    fn test_recipe_config_new_with_details() {
        let config = RecipeConfig::new_with_details(
            "My Recipe",
            "my-recipe",
            RecipeType::PolkadotSdk,
            RecipePathway::Runtime,
            ContentType::Tutorial,
            Difficulty::Beginner,
            "A comprehensive tutorial",
        );
        assert_eq!(config.name, "My Recipe");
        assert_eq!(config.slug, "my-recipe");
        assert_eq!(config.recipe_type, RecipeType::PolkadotSdk);
        assert_eq!(config.pathway, Some(RecipePathway::Runtime));
        assert_eq!(config.content_type, Some(ContentType::Tutorial));
        assert_eq!(config.difficulty, Some(Difficulty::Beginner));
        assert_eq!(config.description, "A comprehensive tutorial");
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
        let runtime = RecipePathway::Runtime;
        let json = serde_json::to_string(&runtime).unwrap();
        assert_eq!(json, "\"runtime\"");

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
    fn test_content_type_serialization() {
        let tutorial = ContentType::Tutorial;
        let json = serde_json::to_string(&tutorial).unwrap();
        assert_eq!(json, "\"tutorial\"");

        let guide = ContentType::Guide;
        let json = serde_json::to_string(&guide).unwrap();
        assert_eq!(json, "\"guide\"");
    }

    #[test]
    fn test_difficulty_serialization() {
        let beginner = Difficulty::Beginner;
        let json = serde_json::to_string(&beginner).unwrap();
        assert_eq!(json, "\"beginner\"");

        let intermediate = Difficulty::Intermediate;
        let json = serde_json::to_string(&intermediate).unwrap();
        assert_eq!(json, "\"intermediate\"");

        let advanced = Difficulty::Advanced;
        let json = serde_json::to_string(&advanced).unwrap();
        assert_eq!(json, "\"advanced\"");
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
