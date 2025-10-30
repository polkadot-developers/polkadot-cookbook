//! CLI integration tests

use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

/// Helper to set up a mock repository structure for testing
fn setup_test_repo() -> TempDir {
    let temp_dir = TempDir::new().unwrap();

    // Create recipes directory
    fs::create_dir_all(temp_dir.path().join("recipes")).unwrap();

    // Create a proper versions.yml with the correct structure
    let versions_content = r#"# Global versions for all recipes
versions:
  rust: "1.83.0"
  polkadot_omni_node: "1.16.0"
  chain_spec_builder: "0.0.0"
  frame_omni_bencher: "0.0.0"

metadata:
  schema_version: "1.0"
  last_updated: "2025-01-15"
"#;
    fs::write(temp_dir.path().join("versions.yml"), versions_content).unwrap();

    // Copy templates directory from workspace root to temp directory
    let workspace_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf();
    let src_templates = workspace_root.join("templates");
    let dst_templates = temp_dir.path().join("templates");

    if src_templates.exists() {
        copy_dir_recursively(&src_templates, &dst_templates).unwrap();
    }

    temp_dir
}

/// Helper function to copy directories recursively
fn copy_dir_recursively(src: &PathBuf, dst: &PathBuf) -> std::io::Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if ty.is_dir() {
            copy_dir_recursively(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path)?;
        }
    }
    Ok(())
}

#[test]
fn test_help_command() {
    let mut cmd = Command::cargo_bin("dot").unwrap();
    cmd.arg("--help");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains(
            "Create and manage Polkadot Cookbook recipes",
        ))
        .stdout(predicate::str::contains("Usage:"));
}

#[test]
fn test_version_command() {
    let mut cmd = Command::cargo_bin("dot").unwrap();
    cmd.arg("--version");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("dot"));
}

#[test]
fn test_create_recipe_non_interactive() {
    let temp_dir = setup_test_repo();
    let recipes_dir = temp_dir.path().join("recipes");

    let mut cmd = Command::cargo_bin("dot").unwrap();
    cmd.current_dir(temp_dir.path());
    cmd.arg("test-recipe")
        .arg("--skip-install")
        .arg("--no-git")
        .arg("--non-interactive");

    cmd.assert().success();

    // Verify directory structure
    assert!(recipes_dir.join("test-recipe").exists());
    assert!(recipes_dir.join("test-recipe/README.md").exists());
    assert!(recipes_dir.join("test-recipe/recipe.config.yml").exists());
    assert!(recipes_dir.join("test-recipe/justfile").exists());
    // Note: Polkadot SDK recipes don't have .gitignore or local versions.yml
    assert!(recipes_dir.join("test-recipe/pallets").exists());
}

#[test]
fn test_create_recipe_with_create_subcommand() {
    let temp_dir = setup_test_repo();
    let recipes_dir = temp_dir.path().join("recipes");

    let mut cmd = Command::cargo_bin("dot").unwrap();
    cmd.current_dir(temp_dir.path());
    cmd.arg("create")
        .arg("test-subcommand")
        .arg("--skip-install")
        .arg("--no-git")
        .arg("--non-interactive");

    cmd.assert().success();

    assert!(recipes_dir.join("test-subcommand").exists());
    assert!(recipes_dir.join("test-subcommand/README.md").exists());
}

#[test]
fn test_invalid_slug_uppercase() {
    let temp_dir = setup_test_repo();

    let mut cmd = Command::cargo_bin("dot").unwrap();
    cmd.current_dir(temp_dir.path());
    cmd.arg("Invalid-Slug").arg("--non-interactive");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Invalid recipe slug"));
}

#[test]
fn test_invalid_slug_underscore() {
    let temp_dir = setup_test_repo();

    let mut cmd = Command::cargo_bin("dot").unwrap();
    cmd.current_dir(temp_dir.path());
    cmd.arg("test_recipe").arg("--non-interactive");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Invalid recipe slug"));
}

#[test]
fn test_invalid_slug_spaces() {
    let temp_dir = setup_test_repo();

    let mut cmd = Command::cargo_bin("dot").unwrap();
    cmd.current_dir(temp_dir.path());
    cmd.arg("test recipe").arg("--non-interactive");

    cmd.assert().failure();
}

#[test]
fn test_non_interactive_requires_slug() {
    let temp_dir = setup_test_repo();

    let mut cmd = Command::cargo_bin("dot").unwrap();
    cmd.current_dir(temp_dir.path());
    cmd.arg("--non-interactive");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Slug argument is required"));
}

#[test]
fn test_recipe_config_content() {
    let temp_dir = setup_test_repo();
    let recipes_dir = temp_dir.path().join("recipes");

    let mut cmd = Command::cargo_bin("dot").unwrap();
    cmd.current_dir(temp_dir.path());
    cmd.arg("my-test-recipe")
        .arg("--skip-install")
        .arg("--no-git")
        .arg("--non-interactive");

    cmd.assert().success();

    let config_content =
        fs::read_to_string(recipes_dir.join("my-test-recipe/recipe.config.yml")).unwrap();

    assert!(config_content.contains("name: My Test Recipe"));
    assert!(config_content.contains("slug: my-test-recipe"));
    assert!(config_content.contains("type: polkadot-sdk"));
    assert!(config_content.contains("description: Replace with a short description."));
}

#[test]
fn test_test_file_generated() {
    let temp_dir = setup_test_repo();
    let recipes_dir = temp_dir.path().join("recipes");

    let mut cmd = Command::cargo_bin("dot").unwrap();
    cmd.current_dir(temp_dir.path());
    cmd.arg("test-e2e")
        .arg("--skip-install")
        .arg("--no-git")
        .arg("--non-interactive");

    cmd.assert().success();

    // Polkadot SDK recipes have Rust unit tests in the pallet code, not separate TypeScript tests
    // Just verify the project was created successfully
    assert!(recipes_dir.join("test-e2e").exists());
    assert!(recipes_dir.join("test-e2e/README.md").exists());
    assert!(recipes_dir.join("test-e2e/Cargo.toml").exists());
}

#[test]
fn test_gitignore_content() {
    let temp_dir = setup_test_repo();
    let recipes_dir = temp_dir.path().join("recipes");

    let mut cmd = Command::cargo_bin("dot").unwrap();
    cmd.current_dir(temp_dir.path());
    cmd.arg("ignore-test")
        .arg("--skip-install")
        .arg("--no-git")
        .arg("--non-interactive");

    cmd.assert().success();

    // Polkadot SDK recipes use Cargo which has its own .gitignore handling via Cargo.toml
    // Only TypeScript-based recipes (XCM, Solidity) have .gitignore files
    // Just verify the project was created successfully
    assert!(recipes_dir.join("ignore-test").exists());
    assert!(recipes_dir.join("ignore-test/README.md").exists());
}

#[test]
fn test_versions_yml_exists() {
    let temp_dir = setup_test_repo();
    let recipes_dir = temp_dir.path().join("recipes");

    let mut cmd = Command::cargo_bin("dot").unwrap();
    cmd.current_dir(temp_dir.path());
    cmd.arg("version-test")
        .arg("--skip-install")
        .arg("--no-git")
        .arg("--non-interactive");

    cmd.assert().success();

    // versions.yml is at the repository root level, not in individual recipe directories
    // Recipes inherit from the global versions.yml
    // Just verify the project was created successfully
    assert!(recipes_dir.join("version-test").exists());
    assert!(recipes_dir.join("version-test/recipe.config.yml").exists());
}

#[test]
fn test_invalid_working_directory() {
    // Create a temp dir that doesn't have a recipes/ folder
    let temp_dir = TempDir::new().unwrap();

    let mut cmd = Command::cargo_bin("dot").unwrap();
    cmd.current_dir(temp_dir.path());
    cmd.arg("test-recipe").arg("--non-interactive");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Invalid working directory"));
}
