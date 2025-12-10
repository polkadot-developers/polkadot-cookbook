//! CLI integration tests

use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

/// Helper to set up a temp directory for testing
fn setup_test_repo() -> TempDir {
    TempDir::new().unwrap()
}

#[test]
fn test_help_command() {
    let mut cmd = Command::cargo_bin("dot").unwrap();
    cmd.arg("--help");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains(
            "command-line tool for Polkadot development",
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
fn test_create_project_non_interactive() {
    let temp_dir = setup_test_repo();

    let mut cmd = Command::cargo_bin("dot").unwrap();
    cmd.current_dir(temp_dir.path());
    cmd.arg("create")
        .arg("--title")
        .arg("Custom Pallet Storage")
        .arg("--skip-install")
        .arg("--no-git")
        .arg("--non-interactive");

    cmd.assert().success();

    // Verify directory structure
    let project_path = temp_dir.path().join("custom-pallet-storage");
    assert!(project_path.exists());
    assert!(project_path.join("README.md").exists());
    assert!(project_path.join("pallets").exists());
}

#[test]
fn test_create_project_with_create_subcommand() {
    let temp_dir = setup_test_repo();

    let mut cmd = Command::cargo_bin("dot").unwrap();
    cmd.current_dir(temp_dir.path());
    cmd.arg("create")
        .arg("--title")
        .arg("Test Subcommand")
        .arg("--skip-install")
        .arg("--no-git")
        .arg("--non-interactive");

    cmd.assert().success();

    let project_path = temp_dir.path().join("test-subcommand");
    assert!(project_path.exists());
    assert!(project_path.join("README.md").exists());
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
    cmd.arg("create").arg("--non-interactive");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Title argument"));
}

#[test]
fn test_project_config_content() {
    let temp_dir = setup_test_repo();

    let mut cmd = Command::cargo_bin("dot").unwrap();
    cmd.current_dir(temp_dir.path());
    cmd.arg("create")
        .arg("--title")
        .arg("Advanced Pallet Configuration")
        .arg("--skip-install")
        .arg("--no-git")
        .arg("--non-interactive");

    cmd.assert().success();

    // Verify README.md frontmatter instead of project metadata
    let readme_content = fs::read_to_string(
        temp_dir
            .path()
            .join("advanced-pallet-configuration/README.md"),
    )
    .unwrap();

    // Check frontmatter contains expected fields
    assert!(readme_content.contains("title: Advanced Pallet Configuration"));
    assert!(readme_content.contains("description: Replace with a short description."));
}

#[test]
fn test_test_file_generated() {
    let temp_dir = setup_test_repo();

    let mut cmd = Command::cargo_bin("dot").unwrap();
    cmd.current_dir(temp_dir.path());
    cmd.arg("create")
        .arg("--title")
        .arg("Test E2E")
        .arg("--skip-install")
        .arg("--no-git")
        .arg("--non-interactive");

    cmd.assert().success();

    // Polkadot SDK projects have Rust unit tests in the pallet code, not separate TypeScript tests
    // Just verify the project was created successfully
    let project_path = temp_dir.path().join("test-e2e");
    assert!(project_path.exists());
    assert!(project_path.join("README.md").exists());
    assert!(project_path.join("Cargo.toml").exists());
}

#[test]
fn test_gitignore_content() {
    let temp_dir = setup_test_repo();

    let mut cmd = Command::cargo_bin("dot").unwrap();
    cmd.current_dir(temp_dir.path());
    cmd.arg("create")
        .arg("--title")
        .arg("Ignore Test")
        .arg("--skip-install")
        .arg("--no-git")
        .arg("--non-interactive");

    cmd.assert().success();

    // Polkadot SDK projects use Cargo which has its own .gitignore handling via Cargo.toml
    // Only TypeScript-based recipes (XCM, Solidity) have .gitignore files
    // Just verify the project was created successfully
    let project_path = temp_dir.path().join("ignore-test");
    assert!(project_path.exists());
    assert!(project_path.join("README.md").exists());
}

#[test]
fn test_create_recipe_with_toolchain() {
    let temp_dir = setup_test_repo();

    let mut cmd = Command::cargo_bin("dot").unwrap();
    cmd.current_dir(temp_dir.path());
    cmd.arg("create")
        .arg("--title")
        .arg("Version Test")
        .arg("--skip-install")
        .arg("--no-git")
        .arg("--non-interactive");

    cmd.assert().success();

    // Verify the project was created successfully
    let project_path = temp_dir.path().join("version-test");
    assert!(project_path.exists());

    // Verify rust-toolchain.toml was created for Polkadot SDK project
    let toolchain_path = project_path.join("rust-toolchain.toml");
    assert!(
        toolchain_path.exists(),
        "rust-toolchain.toml should be created for Polkadot SDK projects"
    );

    // Verify content matches expected format
    let content = fs::read_to_string(&toolchain_path).unwrap();
    assert!(
        content.contains("channel = \"1.91\""),
        "rust-toolchain.toml should specify Rust 1.91 for Polkadot SDK"
    );
    assert!(content.contains("components = [\"rustfmt\", \"clippy\"]"));
    assert!(content.contains("profile = \"minimal\""));
}

#[test]
fn test_create_in_any_directory() {
    // CLI can now create projects in any directory, not just cookbook repos
    let temp_dir = TempDir::new().unwrap();

    let mut cmd = Command::cargo_bin("dot").unwrap();
    cmd.current_dir(temp_dir.path());
    cmd.arg("create")
        .arg("--title")
        .arg("Testing Directory Validation")
        .arg("--skip-install")
        .arg("--no-git")
        .arg("--non-interactive");

    cmd.assert().success();

    // Verify project was created directly in the directory
    assert!(temp_dir
        .path()
        .join("testing-directory-validation")
        .exists());
}
