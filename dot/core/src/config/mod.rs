/// Configuration management for Polkadot Cookbook recipes
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Recipe configuration types and utilities
pub mod recipe;
/// Recipe validation utilities
pub mod validation;

pub use recipe::{ContentType, Difficulty, RecipeConfig, RecipePathway, RecipeType};
pub use validation::{
    is_valid_slug, slug_to_title, title_to_slug, validate_project_config, validate_slug,
    validate_title, validate_working_directory,
};

/// Configuration for creating a new recipe
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectConfig {
    /// Recipe slug (lowercase, dash-separated)
    pub slug: String,

    /// Human-readable recipe title
    pub title: String,

    /// Destination directory (usually "recipes/")
    pub destination: PathBuf,

    /// Whether to initialize git repository
    pub git_init: bool,

    /// Whether to skip npm install
    pub skip_install: bool,

    /// Recipe type
    pub recipe_type: RecipeType,

    /// Recipe category
    pub category: String,

    /// Whether the recipe needs a running node
    pub needs_node: bool,

    /// Recipe description
    pub description: String,

    /// Recipe pathway (optional)
    pub pathway: Option<RecipePathway>,

    /// Content type (optional)
    pub content_type: Option<ContentType>,

    /// Difficulty level (optional)
    pub difficulty: Option<Difficulty>,
}

impl ProjectConfig {
    /// Create a new recipe configuration with defaults
    ///
    /// # Example
    /// ```
    /// use polkadot_cookbook_core::config::ProjectConfig;
    ///
    /// let config = ProjectConfig::new("my-recipe");
    /// assert_eq!(config.slug, "my-recipe");
    /// assert_eq!(config.title, "My Recipe");
    /// ```
    pub fn new(slug: impl Into<String>) -> Self {
        let slug = slug.into();
        let title = slug_to_title(&slug);

        Self {
            slug,
            title,
            destination: PathBuf::from("recipes"),
            git_init: true,
            skip_install: false,
            recipe_type: RecipeType::PolkadotSdk,
            category: "polkadot-sdk-cookbook".to_string(),
            needs_node: true,
            description: "Replace with a short description.".to_string(),
            pathway: None,
            content_type: None,
            difficulty: None,
        }
    }

    /// Set the recipe title
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

    /// Set recipe type
    pub fn with_recipe_type(mut self, recipe_type: RecipeType) -> Self {
        self.recipe_type = recipe_type;
        self
    }

    /// Set recipe category
    pub fn with_category(mut self, category: impl Into<String>) -> Self {
        self.category = category.into();
        self
    }

    /// Set needs_node flag
    pub fn with_needs_node(mut self, needs_node: bool) -> Self {
        self.needs_node = needs_node;
        self
    }

    /// Set recipe description
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }

    /// Set recipe pathway
    pub fn with_pathway(mut self, pathway: RecipePathway) -> Self {
        self.pathway = Some(pathway);
        self
    }

    /// Set content type
    pub fn with_content_type(mut self, content_type: ContentType) -> Self {
        self.content_type = Some(content_type);
        self
    }

    /// Set difficulty level
    pub fn with_difficulty(mut self, difficulty: Difficulty) -> Self {
        self.difficulty = Some(difficulty);
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
        let config = ProjectConfig::new("my-recipe");
        assert_eq!(config.slug, "my-recipe");
        assert_eq!(config.title, "My Recipe");
        assert_eq!(config.destination, PathBuf::from("recipes"));
        assert!(config.git_init);
        assert!(!config.skip_install);
        assert_eq!(config.pathway, None);
        assert_eq!(config.content_type, None);
        assert_eq!(config.difficulty, None);
    }

    #[test]
    fn test_project_config_builder() {
        let config = ProjectConfig::new("test-recipe")
            .with_destination(PathBuf::from("/tmp/recipes"))
            .with_git_init(false)
            .with_skip_install(true)
            .with_recipe_type(RecipeType::Solidity)
            .with_category("advanced")
            .with_needs_node(false)
            .with_pathway(RecipePathway::Contracts)
            .with_content_type(ContentType::Tutorial)
            .with_difficulty(Difficulty::Beginner);

        assert_eq!(config.slug, "test-recipe");
        assert_eq!(config.destination, PathBuf::from("/tmp/recipes"));
        assert!(!config.git_init);
        assert!(config.skip_install);
        assert!(matches!(config.recipe_type, RecipeType::Solidity));
        assert_eq!(config.category, "advanced");
        assert!(!config.needs_node);
        assert_eq!(config.pathway, Some(RecipePathway::Contracts));
        assert_eq!(config.content_type, Some(ContentType::Tutorial));
        assert_eq!(config.difficulty, Some(Difficulty::Beginner));
    }

    #[test]
    fn test_project_path() {
        let config = ProjectConfig::new("my-recipe");
        assert_eq!(config.project_path(), PathBuf::from("recipes/my-recipe"));
    }
}
