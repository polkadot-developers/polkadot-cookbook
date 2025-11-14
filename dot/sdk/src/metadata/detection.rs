/// Auto-detect project type from file presence
use crate::config::ProjectType;
use std::path::Path;

/// Detect project type based on files present in the project directory
///
/// Detection logic:
/// - `polkadot-sdk`: Has Cargo.toml at root (and pallets/ directory)
/// - `solidity`: Has package.json with hardhat dependency
/// - `xcm`: Has chopsticks configuration (chopsticks.yml or .chopsticks directory)
/// - `transactions`: Has package.json without hardhat/chopsticks
/// - `networks`: Has zombienet configuration files
pub async fn detect_project_type(
    project_path: impl AsRef<Path>,
) -> Result<ProjectType, ProjectDetectionError> {
    let path = project_path.as_ref();

    // Check for Polkadot SDK (Rust-based with Cargo.toml)
    if path.join("Cargo.toml").exists() {
        return Ok(ProjectType::PolkadotSdk);
    }

    // Check for package.json (TypeScript-based projects)
    let package_json_path = path.join("package.json");
    if package_json_path.exists() {
        // Read package.json to determine type
        let content = tokio::fs::read_to_string(&package_json_path)
            .await
            .map_err(|e| ProjectDetectionError::IoError(e.to_string()))?;

        // Check for Hardhat (Solidity)
        if content.contains("\"hardhat\"") || content.contains("@nomicfoundation/hardhat") {
            return Ok(ProjectType::Solidity);
        }

        // Check for Chopsticks (XCM)
        if content.contains("\"@acala-network/chopsticks\"")
            || path.join("chopsticks.yml").exists()
            || path.join(".chopsticks").exists()
        {
            return Ok(ProjectType::Xcm);
        }

        // Default TypeScript project is transactions
        return Ok(ProjectType::Transactions);
    }

    // Check for zombienet (Networks infrastructure)
    if path.join("zombienet.toml").exists() || path.join("network.toml").exists() {
        return Ok(ProjectType::Networks);
    }

    Err(ProjectDetectionError::UnknownProjectType)
}

/// Errors that can occur during project type detection
#[derive(Debug, thiserror::Error)]
pub enum ProjectDetectionError {
    /// Could not determine project type from files present
    #[error("Could not determine project type from files present")]
    UnknownProjectType,

    /// IO error reading project files
    #[error("IO error: {0}")]
    IoError(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_detect_polkadot_sdk() {
        let temp_dir = TempDir::new().unwrap();
        let project_path = temp_dir.path();

        // Create Cargo.toml
        fs::write(
            project_path.join("Cargo.toml"),
            "[package]\nname = \"test\"",
        )
        .unwrap();

        let result = detect_project_type(project_path).await.unwrap();
        assert_eq!(result, ProjectType::PolkadotSdk);
    }

    #[tokio::test]
    async fn test_detect_solidity() {
        let temp_dir = TempDir::new().unwrap();
        let project_path = temp_dir.path();

        // Create package.json with hardhat
        let package_json = r#"{
            "name": "test",
            "dependencies": {
                "hardhat": "^2.0.0"
            }
        }"#;
        fs::write(project_path.join("package.json"), package_json).unwrap();

        let result = detect_project_type(project_path).await.unwrap();
        assert_eq!(result, ProjectType::Solidity);
    }

    #[tokio::test]
    async fn test_detect_xcm() {
        let temp_dir = TempDir::new().unwrap();
        let project_path = temp_dir.path();

        // Create package.json with chopsticks
        let package_json = r#"{
            "name": "test",
            "dependencies": {
                "@acala-network/chopsticks": "^0.1.0"
            }
        }"#;
        fs::write(project_path.join("package.json"), package_json).unwrap();

        let result = detect_project_type(project_path).await.unwrap();
        assert_eq!(result, ProjectType::Xcm);
    }

    #[tokio::test]
    async fn test_detect_transactions() {
        let temp_dir = TempDir::new().unwrap();
        let project_path = temp_dir.path();

        // Create package.json without hardhat or chopsticks
        let package_json = r#"{
            "name": "test",
            "dependencies": {
                "@polkadot/api": "^10.0.0"
            }
        }"#;
        fs::write(project_path.join("package.json"), package_json).unwrap();

        let result = detect_project_type(project_path).await.unwrap();
        assert_eq!(result, ProjectType::Transactions);
    }

    #[tokio::test]
    async fn test_detect_networks() {
        let temp_dir = TempDir::new().unwrap();
        let project_path = temp_dir.path();

        // Create zombienet.toml
        fs::write(project_path.join("zombienet.toml"), "# test config").unwrap();

        let result = detect_project_type(project_path).await.unwrap();
        assert_eq!(result, ProjectType::Networks);
    }

    #[tokio::test]
    async fn test_detect_unknown_type() {
        let temp_dir = TempDir::new().unwrap();
        let project_path = temp_dir.path();

        // Empty directory
        let result = detect_project_type(project_path).await;
        assert!(matches!(
            result,
            Err(ProjectDetectionError::UnknownProjectType)
        ));
    }
}
