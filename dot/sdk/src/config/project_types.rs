/// Project configuration types
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Project type classification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ProjectType {
    /// Polkadot SDK project (Runtime pallets with Rust)
    #[serde(rename = "polkadot-sdk")]
    PolkadotSdk,
    /// Solidity smart contracts
    Solidity,
    /// XCM cross-chain interactions (Chopsticks)
    Xcm,
    /// Chain transactions (PAPI, single-chain transactions)
    #[serde(rename = "transactions")]
    Transactions,
    /// Network infrastructure (Zombienet, Chopsticks configs)
    Networks,
}

/// Project pathway classification (high-level categorization)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ProjectPathway {
    /// Pallet development pathway (Polkadot SDK pallets)
    #[serde(rename = "pallets")]
    Pallets,
    /// Smart contracts pathway
    #[serde(rename = "contracts")]
    Contracts,
    /// Chain transactions pathway
    #[serde(rename = "transactions")]
    Transactions,
    /// Cross-chain messaging pathway
    #[serde(rename = "xcm")]
    Xcm,
    /// Network infrastructure pathway
    #[serde(rename = "networks")]
    Networks,
}

impl ProjectPathway {
    /// Convert pathway to folder name for organizing projects in the recipes/ folder
    pub fn to_folder_name(&self) -> &'static str {
        match self {
            ProjectPathway::Pallets => "pallets",
            ProjectPathway::Contracts => "contracts",
            ProjectPathway::Transactions => "transactions",
            ProjectPathway::Xcm => "xcm",
            ProjectPathway::Networks => "networks",
        }
    }
}

/// Project metadata - configuration and metadata for a Polkadot project
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectMetadata {
    /// Project name/title
    pub name: String,

    /// Project slug
    pub slug: String,

    /// Project category (optional, legacy field)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,

    /// Project pathway (high-level categorization)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pathway: Option<ProjectPathway>,

    /// Project description
    pub description: String,

    /// Project type
    #[serde(rename = "type")]
    pub project_type: ProjectType,
}

impl ProjectMetadata {
    /// Create a new project metadata configuration
    pub fn new(
        name: impl Into<String>,
        slug: impl Into<String>,
        project_type: ProjectType,
    ) -> Self {
        Self {
            name: name.into(),
            slug: slug.into(),
            category: None,
            pathway: None,
            description: "Replace with a short description.".to_string(),
            project_type,
        }
    }

    /// Load project metadata from a project directory
    ///
    /// This method:
    /// - Reads frontmatter from README.md for title and description
    /// - Auto-detects project type from file presence
    /// - Derives slug from directory name
    ///
    /// # Arguments
    ///
    /// * `project_path` - Path to the project directory
    ///
    /// # Returns
    ///
    /// ProjectMetadata loaded from the project directory
    pub async fn from_project_directory(
        project_path: impl AsRef<std::path::Path>,
    ) -> crate::error::Result<Self> {
        use crate::metadata::{detect_project_type, parse_frontmatter_from_file};

        let path = project_path.as_ref();

        // Get slug from directory name
        let slug = path
            .file_name()
            .ok_or_else(|| {
                crate::CookbookError::ValidationError("Invalid project path".to_string())
            })?
            .to_str()
            .ok_or_else(|| {
                crate::CookbookError::ValidationError("Invalid UTF-8 in path".to_string())
            })?
            .to_string();

        // Auto-detect project type
        let project_type = detect_project_type(path).await.map_err(|e| {
            crate::CookbookError::ValidationError(format!("Failed to detect project type: {}", e))
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

        // Map project type to pathway
        let pathway = Some(match project_type {
            ProjectType::PolkadotSdk => ProjectPathway::Pallets,
            ProjectType::Solidity => ProjectPathway::Contracts,
            ProjectType::Xcm => ProjectPathway::Xcm,
            ProjectType::Transactions => ProjectPathway::Transactions,
            ProjectType::Networks => ProjectPathway::Networks,
        });

        Ok(Self {
            name: frontmatter.title,
            slug,
            category: None, // Category field is deprecated
            pathway,
            description: frontmatter.description,
            project_type,
        })
    }

    /// Load project metadata from YAML file (deprecated, kept for backward compatibility)
    #[deprecated(note = "Use from_project_directory instead")]
    pub fn from_file(path: &PathBuf) -> crate::error::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: ProjectMetadata = serde_yaml::from_str(&content)?;
        Ok(config)
    }

    /// Save project metadata to YAML file (deprecated)
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
    fn test_project_metadata_new() {
        let config = ProjectMetadata::new("My Project", "my-project", ProjectType::PolkadotSdk);
        assert_eq!(config.name, "My Project");
        assert_eq!(config.slug, "my-project");
        assert_eq!(config.project_type, ProjectType::PolkadotSdk);
        assert_eq!(config.pathway, None);
    }

    #[test]
    fn test_project_type_serialization() {
        let sdk = ProjectType::PolkadotSdk;
        let json = serde_json::to_string(&sdk).unwrap();
        assert_eq!(json, "\"polkadot-sdk\"");

        let solidity = ProjectType::Solidity;
        let json = serde_json::to_string(&solidity).unwrap();
        assert_eq!(json, "\"solidity\"");

        let xcm = ProjectType::Xcm;
        let json = serde_json::to_string(&xcm).unwrap();
        assert_eq!(json, "\"xcm\"");

        let transactions = ProjectType::Transactions;
        let json = serde_json::to_string(&transactions).unwrap();
        assert_eq!(json, "\"transactions\"");

        let networks = ProjectType::Networks;
        let json = serde_json::to_string(&networks).unwrap();
        assert_eq!(json, "\"networks\"");
    }

    #[test]
    fn test_pathway_serialization() {
        let pallets = ProjectPathway::Pallets;
        let json = serde_json::to_string(&pallets).unwrap();
        assert_eq!(json, "\"pallets\"");

        let contracts = ProjectPathway::Contracts;
        let json = serde_json::to_string(&contracts).unwrap();
        assert_eq!(json, "\"contracts\"");

        let transactions = ProjectPathway::Transactions;
        let json = serde_json::to_string(&transactions).unwrap();
        assert_eq!(json, "\"transactions\"");

        let xcm = ProjectPathway::Xcm;
        let json = serde_json::to_string(&xcm).unwrap();
        assert_eq!(json, "\"xcm\"");

        let networks = ProjectPathway::Networks;
        let json = serde_json::to_string(&networks).unwrap();
        assert_eq!(json, "\"networks\"");
    }

    #[test]
    fn test_project_type_clone() {
        let original = ProjectType::PolkadotSdk;
        let cloned = original.clone();
        assert_eq!(original, cloned);
    }

    #[test]
    fn test_project_pathway_clone() {
        let original = ProjectPathway::Pallets;
        let cloned = original.clone();
        assert_eq!(original, cloned);
    }

    #[test]
    fn test_project_type_copy() {
        let original = ProjectType::PolkadotSdk;
        let copied = original;
        assert_eq!(original, copied);
    }

    #[test]
    fn test_project_pathway_copy() {
        let original = ProjectPathway::Pallets;
        let copied = original;
        assert_eq!(original, copied);
    }

    #[test]
    fn test_project_type_debug() {
        let sdk = ProjectType::PolkadotSdk;
        let debug_str = format!("{:?}", sdk);
        assert!(debug_str.contains("PolkadotSdk"));
    }

    #[test]
    fn test_project_pathway_debug() {
        let pallets = ProjectPathway::Pallets;
        let debug_str = format!("{:?}", pallets);
        assert!(debug_str.contains("Pallets"));
    }

    #[test]
    fn test_project_metadata_with_pathway() {
        let mut config = ProjectMetadata::new("My Project", "my-project", ProjectType::PolkadotSdk);
        config.pathway = Some(ProjectPathway::Pallets);

        assert_eq!(config.pathway, Some(ProjectPathway::Pallets));
    }

    #[test]
    fn test_project_metadata_with_description() {
        let mut config = ProjectMetadata::new("My Project", "my-project", ProjectType::PolkadotSdk);
        config.description = "Test description".to_string();

        assert_eq!(config.description, "Test description");
    }

    #[test]
    fn test_project_metadata_yaml_serialization() {
        let config = ProjectMetadata::new("My Project", "my-project", ProjectType::PolkadotSdk);

        #[allow(deprecated)]
        let yaml = config.to_yaml().unwrap();

        assert!(yaml.contains("name: My Project"));
        assert!(yaml.contains("slug: my-project"));
        assert!(yaml.contains("type: polkadot-sdk"));
    }

    #[tokio::test]
    async fn test_from_project_directory() {
        use std::fs;
        use tempfile::TempDir;

        // Create a temporary directory
        let temp_dir = TempDir::new().unwrap();
        let project_path = temp_dir.path().join("test-project");
        fs::create_dir(&project_path).unwrap();

        // Create a README.md with frontmatter
        let readme_content = r#"---
title: Test Project
description: A test project for testing
---

# Test Project

This is a test project.
"#;
        fs::write(project_path.join("README.md"), readme_content).unwrap();

        // Create a Cargo.toml to indicate this is a Polkadot SDK project
        fs::write(
            project_path.join("Cargo.toml"),
            "[package]\nname = \"test\"",
        )
        .unwrap();
        fs::create_dir(project_path.join("pallets")).unwrap();

        // Test loading the project metadata
        let config = ProjectMetadata::from_project_directory(&project_path)
            .await
            .unwrap();

        assert_eq!(config.name, "Test Project");
        assert_eq!(config.slug, "test-project");
        assert_eq!(config.description, "A test project for testing");
        assert_eq!(config.project_type, ProjectType::PolkadotSdk);
        assert_eq!(config.pathway, Some(ProjectPathway::Pallets));
    }

    #[tokio::test]
    async fn test_from_project_directory_solidity() {
        use std::fs;
        use tempfile::TempDir;

        // Create a temporary directory for Solidity project
        let temp_dir = TempDir::new().unwrap();
        let project_path = temp_dir.path().join("solidity-project");
        fs::create_dir(&project_path).unwrap();

        // Create a README.md with frontmatter
        let readme_content = r#"---
title: Solidity Contract
description: A Solidity smart contract project
---

# Solidity Contract
"#;
        fs::write(project_path.join("README.md"), readme_content).unwrap();

        // Create package.json with hardhat dependency to indicate Solidity project
        let package_json = r#"{
  "name": "solidity-project",
  "dependencies": {
    "hardhat": "^2.0.0"
  }
}"#;
        fs::write(project_path.join("package.json"), package_json).unwrap();

        // Test loading the project metadata
        let config = ProjectMetadata::from_project_directory(&project_path)
            .await
            .unwrap();

        assert_eq!(config.name, "Solidity Contract");
        assert_eq!(config.slug, "solidity-project");
        assert_eq!(config.description, "A Solidity smart contract project");
        assert_eq!(config.project_type, ProjectType::Solidity);
        assert_eq!(config.pathway, Some(ProjectPathway::Contracts));
    }
}
