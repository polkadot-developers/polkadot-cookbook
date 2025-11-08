/// Auto-detect recipe type from file presence
use crate::config::RecipeType;
use std::path::Path;

/// Detect recipe type based on files present in the recipe directory
///
/// Detection logic:
/// - `polkadot-sdk`: Has Cargo.toml at root (and pallets/ directory)
/// - `solidity`: Has package.json with hardhat dependency
/// - `xcm`: Has chopsticks configuration (chopsticks.yml or .chopsticks directory)
/// - `basic-interaction`: Has package.json without hardhat/chopsticks
/// - `testing`: Has zombienet configuration files
pub async fn detect_recipe_type(
    recipe_path: impl AsRef<Path>,
) -> Result<RecipeType, RecipeDetectionError> {
    let path = recipe_path.as_ref();

    // Check for Polkadot SDK (Rust-based with Cargo.toml)
    if path.join("Cargo.toml").exists() {
        return Ok(RecipeType::PolkadotSdk);
    }

    // Check for package.json (TypeScript-based recipes)
    let package_json_path = path.join("package.json");
    if package_json_path.exists() {
        // Read package.json to determine type
        let content = tokio::fs::read_to_string(&package_json_path)
            .await
            .map_err(|e| RecipeDetectionError::IoError(e.to_string()))?;

        // Check for Hardhat (Solidity)
        if content.contains("\"hardhat\"") || content.contains("@nomicfoundation/hardhat") {
            return Ok(RecipeType::Solidity);
        }

        // Check for Chopsticks (XCM)
        if content.contains("\"@acala-network/chopsticks\"")
            || path.join("chopsticks.yml").exists()
            || path.join(".chopsticks").exists()
        {
            return Ok(RecipeType::Xcm);
        }

        // Default TypeScript recipe is basic interaction
        return Ok(RecipeType::BasicInteraction);
    }

    // Check for zombienet (Testing infrastructure)
    if path.join("zombienet.toml").exists() || path.join("network.toml").exists() {
        return Ok(RecipeType::Testing);
    }

    Err(RecipeDetectionError::UnknownRecipeType)
}

/// Errors that can occur during recipe type detection
#[derive(Debug, thiserror::Error)]
pub enum RecipeDetectionError {
    /// Could not determine recipe type from files present
    #[error("Could not determine recipe type from files present")]
    UnknownRecipeType,

    /// IO error reading recipe files
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
        let recipe_path = temp_dir.path();

        // Create Cargo.toml
        fs::write(recipe_path.join("Cargo.toml"), "[package]\nname = \"test\"").unwrap();

        let result = detect_recipe_type(recipe_path).await.unwrap();
        assert_eq!(result, RecipeType::PolkadotSdk);
    }

    #[tokio::test]
    async fn test_detect_solidity() {
        let temp_dir = TempDir::new().unwrap();
        let recipe_path = temp_dir.path();

        // Create package.json with hardhat
        let package_json = r#"{
            "name": "test",
            "dependencies": {
                "hardhat": "^2.0.0"
            }
        }"#;
        fs::write(recipe_path.join("package.json"), package_json).unwrap();

        let result = detect_recipe_type(recipe_path).await.unwrap();
        assert_eq!(result, RecipeType::Solidity);
    }

    #[tokio::test]
    async fn test_detect_xcm() {
        let temp_dir = TempDir::new().unwrap();
        let recipe_path = temp_dir.path();

        // Create package.json with chopsticks
        let package_json = r#"{
            "name": "test",
            "dependencies": {
                "@acala-network/chopsticks": "^0.1.0"
            }
        }"#;
        fs::write(recipe_path.join("package.json"), package_json).unwrap();

        let result = detect_recipe_type(recipe_path).await.unwrap();
        assert_eq!(result, RecipeType::Xcm);
    }

    #[tokio::test]
    async fn test_detect_basic_interaction() {
        let temp_dir = TempDir::new().unwrap();
        let recipe_path = temp_dir.path();

        // Create package.json without hardhat or chopsticks
        let package_json = r#"{
            "name": "test",
            "dependencies": {
                "@polkadot/api": "^10.0.0"
            }
        }"#;
        fs::write(recipe_path.join("package.json"), package_json).unwrap();

        let result = detect_recipe_type(recipe_path).await.unwrap();
        assert_eq!(result, RecipeType::BasicInteraction);
    }

    #[tokio::test]
    async fn test_detect_testing() {
        let temp_dir = TempDir::new().unwrap();
        let recipe_path = temp_dir.path();

        // Create zombienet.toml
        fs::write(recipe_path.join("zombienet.toml"), "# test config").unwrap();

        let result = detect_recipe_type(recipe_path).await.unwrap();
        assert_eq!(result, RecipeType::Testing);
    }

    #[tokio::test]
    async fn test_detect_unknown_type() {
        let temp_dir = TempDir::new().unwrap();
        let recipe_path = temp_dir.path();

        // Empty directory
        let result = detect_recipe_type(recipe_path).await;
        assert!(matches!(
            result,
            Err(RecipeDetectionError::UnknownRecipeType)
        ));
    }
}
