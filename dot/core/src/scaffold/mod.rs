//! Recipe scaffolding module
//!
//! This module provides functionality for creating new recipes,
//! including directory structure, template files, and initial configuration.

use crate::config::{ProjectConfig, ProjectInfo, RecipeType};
use crate::error::{CookbookError, Result};
use std::path::Path;
use tracing::{debug, info, warn};

pub mod bootstrap;

pub use bootstrap::Bootstrap;

/// Scaffold manager for creating new projects
pub struct Scaffold {
    /// Whether to perform dry-run (no file writes)
    dry_run: bool,
}

impl Scaffold {
    /// Create a new Scaffold instance
    ///
    /// # Example
    /// ```
    /// use polkadot_cookbook_core::scaffold::Scaffold;
    ///
    /// let scaffold = Scaffold::new();
    /// ```
    pub fn new() -> Self {
        Self { dry_run: false }
    }

    /// Create a scaffold in dry-run mode (no file writes)
    pub fn dry_run() -> Self {
        Self { dry_run: true }
    }

    /// Read Rust version from rust-toolchain.toml file
    ///
    /// Attempts to read the Rust toolchain version from the repository's
    /// rust-toolchain.toml file. Falls back to "1.86" if the file cannot
    /// be read or parsed.
    async fn read_rust_version() -> String {
        let toolchain_path = Path::new("rust-toolchain.toml");

        match tokio::fs::read_to_string(toolchain_path).await {
            Ok(content) => {
                // Simple parser: find line with channel = "X.XX"
                for line in content.lines() {
                    let line = line.trim();
                    if line.starts_with("channel") {
                        // Extract version from: channel = "1.86"
                        if let Some(version) = line
                            .split('=')
                            .nth(1)
                            .and_then(|v| v.trim().trim_matches('"').split_whitespace().next())
                        {
                            debug!("Read Rust version from rust-toolchain.toml: {}", version);
                            return version.to_string();
                        }
                    }
                }
                warn!("Could not parse Rust version from rust-toolchain.toml, using default");
                "1.86".to_string()
            }
            Err(e) => {
                warn!(
                    "Failed to read rust-toolchain.toml: {}, using default rust version",
                    e
                );
                "1.86".to_string()
            }
        }
    }

    /// Create a complete project from configuration
    ///
    /// # Example
    /// ```no_run
    /// use polkadot_cookbook_core::{
    ///     config::ProjectConfig,
    ///     scaffold::Scaffold,
    /// };
    /// use std::path::PathBuf;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = ProjectConfig::new("my-tutorial")
    ///     .with_destination(PathBuf::from("./tutorials"));
    ///
    /// let scaffold = Scaffold::new();
    /// let project_info = scaffold.create_project(config).await?;
    ///
    /// println!("Created project: {}", project_info.slug);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create_project(&self, config: ProjectConfig) -> Result<ProjectInfo> {
        info!("Creating project: {}", config.slug);

        // Validate configuration
        let warnings = crate::config::validate_project_config(&config)?;
        for warning in warnings {
            warn!("{}", warning);
        }

        // Read rust version from rust-toolchain.toml for templates
        let rust_version = Self::read_rust_version().await;
        debug!("Using rust version: {}", rust_version);

        let project_path = config.project_path();

        // Create git branch if requested
        let git_branch = if config.git_init {
            match crate::git::GitOperations::create_branch(&config.slug).await {
                Ok(branch) => {
                    info!("Created git branch: {}", branch);
                    Some(branch)
                }
                Err(e) => {
                    warn!("Failed to create git branch: {}", e);
                    None
                }
            }
        } else {
            None
        };

        // Create directory structure
        self.create_directories(&project_path, config.recipe_type)
            .await?;

        // Generate and write template files
        self.create_files(&project_path, &config, &rust_version)
            .await?;

        // Bootstrap test environment if not skipped
        // Note: Only TypeScript-based recipes need npm install
        if !config.skip_install
            && matches!(
                config.recipe_type,
                RecipeType::Solidity
                    | RecipeType::Xcm
                    | RecipeType::BasicInteraction
                    | RecipeType::Testing
            )
        {
            let bootstrap = Bootstrap::new(project_path.clone());
            bootstrap.setup(&config.slug).await?;
        } else if matches!(config.recipe_type, RecipeType::PolkadotSdk) {
            info!("Skipping npm install for Polkadot SDK recipe (Rust-based)");
        } else {
            info!("Skipping npm install (skip_install = true)");
        }

        info!("Successfully created project: {}", config.slug);

        Ok(ProjectInfo {
            slug: config.slug.clone(),
            title: config.title.clone(),
            project_path,
            git_branch,
        })
    }

    /// Create the directory structure for a project
    async fn create_directories(&self, project_path: &Path, recipe_type: RecipeType) -> Result<()> {
        debug!(
            "Creating directory structure at: {} for recipe type: {:?}",
            project_path.display(),
            recipe_type
        );

        let directories = match recipe_type {
            RecipeType::PolkadotSdk => {
                // For Rust-based recipes, we'll copy from template
                vec![project_path.to_path_buf()]
            }
            RecipeType::Xcm => {
                // For XCM recipes with Chopsticks
                // Template will create src/ and tests/ directories
                vec![project_path.to_path_buf()]
            }
            RecipeType::BasicInteraction => {
                // For basic interaction recipes (TypeScript + PAPI)
                // Template will create src/ and tests/ directories
                vec![project_path.to_path_buf()]
            }
            RecipeType::Testing => {
                // For testing infrastructure recipes (Zombienet/Chopsticks configs)
                // Template will create configs/, scripts/, tests/ directories
                vec![project_path.to_path_buf()]
            }
            RecipeType::Solidity => {
                // For Solidity recipes
                vec![
                    project_path.to_path_buf(),
                    project_path.join("tests"),
                    project_path.join("scripts"),
                    project_path.join("src"),
                ]
            }
        };

        for dir in directories {
            if self.dry_run {
                info!("Would create directory: {}", dir.display());
            } else {
                tokio::fs::create_dir_all(&dir).await.map_err(|e| {
                    CookbookError::FileSystemError {
                        message: format!("Failed to create directory: {e}"),
                        path: Some(dir.clone()),
                    }
                })?;
                debug!("Created directory: {}", dir.display());
            }
        }

        Ok(())
    }

    /// Create template files for a project
    async fn create_files(
        &self,
        project_path: &Path,
        config: &ProjectConfig,
        rust_version: &str,
    ) -> Result<()> {
        debug!("Creating template files in: {}", project_path.display());

        match config.recipe_type {
            RecipeType::PolkadotSdk => {
                self.create_polkadot_sdk_files(project_path, config, rust_version)
                    .await?;
            }
            RecipeType::Xcm => {
                self.create_xcm_files(project_path, config, rust_version)
                    .await?;
            }
            RecipeType::BasicInteraction => {
                self.create_basic_interaction_files(project_path, config, rust_version)
                    .await?;
            }
            RecipeType::Testing => {
                self.create_testing_files(project_path, config, rust_version)
                    .await?;
            }
            RecipeType::Solidity => {
                self.create_solidity_files(project_path, config, rust_version)
                    .await?;
            }
        }

        Ok(())
    }

    /// Create files for Polkadot SDK (Rust-based) recipes
    async fn create_polkadot_sdk_files(
        &self,
        project_path: &Path,
        config: &ProjectConfig,
        rust_version: &str,
    ) -> Result<()> {
        debug!("Creating Polkadot SDK template files");

        // Build absolute path to template directory
        let manifest_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let template_dir = manifest_dir.join("templates/recipe-templates/polkadot-sdk-template");

        self.copy_template_dir(&template_dir, project_path, config, rust_version)
            .await?;

        Ok(())
    }

    /// Create files for XCM recipes (TypeScript with Chopsticks)
    async fn create_xcm_files(
        &self,
        project_path: &Path,
        config: &ProjectConfig,
        rust_version: &str,
    ) -> Result<()> {
        debug!("Creating XCM template files");

        // Build absolute path to template directory
        let manifest_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let template_dir = manifest_dir.join("templates/recipe-templates/xcm-template");

        self.copy_template_dir(&template_dir, project_path, config, rust_version)
            .await?;

        Ok(())
    }

    /// Create files for Solidity recipes (TypeScript)
    async fn create_solidity_files(
        &self,
        project_path: &Path,
        config: &ProjectConfig,
        rust_version: &str,
    ) -> Result<()> {
        debug!("Creating Solidity template files");

        // Build absolute path to template directory
        let manifest_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let template_dir = manifest_dir.join("templates/recipe-templates/solidity-template");

        self.copy_template_dir(&template_dir, project_path, config, rust_version)
            .await?;
        Ok(())
    }

    /// Create files for Basic Interaction recipes (TypeScript with PAPI)
    async fn create_basic_interaction_files(
        &self,
        project_path: &Path,
        config: &ProjectConfig,
        rust_version: &str,
    ) -> Result<()> {
        debug!("Creating Basic Interaction template files");

        // Build absolute path to template directory
        let manifest_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let template_dir =
            manifest_dir.join("templates/recipe-templates/basic-interaction-template");

        self.copy_template_dir(&template_dir, project_path, config, rust_version)
            .await?;
        Ok(())
    }

    /// Create files for Testing Infrastructure recipes (Zombienet/Chopsticks)
    async fn create_testing_files(
        &self,
        project_path: &Path,
        config: &ProjectConfig,
        rust_version: &str,
    ) -> Result<()> {
        debug!("Creating Testing Infrastructure template files");

        // Build absolute path to template directory
        let manifest_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let template_dir = manifest_dir.join("templates/recipe-templates/testing-template");

        self.copy_template_dir(&template_dir, project_path, config, rust_version)
            .await?;
        Ok(())
    }

    /// Copy template directory recursively, replacing placeholders
    fn copy_template_dir<'a>(
        &'a self,
        template_dir: &'a Path,
        dest_dir: &'a Path,
        config: &'a ProjectConfig,
        rust_version: &'a str,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<()>> + 'a>> {
        Box::pin(async move {
            debug!(
                "Copying template from {} to {}",
                template_dir.display(),
                dest_dir.display()
            );

            // Helper function to process file content
            let process_content = |content: String,
                                   config: &ProjectConfig,
                                   rust_version: &str|
             -> String {
                // Format optional fields as YAML lines or empty strings
                let pathway_line = config
                    .pathway
                    .as_ref()
                    .map(|p| {
                        let value = match p {
                            crate::config::RecipePathway::Runtime => "runtime",
                            crate::config::RecipePathway::Contracts => "contracts",
                            crate::config::RecipePathway::BasicInteraction => "basic-interaction",
                            crate::config::RecipePathway::Xcm => "xcm",
                            crate::config::RecipePathway::Testing => "testing",
                            crate::config::RecipePathway::RequestNew => {
                                unreachable!("RequestNew pathway should never reach scaffold code")
                            }
                        };
                        format!("pathway: {value}")
                    })
                    .unwrap_or_default();

                let content_type_line = config
                    .content_type
                    .as_ref()
                    .map(|ct| {
                        let value = match ct {
                            crate::config::ContentType::Tutorial => "tutorial",
                            crate::config::ContentType::Guide => "guide",
                        };
                        format!("content_type: {value}")
                    })
                    .unwrap_or_default();

                let difficulty_line = config
                    .difficulty
                    .as_ref()
                    .map(|d| {
                        let value = match d {
                            crate::config::Difficulty::Beginner => "beginner",
                            crate::config::Difficulty::Intermediate => "intermediate",
                            crate::config::Difficulty::Advanced => "advanced",
                        };
                        format!("difficulty: {value}")
                    })
                    .unwrap_or_default();

                content
                    .replace("{{slug}}", &config.slug)
                    .replace("{{title}}", &config.title)
                    .replace("{{description}}", &config.description)
                    .replace("{{category}}", &config.category)
                    .replace("{{rust_version}}", rust_version)
                    .replace("{{pathway}}", &pathway_line)
                    .replace("{{content_type}}", &content_type_line)
                    .replace("{{difficulty}}", &difficulty_line)
            };

            // Recursive copy function
            let mut entries = tokio::fs::read_dir(template_dir).await.map_err(|e| {
                CookbookError::FileSystemError {
                    message: format!("Failed to read template directory: {e}"),
                    path: Some(template_dir.to_path_buf()),
                }
            })?;

            while let Some(entry) =
                entries
                    .next_entry()
                    .await
                    .map_err(|e| CookbookError::FileSystemError {
                        message: format!("Failed to read directory entry: {e}"),
                        path: Some(template_dir.to_path_buf()),
                    })?
            {
                let path = entry.path();
                let file_name = entry.file_name();
                let file_name_str = file_name.to_string_lossy();

                // Skip hidden files
                if file_name_str.starts_with('.') {
                    continue;
                }

                // Handle content-type-specific README templates
                if file_name_str == "README.tutorial.md.template"
                    || file_name_str == "README.guide.md.template"
                {
                    let is_tutorial = file_name_str == "README.tutorial.md.template";
                    let config_is_tutorial = config
                        .content_type
                        .as_ref()
                        .map(|ct| matches!(ct, crate::config::ContentType::Tutorial))
                        .unwrap_or(true); // Default to tutorial if not specified

                    // Skip if content type doesn't match
                    if is_tutorial != config_is_tutorial {
                        continue;
                    }

                    // Use matching template as README.md
                    let dest_path = dest_dir.join("README.md");
                    let content = tokio::fs::read_to_string(&path).await.map_err(|e| {
                        CookbookError::FileSystemError {
                            message: format!("Failed to read template file: {e}"),
                            path: Some(path.clone()),
                        }
                    })?;
                    let processed_content = process_content(content, config, rust_version);
                    self.write_file(&dest_path, &processed_content).await?;
                    continue;
                }

                let dest_path = if file_name_str.ends_with(".template") {
                    // Remove .template extension
                    let new_name = file_name_str.trim_end_matches(".template");
                    dest_dir.join(new_name)
                } else {
                    dest_dir.join(&file_name)
                };

                if path.is_dir() {
                    // Recursively copy directories
                    if self.dry_run {
                        info!("Would create directory: {}", dest_path.display());
                    } else {
                        tokio::fs::create_dir_all(&dest_path).await.map_err(|e| {
                            CookbookError::FileSystemError {
                                message: format!("Failed to create directory: {e}"),
                                path: Some(dest_path.clone()),
                            }
                        })?;
                    }
                    self.copy_template_dir(&path, &dest_path, config, rust_version)
                        .await?;
                } else {
                    // Copy and process files
                    let content = tokio::fs::read_to_string(&path).await.map_err(|e| {
                        CookbookError::FileSystemError {
                            message: format!("Failed to read template file: {e}"),
                            path: Some(path.clone()),
                        }
                    })?;

                    let processed_content = process_content(content, config, rust_version);
                    self.write_file(&dest_path, &processed_content).await?;
                }
            }

            Ok(())
        })
    }

    /// Write a file (or simulate in dry-run mode)
    async fn write_file(&self, path: &Path, content: &str) -> Result<()> {
        if self.dry_run {
            info!("Would write file: {}", path.display());
            Ok(())
        } else {
            tokio::fs::write(path, content)
                .await
                .map_err(|e| CookbookError::FileSystemError {
                    message: format!("Failed to write file: {e}"),
                    path: Some(path.to_path_buf()),
                })?;
            debug!("Wrote file: {}", path.display());
            Ok(())
        }
    }

    /// Verify that all required files were created
    pub async fn verify_setup(&self, project_path: &Path) -> Result<Vec<String>> {
        debug!("Verifying setup at: {}", project_path.display());

        let required_files = vec![
            project_path.join("package.json"),
            project_path.join("README.md"),
            project_path.join("recipe.config.yml"),
        ];

        let mut missing = Vec::new();

        for file in required_files {
            if !tokio::fs::try_exists(&file).await.unwrap_or(false) {
                missing.push(file.display().to_string());
            }
        }

        if missing.is_empty() {
            info!("All files created successfully");
            Ok(vec![])
        } else {
            warn!("Some files are missing: {:?}", missing);
            Ok(missing)
        }
    }
}

impl Default for Scaffold {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{ContentType, Difficulty, RecipePathway, RecipeType};
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_create_directories() {
        let temp_dir = TempDir::new().unwrap();
        let project_path = temp_dir.path().join("test-project");

        let scaffold = Scaffold::new();
        scaffold
            .create_directories(&project_path, RecipeType::Solidity)
            .await
            .unwrap();

        assert!(project_path.exists());
        assert!(project_path.join("tests").exists());
        assert!(project_path.join("scripts").exists());
        assert!(project_path.join("src").exists());
    }

    #[tokio::test]
    async fn test_dry_run_mode() {
        let temp_dir = TempDir::new().unwrap();
        let project_path = temp_dir.path().join("dry-run-project");

        let scaffold = Scaffold::dry_run();
        scaffold
            .create_directories(&project_path, RecipeType::Solidity)
            .await
            .unwrap();

        // In dry-run mode, directories should NOT be created
        assert!(!project_path.exists());
    }

    #[tokio::test]
    async fn test_create_files() {
        // This test needs to run from workspace root where templates/ directory exists
        // Change to workspace root for this test
        let manifest_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let workspace_root = manifest_dir.parent().unwrap().parent().unwrap();
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(workspace_root).unwrap();

        let temp_dir = TempDir::new().unwrap();
        let project_path = temp_dir.path().join("test-files");
        tokio::fs::create_dir_all(&project_path).await.unwrap();

        let config = ProjectConfig::new("test-tutorial");
        let scaffold = Scaffold::new();
        let result = scaffold.create_files(&project_path, &config, "1.86").await;

        // Restore original directory
        std::env::set_current_dir(original_dir).unwrap();

        result.unwrap();

        // Verify core files were created from templates
        assert!(project_path.join("README.md").exists());
        assert!(project_path.join("recipe.config.yml").exists());
        assert!(project_path.join("Cargo.toml").exists());
        assert!(project_path.join("pallets").exists());

        // Verify rust-toolchain.toml was copied for Polkadot SDK recipes
        assert!(
            project_path.join("rust-toolchain.toml").exists(),
            "rust-toolchain.toml should be copied for Polkadot SDK recipes"
        );

        // Verify content of rust-toolchain.toml
        let toolchain_content =
            std::fs::read_to_string(project_path.join("rust-toolchain.toml")).unwrap();
        assert!(
            toolchain_content.contains("channel = \"1.86\""),
            "rust-toolchain.toml should specify Rust 1.86"
        );
        assert!(toolchain_content.contains("components = [\"rustfmt\", \"clippy\"]"));
        assert!(toolchain_content.contains("profile = \"minimal\""));
    }

    #[test]
    fn test_scaffold_default() {
        let scaffold = Scaffold::default();
        assert!(!scaffold.dry_run);
    }

    #[tokio::test]
    async fn test_create_basic_interaction_recipe() {
        let manifest_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let workspace_root = manifest_dir.parent().unwrap().parent().unwrap();
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(workspace_root).unwrap();

        let temp_dir = TempDir::new().unwrap();
        let project_path = temp_dir.path().join("basic-interaction-test");
        tokio::fs::create_dir_all(&project_path).await.unwrap();

        let config = ProjectConfig::new("basic-interaction-test")
            .with_recipe_type(RecipeType::BasicInteraction);

        let scaffold = Scaffold::new();
        let result = scaffold.create_files(&project_path, &config, "1.86").await;

        std::env::set_current_dir(original_dir).unwrap();
        result.unwrap();

        // Verify TypeScript-based structure was created
        assert!(project_path.join("package.json").exists());
        assert!(project_path.join("tsconfig.json").exists());
        assert!(project_path.join("vitest.config.ts").exists());
        assert!(project_path.join("recipe.config.yml").exists());
        assert!(project_path.join("src").exists());
        assert!(project_path.join("tests").exists());
    }

    #[tokio::test]
    async fn test_create_testing_recipe() {
        let manifest_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let workspace_root = manifest_dir.parent().unwrap().parent().unwrap();
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(workspace_root).unwrap();

        let temp_dir = TempDir::new().unwrap();
        let project_path = temp_dir.path().join("testing-recipe-test");
        tokio::fs::create_dir_all(&project_path).await.unwrap();

        let config =
            ProjectConfig::new("testing-recipe-test").with_recipe_type(RecipeType::Testing);

        let scaffold = Scaffold::new();
        let result = scaffold.create_files(&project_path, &config, "1.86").await;

        std::env::set_current_dir(original_dir).unwrap();
        result.unwrap();

        // Verify testing infrastructure was created
        assert!(project_path.join("recipe.config.yml").exists());
        assert!(project_path.join("configs").exists());
        assert!(project_path.join("tests").exists());
    }

    #[tokio::test]
    async fn test_create_xcm_recipe() {
        let manifest_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let workspace_root = manifest_dir.parent().unwrap().parent().unwrap();
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(workspace_root).unwrap();

        let temp_dir = TempDir::new().unwrap();
        let project_path = temp_dir.path().join("xcm-test");
        tokio::fs::create_dir_all(&project_path).await.unwrap();

        let config = ProjectConfig::new("xcm-test").with_recipe_type(RecipeType::Xcm);

        let scaffold = Scaffold::new();
        let result = scaffold.create_files(&project_path, &config, "1.86").await;

        std::env::set_current_dir(original_dir).unwrap();
        result.unwrap();

        // Verify XCM/Chopsticks structure was created
        assert!(project_path.join("package.json").exists());
        assert!(project_path.join("chopsticks.yml").exists());
        assert!(project_path.join("recipe.config.yml").exists());

        // Verify rust-toolchain.toml was NOT copied for TypeScript recipes
        assert!(
            !project_path.join("rust-toolchain.toml").exists(),
            "rust-toolchain.toml should NOT exist for XCM (TypeScript) recipes"
        );
    }

    #[tokio::test]
    async fn test_optional_fields_in_recipe_config() {
        let manifest_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let workspace_root = manifest_dir.parent().unwrap().parent().unwrap();
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(workspace_root).unwrap();

        let temp_dir = TempDir::new().unwrap();
        let project_path = temp_dir.path().join("optional-fields-test");
        tokio::fs::create_dir_all(&project_path).await.unwrap();

        let config = ProjectConfig::new("optional-fields-test")
            .with_recipe_type(RecipeType::PolkadotSdk)
            .with_pathway(RecipePathway::Runtime)
            .with_content_type(ContentType::Tutorial)
            .with_difficulty(Difficulty::Beginner);

        let scaffold = Scaffold::new();
        scaffold
            .create_files(&project_path, &config, "1.86")
            .await
            .unwrap();

        std::env::set_current_dir(original_dir).unwrap();

        // Read the generated recipe.config.yml
        let config_path = project_path.join("recipe.config.yml");
        let content = tokio::fs::read_to_string(&config_path).await.unwrap();

        // Verify optional fields are present
        assert!(content.contains("pathway: runtime"));
        assert!(content.contains("content_type: tutorial"));
        assert!(content.contains("difficulty: beginner"));
    }

    #[tokio::test]
    async fn test_optional_fields_empty_when_not_provided() {
        let manifest_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let workspace_root = manifest_dir.parent().unwrap().parent().unwrap();
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(workspace_root).unwrap();

        let temp_dir = TempDir::new().unwrap();
        let project_path = temp_dir.path().join("no-optional-fields-test");
        tokio::fs::create_dir_all(&project_path).await.unwrap();

        // Create config WITHOUT optional fields
        let config =
            ProjectConfig::new("no-optional-fields-test").with_recipe_type(RecipeType::PolkadotSdk);

        let scaffold = Scaffold::new();
        scaffold
            .create_files(&project_path, &config, "1.86")
            .await
            .unwrap();

        std::env::set_current_dir(original_dir).unwrap();

        // Read the generated recipe.config.yml
        let config_path = project_path.join("recipe.config.yml");
        let content = tokio::fs::read_to_string(&config_path).await.unwrap();

        // Verify optional fields are NOT present (or are empty lines)
        // They should not have "pathway: ", "content_type: ", or "difficulty: " with values
        let has_pathway_value = content.contains("pathway: runtime")
            || content.contains("pathway: contracts")
            || content.contains("pathway: basic-interaction")
            || content.contains("pathway: xcm")
            || content.contains("pathway: testing");
        assert!(
            !has_pathway_value,
            "Should not have pathway value when not provided"
        );
    }

    #[tokio::test]
    async fn test_read_rust_version_scenarios() {
        use tempfile::TempDir;

        // Run all scenarios sequentially in one test to avoid parallel directory changes
        let original_dir = std::env::current_dir().unwrap();

        // Scenario 1: Valid file with version 1.85
        {
            let temp_dir = TempDir::new().unwrap();
            std::env::set_current_dir(temp_dir.path()).unwrap();

            let toolchain_content = r#"[toolchain]
channel = "1.85"
components = ["rustfmt", "clippy"]
profile = "minimal"
"#;
            tokio::fs::write("rust-toolchain.toml", toolchain_content)
                .await
                .unwrap();

            let version = Scaffold::read_rust_version().await;
            assert_eq!(
                version, "1.85",
                "Should read version 1.85 from rust-toolchain.toml"
            );
        }

        // Scenario 2: Missing file should fallback to 1.86
        {
            let temp_dir = TempDir::new().unwrap();
            std::env::set_current_dir(temp_dir.path()).unwrap();

            // No rust-toolchain.toml exists
            let version = Scaffold::read_rust_version().await;
            assert_eq!(
                version, "1.86",
                "Should fallback to 1.86 when file is missing"
            );
        }

        // Scenario 3: Invalid format should fallback to 1.86
        {
            let temp_dir = TempDir::new().unwrap();
            std::env::set_current_dir(temp_dir.path()).unwrap();

            let toolchain_content = r#"[toolchain]
invalid = "1.85"
components = ["rustfmt", "clippy"]
"#;
            tokio::fs::write("rust-toolchain.toml", toolchain_content)
                .await
                .unwrap();

            let version = Scaffold::read_rust_version().await;
            assert_eq!(
                version, "1.86",
                "Should fallback to 1.86 when format is invalid"
            );
        }

        // Scenario 4: Different spacing around equals
        {
            let temp_dir = TempDir::new().unwrap();
            std::env::set_current_dir(temp_dir.path()).unwrap();

            let toolchain_content = r#"[toolchain]
channel   =   "1.87"
components = ["rustfmt", "clippy"]
"#;
            tokio::fs::write("rust-toolchain.toml", toolchain_content)
                .await
                .unwrap();

            let version = Scaffold::read_rust_version().await;
            assert_eq!(version, "1.87", "Should handle spaces around equals sign");
        }

        // Scenario 5: Stable channel
        {
            let temp_dir = TempDir::new().unwrap();
            std::env::set_current_dir(temp_dir.path()).unwrap();

            let toolchain_content = r#"[toolchain]
channel = "stable"
components = ["rustfmt", "clippy"]
profile = "minimal"
"#;
            tokio::fs::write("rust-toolchain.toml", toolchain_content)
                .await
                .unwrap();

            let version = Scaffold::read_rust_version().await;
            assert_eq!(version, "stable", "Should correctly read 'stable' channel");
        }

        // Restore original directory
        std::env::set_current_dir(original_dir).unwrap();
    }
}
