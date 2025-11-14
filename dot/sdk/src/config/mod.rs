/// Configuration management for Polkadot projects
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Project type configuration and utilities
pub mod project_types;
/// Project validation utilities
pub mod validation;

pub use project_types::{ProjectMetadata, ProjectPathway, ProjectType};
pub use validation::{
    is_valid_slug, slug_to_title, title_to_slug, validate_lock_files, validate_project_config,
    validate_slug, validate_title,
};

/// Configuration for creating a new project
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectConfig {
    /// Project slug (lowercase, dash-separated)
    pub slug: String,

    /// Human-readable project title
    pub title: String,

    /// Destination directory (usually "recipes/")
    pub destination: PathBuf,

    /// Whether to initialize git repository
    pub git_init: bool,

    /// Whether to skip npm install
    pub skip_install: bool,

    /// Project type
    pub project_type: ProjectType,

    /// Project category
    pub category: String,

    /// Project description
    pub description: String,

    /// Project pathway (optional)
    pub pathway: Option<ProjectPathway>,

    /// Pallet-only mode (no runtime, no PAPI)
    #[serde(default)]
    pub pallet_only: bool,
}

impl ProjectConfig {
    /// Create a new project configuration with defaults
    ///
    /// # Example
    /// ```
    /// use polkadot_cookbook_sdk::config::ProjectConfig;
    ///
    /// let config = ProjectConfig::new("my-project");
    /// assert_eq!(config.slug, "my-project");
    /// assert_eq!(config.title, "My Project");
    /// ```
    pub fn new(slug: impl Into<String>) -> Self {
        let slug = slug.into();
        let title = slug_to_title(&slug);

        Self {
            slug,
            title,
            destination: PathBuf::from("."),
            git_init: true,
            skip_install: false,
            project_type: ProjectType::PolkadotSdk,
            category: "polkadot-sdk-cookbook".to_string(),
            description: "Replace with a short description.".to_string(),
            pathway: None,
            pallet_only: false,
        }
    }

    /// Set the project title
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    /// Set the destination directory
    pub fn with_destination(mut self, destination: PathBuf) -> Self {
        self.destination = destination;
        self
    }

    /// Set git initialization option
    pub fn with_git_init(mut self, git_init: bool) -> Self {
        self.git_init = git_init;
        self
    }

    /// Set skip install option
    pub fn with_skip_install(mut self, skip_install: bool) -> Self {
        self.skip_install = skip_install;
        self
    }

    /// Set project type
    pub fn with_project_type(mut self, project_type: ProjectType) -> Self {
        self.project_type = project_type;
        self
    }

    /// Set project category
    pub fn with_category(mut self, category: impl Into<String>) -> Self {
        self.category = category.into();
        self
    }

    /// Set project description
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }

    /// Set project pathway
    pub fn with_pathway(mut self, pathway: ProjectPathway) -> Self {
        self.pathway = Some(pathway);
        self
    }

    /// Get the full project path
    pub fn project_path(&self) -> PathBuf {
        self.destination.join(&self.slug)
    }
}

/// Information about a created project
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectInfo {
    /// Project slug
    pub slug: String,

    /// Project title
    pub title: String,

    /// Full path to project directory
    pub project_path: PathBuf,

    /// Whether a git repository was initialized
    pub git_initialized: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_project_config_new() {
        let config = ProjectConfig::new("my-project");
        assert_eq!(config.slug, "my-project");
        assert_eq!(config.title, "My Project");
        assert_eq!(config.destination, PathBuf::from("."));
        assert!(config.git_init);
        assert!(!config.skip_install);
        assert_eq!(config.pathway, None);
    }

    #[test]
    fn test_project_config_builder() {
        let config = ProjectConfig::new("test-project")
            .with_destination(PathBuf::from("/tmp/recipes"))
            .with_git_init(false)
            .with_skip_install(true)
            .with_project_type(ProjectType::Solidity)
            .with_category("advanced")
            .with_pathway(ProjectPathway::Contracts);

        assert_eq!(config.slug, "test-project");
        assert_eq!(config.destination, PathBuf::from("/tmp/recipes"));
        assert!(!config.git_init);
        assert!(config.skip_install);
        assert!(matches!(config.project_type, ProjectType::Solidity));
        assert_eq!(config.category, "advanced");
        assert_eq!(config.pathway, Some(ProjectPathway::Contracts));
    }

    #[test]
    fn test_project_path() {
        let config = ProjectConfig::new("my-project");
        assert_eq!(config.project_path(), PathBuf::from("./my-project"));
    }
}
