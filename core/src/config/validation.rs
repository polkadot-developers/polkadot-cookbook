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
    fn test_validate_project_config_invalid_slug() {
        let config = ProjectConfig::new("Invalid-Slug");
        let result = validate_project_config(&config);
        assert!(result.is_err());
    }
}
