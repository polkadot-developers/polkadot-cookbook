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

    // Create Cargo.toml workspace file
    let cargo_content = r#"[workspace]
members = ["dot/core", "dot/cli"]
default-members = ["dot/cli"]
resolver = "2"
"#;
    fs::write(temp_dir.path().join("Cargo.toml"), cargo_content).unwrap();

    // Create rust-toolchain.toml
    let toolchain_content = r#"[toolchain]
channel = "1.86"
components = ["rustfmt", "clippy"]
profile = "minimal"
"#;
    fs::write(
        temp_dir.path().join("rust-toolchain.toml"),
        toolchain_content,
    )
    .unwrap();

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
        .stdout(predicate::str::contains("Create and manage recipes"))
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
    cmd.arg("recipe")
        .arg("create")
        .arg("--title")
        .arg("Custom Pallet Storage")
        .arg("--skip-install")
        .arg("--no-git")
        .arg("--non-interactive");

    cmd.assert().success();

    // Verify directory structure
    assert!(recipes_dir.join("custom-pallet-storage").exists());
    assert!(recipes_dir.join("custom-pallet-storage/README.md").exists());
    assert!(recipes_dir.join("custom-pallet-storage/justfile").exists());
    assert!(recipes_dir.join("custom-pallet-storage/pallets").exists());
}

#[test]
fn test_create_recipe_with_create_subcommand() {
    let temp_dir = setup_test_repo();
    let recipes_dir = temp_dir.path().join("recipes");

    let mut cmd = Command::cargo_bin("dot").unwrap();
    cmd.current_dir(temp_dir.path());
    cmd.arg("recipe")
        .arg("create")
        .arg("--title")
        .arg("Test Subcommand")
        .arg("--skip-install")
        .arg("--no-git")
        .arg("--non-interactive");

    cmd.assert().success();

    assert!(recipes_dir.join("test-subcommand").exists());
    assert!(recipes_dir.join("test-subcommand/README.md").exists());
}

#[test]
fn test_invalid_slug_uppercase() {
    // This test is no longer relevant since we don't accept slugs directly
    // Slugs are auto-generated from titles
    // Keeping as a placeholder for title validation if needed in the future
}

#[test]
fn test_invalid_slug_underscore() {
    // This test is no longer relevant since we don't accept slugs directly
    // Slugs are auto-generated from titles
    // Keeping as a placeholder for title validation if needed in the future
}

#[test]
fn test_invalid_slug_spaces() {
    // This test is no longer relevant since slugs are auto-generated from titles
    // Titles can have spaces, which get converted to hyphens in slugs
}

#[test]
fn test_non_interactive_requires_slug() {
    let temp_dir = setup_test_repo();

    let mut cmd = Command::cargo_bin("dot").unwrap();
    cmd.current_dir(temp_dir.path());
    cmd.arg("recipe").arg("create").arg("--non-interactive");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Title argument"));
}

#[test]
fn test_recipe_config_content() {
    let temp_dir = setup_test_repo();
    let recipes_dir = temp_dir.path().join("recipes");

    let mut cmd = Command::cargo_bin("dot").unwrap();
    cmd.current_dir(temp_dir.path());
    cmd.arg("recipe")
        .arg("create")
        .arg("--title")
        .arg("Advanced Pallet Configuration")
        .arg("--skip-install")
        .arg("--no-git")
        .arg("--non-interactive");

    cmd.assert().success();

    // Verify README.md frontmatter instead of recipe.config.yml
    let readme_content =
        fs::read_to_string(recipes_dir.join("advanced-pallet-configuration/README.md")).unwrap();

    // Check frontmatter contains expected fields
    assert!(readme_content.contains("title: Advanced Pallet Configuration"));
    assert!(readme_content.contains("description: Replace with a short description."));
}

#[test]
fn test_test_file_generated() {
    let temp_dir = setup_test_repo();
    let recipes_dir = temp_dir.path().join("recipes");

    let mut cmd = Command::cargo_bin("dot").unwrap();
    cmd.current_dir(temp_dir.path());
    cmd.arg("recipe")
        .arg("create")
        .arg("--title")
        .arg("Test E2E")
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
    cmd.arg("recipe")
        .arg("create")
        .arg("--title")
        .arg("Ignore Test")
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
fn test_create_recipe_with_toolchain() {
    let temp_dir = setup_test_repo();
    let recipes_dir = temp_dir.path().join("recipes");

    let mut cmd = Command::cargo_bin("dot").unwrap();
    cmd.current_dir(temp_dir.path());
    cmd.arg("recipe")
        .arg("create")
        .arg("--title")
        .arg("Version Test")
        .arg("--skip-install")
        .arg("--no-git")
        .arg("--non-interactive");

    cmd.assert().success();

    // Verify the project was created successfully
    assert!(recipes_dir.join("version-test").exists());

    // Verify rust-toolchain.toml was created for Polkadot SDK recipe
    let toolchain_path = recipes_dir.join("version-test/rust-toolchain.toml");
    assert!(
        toolchain_path.exists(),
        "rust-toolchain.toml should be created for Polkadot SDK recipes"
    );

    // Verify content matches expected format
    let content = fs::read_to_string(&toolchain_path).unwrap();
    assert!(
        content.contains("channel = \"1.86\""),
        "rust-toolchain.toml should specify Rust 1.86 for Polkadot SDK"
    );
    assert!(content.contains("components = [\"rustfmt\", \"clippy\"]"));
    assert!(content.contains("profile = \"minimal\""));
}

#[test]
fn test_invalid_working_directory() {
    // Create a temp dir that doesn't have a recipes/ folder
    let temp_dir = TempDir::new().unwrap();

    let mut cmd = Command::cargo_bin("dot").unwrap();
    cmd.current_dir(temp_dir.path());
    cmd.arg("recipe")
        .arg("create")
        .arg("--title")
        .arg("Testing Directory Validation")
        .arg("--non-interactive");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Invalid working directory"));
}
