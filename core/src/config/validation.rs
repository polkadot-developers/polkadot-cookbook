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
/// use polkadot_cookbook_core::config::validate_slug;
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
/// use polkadot_cookbook_core::config::is_valid_slug;
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
/// use polkadot_cookbook_core::config::slug_to_title;
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
/// use polkadot_cookbook_core::config::title_to_slug;
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

/// Validates recipe title follows naming conventions
///
/// Rejects titles with:
/// - Personal pronouns (my, our, your)
/// - Meta terms (tutorial, guide, recipe, example)
/// - Vague qualifiers (simple, basic, easy, favorite)
/// - Generic test terms (test, demo) when used alone
///
/// # Example
/// ```
/// use polkadot_cookbook_core::config::validate_title;
///
/// assert!(validate_title("NFT Pallet with Minting").is_ok());
/// assert!(validate_title("My NFT Pallet").is_err());
/// assert!(validate_title("Simple Tutorial").is_err());
/// ```
pub fn validate_title(title: &str) -> Result<()> {
    let title_lower = title.to_lowercase();

    // Check for personal pronouns
    let personal_pronouns = ["my ", " my", "our ", " our", "your ", " your"];
    for pronoun in &personal_pronouns {
        if title_lower.contains(pronoun) {
            return Err(CookbookError::ValidationError(format!(
                "Recipe title should not contain personal pronouns like '{}'.\n\
                 Use a descriptive, SEO-friendly title instead.\n\
                 Example: Instead of 'My NFT Pallet', use 'NFT Pallet with Minting'",
                pronoun.trim()
            )));
        }
    }

    // Check for meta terms
    let meta_terms = ["tutorial", "guide", "recipe", "example", "walkthrough"];
    for term in &meta_terms {
        if title_lower.contains(term) {
            return Err(CookbookError::ValidationError(format!(
                "Recipe title should not contain meta terms like '{term}'.\n\
                 The content type (tutorial/guide) is specified separately.\n\
                 Example: Instead of 'NFT Pallet Tutorial', use 'NFT Pallet with Minting'"
            )));
        }
    }

    // Check for vague qualifiers
    let vague_qualifiers = ["simple", "basic", "easy", "favorite", "best"];
    for qualifier in &vague_qualifiers {
        // Check as whole word to avoid false positives like "simplify"
        let pattern_start = format!("{qualifier} ");
        let pattern_end = format!(" {qualifier}");
        if title_lower.starts_with(&pattern_start)
            || title_lower.ends_with(&pattern_end)
            || title_lower.contains(&format!(" {qualifier} "))
        {
            return Err(CookbookError::ValidationError(format!(
                "Recipe title should not contain vague qualifiers like '{qualifier}'.\n\
                 Use the difficulty field instead of describing difficulty in the title.\n\
                 Example: Instead of 'Simple Counter', use 'Counter Pallet' with difficulty=beginner"
            )));
        }
    }

    // Check for generic test terms
    if title_lower == "test" || title_lower == "demo" {
        return Err(CookbookError::ValidationError(
            "Recipe title is too generic. Use a descriptive, production-oriented name.\n\
             Example: 'Token Staking System' instead of 'Test'"
                .to_string(),
        ));
    }

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
            "This must be run from the repository root! Expected directory structure: ./recipes/, ./versions.yml, etc.".to_string()
        ));
    }

    if !Path::new("versions.yml").exists() {
        return Err(CookbookError::WorkingDirectoryError(
            "versions.yml not found. Are you in the correct repository?".to_string(),
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
    }

    #[test]
    fn test_validate_title_rejects_personal_pronouns() {
        // Reject "my"
        assert!(validate_title("My NFT Pallet").is_err());
        assert!(validate_title("My Favorite Recipe").is_err());

        // Reject "our"
        assert!(validate_title("Our Token System").is_err());

        // Reject "your"
        assert!(validate_title("Your First Pallet").is_err());

        // Valid: "my" as part of a word is OK
        assert!(validate_title("Mythology Pallet").is_ok());
    }

    #[test]
    fn test_validate_title_rejects_meta_terms() {
        // Reject "tutorial"
        assert!(validate_title("NFT Pallet Tutorial").is_err());

        // Reject "guide"
        assert!(validate_title("Beginner's Guide to Pallets").is_err());

        // Reject "recipe"
        assert!(validate_title("Recipe for Token Transfer").is_err());

        // Reject "example"
        assert!(validate_title("Example Pallet").is_err());

        // Reject "walkthrough"
        assert!(validate_title("Governance Walkthrough").is_err());
    }

    #[test]
    fn test_validate_title_rejects_vague_qualifiers() {
        // Reject "simple"
        assert!(validate_title("Simple Counter").is_err());

        // Reject "basic"
        assert!(validate_title("Basic Token Pallet").is_err());

        // Reject "easy"
        assert!(validate_title("Easy NFT Minting").is_err());

        // Reject "favorite"
        assert!(validate_title("My Favorite Recipe").is_err());

        // Reject "best"
        assert!(validate_title("Best Practices Pallet").is_err());

        // Valid: "simple" as part of a word is OK
        assert!(validate_title("Simplify Token Transfer").is_ok());
    }

    #[test]
    fn test_validate_title_rejects_generic_test_terms() {
        // Reject standalone "test"
        assert!(validate_title("test").is_err());
        assert!(validate_title("Test").is_err());
        assert!(validate_title("TEST").is_err());

        // Reject standalone "demo"
        assert!(validate_title("demo").is_err());
        assert!(validate_title("Demo").is_err());

        // Valid: "test" as part of a longer title is OK
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
    }

    #[test]
    fn test_validate_title_error_messages() {
        // Check that error messages are helpful
        let result = validate_title("My NFT Pallet");
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("personal pronouns"));
        assert!(err_msg.contains("NFT Pallet with Minting"));

        let result = validate_title("Simple Counter");
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("vague qualifiers"));
        assert!(err_msg.contains("difficulty field"));

        let result = validate_title("NFT Pallet Tutorial");
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("meta terms"));
        assert!(err_msg.contains("content type"));
    }
}
