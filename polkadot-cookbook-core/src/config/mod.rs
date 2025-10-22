/// Configuration management for Polkadot Cookbook projects
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

pub mod tutorial;
pub mod validation;

pub use tutorial::{TutorialConfig, TutorialType};
pub use validation::{
    is_valid_slug, slug_to_title, validate_project_config, validate_slug,
    validate_working_directory,
};

/// Configuration for creating a new project
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectConfig {
    /// Project slug (lowercase, dash-separated)
    pub slug: String,

    /// Human-readable project title
    pub title: String,

    /// Destination directory (usually "tutorials/")
    pub destination: PathBuf,

    /// Whether to initialize git repository
    pub git_init: bool,

    /// Whether to skip npm install
    pub skip_install: bool,

    /// Tutorial type
    pub tutorial_type: TutorialType,

    /// Tutorial category
    pub category: String,

    /// Whether the tutorial needs a running node
    pub needs_node: bool,
}

impl ProjectConfig {
    /// Create a new project configuration with defaults
    ///
    /// # Example
    /// ```
    /// use polkadot_cookbook_core::config::ProjectConfig;
    ///
    /// let config = ProjectConfig::new("my-tutorial");
    /// assert_eq!(config.slug, "my-tutorial");
    /// assert_eq!(config.title, "My Tutorial");
    /// ```
    pub fn new(slug: impl Into<String>) -> Self {
        let slug = slug.into();
        let title = slug_to_title(&slug);

        Self {
            slug,
            title,
            destination: PathBuf::from("tutorials"),
            git_init: true,
            skip_install: false,
            tutorial_type: TutorialType::Sdk,
            category: "polkadot-sdk-cookbook".to_string(),
            needs_node: true,
        }
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

    /// Set tutorial type
    pub fn with_tutorial_type(mut self, tutorial_type: TutorialType) -> Self {
        self.tutorial_type = tutorial_type;
        self
    }

    /// Set tutorial category
    pub fn with_category(mut self, category: impl Into<String>) -> Self {
        self.category = category.into();
        self
    }

    /// Set needs_node flag
    pub fn with_needs_node(mut self, needs_node: bool) -> Self {
        self.needs_node = needs_node;
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

    /// Git branch name (if created)
    pub git_branch: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_project_config_new() {
        let config = ProjectConfig::new("my-tutorial");
        assert_eq!(config.slug, "my-tutorial");
        assert_eq!(config.title, "My Tutorial");
        assert_eq!(config.destination, PathBuf::from("tutorials"));
        assert!(config.git_init);
        assert!(!config.skip_install);
    }

    #[test]
    fn test_project_config_builder() {
        let config = ProjectConfig::new("test-project")
            .with_destination(PathBuf::from("/tmp/projects"))
            .with_git_init(false)
            .with_skip_install(true)
            .with_tutorial_type(TutorialType::Contracts)
            .with_category("advanced")
            .with_needs_node(false);

        assert_eq!(config.slug, "test-project");
        assert_eq!(config.destination, PathBuf::from("/tmp/projects"));
        assert!(!config.git_init);
        assert!(config.skip_install);
        assert!(matches!(config.tutorial_type, TutorialType::Contracts));
        assert_eq!(config.category, "advanced");
        assert!(!config.needs_node);
    }

    #[test]
    fn test_project_path() {
        let config = ProjectConfig::new("my-tutorial");
        assert_eq!(
            config.project_path(),
            PathBuf::from("tutorials/my-tutorial")
        );
    }
}
