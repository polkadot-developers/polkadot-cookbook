/// Input validation utilities
use crate::error::{CookbookError, Result};
use regex::Regex;
use std::path::Path;

/// Validates that the slug follows the correct format:
/// - lowercase letters and numbers only
/// - words separated by single dashes
/// - no leading/trailing dashes
///
/// # Example
/// ```
/// use polkadot_cookbook_sdk::config::validate_slug;
///
/// assert!(validate_slug("my-tutorial").is_ok());
/// assert!(validate_slug("zero-to-hero").is_ok());
/// assert!(validate_slug("Invalid-Slug").is_err());
/// ```
pub fn validate_slug(slug: &str) -> Result<()> {
    let slug_regex = Regex::new(r"^[a-z0-9]+(-[a-z0-9]+)*$")?;

    if !slug_regex.is_match(slug) {
        return Err(CookbookError::ValidationError(format!(
            "Invalid slug format: '{slug}'. Slug must be lowercase, with words separated by dashes."
        )));
    }

    Ok(())
}

/// Checks if a slug is valid (returns bool)
///
/// # Example
/// ```
/// use polkadot_cookbook_sdk::config::is_valid_slug;
///
/// assert!(is_valid_slug("my-tutorial"));
/// assert!(!is_valid_slug("My-Tutorial"));
/// ```
pub fn is_valid_slug(slug: &str) -> bool {
    validate_slug(slug).is_ok()
}

/// Converts a slug to a title (e.g., "my-tutorial" -> "My Tutorial")
///
/// # Example
/// ```
/// use polkadot_cookbook_sdk::config::slug_to_title;
///
/// assert_eq!(slug_to_title("my-tutorial"), "My Tutorial");
/// assert_eq!(slug_to_title("zero-to-hero"), "Zero To Hero");
/// ```
pub fn slug_to_title(slug: &str) -> String {
    slug.split('-')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// Converts a title to a slug (e.g., "My Tutorial" -> "my-tutorial")
///
/// This function:
/// - Converts to lowercase
/// - Replaces spaces and underscores with dashes
/// - Removes special characters except dashes
/// - Collapses multiple dashes into single dashes
/// - Removes leading and trailing dashes
///
/// # Example
/// ```
/// use polkadot_cookbook_sdk::config::title_to_slug;
///
/// assert_eq!(title_to_slug("My Tutorial"), "my-tutorial");
/// assert_eq!(title_to_slug("NFT Pallet Tutorial"), "nft-pallet-tutorial");
/// assert_eq!(title_to_slug("Zero to Hero!"), "zero-to-hero");
/// ```
pub fn title_to_slug(title: &str) -> String {
    title
        .to_lowercase()
        .replace([' ', '_'], "-")
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || *c == '-')
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

/// Validates recipe title follows basic naming conventions
///
/// Only rejects:
/// - Titles that are too short (< 3 characters)
/// - Empty or whitespace-only titles
///
/// # Example
/// ```
/// use polkadot_cookbook_sdk::config::validate_title;
///
/// assert!(validate_title("NFT Pallet with Minting").is_ok());
/// assert!(validate_title("My NFT Pallet").is_ok());
/// assert!(validate_title("Simple Tutorial").is_ok());
/// assert!(validate_title("XY").is_err()); // too short
/// ```
pub fn validate_title(title: &str) -> Result<()> {
    // Check minimum length (at least 3 characters)
    if title.trim().len() < 3 {
        return Err(CookbookError::ValidationError(
            "Recipe title is too short. Use a descriptive name (minimum 3 characters).".to_string(),
        ));
    }

    Ok(())
}

/// Validates that the script is being run from the repository root
pub fn validate_working_directory() -> Result<()> {
    if !Path::new("recipes").exists() {
        return Err(CookbookError::WorkingDirectoryError(
            "This must be run from the repository root! Expected directory structure: ./recipes/, ./Cargo.toml, etc.".to_string()
        ));
    }

    if !Path::new("Cargo.toml").exists() {
        return Err(CookbookError::WorkingDirectoryError(
            "Cargo.toml not found. Are you in the correct repository?".to_string(),
        ));
    }

    Ok(())
}

/// Validates a complete project configuration
pub fn validate_project_config(config: &crate::config::ProjectConfig) -> Result<Vec<String>> {
    let mut warnings = Vec::new();

    // Validate slug format
    validate_slug(&config.slug)?;

    // Check if destination directory exists
    if !config.destination.exists() {
        warnings.push(format!(
            "Destination directory '{}' does not exist and will be created",
            config.destination.display()
        ));
    }

    // Check if project already exists
    if config.project_path().exists() {
        return Err(CookbookError::ProjectExistsError(
            config.project_path().display().to_string(),
        ));
    }

    Ok(warnings)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::ProjectConfig;

    #[test]
    fn test_validate_slug_valid() {
        assert!(validate_slug("my-tutorial").is_ok());
        assert!(validate_slug("add-nft-pallet").is_ok());
        assert!(validate_slug("zero-to-hero").is_ok());
        assert!(validate_slug("a").is_ok());
        assert!(validate_slug("test123").is_ok());
    }

    #[test]
    fn test_validate_slug_invalid() {
        assert!(validate_slug("My-Tutorial").is_err()); // uppercase
        assert!(validate_slug("my_tutorial").is_err()); // underscore
        assert!(validate_slug("my--tutorial").is_err()); // double dash
        assert!(validate_slug("-my-tutorial").is_err()); // leading dash
        assert!(validate_slug("my-tutorial-").is_err()); // trailing dash
        assert!(validate_slug("my tutorial").is_err()); // space
        assert!(validate_slug("").is_err()); // empty
    }

    #[test]
    fn test_is_valid_slug() {
        assert!(is_valid_slug("my-tutorial"));
        assert!(!is_valid_slug("My-Tutorial"));
    }

    #[test]
    fn test_slug_to_title() {
        assert_eq!(slug_to_title("my-tutorial"), "My Tutorial");
        assert_eq!(slug_to_title("zero-to-hero"), "Zero To Hero");
        assert_eq!(slug_to_title("add-nft-pallet"), "Add Nft Pallet");
        assert_eq!(slug_to_title("test"), "Test");
    }

    #[test]
    fn test_title_to_slug() {
        assert_eq!(title_to_slug("My Tutorial"), "my-tutorial");
        assert_eq!(title_to_slug("Zero to Hero"), "zero-to-hero");
        assert_eq!(title_to_slug("NFT Pallet Tutorial"), "nft-pallet-tutorial");
        assert_eq!(title_to_slug("Add NFT Pallet"), "add-nft-pallet");
        assert_eq!(title_to_slug("Test"), "test");

        // Handle special characters
        assert_eq!(title_to_slug("My Tutorial!"), "my-tutorial");
        assert_eq!(
            title_to_slug("Zero to Hero (Part 1)"),
            "zero-to-hero-part-1"
        );
        assert_eq!(
            title_to_slug("Test_With_Underscores"),
            "test-with-underscores"
        );

        // Handle multiple spaces and dashes
        assert_eq!(title_to_slug("My   Tutorial"), "my-tutorial");
        assert_eq!(title_to_slug("My--Tutorial"), "my-tutorial");

        // Handle edge cases
        assert_eq!(
            title_to_slug(" Leading and trailing "),
            "leading-and-trailing"
        );
    }

    #[test]
    fn test_title_to_slug_roundtrip() {
        let slug = "my-tutorial";
        let title = slug_to_title(slug);
        let new_slug = title_to_slug(&title);
        assert_eq!(slug, new_slug);
    }

    #[test]
    fn test_validate_project_config_invalid_slug() {
        let config = ProjectConfig::new("Invalid-Slug");
        let result = validate_project_config(&config);
        assert!(result.is_err());
    }

    // Title validation tests
    #[test]
    fn test_validate_title_valid() {
        // Valid descriptive titles
        assert!(validate_title("NFT Pallet with Minting").is_ok());
        assert!(validate_title("Governance Pallet for Token Holders").is_ok());
        assert!(validate_title("Asset Transfer using XCM").is_ok());
        assert!(validate_title("Multi-Signature Wallet").is_ok());
        assert!(validate_title("Token Staking System").is_ok());
        assert!(validate_title("Cross-Chain Asset Bridge").is_ok());
        assert!(validate_title("Identity Verification Pallet").is_ok());

        // Now also valid: personal pronouns, meta terms, vague qualifiers
        assert!(validate_title("My NFT Pallet").is_ok());
        assert!(validate_title("My Favorite Recipe").is_ok());
        assert!(validate_title("Our Token System").is_ok());
        assert!(validate_title("Your First Pallet").is_ok());
        assert!(validate_title("NFT Pallet Tutorial").is_ok());
        assert!(validate_title("Beginner's Guide to Pallets").is_ok());
        assert!(validate_title("Recipe for Token Transfer").is_ok());
        assert!(validate_title("Example Pallet").is_ok());
        assert!(validate_title("Simple Counter").is_ok());
        assert!(validate_title("Basic Token Pallet").is_ok());
        assert!(validate_title("Easy NFT Minting").is_ok());
        assert!(validate_title("Best Practices Pallet").is_ok());

        // Even single words are OK as long as they're >= 3 chars
        assert!(validate_title("Test").is_ok());
        assert!(validate_title("Demo").is_ok());
        assert!(validate_title("Testing Framework for Pallets").is_ok());
    }

    #[test]
    fn test_validate_title_minimum_length() {
        // Too short
        assert!(validate_title("A").is_err());
        assert!(validate_title("AB").is_err());
        assert!(validate_title("  ").is_err());

        // Minimum valid length (3 characters)
        assert!(validate_title("NFT").is_ok());
        assert!(validate_title("XCM").is_ok());
        assert!(validate_title("Foo").is_ok());
    }

    #[test]
    fn test_validate_title_error_messages() {
        // Check that error messages are helpful for the one case we still validate
        let result = validate_title("AB");
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("too short"));
        assert!(err_msg.contains("minimum 3 characters"));
    }
}
