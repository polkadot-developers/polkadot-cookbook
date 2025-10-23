/// Tutorial configuration types
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Tutorial type classification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TutorialType {
    /// Polkadot SDK tutorial
    Sdk,
    /// Smart contracts tutorial
    Contracts,
}

/// Tutorial metadata from tutorial.config.yml
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TutorialConfig {
    /// Tutorial name/title
    pub name: String,

    /// Tutorial slug
    pub slug: String,

    /// Tutorial category
    pub category: String,

    /// Whether tutorial needs a running node
    pub needs_node: bool,

    /// Tutorial description
    pub description: String,

    /// Tutorial type
    #[serde(rename = "type")]
    pub tutorial_type: TutorialType,
}

impl TutorialConfig {
    /// Create a new tutorial configuration
    pub fn new(
        name: impl Into<String>,
        slug: impl Into<String>,
        tutorial_type: TutorialType,
    ) -> Self {
        Self {
            name: name.into(),
            slug: slug.into(),
            category: "polkadot-sdk-cookbook".to_string(),
            needs_node: true,
            description: "Replace with a short description.".to_string(),
            tutorial_type,
        }
    }

    /// Load tutorial config from YAML file
    pub fn from_file(path: &PathBuf) -> crate::error::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: TutorialConfig = serde_yaml::from_str(&content)?;
        Ok(config)
    }

    /// Save tutorial config to YAML file
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
    fn test_tutorial_config_new() {
        let config = TutorialConfig::new("My Tutorial", "my-tutorial", TutorialType::Sdk);
        assert_eq!(config.name, "My Tutorial");
        assert_eq!(config.slug, "my-tutorial");
        assert_eq!(config.tutorial_type, TutorialType::Sdk);
        assert!(config.needs_node);
    }

    #[test]
    fn test_tutorial_type_serialization() {
        let sdk = TutorialType::Sdk;
        let json = serde_json::to_string(&sdk).unwrap();
        assert_eq!(json, "\"sdk\"");

        let contracts = TutorialType::Contracts;
        let json = serde_json::to_string(&contracts).unwrap();
        assert_eq!(json, "\"contracts\"");
    }

    #[test]
    fn test_tutorial_config_to_yaml() {
        let config = TutorialConfig::new("Test", "test", TutorialType::Sdk);
        let yaml = config.to_yaml().unwrap();
        assert!(yaml.contains("name: Test"));
        assert!(yaml.contains("slug: test"));
        assert!(yaml.contains("type: sdk"));
    }
}
