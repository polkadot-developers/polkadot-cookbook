//! Project scaffolding module
//!
//! This module provides functionality for creating new tutorial projects,
//! including directory structure, template files, and initial configuration.

use crate::config::{ProjectConfig, ProjectInfo};
use crate::error::{CookbookError, Result};
use crate::templates::{
    JustfileTemplate, ReadmeTemplate, Template, TestTemplate, TutorialYmlTemplate,
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
        self.create_directories(&project_path).await?;

        // Generate and write template files
        self.create_files(&project_path, &config).await?;

        // Bootstrap test environment if not skipped
        if !config.skip_install {
            let bootstrap = Bootstrap::new(project_path.clone());
            bootstrap.setup(&config.slug).await?;
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
    async fn create_directories(&self, project_path: &Path) -> Result<()> {
        debug!(
            "Creating directory structure at: {}",
            project_path.display()
        );

        let directories = vec![
            project_path.to_path_buf(),
            project_path.join("tests"),
            project_path.join("scripts"),
            project_path.join("src"),
        ];

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

        // Generate tutorial.config.yml
        let tutorial_yml_content = TutorialYmlTemplate::new(&config.slug, &config.title).generate();
        self.write_file(
            &project_path.join("tutorial.config.yml"),
            &tutorial_yml_content,
        )
        .await?;

        // Generate README.md
        let readme_content = ReadmeTemplate::new(&config.slug).generate();
        self.write_file(&project_path.join("README.md"), &readme_content)
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
            project_path.join("tutorial.config.yml"),
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
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_create_directories() {
        let temp_dir = TempDir::new().unwrap();
        let project_path = temp_dir.path().join("test-project");

        let scaffold = Scaffold::new();
        scaffold.create_directories(&project_path).await.unwrap();

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
        scaffold.create_directories(&project_path).await.unwrap();

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
        assert!(project_path.join("tutorial.config.yml").exists());
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
