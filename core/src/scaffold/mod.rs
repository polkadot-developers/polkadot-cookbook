//! Recipe scaffolding module
//!
//! This module provides functionality for creating new recipes,
//! including directory structure, template files, and initial configuration.

use crate::config::{ProjectConfig, ProjectInfo, RecipeType};
use crate::error::{CookbookError, Result};
use crate::templates::{
    JustfileTemplate, ReadmeTemplate, RecipeYmlTemplate, Template, TestTemplate,
    VersionsYmlTemplate,
};
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
        self.create_files(&project_path, &config).await?;

        // Bootstrap test environment if not skipped
        // Note: Only TypeScript-based recipes (Solidity, XCM) need npm install
        if !config.skip_install
            && matches!(config.recipe_type, RecipeType::Solidity | RecipeType::Xcm)
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
            RecipeType::Solidity | RecipeType::Xcm => {
                // For TypeScript-based recipes (Solidity with Hardhat, XCM with Chopsticks)
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
    async fn create_files(&self, project_path: &Path, config: &ProjectConfig) -> Result<()> {
        debug!("Creating template files in: {}", project_path.display());

        match config.recipe_type {
            RecipeType::PolkadotSdk => {
                self.create_polkadot_sdk_files(project_path, config).await?;
            }
            RecipeType::Solidity | RecipeType::Xcm => {
                self.create_typescript_files(project_path, config).await?;
            }
        }

        Ok(())
    }

    /// Create files for Polkadot SDK (Rust-based) recipes
    async fn create_polkadot_sdk_files(
        &self,
        project_path: &Path,
        config: &ProjectConfig,
    ) -> Result<()> {
        debug!("Creating Polkadot SDK template files");

        // Copy template files from templates/recipe-templates/polkadot-sdk-template/
        let template_dir = Path::new("templates/recipe-templates/polkadot-sdk-template");

        self.copy_template_dir(template_dir, project_path, config)
            .await?;

        Ok(())
    }

    /// Create files for TypeScript-based recipes (Solidity, XCM)
    async fn create_typescript_files(
        &self,
        project_path: &Path,
        config: &ProjectConfig,
    ) -> Result<()> {
        debug!("Creating TypeScript template files");

        // Generate justfile
        let justfile_content = JustfileTemplate::new().generate();
        self.write_file(&project_path.join("justfile"), &justfile_content)
            .await?;

        // Generate example test
        let test_content = TestTemplate::new(&config.slug).generate();
        let test_filename = format!("{}-e2e.test.ts", config.slug);
        self.write_file(
            &project_path.join("tests").join(test_filename),
            &test_content,
        )
        .await?;

        // Generate recipe.config.yml
        let recipe_yml_content = RecipeYmlTemplate::new(
            &config.slug,
            &config.title,
            &config.description,
            config.recipe_type,
            &config.category,
            config.needs_node,
        )
        .generate();
        self.write_file(&project_path.join("recipe.config.yml"), &recipe_yml_content)
            .await?;

        // Generate README.md
        let readme_content = ReadmeTemplate::new(&config.slug).generate();
        self.write_file(&project_path.join("README.md"), &readme_content)
            .await?;

        // Generate versions.yml
        let versions_yml_content = VersionsYmlTemplate.generate();
        self.write_file(&project_path.join("versions.yml"), &versions_yml_content)
            .await?;

        // Create .gitkeep in scripts/
        self.write_file(&project_path.join("scripts").join(".gitkeep"), "")
            .await?;

        // Create .gitignore
        let gitignore_content = "node_modules/\ndist/\n*.log\n.DS_Store\ncoverage/\n";
        self.write_file(&project_path.join(".gitignore"), gitignore_content)
            .await?;

        Ok(())
    }

    /// Copy template directory recursively, replacing placeholders
    fn copy_template_dir<'a>(
        &'a self,
        template_dir: &'a Path,
        dest_dir: &'a Path,
        config: &'a ProjectConfig,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<()>> + 'a>> {
        Box::pin(async move {
            debug!(
                "Copying template from {} to {}",
                template_dir.display(),
                dest_dir.display()
            );

            // Helper function to process file content
            let process_content = |content: String, config: &ProjectConfig| -> String {
                content
                    .replace("{{slug}}", &config.slug)
                    .replace("{{title}}", &config.title)
                    .replace("{{description}}", &config.description)
                    .replace("{{rust_version}}", "1.81.0") // TODO: Get from versions.yml
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

                let dest_path = if file_name_str.ends_with(".template") {
                    // Remove .template extension
                    let new_name = file_name_str.trim_end_matches(".template");
                    dest_dir.join(new_name)
                } else {
                    dest_dir.join(&file_name)
                };

                if path.is_dir() {
                    // Recursively copy directories
                    tokio::fs::create_dir_all(&dest_path).await.map_err(|e| {
                        CookbookError::FileSystemError {
                            message: format!("Failed to create directory: {e}"),
                            path: Some(dest_path.clone()),
                        }
                    })?;
                    self.copy_template_dir(&path, &dest_path, config).await?;
                } else {
                    // Copy and process files
                    let content = tokio::fs::read_to_string(&path).await.map_err(|e| {
                        CookbookError::FileSystemError {
                            message: format!("Failed to read template file: {e}"),
                            path: Some(path.clone()),
                        }
                    })?;

                    let processed_content = process_content(content, config);
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
    use crate::config::RecipeType;
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
        let temp_dir = TempDir::new().unwrap();
        let project_path = temp_dir.path().join("test-files");
        tokio::fs::create_dir_all(&project_path).await.unwrap();
        tokio::fs::create_dir_all(project_path.join("tests"))
            .await
            .unwrap();
        tokio::fs::create_dir_all(project_path.join("scripts"))
            .await
            .unwrap();

        let config = ProjectConfig::new("test-tutorial");
        let scaffold = Scaffold::new();
        scaffold.create_files(&project_path, &config).await.unwrap();

        assert!(project_path.join("justfile").exists());
        assert!(project_path.join("README.md").exists());
        assert!(project_path.join("recipe.config.yml").exists());
        assert!(project_path.join(".gitignore").exists());
        assert!(project_path
            .join("tests/test-tutorial-e2e.test.ts")
            .exists());
    }

    #[test]
    fn test_scaffold_default() {
        let scaffold = Scaffold::default();
        assert!(!scaffold.dry_run);
    }
}
