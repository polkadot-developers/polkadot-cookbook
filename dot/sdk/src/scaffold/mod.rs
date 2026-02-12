//! Project scaffolding module
//!
//! This module provides functionality for creating new projects,
//! including directory structure, template files, and initial configuration.

use crate::config::{ProjectConfig, ProjectInfo, ProjectType};
use crate::error::{CookbookError, Result};
use include_dir::{include_dir, Dir};
use std::path::{Path, PathBuf};
use tracing::{debug, info, warn};

// Embed non-PolkadotSdk template directories at compile time
static XCM_TEMPLATE: Dir<'_> =
    include_dir!("$CARGO_MANIFEST_DIR/templates/project-templates/xcm-template");
static SOLIDITY_TEMPLATE: Dir<'_> =
    include_dir!("$CARGO_MANIFEST_DIR/templates/project-templates/solidity-template");
static TRANSACTIONS_TEMPLATE: Dir<'_> =
    include_dir!("$CARGO_MANIFEST_DIR/templates/project-templates/transactions-template");
static NETWORKS_TEMPLATE: Dir<'_> =
    include_dir!("$CARGO_MANIFEST_DIR/templates/project-templates/networks-template");

// Embed the SDK overlay files for PolkadotSdk projects
static POLKADOT_SDK_OVERLAY: Dir<'_> =
    include_dir!("$CARGO_MANIFEST_DIR/templates/project-templates/polkadot-sdk-template");

/// Upstream template repository URL
const UPSTREAM_TEMPLATE_REPO: &str =
    "https://github.com/paritytech/polkadot-sdk-parachain-template.git";

/// Upstream template tag to clone
const UPSTREAM_TEMPLATE_TAG: &str = "v0.0.5";

/// Rust toolchain version for generated projects
const RUST_VERSION: &str = "1.88";

/// File extensions considered as text files for name replacement
const TEXT_EXTENSIONS: &[&str] = &[
    "rs",
    "toml",
    "json",
    "md",
    "yml",
    "yaml",
    "sh",
    "ts",
    "txt",
    "lock",
    "toml",
    "cfg",
    "dockerfile",
];

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
    /// Returns the Rust toolchain version to use in generated templates.
    fn get_rust_version() -> String {
        let version = RUST_VERSION;
        debug!("Using default Rust version: {}", version);
        version.to_string()
    }

    /// Get the template cache directory for a given tag
    fn cache_dir(tag: &str) -> PathBuf {
        let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
        PathBuf::from(home)
            .join(".dot")
            .join("templates")
            .join("polkadot-sdk-parachain-template")
            .join(tag)
    }

    /// Clone the upstream template to the cache directory, or reuse cached copy.
    /// Returns the path to the cached clone.
    async fn clone_upstream_template(tag: &str) -> Result<PathBuf> {
        let cache = Self::cache_dir(tag);

        // Check if already cached
        if cache.join("Cargo.toml").exists() {
            debug!("Using cached template at: {}", cache.display());
            return Ok(cache);
        }

        info!(
            "Cloning upstream template {} at tag {}",
            UPSTREAM_TEMPLATE_REPO, tag
        );

        // Create cache directory
        tokio::fs::create_dir_all(&cache)
            .await
            .map_err(|e| CookbookError::FileSystemError {
                message: format!("Failed to create cache directory: {e}"),
                path: Some(cache.clone()),
            })?;

        // Clone with git
        let output = tokio::process::Command::new("git")
            .args([
                "clone",
                "--branch",
                tag,
                "--depth",
                "1",
                UPSTREAM_TEMPLATE_REPO,
                &cache.display().to_string(),
            ])
            .output()
            .await
            .map_err(|e| CookbookError::CommandError {
                command: "git clone".to_string(),
                message: format!("Failed to execute git clone: {e}"),
            })?;

        if !output.status.success() {
            // Clean up failed clone
            let _ = tokio::fs::remove_dir_all(&cache).await;
            return Err(CookbookError::CommandError {
                command: "git clone".to_string(),
                message: format!(
                    "git clone failed: {}",
                    String::from_utf8_lossy(&output.stderr)
                ),
            });
        }

        // Remove .git directory
        let git_dir = cache.join(".git");
        if git_dir.exists() {
            tokio::fs::remove_dir_all(&git_dir).await.map_err(|e| {
                CookbookError::FileSystemError {
                    message: format!("Failed to remove .git directory: {e}"),
                    path: Some(git_dir),
                }
            })?;
        }

        info!("Template cached at: {}", cache.display());
        Ok(cache)
    }

    /// Copy the cached template clone to the project directory
    async fn copy_cached_template(cache_dir: &Path, dest_dir: &Path) -> Result<()> {
        debug!(
            "Copying cached template from {} to {}",
            cache_dir.display(),
            dest_dir.display()
        );

        Self::copy_dir_recursive(cache_dir, dest_dir).await
    }

    /// Recursively copy a directory
    async fn copy_dir_recursive(src: &Path, dest: &Path) -> Result<()> {
        tokio::fs::create_dir_all(dest)
            .await
            .map_err(|e| CookbookError::FileSystemError {
                message: format!("Failed to create directory: {e}"),
                path: Some(dest.to_path_buf()),
            })?;

        let mut entries =
            tokio::fs::read_dir(src)
                .await
                .map_err(|e| CookbookError::FileSystemError {
                    message: format!("Failed to read directory: {e}"),
                    path: Some(src.to_path_buf()),
                })?;

        while let Some(entry) =
            entries
                .next_entry()
                .await
                .map_err(|e| CookbookError::FileSystemError {
                    message: format!("Failed to read directory entry: {e}"),
                    path: Some(src.to_path_buf()),
                })?
        {
            let src_path = entry.path();
            let file_name = entry.file_name();
            let dest_path = dest.join(&file_name);

            let file_type =
                entry
                    .file_type()
                    .await
                    .map_err(|e| CookbookError::FileSystemError {
                        message: format!("Failed to get file type: {e}"),
                        path: Some(src_path.clone()),
                    })?;

            if file_type.is_dir() {
                Box::pin(Self::copy_dir_recursive(&src_path, &dest_path)).await?;
            } else {
                tokio::fs::copy(&src_path, &dest_path).await.map_err(|e| {
                    CookbookError::FileSystemError {
                        message: format!("Failed to copy file: {e}"),
                        path: Some(src_path.clone()),
                    }
                })?;
            }
        }

        Ok(())
    }

    /// Check if a file is a text file based on extension (for name replacement)
    fn is_text_file(path: &Path) -> bool {
        let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");

        // Extensionless files that are commonly text
        if file_name == "Dockerfile" || file_name == "LICENSE" || file_name == "Makefile" {
            return true;
        }

        // Dotfiles like .gitignore
        if file_name.starts_with('.') {
            return true;
        }

        match path.extension().and_then(|e| e.to_str()) {
            Some(ext) => TEXT_EXTENSIONS.contains(&ext.to_lowercase().as_str()),
            None => false,
        }
    }

    /// Replace upstream names with user's project slug in all text files.
    /// Replacements are ordered most-specific-first to avoid partial matches.
    async fn replace_names(project_dir: &Path, slug: &str) -> Result<()> {
        let slug_underscore = slug.replace('-', "_");

        // Ordered most-specific-first to prevent partial matches
        let replacements = vec![
            ("parachain-template-runtime", format!("{slug}-runtime")),
            (
                "parachain_template_runtime",
                format!("{slug_underscore}_runtime"),
            ),
            ("pallet-parachain-template", format!("pallet-{slug}")),
            (
                "pallet_parachain_template",
                format!("pallet_{slug_underscore}"),
            ),
            ("parachain-template", slug.to_string()),
            ("parachain_template", slug_underscore.to_string()),
        ];

        Self::replace_names_in_dir(project_dir, &replacements).await
    }

    /// Recursively replace names in all text files within a directory
    fn replace_names_in_dir<'a>(
        dir: &'a Path,
        replacements: &'a [(&'a str, String)],
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<()>> + 'a>> {
        Box::pin(async move {
            let mut entries =
                tokio::fs::read_dir(dir)
                    .await
                    .map_err(|e| CookbookError::FileSystemError {
                        message: format!("Failed to read directory: {e}"),
                        path: Some(dir.to_path_buf()),
                    })?;

            while let Some(entry) =
                entries
                    .next_entry()
                    .await
                    .map_err(|e| CookbookError::FileSystemError {
                        message: format!("Failed to read directory entry: {e}"),
                        path: Some(dir.to_path_buf()),
                    })?
            {
                let path = entry.path();
                let file_type =
                    entry
                        .file_type()
                        .await
                        .map_err(|e| CookbookError::FileSystemError {
                            message: format!("Failed to get file type: {e}"),
                            path: Some(path.clone()),
                        })?;

                if file_type.is_dir() {
                    // Rename directory if its name contains upstream names
                    let dir_name = entry.file_name().to_str().unwrap_or("").to_string();

                    let mut new_name = dir_name.clone();
                    for (from, to) in replacements {
                        new_name = new_name.replace(from, to);
                    }

                    let target_path = if new_name != dir_name {
                        let renamed = path.parent().unwrap().join(&new_name);
                        tokio::fs::rename(&path, &renamed).await.map_err(|e| {
                            CookbookError::FileSystemError {
                                message: format!("Failed to rename directory: {e}"),
                                path: Some(path.clone()),
                            }
                        })?;
                        renamed
                    } else {
                        path
                    };

                    Self::replace_names_in_dir(&target_path, replacements).await?;
                } else if file_type.is_file() && Self::is_text_file(&path) {
                    // Read, replace, write
                    match tokio::fs::read_to_string(&path).await {
                        Ok(content) => {
                            let mut new_content = content.clone();
                            for (from, to) in replacements {
                                new_content = new_content.replace(from, to);
                            }
                            if new_content != content {
                                tokio::fs::write(&path, &new_content).await.map_err(|e| {
                                    CookbookError::FileSystemError {
                                        message: format!("Failed to write file: {e}"),
                                        path: Some(path.clone()),
                                    }
                                })?;
                            }
                        }
                        Err(_) => {
                            // Skip binary files that can't be read as UTF-8
                            debug!("Skipping non-text file: {}", path.display());
                        }
                    }
                }
            }

            Ok(())
        })
    }

    /// Overlay SDK-specific files on top of the cloned template.
    /// Processes {{slug}}, {{title}}, {{description}}, etc. placeholders in overlay files.
    async fn overlay_sdk_files(
        &self,
        overlay_dir: &Dir<'_>,
        dest_dir: &Path,
        config: &ProjectConfig,
        rust_version: &str,
    ) -> Result<()> {
        self.overlay_sdk_files_impl(overlay_dir, dest_dir, config, rust_version)
            .await
    }

    /// Internal implementation for overlaying SDK files
    fn overlay_sdk_files_impl<'a>(
        &'a self,
        overlay_dir: &'a Dir<'_>,
        dest_dir: &'a Path,
        config: &'a ProjectConfig,
        rust_version: &'a str,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<()>> + 'a>> {
        Box::pin(async move {
            let process_content = Self::make_process_content(config, rust_version);

            for file in overlay_dir.files() {
                let file_path = file.path();
                let file_name = file_path.file_name().and_then(|n| n.to_str()).unwrap_or("");

                // Skip hidden files except .gitignore
                if file_name.starts_with('.') && file_name != ".gitignore" {
                    continue;
                }

                // In pallet-only mode, skip files not relevant
                if config.pallet_only {
                    let excluded_files = [
                        "package.json",
                        "tsconfig.json",
                        "vitest.config.ts",
                        "papi.json",
                        "chopsticks.yml.template",
                    ];
                    if excluded_files.contains(&file_name) {
                        debug!("Skipping overlay file in pallet-only mode: {}", file_name);
                        continue;
                    }
                }

                // Handle Cargo.pallet-only.toml.template
                if file_name == "Cargo.pallet-only.toml.template" {
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
                        let processed_content = process_content(content);
                        self.write_file(&dest_path, &processed_content).await?;
                    }
                    continue;
                }

                // Handle README templates
                if file_name == "README.pallet-only.md.template" {
                    if config.pallet_only {
                        let dest_path = dest_dir.join("README.md");
                        let content =
                            file.contents_utf8()
                                .ok_or_else(|| CookbookError::FileSystemError {
                                    message: "Failed to read pallet-only README template as UTF-8"
                                        .to_string(),
                                    path: Some(file_path.to_path_buf()),
                                })?;
                        let processed_content = process_content(content);
                        self.write_file(&dest_path, &processed_content).await?;
                    }
                    continue;
                }

                if file_name == "README.full.md.template" {
                    if !config.pallet_only {
                        let dest_path = dest_dir.join("README.md");
                        let content =
                            file.contents_utf8()
                                .ok_or_else(|| CookbookError::FileSystemError {
                                    message: "Failed to read full README template as UTF-8"
                                        .to_string(),
                                    path: Some(file_path.to_path_buf()),
                                })?;
                        let processed_content = process_content(content);
                        self.write_file(&dest_path, &processed_content).await?;
                    }
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
                            message: format!("Failed to read overlay file as UTF-8: {}", file_name),
                            path: Some(file_path.to_path_buf()),
                        })?;

                let processed_content = process_content(content);
                self.write_file(&dest_path, &processed_content).await?;
            }

            // Process subdirectories
            for dir in overlay_dir.dirs() {
                let dir_name = dir
                    .path()
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("");

                // Skip hidden directories
                if dir_name.starts_with('.') {
                    continue;
                }

                // In pallet-only mode, skip tests and scripts overlays
                if config.pallet_only {
                    let excluded_dirs = ["tests", "scripts"];
                    if excluded_dirs.contains(&dir_name) {
                        debug!(
                            "Skipping overlay directory in pallet-only mode: {}",
                            dir_name
                        );
                        continue;
                    }
                }

                let dest_subdir = dest_dir.join(dir_name);
                if !self.dry_run {
                    tokio::fs::create_dir_all(&dest_subdir).await.map_err(|e| {
                        CookbookError::FileSystemError {
                            message: format!("Failed to create directory: {e}"),
                            path: Some(dest_subdir.clone()),
                        }
                    })?;
                }

                self.overlay_sdk_files_impl(dir, &dest_subdir, config, rust_version)
                    .await?;
            }

            Ok(())
        })
    }

    /// Create a content processing closure for placeholder replacement
    fn make_process_content<'a>(
        config: &'a ProjectConfig,
        rust_version: &'a str,
    ) -> impl Fn(&str) -> String + 'a {
        let pathway_line = config
            .pathway
            .as_ref()
            .map(|p| {
                let value = match p {
                    crate::config::ProjectPathway::Pallets => "pallets",
                    crate::config::ProjectPathway::Contracts => "contracts",
                    crate::config::ProjectPathway::Transactions => "transactions",
                    crate::config::ProjectPathway::Xcm => "xcm",
                    crate::config::ProjectPathway::Networks => "networks",
                };
                format!("pathway: {value}")
            })
            .unwrap_or_default();

        let slug_underscore = config.slug.replace('-', "_");

        move |content: &str| -> String {
            content
                .replace("{{slug}}", &config.slug)
                .replace("{{slug_underscore}}", &slug_underscore)
                .replace("{{title}}", &config.title)
                .replace("{{description}}", &config.description)
                .replace("{{category}}", &config.category)
                .replace("{{rust_version}}", rust_version)
                .replace("{{pathway}}", &pathway_line)
        }
    }

    /// Handle pallet-only mode by removing full parachain files
    async fn handle_pallet_only(project_dir: &Path) -> Result<()> {
        debug!("Handling pallet-only mode: pruning full parachain files");

        // Remove directories not needed in pallet-only mode
        let dirs_to_remove = ["node", "runtime", ".github"];
        for dir_name in &dirs_to_remove {
            let dir_path = project_dir.join(dir_name);
            if dir_path.exists() {
                tokio::fs::remove_dir_all(&dir_path).await.map_err(|e| {
                    CookbookError::FileSystemError {
                        message: format!("Failed to remove directory {dir_name}: {e}"),
                        path: Some(dir_path),
                    }
                })?;
                debug!("Removed directory: {}", dir_name);
            }
        }

        // Remove files not needed in pallet-only mode
        let files_to_remove = [
            "Dockerfile",
            "dev_chain_spec.json",
            "zombienet.toml",
            "zombienet-omni-node.toml",
            "chopsticks.yml",
        ];
        for file_name in &files_to_remove {
            let file_path = project_dir.join(file_name);
            if file_path.exists() {
                tokio::fs::remove_file(&file_path).await.map_err(|e| {
                    CookbookError::FileSystemError {
                        message: format!("Failed to remove file {file_name}: {e}"),
                        path: Some(file_path),
                    }
                })?;
                debug!("Removed file: {}", file_name);
            }
        }

        // Remove the upstream Cargo.toml (will be replaced by pallet-only version from overlay)
        let cargo_toml = project_dir.join("Cargo.toml");
        if cargo_toml.exists() {
            tokio::fs::remove_file(&cargo_toml).await.map_err(|e| {
                CookbookError::FileSystemError {
                    message: format!("Failed to remove Cargo.toml: {e}"),
                    path: Some(cargo_toml),
                }
            })?;
        }

        // Remove the upstream LICENSE (pallet-only README covers licensing)
        let license = project_dir.join("LICENSE");
        if license.exists() {
            tokio::fs::remove_file(&license)
                .await
                .map_err(|e| CookbookError::FileSystemError {
                    message: format!("Failed to remove LICENSE: {e}"),
                    path: Some(license),
                })?;
        }

        Ok(())
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

        // Create directory structure first
        if let Some(cb) = progress {
            cb("Creating project directory...");
        }
        self.create_directories(&project_path, config.project_type)
            .await?;

        // Initialize git repository if requested
        let git_initialized = if config.git_init {
            if let Some(cb) = progress {
                cb("Initializing git repository...");
            }
            match crate::git::GitOperations::init(&project_path).await {
                Ok(()) => {
                    info!("Initialized git repository at: {}", project_path.display());
                    true
                }
                Err(e) => {
                    warn!("Failed to initialize git repository: {}", e);
                    false
                }
            }
        } else {
            false
        };

        // Generate and write template files
        if let Some(cb) = progress {
            cb("Copying template files...");
        }
        self.create_files(&project_path, &config, &rust_version)
            .await?;

        // Bootstrap test environment if not skipped
        // Note: Only TypeScript-based projects with vitest need bootstrap
        // Transactions, XCM, and Solidity projects have their own package.json with dependencies
        if !config.skip_install
            && matches!(
                config.project_type,
                ProjectType::Xcm | ProjectType::Transactions
            )
        {
            // These templates have complete package.json with PAPI dependencies
            // Just run npm install to install dependencies and trigger postinstall (papi add)
            if let Some(cb) = progress {
                cb("Installing dependencies (this may take a moment)...");
            }
            let install_result = tokio::process::Command::new("npm")
                .arg("install")
                .current_dir(&project_path)
                .stdout(std::process::Stdio::inherit())
                .stderr(std::process::Stdio::inherit())
                .status()
                .await;

            match install_result {
                Ok(status) if status.success() => {
                    debug!("Dependencies installed successfully");
                }
                Ok(status) => {
                    warn!("npm install failed: {}", status);
                }
                Err(e) => {
                    warn!("Failed to run npm install: {}", e);
                }
            }
        } else if !config.skip_install && matches!(config.project_type, ProjectType::Networks) {
            // Networks template uses the bootstrap for setting up test environment
            if let Some(cb) = progress {
                cb("Setting up test environment...");
            }
            let bootstrap = Bootstrap::new(project_path.clone());
            bootstrap.setup(&config.slug, progress).await?;
        } else if matches!(config.project_type, ProjectType::Solidity) {
            // Solidity projects come with their own package.json and dependencies
            // Just run npm install to install hardhat and dependencies
            if !config.skip_install {
                if let Some(cb) = progress {
                    cb("Installing dependencies (this may take a moment)...");
                }
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
                        warn!("npm install failed for Solidity project: {}", status);
                    }
                    Err(e) => {
                        warn!("Failed to run npm install for Solidity project: {}", e);
                    }
                }
            }
        } else if matches!(config.project_type, ProjectType::PolkadotSdk) {
            // Parachain projects: install PAPI dependencies unless pallet-only mode
            if !config.skip_install && !config.pallet_only {
                if let Some(cb) = progress {
                    cb("Installing dependencies (this may take a moment)...");
                }
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
                        warn!("npm install failed for Parachain project: {}", status);
                    }
                    Err(e) => {
                        warn!("Failed to run npm install for Parachain project: {}", e);
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
            git_initialized,
        })
    }

    /// Create the directory structure for a project
    async fn create_directories(
        &self,
        project_path: &Path,
        project_type: ProjectType,
    ) -> Result<()> {
        debug!(
            "Creating directory structure at: {} for project type: {:?}",
            project_path.display(),
            project_type
        );

        let directories = match project_type {
            ProjectType::PolkadotSdk => {
                // For Rust-based projects, clone+overlay handles structure
                vec![project_path.to_path_buf()]
            }
            ProjectType::Xcm => {
                vec![project_path.to_path_buf()]
            }
            ProjectType::Transactions => {
                vec![project_path.to_path_buf()]
            }
            ProjectType::Networks => {
                vec![project_path.to_path_buf()]
            }
            ProjectType::Solidity => {
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

        match config.project_type {
            ProjectType::PolkadotSdk => {
                self.create_polkadot_sdk_files(project_path, config, rust_version)
                    .await?;
            }
            ProjectType::Xcm => {
                self.create_xcm_files(project_path, config, rust_version)
                    .await?;
            }
            ProjectType::Transactions => {
                self.create_basic_interaction_files(project_path, config, rust_version)
                    .await?;
            }
            ProjectType::Networks => {
                self.create_testing_files(project_path, config, rust_version)
                    .await?;
            }
            ProjectType::Solidity => {
                self.create_solidity_files(project_path, config, rust_version)
                    .await?;
            }
        }

        Ok(())
    }

    /// Create files for Polkadot SDK projects using clone + overlay approach.
    ///
    /// 1. Clone upstream template from cache (or git clone if not cached)
    /// 2. Copy cached clone to project directory
    /// 3. String-replace upstream names with user's project slug
    /// 4. Handle pallet-only mode (remove node/, runtime/, etc.)
    /// 5. Overlay SDK-specific files (package.json, tests/, scripts/, READMEs, etc.)
    async fn create_polkadot_sdk_files(
        &self,
        project_path: &Path,
        config: &ProjectConfig,
        rust_version: &str,
    ) -> Result<()> {
        debug!("Creating Polkadot SDK template files via clone+overlay");

        if self.dry_run {
            info!("Would clone upstream template and overlay SDK files");
            return Ok(());
        }

        // Step 1: Clone or retrieve from cache
        let cache_dir = Self::clone_upstream_template(UPSTREAM_TEMPLATE_TAG).await?;

        // Step 2: Copy cached clone to project directory
        Self::copy_cached_template(&cache_dir, project_path).await?;

        // Step 3: Replace upstream names with user's slug
        Self::replace_names(project_path, &config.slug).await?;

        // Step 4: Handle pallet-only mode (prune before overlay)
        if config.pallet_only {
            Self::handle_pallet_only(project_path).await?;
        }

        // Step 5: Overlay SDK-specific files
        self.overlay_sdk_files(&POLKADOT_SDK_OVERLAY, project_path, config, rust_version)
            .await?;

        Ok(())
    }

    /// Create files for XCM projects (TypeScript with Chopsticks)
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

    /// Create files for Solidity projects (TypeScript)
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

    /// Create files for Chain Transaction projects (TypeScript with PAPI)
    async fn create_basic_interaction_files(
        &self,
        project_path: &Path,
        config: &ProjectConfig,
        rust_version: &str,
    ) -> Result<()> {
        debug!("Creating Transactions template files");

        self.copy_embedded_template(&TRANSACTIONS_TEMPLATE, project_path, config, rust_version)
            .await?;
        Ok(())
    }

    /// Create files for Network Infrastructure projects (Zombienet/Chopsticks)
    async fn create_testing_files(
        &self,
        project_path: &Path,
        config: &ProjectConfig,
        rust_version: &str,
    ) -> Result<()> {
        debug!("Creating Network Infrastructure template files");

        self.copy_embedded_template(&NETWORKS_TEMPLATE, project_path, config, rust_version)
            .await?;
        Ok(())
    }

    /// Copy embedded template directory recursively, replacing placeholders.
    /// Used for non-PolkadotSdk project types (XCM, Solidity, Transactions, Networks).
    async fn copy_embedded_template(
        &self,
        template_dir: &Dir<'_>,
        dest_dir: &Path,
        config: &ProjectConfig,
        rust_version: &str,
    ) -> Result<()> {
        self.copy_embedded_template_impl(template_dir, dest_dir, config, rust_version)
            .await
    }

    /// Internal implementation for copying embedded templates
    fn copy_embedded_template_impl<'a>(
        &'a self,
        template_dir: &'a Dir<'_>,
        dest_dir: &'a Path,
        config: &'a ProjectConfig,
        rust_version: &'a str,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<()>> + 'a>> {
        Box::pin(async move {
            debug!("Copying embedded template to {}", dest_dir.display());

            let process_content = Self::make_process_content(config, rust_version);

            // Process all files in the embedded directory
            for file in template_dir.files() {
                let file_path = file.path();
                let file_name = file_path.file_name().and_then(|n| n.to_str()).unwrap_or("");

                // Skip hidden files except .gitignore
                if file_name.starts_with('.') && file_name != ".gitignore" {
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

                let processed_content = process_content(content);
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
                self.copy_embedded_template_impl(dir, &dest_subdir, config, rust_version)
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

            // Make shell scripts executable
            if path.extension().is_some_and(|ext| ext == "sh") {
                #[cfg(unix)]
                {
                    use std::os::unix::fs::PermissionsExt;
                    let mut perms = std::fs::metadata(path)
                        .map_err(|e| CookbookError::FileSystemError {
                            message: format!("Failed to get file permissions: {e}"),
                            path: Some(path.to_path_buf()),
                        })?
                        .permissions();
                    perms.set_mode(0o755);
                    std::fs::set_permissions(path, perms).map_err(|e| {
                        CookbookError::FileSystemError {
                            message: format!("Failed to set file permissions: {e}"),
                            path: Some(path.to_path_buf()),
                        }
                    })?;
                    debug!("Made executable: {}", path.display());
                }
            }

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
    use crate::config::ProjectType;
    use serial_test::serial;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_create_directories() {
        let temp_dir = TempDir::new().unwrap();
        let project_path = temp_dir.path().join("test-project");

        let scaffold = Scaffold::new();
        scaffold
            .create_directories(&project_path, ProjectType::Solidity)
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
            .create_directories(&project_path, ProjectType::Solidity)
            .await
            .unwrap();

        // In dry-run mode, directories should NOT be created
        assert!(!project_path.exists());
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
            .with_project_type(ProjectType::Transactions);

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
        let project_path = temp_dir.path().join("testing-project-test");
        tokio::fs::create_dir_all(&project_path).await.unwrap();

        let config =
            ProjectConfig::new("testing-project-test").with_project_type(ProjectType::Networks);

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

        let config = ProjectConfig::new("xcm-test").with_project_type(ProjectType::Xcm);

        let scaffold = Scaffold::new();
        let result = scaffold.create_files(&project_path, &config, "1.86").await;

        std::env::set_current_dir(original_dir).unwrap();
        result.unwrap();

        // Verify XCM/Chopsticks structure was created
        assert!(project_path.join("package.json").exists());
        assert!(project_path.join("chopsticks.yml").exists());

        // Verify rust-toolchain.toml was NOT copied for TypeScript projects
        assert!(
            !project_path.join("rust-toolchain.toml").exists(),
            "rust-toolchain.toml should NOT exist for XCM (TypeScript) recipes"
        );
    }

    #[test]
    fn test_get_rust_version() {
        let version = Scaffold::get_rust_version();
        assert_eq!(version, RUST_VERSION, "Should return RUST_VERSION constant");
    }

    #[test]
    fn test_upstream_tag_constant() {
        assert_eq!(UPSTREAM_TEMPLATE_TAG, "v0.0.5");
    }

    #[test]
    fn test_rust_version_constant() {
        assert_eq!(RUST_VERSION, "1.88");
    }

    #[tokio::test]
    async fn test_replace_names_ordering() {
        let temp_dir = TempDir::new().unwrap();
        let project_dir = temp_dir.path().join("test-replace");
        tokio::fs::create_dir_all(&project_dir).await.unwrap();

        // Create a Cargo.toml with all upstream name variants
        let cargo_content = r#"[workspace]
members = ["node", "runtime", "pallets/template"]

[workspace.dependencies]
parachain-template-runtime = { path = "runtime" }
pallet-parachain-template = { path = "pallets/template" }

[package]
name = "parachain-template"
description = "A parachain_template project"
"#;
        tokio::fs::write(project_dir.join("Cargo.toml"), cargo_content)
            .await
            .unwrap();

        Scaffold::replace_names(&project_dir, "my-project")
            .await
            .unwrap();

        let result = tokio::fs::read_to_string(project_dir.join("Cargo.toml"))
            .await
            .unwrap();

        // Verify most-specific replacements happened correctly
        assert!(
            result.contains("my-project-runtime"),
            "Should contain 'my-project-runtime', got:\n{result}"
        );
        assert!(
            result.contains("pallet-my-project"),
            "Should contain 'pallet-my-project', got:\n{result}"
        );
        assert!(
            result.contains("name = \"my-project\""),
            "Should contain 'name = \"my-project\"', got:\n{result}"
        );
        assert!(
            result.contains("my_project project"),
            "Should contain 'my_project project', got:\n{result}"
        );
        // Ensure no leftover upstream names
        assert!(
            !result.contains("parachain-template"),
            "Should not contain 'parachain-template', got:\n{result}"
        );
        assert!(
            !result.contains("parachain_template"),
            "Should not contain 'parachain_template', got:\n{result}"
        );
    }

    #[tokio::test]
    async fn test_replace_names_binary_skip() {
        let temp_dir = TempDir::new().unwrap();
        let project_dir = temp_dir.path().join("test-binary-skip");
        tokio::fs::create_dir_all(&project_dir).await.unwrap();

        // Create a .wasm file with upstream name (should NOT be modified)
        let wasm_content = b"parachain-template binary data";
        tokio::fs::write(project_dir.join("test.wasm"), wasm_content)
            .await
            .unwrap();

        // Create a .rs file (should be modified)
        tokio::fs::write(
            project_dir.join("lib.rs"),
            "use parachain_template::Config;",
        )
        .await
        .unwrap();

        Scaffold::replace_names(&project_dir, "my-chain")
            .await
            .unwrap();

        // Binary file should be unchanged
        let wasm = tokio::fs::read(project_dir.join("test.wasm"))
            .await
            .unwrap();
        assert_eq!(wasm, wasm_content);

        // Text file should be modified
        let rs = tokio::fs::read_to_string(project_dir.join("lib.rs"))
            .await
            .unwrap();
        assert!(rs.contains("my_chain"));
        assert!(!rs.contains("parachain_template"));
    }

    #[tokio::test]
    async fn test_overlay_placeholder_replacement() {
        let temp_dir = TempDir::new().unwrap();
        let project_dir = temp_dir.path().join("test-overlay");
        tokio::fs::create_dir_all(&project_dir).await.unwrap();

        let config = ProjectConfig::new("my-project")
            .with_title("My Project".to_string())
            .with_description("A test project".to_string());

        let scaffold = Scaffold::new();
        scaffold
            .overlay_sdk_files(&POLKADOT_SDK_OVERLAY, &project_dir, &config, "1.88")
            .await
            .unwrap();

        // Verify package.json has placeholders replaced
        let package_json = tokio::fs::read_to_string(project_dir.join("package.json"))
            .await
            .unwrap();
        assert!(
            package_json.contains("\"name\": \"my-project\""),
            "package.json should have slug replaced"
        );
        assert!(
            package_json.contains("\"description\": \"A test project\""),
            "package.json should have description replaced"
        );

        // Verify rust-toolchain.toml has rust version replaced
        let toolchain = tokio::fs::read_to_string(project_dir.join("rust-toolchain.toml"))
            .await
            .unwrap();
        assert!(
            toolchain.contains("channel = \"1.88\""),
            "rust-toolchain.toml should have rust version replaced"
        );

        // Verify README.md was created from full template
        assert!(
            project_dir.join("README.md").exists(),
            "README.md should exist"
        );
        let readme = tokio::fs::read_to_string(project_dir.join("README.md"))
            .await
            .unwrap();
        assert!(
            readme.contains("My Project"),
            "README should contain project title"
        );
    }

    #[tokio::test]
    async fn test_pallet_only_mode_pruning() {
        let temp_dir = TempDir::new().unwrap();
        let project_dir = temp_dir.path().join("test-pallet-only");
        tokio::fs::create_dir_all(&project_dir).await.unwrap();

        // Create a full project structure
        for dir in &[
            "node/src",
            "runtime/src",
            "pallets/template/src",
            ".github/workflows",
        ] {
            tokio::fs::create_dir_all(project_dir.join(dir))
                .await
                .unwrap();
        }
        for file in &[
            "Cargo.toml",
            "Dockerfile",
            "LICENSE",
            "dev_chain_spec.json",
            "zombienet.toml",
            "zombienet-omni-node.toml",
            "chopsticks.yml",
        ] {
            tokio::fs::write(project_dir.join(file), "placeholder")
                .await
                .unwrap();
        }
        tokio::fs::write(
            project_dir.join("pallets/template/src/lib.rs"),
            "mod pallet;",
        )
        .await
        .unwrap();

        Scaffold::handle_pallet_only(&project_dir).await.unwrap();

        // Directories removed
        assert!(
            !project_dir.join("node").exists(),
            "node/ should be removed"
        );
        assert!(
            !project_dir.join("runtime").exists(),
            "runtime/ should be removed"
        );
        assert!(
            !project_dir.join(".github").exists(),
            ".github/ should be removed"
        );

        // Files removed
        assert!(
            !project_dir.join("Dockerfile").exists(),
            "Dockerfile should be removed"
        );
        assert!(
            !project_dir.join("dev_chain_spec.json").exists(),
            "dev_chain_spec.json should be removed"
        );
        assert!(
            !project_dir.join("zombienet.toml").exists(),
            "zombienet.toml should be removed"
        );
        assert!(
            !project_dir.join("zombienet-omni-node.toml").exists(),
            "zombienet-omni-node.toml should be removed"
        );
        assert!(
            !project_dir.join("Cargo.toml").exists(),
            "Cargo.toml should be removed (replaced by overlay)"
        );
        assert!(
            !project_dir.join("LICENSE").exists(),
            "LICENSE should be removed"
        );
        assert!(
            !project_dir.join("chopsticks.yml").exists(),
            "chopsticks.yml should be removed"
        );

        // pallets/ should still exist
        assert!(
            project_dir.join("pallets/template/src/lib.rs").exists(),
            "pallets/ should be kept"
        );
    }

    #[tokio::test]
    async fn test_is_text_file() {
        assert!(Scaffold::is_text_file(Path::new("lib.rs")));
        assert!(Scaffold::is_text_file(Path::new("Cargo.toml")));
        assert!(Scaffold::is_text_file(Path::new("package.json")));
        assert!(Scaffold::is_text_file(Path::new("README.md")));
        assert!(Scaffold::is_text_file(Path::new("config.yml")));
        assert!(Scaffold::is_text_file(Path::new("test.sh")));
        assert!(Scaffold::is_text_file(Path::new("test.ts")));
        assert!(Scaffold::is_text_file(Path::new("Dockerfile")));
        assert!(Scaffold::is_text_file(Path::new(".gitignore")));

        // Binary files
        assert!(!Scaffold::is_text_file(Path::new("test.wasm")));
        assert!(!Scaffold::is_text_file(Path::new("image.png")));
        assert!(!Scaffold::is_text_file(Path::new("archive.tar.gz")));
    }

    #[tokio::test]
    #[serial]
    async fn test_create_files_polkadot_sdk() {
        // This test requires git access to clone the upstream template
        let temp_dir = TempDir::new().unwrap();
        let project_path = temp_dir.path().join("test-sdk-files");
        tokio::fs::create_dir_all(&project_path).await.unwrap();

        let config = ProjectConfig::new("test-tutorial");
        let scaffold = Scaffold::new();
        let result = scaffold
            .create_files(&project_path, &config, RUST_VERSION)
            .await;

        result.unwrap();

        // Verify core files were created from upstream clone + overlay
        assert!(project_path.join("README.md").exists());
        assert!(project_path.join("Cargo.toml").exists());
        assert!(project_path.join("pallets").exists());
        assert!(project_path.join("runtime").exists());
        assert!(project_path.join("node").exists());

        // Verify rust-toolchain.toml was created from overlay template
        assert!(
            project_path.join("rust-toolchain.toml").exists(),
            "rust-toolchain.toml should be created from overlay"
        );

        let toolchain_content =
            std::fs::read_to_string(project_path.join("rust-toolchain.toml")).unwrap();
        assert!(
            toolchain_content.contains(&format!("channel = \"{}\"", RUST_VERSION)),
            "rust-toolchain.toml should specify Rust {}, but contains: {}",
            RUST_VERSION,
            toolchain_content
        );
        assert!(toolchain_content.contains("components = [\"rustfmt\", \"clippy\"]"));
        assert!(toolchain_content.contains("profile = \"minimal\""));

        // Verify name replacement happened
        let cargo_content = std::fs::read_to_string(project_path.join("Cargo.toml")).unwrap();
        assert!(
            cargo_content.contains("test-tutorial"),
            "Cargo.toml should contain the project slug"
        );
        assert!(
            !cargo_content.contains("parachain-template"),
            "Cargo.toml should not contain upstream name 'parachain-template'"
        );

        // Verify SDK overlay files
        assert!(project_path.join("package.json").exists());
        assert!(project_path.join("tsconfig.json").exists());
        assert!(project_path.join("vitest.config.ts").exists());
        assert!(project_path.join("tests").exists());
        assert!(project_path.join("scripts").exists());
    }

    #[tokio::test]
    async fn test_cache_dir() {
        let cache = Scaffold::cache_dir("v0.0.5");
        assert!(cache
            .to_string_lossy()
            .contains(".dot/templates/polkadot-sdk-parachain-template/v0.0.5"));
    }
}
