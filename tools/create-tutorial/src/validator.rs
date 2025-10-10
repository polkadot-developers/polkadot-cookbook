use anyhow::Result;
use regex::Regex;
use std::path::Path;

/// Validates that the script is being run from the repository root
pub fn validate_working_directory() -> Result<()> {
    if !Path::new("tutorials").exists() {
        anyhow::bail!("This script must be run from the repository root!\nExpected directory structure: ./tutorials/, ./utils/, etc.");
    }

    if !Path::new("versions.yml").exists() {
        anyhow::bail!("versions.yml not found. Are you in the correct repository?");
    }

    Ok(())
}

/// Validates that the slug follows the correct format:
/// - lowercase letters and numbers only
/// - words separated by single dashes
/// - no leading/trailing dashes
pub fn is_valid_slug(slug: &str) -> bool {
    let slug_regex = Regex::new(r"^[a-z0-9]+(-[a-z0-9]+)*$").unwrap();
    slug_regex.is_match(slug)
}

/// Converts a slug to a title (e.g., "my-tutorial" -> "My Tutorial")
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_slug() {
        assert!(is_valid_slug("my-tutorial"));
        assert!(is_valid_slug("add-nft-pallet"));
        assert!(is_valid_slug("zero-to-hero"));
        assert!(is_valid_slug("a"));
        assert!(is_valid_slug("test123"));
    }

    #[test]
    fn test_invalid_slug() {
        assert!(!is_valid_slug("My-Tutorial")); // uppercase
        assert!(!is_valid_slug("my_tutorial")); // underscore
        assert!(!is_valid_slug("my--tutorial")); // double dash
        assert!(!is_valid_slug("-my-tutorial")); // leading dash
        assert!(!is_valid_slug("my-tutorial-")); // trailing dash
        assert!(!is_valid_slug("my tutorial")); // space
        assert!(!is_valid_slug("")); // empty
    }

    #[test]
    fn test_slug_to_title() {
        assert_eq!(slug_to_title("my-tutorial"), "My Tutorial");
        assert_eq!(slug_to_title("zero-to-hero"), "Zero To Hero");
        assert_eq!(slug_to_title("add-nft-pallet"), "Add Nft Pallet");
        assert_eq!(slug_to_title("test"), "Test");
    }
}
