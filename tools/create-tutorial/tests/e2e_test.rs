use anyhow::Result;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use tempfile::TempDir;

/// Helper to run the create-tutorial binary in a test directory
fn run_create_tutorial(test_dir: &PathBuf, slug: &str) -> Result<std::process::Output> {
    let binary_path = env!("CARGO_BIN_EXE_create-tutorial");

    let output = Command::new(binary_path)
        .arg(slug)
        .current_dir(test_dir)
        .output()?;

    Ok(output)
}

/// Sets up a minimal test environment with required directories and files
fn setup_test_environment() -> Result<TempDir> {
    let temp_dir = TempDir::new()?;
    let temp_path = temp_dir.path();

    // Create required directories
    fs::create_dir(temp_path.join("tutorials"))?;

    // Create versions.yml
    fs::write(temp_path.join("versions.yml"), "# test versions file")?;

    // Initialize git repo
    Command::new("git")
        .args(&["init"])
        .current_dir(temp_path)
        .output()?;

    Command::new("git")
        .args(&["config", "user.name", "Test User"])
        .current_dir(temp_path)
        .output()?;

    Command::new("git")
        .args(&["config", "user.email", "test@example.com"])
        .current_dir(temp_path)
        .output()?;

    Ok(temp_dir)
}

#[test]
fn test_creates_tutorial_directory_structure() -> Result<()> {
    let temp_dir = setup_test_environment()?;
    let temp_path = temp_dir.path();

    let slug = "test-tutorial";
    let output = run_create_tutorial(&temp_path.to_path_buf(), slug)?;

    // Check that the command succeeded (or at least ran)
    // It might fail on npm install in CI, but structure should be created
    let tutorial_dir = temp_path.join("tutorials").join(slug);

    // Verify directory structure
    assert!(tutorial_dir.exists(), "Tutorial directory should exist");
    assert!(tutorial_dir.join("tests").exists(), "tests/ should exist");
    assert!(tutorial_dir.join("scripts").exists(), "scripts/ should exist");
    assert!(tutorial_dir.join("src").exists(), "src/ should exist");

    Ok(())
}

#[test]
fn test_creates_required_files() -> Result<()> {
    let temp_dir = setup_test_environment()?;
    let temp_path = temp_dir.path();

    let slug = "test-tutorial";
    run_create_tutorial(&temp_path.to_path_buf(), slug)?;

    let tutorial_dir = temp_path.join("tutorials").join(slug);

    // Verify required files exist
    assert!(tutorial_dir.join("README.md").exists(), "README.md should exist");
    assert!(tutorial_dir.join("tutorial.config.yml").exists(), "tutorial.config.yml should exist");
    assert!(tutorial_dir.join("justfile").exists(), "justfile should exist");
    assert!(tutorial_dir.join(".gitignore").exists(), ".gitignore should exist");
    assert!(tutorial_dir.join("tests").join(format!("{}-e2e.test.ts", slug)).exists(),
        "e2e test file should exist");

    Ok(())
}

#[test]
fn test_readme_contains_correct_slug() -> Result<()> {
    let temp_dir = setup_test_environment()?;
    let temp_path = temp_dir.path();

    let slug = "my-awesome-tutorial";
    run_create_tutorial(&temp_path.to_path_buf(), slug)?;

    let tutorial_dir = temp_path.join("tutorials").join(slug);
    let readme_content = fs::read_to_string(tutorial_dir.join("README.md"))?;

    assert!(readme_content.contains("# my-awesome-tutorial"), "README should have correct title");
    assert!(readme_content.contains("cd tutorials/my-awesome-tutorial"), "README should reference correct path");

    Ok(())
}

#[test]
fn test_tutorial_yml_has_correct_metadata() -> Result<()> {
    let temp_dir = setup_test_environment()?;
    let temp_path = temp_dir.path();

    let slug = "add-nft-pallet";
    run_create_tutorial(&temp_path.to_path_buf(), slug)?;

    let tutorial_dir = temp_path.join("tutorials").join(slug);
    let yml_content = fs::read_to_string(tutorial_dir.join("tutorial.config.yml"))?;

    assert!(yml_content.contains("name: Add Nft Pallet"), "Should have correct title");
    assert!(yml_content.contains("slug: add-nft-pallet"), "Should have correct slug");
    assert!(yml_content.contains("category:"), "Should have category field");
    assert!(yml_content.contains("needs_node:"), "Should have needs_node field");

    Ok(())
}

#[test]
fn test_rejects_invalid_slug_formats() -> Result<()> {
    let temp_dir = setup_test_environment()?;
    let temp_path = temp_dir.path();

    // Test uppercase slug
    let output = run_create_tutorial(&temp_path.to_path_buf(), "My-Tutorial")?;
    assert!(!output.status.success(), "Should reject uppercase slug");

    // Test underscore slug
    let output = run_create_tutorial(&temp_path.to_path_buf(), "my_tutorial")?;
    assert!(!output.status.success(), "Should reject underscore slug");

    // Test double dash
    let output = run_create_tutorial(&temp_path.to_path_buf(), "my--tutorial")?;
    assert!(!output.status.success(), "Should reject double dash");

    Ok(())
}

#[test]
fn test_prevents_duplicate_tutorial_creation() -> Result<()> {
    let temp_dir = setup_test_environment()?;
    let temp_path = temp_dir.path();

    let slug = "test-tutorial";

    // Create tutorial first time
    run_create_tutorial(&temp_path.to_path_buf(), slug)?;

    // Try to create again
    let output = run_create_tutorial(&temp_path.to_path_buf(), slug)?;

    // Should fail because a tutorial with the same slug already exists
    assert!(!output.status.success(), "Should reject duplicate tutorial");

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("already exists") ||
            String::from_utf8_lossy(&output.stdout).contains("already exists"),
            "Error message should mention tutorial already exists");

    Ok(())
}
