//! Integration tests for polkadot-cookbook-core
//!
//! These tests verify end-to-end functionality of the core library.

use polkadot_cookbook_core::{config::ProjectConfig, Scaffold};
use std::path::PathBuf;
use tempfile::TempDir;

#[tokio::test]
async fn test_create_project_end_to_end() {
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
    assert_eq!(project_info.project_path, destination.join("integration-test"));
    assert!(project_info.git_branch.is_none());

    // Verify directories were created
    let project_path = destination.join("integration-test");
    assert!(project_path.exists());
    assert!(project_path.join("src").exists());
    assert!(project_path.join("tests").exists());
    assert!(project_path.join("scripts").exists());

    // Verify files were created
    assert!(project_path.join("README.md").exists());
    assert!(project_path.join("tutorial.config.yml").exists());
    assert!(project_path.join("justfile").exists());
    assert!(project_path.join(".gitignore").exists());
    assert!(project_path.join("tests/integration-test-e2e.test.ts").exists());

    // Verify file contents
    let readme = tokio::fs::read_to_string(project_path.join("README.md"))
        .await
        .unwrap();
    assert!(readme.contains("# integration-test"));
    assert!(readme.contains("cd tutorials/integration-test"));

    let tutorial_config = tokio::fs::read_to_string(project_path.join("tutorial.config.yml"))
        .await
        .unwrap();
    assert!(tutorial_config.contains("name: Integration Test"));
    assert!(tutorial_config.contains("slug: integration-test"));
    assert!(tutorial_config.contains("category: polkadot-sdk-cookbook"));
}

#[tokio::test]
async fn test_create_project_with_existing_directory_fails() {
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
        JustfileTemplate, ReadmeTemplate, Template, TestTemplate, TutorialYmlTemplate,
    };

    // Test justfile template
    let justfile = JustfileTemplate::new();
    let content = justfile.generate();
    assert!(content.contains("default:"));
    assert!(content.contains("@just --list"));

    // Test readme template
    let readme = ReadmeTemplate::new("my-tutorial");
    let content = readme.generate();
    assert!(content.contains("# my-tutorial"));
    assert!(content.contains("## Prerequisites"));

    // Test test template
    let test = TestTemplate::new("my-tutorial");
    let content = test.generate();
    assert!(content.contains("my-tutorial e2e"));
    assert!(content.contains("@polkadot/api"));

    // Test tutorial yml template
    let yml = TutorialYmlTemplate::new("my-tutorial", "My Tutorial");
    let content = yml.generate();
    assert!(content.contains("name: My Tutorial"));
    assert!(content.contains("slug: my-tutorial"));
}

#[tokio::test]
async fn test_dry_run_mode() {
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
    use polkadot_cookbook_core::config::TutorialType;

    let config = ProjectConfig::new("builder-test")
        .with_destination(PathBuf::from("/tmp/test"))
        .with_git_init(false)
        .with_skip_install(true)
        .with_tutorial_type(TutorialType::Contracts)
        .with_category("advanced")
        .with_needs_node(false);

    assert_eq!(config.slug, "builder-test");
    assert_eq!(config.title, "Builder Test");
    assert_eq!(config.destination, PathBuf::from("/tmp/test"));
    assert!(!config.git_init);
    assert!(config.skip_install);
    assert_eq!(config.tutorial_type, TutorialType::Contracts);
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
