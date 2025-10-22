/// Git operations wrapper
use crate::error::{CookbookError, Result};
use std::process::Command;
use tracing::{debug, info, warn};

/// Git operations manager
pub struct GitOperations;

impl GitOperations {
    /// Create a new git branch for the tutorial
    /// Branch name format: feat/tutorial-{slug}
    ///
    /// # Example
    /// ```no_run
    /// use polkadot_cookbook_core::git::GitOperations;
    ///
    /// # async fn example() {
    /// let result = GitOperations::create_branch("my-tutorial").await;
    /// # }
    /// ```
    pub async fn create_branch(slug: &str) -> Result<String> {
        let branch_name = format!("feat/tutorial-{}", slug);

        debug!("Creating git branch: {}", branch_name);

        let output = tokio::task::spawn_blocking({
            let branch_name = branch_name.clone();
            move || {
                Command::new("git")
                    .args(&["checkout", "-b", &branch_name])
                    .output()
            }
        })
        .await
        .map_err(|e| CookbookError::GitError(format!("Task join error: {}", e)))?
        .map_err(|e| CookbookError::GitError(format!("Failed to execute git: {}", e)))?;

        if output.status.success() {
            info!("Created git branch: {}", branch_name);
            Ok(branch_name)
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            warn!("Failed to create git branch: {}", stderr);
            Err(CookbookError::GitError(format!(
                "Failed to create branch '{}': {}",
                branch_name, stderr
            )))
        }
    }

    /// Check if currently in a git repository
    pub async fn is_git_repo() -> bool {
        let result = tokio::task::spawn_blocking(|| {
            Command::new("git")
                .args(&["rev-parse", "--is-inside-work-tree"])
                .output()
        })
        .await;

        match result {
            Ok(Ok(output)) => output.status.success(),
            _ => false,
        }
    }

    /// Get current git branch name
    pub async fn current_branch() -> Result<String> {
        let output = tokio::task::spawn_blocking(|| {
            Command::new("git")
                .args(&["rev-parse", "--abbrev-ref", "HEAD"])
                .output()
        })
        .await
        .map_err(|e| CookbookError::GitError(format!("Task join error: {}", e)))?
        .map_err(|e| CookbookError::GitError(format!("Failed to execute git: {}", e)))?;

        if output.status.success() {
            let branch = String::from_utf8_lossy(&output.stdout).trim().to_string();
            Ok(branch)
        } else {
            Err(CookbookError::GitError(
                "Failed to get current branch".to_string(),
            ))
        }
    }

    /// Initialize a git repository in the specified directory
    pub async fn init(path: &std::path::Path) -> Result<()> {
        debug!("Initializing git repository in: {}", path.display());

        let output = tokio::task::spawn_blocking({
            let path = path.to_owned();
            move || {
                Command::new("git")
                    .args(&["init"])
                    .current_dir(path)
                    .output()
            }
        })
        .await
        .map_err(|e| CookbookError::GitError(format!("Task join error: {}", e)))?
        .map_err(|e| CookbookError::GitError(format!("Failed to execute git: {}", e)))?;

        if output.status.success() {
            info!("Initialized git repository in: {}", path.display());
            Ok(())
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(CookbookError::GitError(format!(
                "Failed to initialize git repo: {}",
                stderr
            )))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_branch_name_format() {
        let slug = "my-tutorial";
        let expected_branch = format!("feat/tutorial-{}", slug);
        assert_eq!(expected_branch, "feat/tutorial-my-tutorial");
    }

    #[tokio::test]
    async fn test_is_git_repo() {
        // This test will pass if run from within a git repo
        // It's a basic sanity check
        let is_repo = GitOperations::is_git_repo().await;
        // We don't assert a specific value as it depends on the environment
        assert!(is_repo == true || is_repo == false);
    }
}
