//! Bootstrap module for setting up test environments
//!
//! This module handles npm package installation, configuration file generation,
//! and test environment setup for tutorial projects.

use crate::error::{CookbookError, Result};
use std::path::PathBuf;
use std::process::Stdio;
use tokio::process::Command;
use tracing::{debug, info, warn};

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
    /// use polkadot_cookbook_core::scaffold::Bootstrap;
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
    /// use polkadot_cookbook_core::scaffold::Bootstrap;
    /// use std::path::PathBuf;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let bootstrap = Bootstrap::new(PathBuf::from("./tutorials/my-tutorial"));
    /// bootstrap.setup("my-tutorial").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn setup(&self, slug: &str) -> Result<()> {
        info!("Bootstrapping test environment for: {}", slug);

        // Create package.json
        self.create_package_json(slug).await?;

        // Install dev dependencies
        self.install_dev_dependencies().await?;

        // Install dependencies
        self.install_dependencies().await?;

        // Set npm scripts
        self.set_npm_scripts().await?;

        // Create configuration files
        self.create_config_files().await?;

        info!("Bootstrap complete for: {}", slug);
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

        info!("Created package.json");
        Ok(())
    }

    /// Install development dependencies
    async fn install_dev_dependencies(&self) -> Result<()> {
        info!("Installing dev dependencies (vitest, typescript, ts-node, @types/node)...");

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

        info!("Dev dependencies installed");
        Ok(())
    }

    /// Install runtime dependencies
    async fn install_dependencies(&self) -> Result<()> {
        info!("Installing dependencies (@polkadot/api, ws)...");

        self.run_command("npm", &["install", "@polkadot/api", "ws"])
            .await?;

        info!("Dependencies installed");
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

        info!("Configuration files created");
        Ok(())
    }

    /// Run a command in the project directory
    async fn run_command(&self, program: &str, args: &[&str]) -> Result<()> {
        debug!("Running command: {} {}", program, args.join(" "));

        let output = Command::new(program)
            .args(args)
            .current_dir(&self.project_path)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await
            .map_err(|e| CookbookError::CommandError {
                command: format!("{} {}", program, args.join(" ")),
                message: e.to_string(),
            })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            warn!("Command failed: {} - {}", program, stderr);
            return Err(CookbookError::CommandError {
                command: format!("{} {}", program, args.join(" ")),
                message: stderr.to_string(),
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
}
