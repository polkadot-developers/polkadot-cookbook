/// Justfile template generator
use super::Template;

/// Generates justfile content for a tutorial project
pub struct JustfileTemplate;

impl JustfileTemplate {
    /// Create a new justfile template
    pub fn new() -> Self {
        Self
    }
}

impl Default for JustfileTemplate {
    fn default() -> Self {
        Self::new()
    }
}

impl Template for JustfileTemplate {
    fn generate(&self) -> String {
        r#"default:
  @just --list

say-hello:
  echo "Hello, world!"
"#
        .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_justfile_contains_default() {
        let template = JustfileTemplate::new();
        let content = template.generate();
        assert!(content.contains("default:"));
        assert!(content.contains("@just --list"));
    }

    #[test]
    fn test_justfile_contains_say_hello() {
        let template = JustfileTemplate::new();
        let content = template.generate();
        assert!(content.contains("say-hello:"));
    }
}
