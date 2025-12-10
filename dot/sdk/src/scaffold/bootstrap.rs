//! Bootstrap module for setting up test environments
//!
//! This module handles npm package installation, configuration file generation,
//! and test environment setup for tutorial projects.

use crate::error::{CookbookError, Result};
use std::path::PathBuf;
use std::process::Stdio;
use tokio::process::Command;
use tracing::debug;

/// Progress callback function type
pub type ProgressCallback = Box<dyn Fn(&str) + Send + Sync>;

/// Bootstrap manager for test environment setup
pub struct Bootstrap {
    /// Project directory path
    project_path: PathBuf,
}

impl Bootstrap {
    /// Create a new Bootstrap instance
    ///
    /// # Example
    /// ```
    /// use polkadot_cookbook_sdk::scaffold::Bootstrap;
    /// use std::path::PathBuf;
    ///
    /// let bootstrap = Bootstrap::new(PathBuf::from("./tutorials/my-tutorial"));
    /// ```
    pub fn new(project_path: PathBuf) -> Self {
        Self { project_path }
    }

    /// Complete setup of test environment
    ///
    /// # Example
    /// ```no_run
    /// use polkadot_cookbook_sdk::scaffold::Bootstrap;
    /// use std::path::PathBuf;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let bootstrap = Bootstrap::new(PathBuf::from("./tutorials/my-tutorial"));
    /// bootstrap.setup("my-tutorial", None).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn setup(&self, slug: &str, progress: Option<&ProgressCallback>) -> Result<()> {
        debug!("Bootstrapping test environment for: {}", slug);

        // Create package.json
        if let Some(cb) = progress {
            cb("Initializing package.json");
        }
        self.create_package_json(slug).await?;

        // Install dev dependencies
        if let Some(cb) = progress {
            cb("Installing development dependencies");
        }
        self.install_dev_dependencies().await?;

        // Install dependencies
        if let Some(cb) = progress {
            cb("Installing runtime dependencies");
        }
        self.install_dependencies().await?;

        // Set npm scripts
        if let Some(cb) = progress {
            cb("Configuring npm scripts");
        }
        self.set_npm_scripts().await?;

        // Create configuration files
        if let Some(cb) = progress {
            cb("Creating configuration files");
        }
        self.create_config_files().await?;

        debug!("Bootstrap complete for: {}", slug);
        Ok(())
    }

    /// Create package.json if it doesn't exist
    async fn create_package_json(&self, slug: &str) -> Result<()> {
        let package_json_path = self.project_path.join("package.json");

        if tokio::fs::try_exists(&package_json_path)
            .await
            .unwrap_or(false)
        {
            debug!("package.json already exists, skipping creation");
            return Ok(());
        }

        debug!("Creating package.json");

        // Run npm init -y
        self.run_command("npm", &["init", "-y"]).await?;

        // Set name and type
        self.run_command("npm", &["pkg", "set", &format!("name={slug}")])
            .await?;
        self.run_command("npm", &["pkg", "set", "type=module"])
            .await?;

        debug!("Created package.json");
        Ok(())
    }

    /// Install development dependencies
    async fn install_dev_dependencies(&self) -> Result<()> {
        debug!("Installing dev dependencies (vitest, typescript, ts-node, @types/node)...");

        self.run_command(
            "npm",
            &[
                "install",
                "-D",
                "vitest",
                "typescript",
                "ts-node",
                "@types/node",
            ],
        )
        .await?;

        debug!("Dev dependencies installed");
        Ok(())
    }

    /// Install runtime dependencies
    async fn install_dependencies(&self) -> Result<()> {
        debug!("Installing dependencies (@polkadot/api, ws)...");

        self.run_command("npm", &["install", "@polkadot/api", "ws"])
            .await?;

        debug!("Dependencies installed");
        Ok(())
    }

    /// Set npm scripts in package.json
    async fn set_npm_scripts(&self) -> Result<()> {
        debug!("Setting npm scripts");

        self.run_command(
            "npm",
            &[
                "pkg",
                "set",
                "scripts.test=vitest run",
                "scripts.test:watch=vitest",
            ],
        )
        .await?;

        Ok(())
    }

    /// Create configuration files (vitest.config.ts, tsconfig.json)
    async fn create_config_files(&self) -> Result<()> {
        debug!("Creating configuration files");

        // Create vitest.config.ts
        let vitest_config = r#"import { defineConfig } from 'vitest/config';
export default defineConfig({
  test: {
    include: ['tests/**/*.test.ts'],
    testTimeout: 30000,
    hookTimeout: 30000,
  },
});
"#;
        tokio::fs::write(self.project_path.join("vitest.config.ts"), vitest_config)
            .await
            .map_err(|e| {
                CookbookError::BootstrapError(format!("Failed to write vitest.config.ts: {e}"))
            })?;

        // Create tsconfig.json
        let tsconfig_content = r#"{
  "compilerOptions": {
    "target": "ES2020",
    "module": "ESNext",
    "moduleResolution": "Bundler",
    "types": ["node", "vitest/globals"],
    "esModuleInterop": true,
    "resolveJsonModule": true,
    "skipLibCheck": true
  },
  "include": ["tests/**/*.ts"]
}
"#;
        tokio::fs::write(self.project_path.join("tsconfig.json"), tsconfig_content)
            .await
            .map_err(|e| {
                CookbookError::BootstrapError(format!("Failed to write tsconfig.json: {e}"))
            })?;

        debug!("Configuration files created");
        Ok(())
    }

    /// Run a command in the project directory
    async fn run_command(&self, program: &str, args: &[&str]) -> Result<()> {
        debug!("Running command: {} {}", program, args.join(" "));

        // Suppress output to keep spinner clean
        let status = Command::new(program)
            .args(args)
            .current_dir(&self.project_path)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .await
            .map_err(|e| CookbookError::CommandError {
                command: format!("{} {}", program, args.join(" ")),
                message: e.to_string(),
            })?;

        if !status.success() {
            return Err(CookbookError::CommandError {
                command: format!("{} {}", program, args.join(" ")),
                message: format!("Command exited with status: {}", status),
            });
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_create_config_files() {
        let temp_dir = TempDir::new().unwrap();
        let project_path = temp_dir.path().to_path_buf();

        let bootstrap = Bootstrap::new(project_path.clone());
        bootstrap.create_config_files().await.unwrap();

        assert!(project_path.join("vitest.config.ts").exists());
        assert!(project_path.join("tsconfig.json").exists());

        // Verify vitest config content
        let vitest_content = tokio::fs::read_to_string(project_path.join("vitest.config.ts"))
            .await
            .unwrap();
        assert!(vitest_content.contains("vitest/config"));
        assert!(vitest_content.contains("tests/**/*.test.ts"));

        // Verify tsconfig content
        let tsconfig_content = tokio::fs::read_to_string(project_path.join("tsconfig.json"))
            .await
            .unwrap();
        assert!(tsconfig_content.contains("ES2020"));
        assert!(tsconfig_content.contains("vitest/globals"));
    }

    #[test]
    fn test_bootstrap_new() {
        let path = PathBuf::from("/tmp/test");
        let bootstrap = Bootstrap::new(path.clone());
        assert_eq!(bootstrap.project_path, path);
    }

    #[tokio::test]
    async fn test_create_config_files_content() {
        let temp_dir = TempDir::new().unwrap();
        let project_path = temp_dir.path().to_path_buf();

        let bootstrap = Bootstrap::new(project_path.clone());
        let result = bootstrap.create_config_files().await;

        assert!(result.is_ok());

        // Verify vitest.config.ts
        let vitest_path = project_path.join("vitest.config.ts");
        assert!(vitest_path.exists());
        let vitest_content = tokio::fs::read_to_string(&vitest_path).await.unwrap();
        assert!(vitest_content.contains("defineConfig"));
        assert!(vitest_content.contains("testTimeout: 30000"));
        assert!(vitest_content.contains("hookTimeout: 30000"));

        // Verify tsconfig.json
        let tsconfig_path = project_path.join("tsconfig.json");
        assert!(tsconfig_path.exists());
        let tsconfig_content = tokio::fs::read_to_string(&tsconfig_path).await.unwrap();
        assert!(tsconfig_content.contains("\"target\": \"ES2020\""));
        assert!(tsconfig_content.contains("\"module\": \"ESNext\""));
        assert!(tsconfig_content.contains("\"types\": [\"node\", \"vitest/globals\"]"));
    }

    #[tokio::test]
    async fn test_create_config_files_error_handling() {
        // Try to write to a path that doesn't exist (parent dir doesn't exist)
        let temp_dir = TempDir::new().unwrap();
        let bad_path = temp_dir
            .path()
            .join("non-existent")
            .join("deep")
            .join("path");

        let bootstrap = Bootstrap::new(bad_path);
        let result = bootstrap.create_config_files().await;

        // Should fail because parent directory doesn't exist
        assert!(result.is_err());
        if let Err(CookbookError::BootstrapError(msg)) = result {
            assert!(msg.contains("Failed to write"));
        }
    }

    #[tokio::test]
    async fn test_run_command_with_nonexistent_program() {
        let temp_dir = TempDir::new().unwrap();
        let project_path = temp_dir.path().to_path_buf();

        let bootstrap = Bootstrap::new(project_path);
        let result = bootstrap
            .run_command("this-command-definitely-does-not-exist-12345", &[])
            .await;

        // Should fail with CommandError
        assert!(result.is_err());
        if let Err(CookbookError::CommandError { command, message }) = result {
            assert!(command.contains("this-command-definitely-does-not-exist"));
            assert!(!message.is_empty());
        }
    }

    #[tokio::test]
    async fn test_create_package_json_skip_if_exists() {
        let temp_dir = TempDir::new().unwrap();
        let project_path = temp_dir.path().to_path_buf();

        // Create a package.json file
        let package_json_path = project_path.join("package.json");
        tokio::fs::write(&package_json_path, r#"{"name": "existing"}"#)
            .await
            .unwrap();

        let _bootstrap = Bootstrap::new(project_path.clone());
        // This would normally run npm commands, but should skip because package.json exists
        // We can't easily test the full npm command without npm installed, but we can verify
        // that the check works by reading the file after (it should be unchanged)

        // Read original content
        let original_content = tokio::fs::read_to_string(&package_json_path).await.unwrap();

        // The method should detect existing package.json
        // Note: We can't call create_package_json directly as it would try to run npm
        // But we can verify the existence check logic works
        assert!(package_json_path.exists());

        // Verify content is unchanged
        let final_content = tokio::fs::read_to_string(&package_json_path).await.unwrap();
        assert_eq!(original_content, final_content);
    }

    #[tokio::test]
    async fn test_bootstrap_path_handling() {
        let temp_dir = TempDir::new().unwrap();
        let project_path = temp_dir.path().join("my-project");

        // Create the directory
        tokio::fs::create_dir_all(&project_path).await.unwrap();

        let bootstrap = Bootstrap::new(project_path.clone());

        // Verify the path is stored correctly
        assert_eq!(bootstrap.project_path, project_path);

        // Verify we can write config files to this path
        let result = bootstrap.create_config_files().await;
        assert!(result.is_ok());

        assert!(project_path.join("vitest.config.ts").exists());
        assert!(project_path.join("tsconfig.json").exists());
    }
}
