//! Recipe scaffolding module
//!
//! This module provides functionality for creating new recipes,
//! including directory structure, template files, and initial configuration.

use crate::config::{ProjectConfig, ProjectInfo, RecipeType};
use crate::error::{CookbookError, Result};
use include_dir::{include_dir, Dir};
use std::path::Path;
use tracing::{debug, info, warn};

// Embed all template directories at compile time
static POLKADOT_SDK_TEMPLATE: Dir<'_> =
    include_dir!("$CARGO_MANIFEST_DIR/templates/recipe-templates/polkadot-sdk-template");
static XCM_TEMPLATE: Dir<'_> =
    include_dir!("$CARGO_MANIFEST_DIR/templates/recipe-templates/xcm-template");
static SOLIDITY_TEMPLATE: Dir<'_> =
    include_dir!("$CARGO_MANIFEST_DIR/templates/recipe-templates/solidity-template");
static BASIC_INTERACTION_TEMPLATE: Dir<'_> =
    include_dir!("$CARGO_MANIFEST_DIR/templates/recipe-templates/basic-interaction-template");
static TESTING_TEMPLATE: Dir<'_> =
    include_dir!("$CARGO_MANIFEST_DIR/templates/recipe-templates/testing-template");

pub mod bootstrap;

pub use bootstrap::{Bootstrap, ProgressCallback};

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
    /// use polkadot_cookbook_sdk::scaffold::Scaffold;
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

    /// Get the default Rust version for templates
    ///
    /// Returns a hardcoded Rust toolchain version to use in generated templates.
    /// This ensures the CLI works standalone without needing access to a
    /// rust-toolchain.toml file in the current directory.
    fn get_rust_version() -> String {
        let version = "1.91";
        debug!("Using default Rust version: {}", version);
        version.to_string()
    }

    /// Create a complete project from configuration
    ///
    /// # Example
    /// ```no_run
    /// use polkadot_cookbook_sdk::{
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
    /// let project_info = scaffold.create_project(config, None).await?;
    ///
    /// println!("Created project: {}", project_info.slug);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create_project(
        &self,
        config: ProjectConfig,
        progress: Option<&ProgressCallback>,
    ) -> Result<ProjectInfo> {
        info!("Creating project: {}", config.slug);

        // Validate configuration
        let warnings = crate::config::validate_project_config(&config)?;
        for warning in warnings {
            warn!("{}", warning);
        }

        // Get default rust version for templates
        let rust_version = Self::get_rust_version();
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
        // Note: Only TypeScript-based recipes with vitest need bootstrap
        // Solidity recipes have their own package.json with hardhat
        if !config.skip_install
            && matches!(
                config.recipe_type,
                RecipeType::Xcm | RecipeType::BasicInteraction | RecipeType::Testing
            )
        {
            let bootstrap = Bootstrap::new(project_path.clone());
            bootstrap.setup(&config.slug, progress).await?;
        } else if matches!(config.recipe_type, RecipeType::Solidity) {
            // Solidity recipes come with their own package.json and dependencies
            // Just run npm install to install hardhat and dependencies
            if !config.skip_install {
                debug!("Installing Solidity recipe dependencies");
                // Show npm install output in real-time (like create-react-app)
                let install_result = tokio::process::Command::new("npm")
                    .arg("install")
                    .current_dir(&project_path)
                    .stdout(std::process::Stdio::inherit())
                    .stderr(std::process::Stdio::inherit())
                    .status()
                    .await;

                match install_result {
                    Ok(status) if status.success() => {
                        debug!("Solidity dependencies installed successfully");
                    }
                    Ok(status) => {
                        warn!("npm install failed for Solidity recipe: {}", status);
                    }
                    Err(e) => {
                        warn!("Failed to run npm install for Solidity recipe: {}", e);
                    }
                }
            }
        } else if matches!(config.recipe_type, RecipeType::PolkadotSdk) {
            // Parachain recipes: install PAPI dependencies unless pallet-only mode
            if !config.skip_install && !config.pallet_only {
                debug!("Installing Parachain recipe PAPI dependencies");
                // Show npm install output in real-time (like create-react-app)
                let install_result = tokio::process::Command::new("npm")
                    .arg("install")
                    .current_dir(&project_path)
                    .stdout(std::process::Stdio::inherit())
                    .stderr(std::process::Stdio::inherit())
                    .status()
                    .await;

                match install_result {
                    Ok(status) if status.success() => {
                        debug!("PAPI dependencies installed successfully");
                    }
                    Ok(status) => {
                        warn!("npm install failed for Parachain recipe: {}", status);
                    }
                    Err(e) => {
                        warn!("Failed to run npm install for Parachain recipe: {}", e);
                    }
                }
            } else if config.pallet_only {
                debug!("Skipping npm install for pallet-only mode (Rust-only)");
            } else {
                debug!("Skipping npm install (skip_install = true)");
            }
        } else {
            debug!("Skipping npm install (skip_install = true)");
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

        self.copy_embedded_template(&POLKADOT_SDK_TEMPLATE, project_path, config, rust_version)
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

        self.copy_embedded_template(&XCM_TEMPLATE, project_path, config, rust_version)
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

        self.copy_embedded_template(&SOLIDITY_TEMPLATE, project_path, config, rust_version)
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

        self.copy_embedded_template(
            &BASIC_INTERACTION_TEMPLATE,
            project_path,
            config,
            rust_version,
        )
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

        self.copy_embedded_template(&TESTING_TEMPLATE, project_path, config, rust_version)
            .await?;
        Ok(())
    }

    /// Copy embedded template directory recursively, replacing placeholders
    async fn copy_embedded_template(
        &self,
        template_dir: &Dir<'_>,
        dest_dir: &Path,
        config: &ProjectConfig,
        rust_version: &str,
    ) -> Result<()> {
        self.copy_embedded_template_impl(template_dir, dest_dir, config, rust_version, true)
            .await
    }

    /// Internal implementation that tracks whether we're at the root level
    fn copy_embedded_template_impl<'a>(
        &'a self,
        template_dir: &'a Dir<'_>,
        dest_dir: &'a Path,
        config: &'a ProjectConfig,
        rust_version: &'a str,
        is_root: bool,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<()>> + 'a>> {
        Box::pin(async move {
            debug!("Copying embedded template to {}", dest_dir.display());

            // Helper function to process file content
            let process_content = |content: &str,
                                   config: &ProjectConfig,
                                   rust_version: &str|
             -> String {
                // Format optional fields as YAML lines or empty strings
                let pathway_line = config
                    .pathway
                    .as_ref()
                    .map(|p| {
                        let value = match p {
                            crate::config::RecipePathway::Parachain => "parachain",
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

                // Convert slug hyphens to underscores for Rust identifiers
                let slug_underscore = config.slug.replace("-", "_");

                content
                    .replace("{{slug}}", &config.slug)
                    .replace("{{slug_underscore}}", &slug_underscore)
                    .replace("{{title}}", &config.title)
                    .replace("{{description}}", &config.description)
                    .replace("{{category}}", &config.category)
                    .replace("{{rust_version}}", rust_version)
                    .replace("{{pathway}}", &pathway_line)
            };

            // Process all files in the embedded directory
            for file in template_dir.files() {
                let file_path = file.path();
                let file_name = file_path.file_name().and_then(|n| n.to_str()).unwrap_or("");

                // Skip hidden files
                if file_name.starts_with('.') {
                    continue;
                }

                // Skip full parachain and PAPI files in pallet-only mode
                if config.pallet_only && matches!(config.recipe_type, RecipeType::PolkadotSdk) {
                    let excluded_files = [
                        "package.json",
                        "tsconfig.json",
                        "vitest.config.ts",
                        "papi.json",
                    ];
                    if excluded_files.contains(&file_name) {
                        debug!("Skipping file in pallet-only mode: {}", file_name);
                        continue;
                    }

                    // Use special pallet-only Cargo.toml instead of root parachain one
                    if file_name == "Cargo.toml" && is_root {
                        debug!("Skipping root Cargo.toml in pallet-only mode");
                        continue;
                    }
                }

                // Use pallet-only Cargo.toml template in pallet-only mode
                if matches!(config.recipe_type, RecipeType::PolkadotSdk)
                    && file_name == "Cargo.pallet-only.toml.template"
                {
                    if config.pallet_only {
                        let dest_path = dest_dir.join("Cargo.toml");
                        let content =
                            file.contents_utf8()
                                .ok_or_else(|| CookbookError::FileSystemError {
                                    message:
                                        "Failed to read pallet-only Cargo.toml template as UTF-8"
                                            .to_string(),
                                    path: Some(file_path.to_path_buf()),
                                })?;
                        let processed_content = process_content(content, config, rust_version);
                        self.write_file(&dest_path, &processed_content).await?;
                    }
                    continue;
                }

                // Skip XCM zombienet config in pallet-only mode
                if config.pallet_only && matches!(config.recipe_type, RecipeType::PolkadotSdk) {
                    let xcm_files = ["zombienet-xcm.toml", "zombienet-xcm.toml.template"];
                    if xcm_files.contains(&file_name) {
                        debug!("Skipping XCM zombienet config in pallet-only mode");
                        continue;
                    }
                }

                // Skip the base README.md from template
                if file_name == "README.md" && matches!(config.recipe_type, RecipeType::PolkadotSdk)
                {
                    debug!("Skipping base README.md");
                    continue;
                }

                // Handle README templates based on mode
                if file_name == "README.pallet-only.md.template" {
                    if config.pallet_only && matches!(config.recipe_type, RecipeType::PolkadotSdk) {
                        let dest_path = dest_dir.join("README.md");
                        let content =
                            file.contents_utf8()
                                .ok_or_else(|| CookbookError::FileSystemError {
                                    message: "Failed to read pallet-only README template as UTF-8"
                                        .to_string(),
                                    path: Some(file_path.to_path_buf()),
                                })?;
                        let processed_content = process_content(content, config, rust_version);
                        self.write_file(&dest_path, &processed_content).await?;
                    }
                    continue;
                }

                // Handle README templates - use tutorial version by default
                if file_name == "README.tutorial.md.template" {
                    if config.pallet_only && matches!(config.recipe_type, RecipeType::PolkadotSdk) {
                        continue;
                    }
                    let dest_path = dest_dir.join("README.md");
                    let content =
                        file.contents_utf8()
                            .ok_or_else(|| CookbookError::FileSystemError {
                                message: "Failed to read tutorial README template as UTF-8"
                                    .to_string(),
                                path: Some(file_path.to_path_buf()),
                            })?;
                    let processed_content = process_content(content, config, rust_version);
                    self.write_file(&dest_path, &processed_content).await?;
                    continue;
                }

                // Skip guide version
                if file_name == "README.guide.md.template" {
                    continue;
                }

                // Determine destination path (remove .template extension if present)
                let dest_path = if file_name.ends_with(".template") {
                    let new_name = file_name.trim_end_matches(".template");
                    dest_dir.join(new_name)
                } else {
                    dest_dir.join(file_name)
                };

                // Read and process file content
                let content =
                    file.contents_utf8()
                        .ok_or_else(|| CookbookError::FileSystemError {
                            message: format!(
                                "Failed to read template file as UTF-8: {}",
                                file_name
                            ),
                            path: Some(file_path.to_path_buf()),
                        })?;

                let processed_content = process_content(content, config, rust_version);
                self.write_file(&dest_path, &processed_content).await?;
            }

            // Process all subdirectories
            for dir in template_dir.dirs() {
                let dir_name = dir
                    .path()
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("");

                // Skip hidden directories
                if dir_name.starts_with('.') {
                    continue;
                }

                // Skip full parachain directories in pallet-only mode
                if config.pallet_only && matches!(config.recipe_type, RecipeType::PolkadotSdk) {
                    let excluded_dirs = ["node", "runtime", "tests", "scripts", ".github"];
                    if excluded_dirs.contains(&dir_name) {
                        debug!("Skipping directory in pallet-only mode: {}", dir_name);
                        continue;
                    }
                }

                // Create destination directory
                let dest_subdir = dest_dir.join(dir_name);
                if self.dry_run {
                    info!("Would create directory: {}", dest_subdir.display());
                } else {
                    tokio::fs::create_dir_all(&dest_subdir).await.map_err(|e| {
                        CookbookError::FileSystemError {
                            message: format!("Failed to create directory: {e}"),
                            path: Some(dest_subdir.clone()),
                        }
                    })?;
                }

                // Recursively copy subdirectory
                self.copy_embedded_template_impl(dir, &dest_subdir, config, rust_version, false)
                    .await?;
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
            "rust-toolchain.toml should specify Rust 1.86 (as passed to create_files), but contains: {}",
            toolchain_content
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

        // Verify rust-toolchain.toml was NOT copied for TypeScript recipes
        assert!(
            !project_path.join("rust-toolchain.toml").exists(),
            "rust-toolchain.toml should NOT exist for XCM (TypeScript) recipes"
        );
    }

    #[test]
    fn test_get_rust_version() {
        // Test that the function returns the expected default version
        let version = Scaffold::get_rust_version();
        assert_eq!(version, "1.91", "Should return default Rust version 1.91");
    }
}
