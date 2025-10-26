use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

/// Helper to set up a mock repository structure for testing
fn setup_test_repo() -> TempDir {
    let temp_dir = TempDir::new().unwrap();

    // Create recipes directory
    fs::create_dir_all(temp_dir.path().join("recipes")).unwrap();

    // Create a minimal versions.yml to satisfy working directory validation
    let versions_content = r#"# Global versions for all recipes
polkadot-sdk: "1.0.0"
"#;
    fs::write(temp_dir.path().join("versions.yml"), versions_content).unwrap();

    temp_dir
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
    assert!(recipes_dir.join("test-recipe/versions.yml").exists());
    assert!(recipes_dir.join("test-recipe/justfile").exists());
    assert!(recipes_dir.join("test-recipe/.gitignore").exists());
    assert!(recipes_dir.join("test-recipe/tests").exists());
    assert!(recipes_dir.join("test-recipe/scripts").exists());
    assert!(recipes_dir.join("test-recipe/src").exists());
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
    assert!(config_content.contains("type: sdk"));
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

    let test_file = recipes_dir.join("test-e2e/tests/test-e2e-e2e.test.ts");
    assert!(test_file.exists());

    let test_content = fs::read_to_string(test_file).unwrap();
    assert!(test_content.contains("test-e2e"));
    assert!(test_content.contains("describe"));
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

    let gitignore_content = fs::read_to_string(recipes_dir.join("ignore-test/.gitignore")).unwrap();

    assert!(gitignore_content.contains("node_modules/"));
    assert!(gitignore_content.contains("dist/"));
    assert!(gitignore_content.contains("*.log"));
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

    let versions_file = recipes_dir.join("version-test/versions.yml");
    assert!(versions_file.exists());

    let versions_content = fs::read_to_string(versions_file).unwrap();
    assert!(versions_content.contains("# Tutorial-specific version overrides"));
    assert!(versions_content.contains("versions:"));
    assert!(versions_content.contains("metadata:"));
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
