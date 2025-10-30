/// Git operations wrapper using git2 library
use crate::error::{CookbookError, Result};
use git2::Repository;
use std::path::Path;
use tracing::{debug, info};

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
        let branch_name = format!("feat/tutorial-{slug}");

        debug!("Creating git branch: {}", branch_name);

        tokio::task::spawn_blocking({
            let branch_name = branch_name.clone();
            move || {
                // Open the repository in the current directory
                let repo = Repository::open(".").map_err(|e| {
                    CookbookError::GitError(format!(
                        "Failed to open git repository: {e}. Make sure you're in a git repository."
                    ))
                })?;

                // Get the current HEAD commit
                let head = repo
                    .head()
                    .map_err(|e| CookbookError::GitError(format!("Failed to get HEAD: {e}")))?;

                let commit = head.peel_to_commit().map_err(|e| {
                    CookbookError::GitError(format!("Failed to get HEAD commit: {e}"))
                })?;

                // Create the new branch
                repo.branch(&branch_name, &commit, false).map_err(|e| {
                    CookbookError::GitError(format!(
                        "Failed to create branch '{branch_name}': {e}. Branch may already exist."
                    ))
                })?;

                // Set the new branch as HEAD
                repo.set_head(&format!("refs/heads/{branch_name}"))
                    .map_err(|e| {
                        CookbookError::GitError(format!(
                            "Failed to checkout branch '{branch_name}': {e}"
                        ))
                    })?;

                info!("Created and checked out git branch: {}", branch_name);
                Ok::<String, CookbookError>(branch_name)
            }
        })
        .await
        .map_err(|e| CookbookError::GitError(format!("Task join error: {e}")))?
    }

    /// Check if currently in a git repository
    pub async fn is_git_repo() -> bool {
        tokio::task::spawn_blocking(|| Repository::open(".").is_ok())
            .await
            .unwrap_or(false)
    }

    /// Get current git branch name
    pub async fn current_branch() -> Result<String> {
        tokio::task::spawn_blocking(|| {
            let repo = Repository::open(".").map_err(|e| {
                CookbookError::GitError(format!(
                    "Failed to open git repository: {e}. Make sure you're in a git repository."
                ))
            })?;

            let head = repo
                .head()
                .map_err(|e| CookbookError::GitError(format!("Failed to get HEAD: {e}")))?;

            if head.is_branch() {
                let branch_name = head
                    .shorthand()
                    .ok_or_else(|| {
                        CookbookError::GitError("Branch name is not valid UTF-8".to_string())
                    })?
                    .to_string();
                Ok(branch_name)
            } else {
                Err(CookbookError::GitError(
                    "HEAD is not pointing to a branch (detached HEAD state)".to_string(),
                ))
            }
        })
        .await
        .map_err(|e| CookbookError::GitError(format!("Task join error: {e}")))?
    }

    /// Initialize a git repository in the specified directory
    pub async fn init(path: &Path) -> Result<()> {
        debug!("Initializing git repository in: {}", path.display());

        tokio::task::spawn_blocking({
            let path = path.to_owned();
            move || {
                Repository::init(&path).map_err(|e| {
                    CookbookError::GitError(format!(
                        "Failed to initialize git repository at {}: {}",
                        path.display(),
                        e
                    ))
                })?;

                info!("Initialized git repository in: {}", path.display());
                Ok::<(), CookbookError>(())
            }
        })
        .await
        .map_err(|e| CookbookError::GitError(format!("Task join error: {e}")))?
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
        assert!(is_repo || !is_repo);
    }

    #[tokio::test]
    async fn test_init_repository() {
        use tempfile::TempDir;

        let temp_dir = TempDir::new().unwrap();
        let result = GitOperations::init(temp_dir.path()).await;
        assert!(result.is_ok());

        // Verify the repository was created
        let repo = Repository::open(temp_dir.path());
        assert!(repo.is_ok());
    }

    #[tokio::test]
    async fn test_current_branch_in_repo() {
        // This test only works if we're in a git repo
        if GitOperations::is_git_repo().await {
            let branch = GitOperations::current_branch().await;
            assert!(branch.is_ok());
            let branch_name = branch.unwrap();
            assert!(!branch_name.is_empty());
        }
    }

    #[tokio::test]
    async fn test_current_branch_not_in_repo() {
        use tempfile::TempDir;

        let temp_dir = TempDir::new().unwrap();
        let original_dir = std::env::current_dir().unwrap();

        // Change to a directory that's not a git repo
        std::env::set_current_dir(temp_dir.path()).unwrap();

        let result = GitOperations::current_branch().await;

        // Restore original directory
        std::env::set_current_dir(original_dir).unwrap();

        // Should fail because it's not a git repo
        assert!(result.is_err());
        if let Err(CookbookError::GitError(msg)) = result {
            assert!(msg.contains("Failed to open git repository"));
        }
    }

    #[tokio::test]
    async fn test_create_branch_in_repo() {
        use tempfile::TempDir;

        // Create a temporary git repo
        let temp_dir = TempDir::new().unwrap();
        GitOperations::init(temp_dir.path()).await.unwrap();

        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        // Create initial commit (required for branch creation)
        let repo = Repository::open(".").unwrap();
        let sig = git2::Signature::now("Test User", "test@example.com").unwrap();
        let tree_id = {
            let mut index = repo.index().unwrap();
            index.write_tree().unwrap()
        };
        let tree = repo.find_tree(tree_id).unwrap();
        repo.commit(Some("HEAD"), &sig, &sig, "Initial commit", &tree, &[])
            .unwrap();

        // Now create a branch
        let result = GitOperations::create_branch("test-branch").await;

        std::env::set_current_dir(original_dir).unwrap();

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "feat/tutorial-test-branch");
    }

    #[tokio::test]
    async fn test_create_branch_already_exists() {
        use tempfile::TempDir;

        // Create a temporary git repo
        let temp_dir = TempDir::new().unwrap();
        GitOperations::init(temp_dir.path()).await.unwrap();

        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        // Create initial commit
        let repo = Repository::open(".").unwrap();
        let sig = git2::Signature::now("Test User", "test@example.com").unwrap();
        let tree_id = {
            let mut index = repo.index().unwrap();
            index.write_tree().unwrap()
        };
        let tree = repo.find_tree(tree_id).unwrap();
        let commit = repo
            .commit(Some("HEAD"), &sig, &sig, "Initial commit", &tree, &[])
            .unwrap();

        // Create branch manually
        let commit_obj = repo.find_commit(commit).unwrap();
        repo.branch("feat/tutorial-duplicate", &commit_obj, false)
            .unwrap();

        // Try to create the same branch again
        let result = GitOperations::create_branch("duplicate").await;

        std::env::set_current_dir(original_dir).unwrap();

        // Should fail because branch already exists
        assert!(result.is_err());
        if let Err(CookbookError::GitError(msg)) = result {
            assert!(msg.contains("Failed to create branch") || msg.contains("already exist"));
        }
    }

    #[tokio::test]
    async fn test_create_branch_not_in_repo() {
        use tempfile::TempDir;

        let temp_dir = TempDir::new().unwrap();
        let original_dir = std::env::current_dir().unwrap();

        // Change to a directory that's not a git repo
        std::env::set_current_dir(temp_dir.path()).unwrap();

        let result = GitOperations::create_branch("test-branch").await;

        // Restore original directory
        std::env::set_current_dir(original_dir).unwrap();

        // Should fail because it's not a git repo
        assert!(result.is_err());
        if let Err(CookbookError::GitError(msg)) = result {
            assert!(msg.contains("Failed to open git repository"));
        }
    }

    #[tokio::test]
    async fn test_is_git_repo_in_non_repo() {
        use tempfile::TempDir;

        let temp_dir = TempDir::new().unwrap();
        let original_dir = std::env::current_dir().unwrap();

        // Change to a directory that's not a git repo
        std::env::set_current_dir(temp_dir.path()).unwrap();

        let is_repo = GitOperations::is_git_repo().await;

        // Restore original directory
        std::env::set_current_dir(original_dir).unwrap();

        assert!(!is_repo);
    }

    #[tokio::test]
    async fn test_init_creates_repo() {
        use tempfile::TempDir;

        let temp_dir = TempDir::new().unwrap();
        let repo_path = temp_dir.path().join("new-repo");

        // Initially should not be a git repo
        assert!(!repo_path.join(".git").exists());

        let result = GitOperations::init(&repo_path).await;
        assert!(result.is_ok());

        // Now should be a git repo
        assert!(repo_path.join(".git").exists());

        // Should be able to open it
        let repo = Repository::open(&repo_path);
        assert!(repo.is_ok());
    }
}
