/// Parse YAML frontmatter from README files
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Frontmatter data extracted from README
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FrontmatterData {
    /// Recipe title
    pub title: String,
    /// Recipe description
    pub description: String,
}

/// Parse frontmatter from a markdown file
///
/// Expected format:
/// ```markdown
/// ---
/// title: Recipe Title
/// description: Recipe description
/// ---
///
/// # Rest of markdown...
/// ```
pub fn parse_frontmatter(content: &str) -> Result<FrontmatterData, FrontmatterError> {
    // Check if content starts with frontmatter delimiter
    if !content.trim_start().starts_with("---") {
        return Err(FrontmatterError::NoFrontmatter);
    }

    // Find the closing delimiter
    let content_after_first = content.trim_start().strip_prefix("---").unwrap();
    let end_index = content_after_first
        .find("\n---")
        .ok_or(FrontmatterError::UnclosedFrontmatter)?;

    // Extract YAML content
    let yaml_content = &content_after_first[..end_index];

    // Parse YAML
    serde_yaml::from_str(yaml_content).map_err(|e| FrontmatterError::YamlParseError(e.to_string()))
}

/// Parse frontmatter from a file
pub async fn parse_frontmatter_from_file(
    path: impl AsRef<Path>,
) -> Result<FrontmatterData, FrontmatterError> {
    let content = tokio::fs::read_to_string(path)
        .await
        .map_err(|e| FrontmatterError::IoError(e.to_string()))?;

    parse_frontmatter(&content)
}

/// Errors that can occur when parsing frontmatter
#[derive(Debug, thiserror::Error)]
pub enum FrontmatterError {
    /// No frontmatter found in file
    #[error("No frontmatter found in file")]
    NoFrontmatter,

    /// Frontmatter is not closed with --- delimiter
    #[error("Frontmatter is not closed with --- delimiter")]
    UnclosedFrontmatter,

    /// Failed to parse YAML frontmatter
    #[error("Failed to parse YAML: {0}")]
    YamlParseError(String),

    /// IO error reading file
    #[error("IO error: {0}")]
    IoError(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid_frontmatter() {
        let markdown = r#"---
title: Validator Key Management
description: Learn how to generate and manage validator keys
---

# Validator Key Management

Rest of the content...
"#;

        let result = parse_frontmatter(markdown).unwrap();
        assert_eq!(result.title, "Validator Key Management");
        assert_eq!(
            result.description,
            "Learn how to generate and manage validator keys"
        );
    }

    #[test]
    fn test_parse_no_frontmatter() {
        let markdown = r#"# Recipe Title

No frontmatter here.
"#;

        let result = parse_frontmatter(markdown);
        assert!(matches!(result, Err(FrontmatterError::NoFrontmatter)));
    }

    #[test]
    fn test_parse_unclosed_frontmatter() {
        let markdown = r#"---
title: Test
description: Test description

# Missing closing ---
"#;

        let result = parse_frontmatter(markdown);
        assert!(matches!(result, Err(FrontmatterError::UnclosedFrontmatter)));
    }

    #[test]
    fn test_parse_invalid_yaml() {
        let markdown = r#"---
title: Test
description: [unclosed bracket
---
"#;

        let result = parse_frontmatter(markdown);
        assert!(matches!(result, Err(FrontmatterError::YamlParseError(_))));
    }
}
