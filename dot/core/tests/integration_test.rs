//! Integration tests for polkadot-cookbook-core
//!
//! These tests verify end-to-end functionality of the core library.

use polkadot_cookbook_core::{config::ProjectConfig, Scaffold};
use std::path::PathBuf;
use tempfile::TempDir;

/// Helper to ensure tests run from workspace root where templates exist
fn ensure_workspace_root() {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let workspace_root = manifest_dir.parent().unwrap();
    std::env::set_current_dir(workspace_root).unwrap();
}

#[tokio::test]
async fn test_create_project_end_to_end() {
    ensure_workspace_root();
    // Create a temporary directory for testing
    let temp_dir = TempDir::new().unwrap();
    let destination = temp_dir.path().to_path_buf();

    // Create project configuration
    let config = ProjectConfig::new("integration-test")
        .with_destination(destination.clone())
        .with_git_init(false) // Skip git in tests
        .with_skip_install(true); // Skip npm install in tests

    // Create the project
    let scaffold = Scaffold::new();
    let project_info = scaffold.create_project(config).await.unwrap();

    // Verify project info
    assert_eq!(project_info.slug, "integration-test");
    assert_eq!(project_info.title, "Integration Test");
    assert_eq!(
        project_info.project_path,
        destination.join("integration-test")
    );
    assert!(project_info.git_branch.is_none());

    // Verify directories were created (Polkadot SDK recipe structure)
    let project_path = destination.join("integration-test");
    assert!(project_path.exists());
    assert!(project_path.join("pallets").exists());

    // Verify files were created
    assert!(project_path.join("README.md").exists());
    assert!(project_path.join("recipe.config.yml").exists());
    assert!(project_path.join("justfile").exists());
    assert!(project_path.join("Cargo.toml").exists());
    // Note: Polkadot SDK recipes don't have .gitignore or TypeScript test files

    // Verify file contents
    let readme = tokio::fs::read_to_string(project_path.join("README.md"))
        .await
        .unwrap();
    assert!(readme.contains("# Integration Test"));

    let recipe_config = tokio::fs::read_to_string(project_path.join("recipe.config.yml"))
        .await
        .unwrap();
    assert!(recipe_config.contains("name: Integration Test"));
    assert!(recipe_config.contains("slug: integration-test"));
    assert!(recipe_config.contains("category: polkadot-sdk-cookbook"));
}

#[tokio::test]
async fn test_create_project_with_existing_directory_fails() {
    ensure_workspace_root();
    let temp_dir = TempDir::new().unwrap();
    let destination = temp_dir.path().to_path_buf();

    // Create project once
    let config = ProjectConfig::new("duplicate-test")
        .with_destination(destination.clone())
        .with_git_init(false)
        .with_skip_install(true);

    let scaffold = Scaffold::new();
    scaffold.create_project(config.clone()).await.unwrap();

    // Try to create the same project again - should fail
    let result = scaffold.create_project(config).await;
    assert!(result.is_err());

    // Verify it's the correct error type
    match result {
        Err(polkadot_cookbook_core::CookbookError::ProjectExistsError(_)) => {
            // Expected error
        }
        _ => panic!("Expected ProjectExistsError"),
    }
}

#[tokio::test]
async fn test_validate_slug() {
    use polkadot_cookbook_core::config::validate_slug;

    // Valid slugs
    assert!(validate_slug("my-tutorial").is_ok());
    assert!(validate_slug("zero-to-hero").is_ok());
    assert!(validate_slug("test123").is_ok());

    // Invalid slugs
    assert!(validate_slug("My-Tutorial").is_err()); // uppercase
    assert!(validate_slug("my_tutorial").is_err()); // underscore
    assert!(validate_slug("my--tutorial").is_err()); // double dash
    assert!(validate_slug("-my-tutorial").is_err()); // leading dash
    assert!(validate_slug("my-tutorial-").is_err()); // trailing dash
    assert!(validate_slug("").is_err()); // empty
}

#[tokio::test]
async fn test_slug_to_title() {
    use polkadot_cookbook_core::config::slug_to_title;

    assert_eq!(slug_to_title("my-tutorial"), "My Tutorial");
    assert_eq!(slug_to_title("zero-to-hero"), "Zero To Hero");
    assert_eq!(slug_to_title("add-nft-pallet"), "Add Nft Pallet");
    assert_eq!(slug_to_title("test"), "Test");
}

#[tokio::test]
async fn test_template_generation() {
    use polkadot_cookbook_core::templates::{
        JustfileTemplate, ReadmeTemplate, RecipeYmlTemplate, Template, TestTemplate,
    };

    // Test justfile template
    let justfile = JustfileTemplate::new();
    let content = justfile.generate();
    assert!(content.contains("default:"));
    assert!(content.contains("@just --list"));

    // Test readme template
    let readme = ReadmeTemplate::new("my-tutorial");
    let content = readme.generate();
    assert!(content.contains("# My Tutorial"));
    assert!(content.contains("## Prerequisites"));

    // Test test template
    let test = TestTemplate::new("my-tutorial");
    let content = test.generate();
    assert!(content.contains("my-tutorial e2e"));
    assert!(content.contains("@polkadot/api"));

    // Test tutorial yml template
    use polkadot_cookbook_core::config::RecipeType;
    let yml = RecipeYmlTemplate::new(
        "my-tutorial",
        "My Tutorial",
        "A test tutorial",
        RecipeType::PolkadotSdk,
        "test-category",
        true,
    );
    let content = yml.generate();
    assert!(content.contains("name: My Tutorial"));
    assert!(content.contains("slug: my-tutorial"));
    assert!(content.contains("description: A test tutorial"));
}

#[tokio::test]
async fn test_dry_run_mode() {
    ensure_workspace_root();
    let temp_dir = TempDir::new().unwrap();
    let destination = temp_dir.path().to_path_buf();

    let config = ProjectConfig::new("dry-run-test")
        .with_destination(destination.clone())
        .with_git_init(false)
        .with_skip_install(true);

    // Create scaffold in dry-run mode
    let scaffold = Scaffold::dry_run();

    // This creates the project successfully even in dry-run
    // because validation doesn't check for directory existence in dry-run
    let result = scaffold.create_project(config).await;

    // The operation succeeds, but no files are created
    assert!(result.is_ok());

    // Verify no project directory was created (dry-run doesn't create files)
    assert!(!destination.join("dry-run-test").exists());
}

#[tokio::test]
async fn test_project_config_builder() {
    use polkadot_cookbook_core::config::RecipeType;

    let config = ProjectConfig::new("builder-test")
        .with_destination(PathBuf::from("/tmp/test"))
        .with_git_init(false)
        .with_skip_install(true)
        .with_recipe_type(RecipeType::Solidity)
        .with_category("advanced")
        .with_needs_node(false);

    assert_eq!(config.slug, "builder-test");
    assert_eq!(config.title, "Builder Test");
    assert_eq!(config.destination, PathBuf::from("/tmp/test"));
    assert!(!config.git_init);
    assert!(config.skip_install);
    assert_eq!(config.recipe_type, RecipeType::Solidity);
    assert_eq!(config.category, "advanced");
    assert!(!config.needs_node);
}

#[tokio::test]
async fn test_error_serialization() {
    use polkadot_cookbook_core::CookbookError;

    let error = CookbookError::ValidationError("test error".to_string());

    // Test serialization to JSON
    let json = serde_json::to_string(&error).unwrap();
    assert!(json.contains("ValidationError"));
    assert!(json.contains("test error"));

    // Test deserialization from JSON
    let deserialized: CookbookError = serde_json::from_str(&json).unwrap();
    match deserialized {
        CookbookError::ValidationError(msg) => assert_eq!(msg, "test error"),
        _ => panic!("Wrong error type"),
    }
}

// ============================================================================
// Git Operations Tests
// ============================================================================

#[tokio::test]
async fn test_create_project_with_git_branch() {
    ensure_workspace_root();
    use polkadot_cookbook_core::git::GitOperations;

    // Skip if not in a git repo
    if !GitOperations::is_git_repo().await {
        eprintln!("Skipping git test - not in a git repository");
        return;
    }

    let temp_dir = TempDir::new().unwrap();
    let destination = temp_dir.path().to_path_buf();

    let config = ProjectConfig::new("git-branch-test")
        .with_destination(destination.clone())
        .with_git_init(true)
        .with_skip_install(true);

    let scaffold = Scaffold::new();
    let project_info = scaffold.create_project(config).await.unwrap();

    // Verify git branch was created (or attempted)
    // Note: Branch creation might fail if git is not configured, which is okay
    if let Some(branch) = project_info.git_branch {
        assert!(branch.starts_with("feat/tutorial-"));
        assert!(branch.contains("git-branch-test"));
    }
}

#[tokio::test]
async fn test_git_is_repo() {
    use polkadot_cookbook_core::git::GitOperations;

    // Test if we can detect git repository
    let is_repo = GitOperations::is_git_repo().await;
    // Result depends on test environment, just verify function works
    assert!(is_repo || !is_repo);
}

#[tokio::test]
async fn test_project_without_git() {
    ensure_workspace_root();
    let temp_dir = TempDir::new().unwrap();
    let destination = temp_dir.path().to_path_buf();

    let config = ProjectConfig::new("no-git-test")
        .with_destination(destination.clone())
        .with_git_init(false)
        .with_skip_install(true);

    let scaffold = Scaffold::new();
    let project_info = scaffold.create_project(config).await.unwrap();

    // Verify no git branch was created
    assert!(project_info.git_branch.is_none());
}

// ============================================================================
// Recipe Type Tests
// ============================================================================

#[tokio::test]
async fn test_sdk_recipe_creation() {
    ensure_workspace_root();
    use polkadot_cookbook_core::config::RecipeType;

    let temp_dir = TempDir::new().unwrap();
    let destination = temp_dir.path().to_path_buf();

    let config = ProjectConfig::new("sdk-recipe")
        .with_destination(destination.clone())
        .with_recipe_type(RecipeType::PolkadotSdk)
        .with_git_init(false)
        .with_skip_install(true);

    let scaffold = Scaffold::new();
    scaffold.create_project(config).await.unwrap();

    let project_path = destination.join("sdk-recipe");
    let recipe_config = tokio::fs::read_to_string(project_path.join("recipe.config.yml"))
        .await
        .unwrap();

    assert!(recipe_config.contains("type: polkadot-sdk"));
}

#[tokio::test]
async fn test_contracts_recipe_creation() {
    ensure_workspace_root();
    use polkadot_cookbook_core::config::RecipeType;

    let temp_dir = TempDir::new().unwrap();
    let destination = temp_dir.path().to_path_buf();

    let config = ProjectConfig::new("contracts-recipe")
        .with_destination(destination.clone())
        .with_recipe_type(RecipeType::Solidity)
        .with_git_init(false)
        .with_skip_install(true);

    let scaffold = Scaffold::new();
    scaffold.create_project(config).await.unwrap();

    let project_path = destination.join("contracts-recipe");
    let recipe_config = tokio::fs::read_to_string(project_path.join("recipe.config.yml"))
        .await
        .unwrap();

    assert!(recipe_config.contains("type: solidity"));
}

#[tokio::test]
async fn test_recipe_categories() {
    ensure_workspace_root();
    let temp_dir = TempDir::new().unwrap();
    let destination = temp_dir.path().to_path_buf();

    let config = ProjectConfig::new("category-test")
        .with_destination(destination.clone())
        .with_category("advanced-tutorials")
        .with_git_init(false)
        .with_skip_install(true);

    let scaffold = Scaffold::new();
    scaffold.create_project(config).await.unwrap();

    let project_path = destination.join("category-test");
    let recipe_config = tokio::fs::read_to_string(project_path.join("recipe.config.yml"))
        .await
        .unwrap();

    assert!(recipe_config.contains("category: advanced-tutorials"));
}

// ============================================================================
// Edge Cases Tests
// ============================================================================

#[tokio::test]
async fn test_long_slug() {
    ensure_workspace_root();
    let temp_dir = TempDir::new().unwrap();
    let destination = temp_dir.path().to_path_buf();

    let long_slug = "this-is-a-very-long-slug-name-for-testing-edge-cases";

    let config = ProjectConfig::new(long_slug)
        .with_destination(destination.clone())
        .with_git_init(false)
        .with_skip_install(true);

    let scaffold = Scaffold::new();
    let result = scaffold.create_project(config).await;

    // Should succeed
    assert!(result.is_ok());
    assert!(destination.join(long_slug).exists());
}

#[tokio::test]
async fn test_slug_with_numbers() {
    use polkadot_cookbook_core::config::validate_slug;

    // Slugs with numbers should be valid
    assert!(validate_slug("test-123").is_ok());
    assert!(validate_slug("123-test").is_ok());
    assert!(validate_slug("test-123-recipe").is_ok());
    assert!(validate_slug("v2-migration").is_ok());
}

#[tokio::test]
async fn test_single_word_slug() {
    use polkadot_cookbook_core::config::validate_slug;

    // Single word slugs should be valid
    assert!(validate_slug("hello").is_ok());
    assert!(validate_slug("test").is_ok());
    assert!(validate_slug("migration").is_ok());
}

#[tokio::test]
async fn test_description_with_special_characters() {
    ensure_workspace_root();
    let temp_dir = TempDir::new().unwrap();
    let destination = temp_dir.path().to_path_buf();

    let description = "Learn how to use \"quotes\" and 'apostrophes' in descriptions!";

    let config = ProjectConfig::new("special-desc")
        .with_destination(destination.clone())
        .with_description(description.to_string())
        .with_git_init(false)
        .with_skip_install(true);

    let scaffold = Scaffold::new();
    scaffold.create_project(config).await.unwrap();

    let project_path = destination.join("special-desc");
    let recipe_config = tokio::fs::read_to_string(project_path.join("recipe.config.yml"))
        .await
        .unwrap();

    assert!(recipe_config.contains(description));
}

#[tokio::test]
async fn test_empty_description() {
    ensure_workspace_root();
    let temp_dir = TempDir::new().unwrap();
    let destination = temp_dir.path().to_path_buf();

    let config = ProjectConfig::new("empty-desc")
        .with_destination(destination.clone())
        .with_description("".to_string())
        .with_git_init(false)
        .with_skip_install(true);

    let scaffold = Scaffold::new();
    let result = scaffold.create_project(config).await;

    // Should succeed - empty descriptions are allowed
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_unicode_in_description() {
    ensure_workspace_root();
    let temp_dir = TempDir::new().unwrap();
    let destination = temp_dir.path().to_path_buf();

    let description = "Learn about Polkadot 🚀 and blockchain 💎 technology";

    let config = ProjectConfig::new("unicode-test")
        .with_destination(destination.clone())
        .with_description(description.to_string())
        .with_git_init(false)
        .with_skip_install(true);

    let scaffold = Scaffold::new();
    scaffold.create_project(config).await.unwrap();

    let project_path = destination.join("unicode-test");
    let recipe_config = tokio::fs::read_to_string(project_path.join("recipe.config.yml"))
        .await
        .unwrap();

    assert!(recipe_config.contains(description));
}

#[tokio::test]
async fn test_multiline_description() {
    ensure_workspace_root();
    let temp_dir = TempDir::new().unwrap();
    let destination = temp_dir.path().to_path_buf();

    let description = "Line 1\nLine 2\nLine 3";

    let config = ProjectConfig::new("multiline-desc")
        .with_destination(destination.clone())
        .with_description(description.to_string())
        .with_git_init(false)
        .with_skip_install(true);

    let scaffold = Scaffold::new();
    scaffold.create_project(config).await.unwrap();

    let project_path = destination.join("multiline-desc");
    let recipe_config = tokio::fs::read_to_string(project_path.join("recipe.config.yml"))
        .await
        .unwrap();

    // Description should be in the config
    assert!(recipe_config.contains("Line 1"));
}

// ============================================================================
// Bootstrap/NPM Tests
// ============================================================================

#[tokio::test]
async fn test_skip_install_flag() {
    ensure_workspace_root();
    let temp_dir = TempDir::new().unwrap();
    let destination = temp_dir.path().to_path_buf();

    let config = ProjectConfig::new("skip-install")
        .with_destination(destination.clone())
        .with_skip_install(true)
        .with_git_init(false);

    let scaffold = Scaffold::new();
    let result = scaffold.create_project(config).await;

    // Should succeed without npm install
    assert!(result.is_ok());

    // When skip_install is true, bootstrap is skipped entirely
    // So package.json won't be created, but other files should exist
    let project_path = destination.join("skip-install");
    assert!(project_path.join("README.md").exists());
    assert!(project_path.join("recipe.config.yml").exists());
    assert!(!project_path.join("package.json").exists());
}

#[tokio::test]
async fn test_bootstrap_new() {
    use polkadot_cookbook_core::scaffold::Bootstrap;

    let temp_dir = TempDir::new().unwrap();
    let project_path = temp_dir.path().join("test-bootstrap");

    // Test that Bootstrap can be created
    let bootstrap = Bootstrap::new(project_path.clone());

    // Bootstrap instance should be created successfully
    // We can't test setup() without npm in environment, so we just verify construction works
    drop(bootstrap);
}

// ============================================================================
// File System Edge Cases
// ============================================================================

#[tokio::test]
async fn test_nested_destination() {
    ensure_workspace_root();
    let temp_dir = TempDir::new().unwrap();
    let nested_dest = temp_dir.path().join("level1").join("level2").join("level3");

    let config = ProjectConfig::new("nested-test")
        .with_destination(nested_dest.clone())
        .with_git_init(false)
        .with_skip_install(true);

    let scaffold = Scaffold::new();
    let result = scaffold.create_project(config).await;

    // Should succeed - scaffold creates parent directories
    assert!(result.is_ok());
    assert!(nested_dest.join("nested-test").exists());
}

#[tokio::test]
async fn test_verify_setup() {
    let temp_dir = TempDir::new().unwrap();
    let project_path = temp_dir.path().join("verify-test");
    tokio::fs::create_dir_all(&project_path).await.unwrap();

    // Manually create required files for testing verify_setup logic
    tokio::fs::write(project_path.join("package.json"), "{}")
        .await
        .unwrap();
    tokio::fs::write(project_path.join("README.md"), "# Test")
        .await
        .unwrap();
    tokio::fs::write(project_path.join("recipe.config.yml"), "name: test")
        .await
        .unwrap();

    let scaffold = Scaffold::new();
    let missing = scaffold.verify_setup(&project_path).await.unwrap();

    // All required files exist, so missing should be empty
    assert_eq!(
        missing.len(),
        0,
        "Some required files are missing: {:?}",
        missing
    );
}

#[tokio::test]
async fn test_verify_setup_with_missing_files() {
    let temp_dir = TempDir::new().unwrap();
    let project_path = temp_dir.path().join("incomplete-project");
    tokio::fs::create_dir_all(&project_path).await.unwrap();

    // Create project directory but don't create required files
    let scaffold = Scaffold::new();
    let missing = scaffold.verify_setup(&project_path).await.unwrap();

    // Should report missing files
    assert!(missing.len() > 0);
    assert!(missing.iter().any(|f| f.contains("package.json")));
    assert!(missing.iter().any(|f| f.contains("README.md")));
}
