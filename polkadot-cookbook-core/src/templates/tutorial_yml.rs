/// Tutorial configuration YAML template generator
use super::Template;

/// Generates tutorial.config.yml content for a tutorial project
pub struct TutorialYmlTemplate {
    slug: String,
    title: String,
}

impl TutorialYmlTemplate {
    pub fn new(slug: impl Into<String>, title: impl Into<String>) -> Self {
        Self {
            slug: slug.into(),
            title: title.into(),
        }
    }
}

impl Template for TutorialYmlTemplate {
    fn generate(&self) -> String {
        format!(
            r#"name: {}
slug: {}
category: polkadot-sdk-cookbook
needs_node: true
description: Replace with a short description.
type: sdk # or contracts
"#,
            self.title, self.slug
        )
    }
}

/// Legacy function for backward compatibility
pub fn generate_tutorial_yml(slug: &str, title: &str) -> String {
    TutorialYmlTemplate::new(slug, title).generate()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tutorial_yml_includes_slug_and_title() {
        let template = TutorialYmlTemplate::new("my-tutorial", "My Tutorial");
        let yml = template.generate();
        assert!(yml.contains("name: My Tutorial"));
        assert!(yml.contains("slug: my-tutorial"));
    }

    #[test]
    fn test_tutorial_yml_has_required_fields() {
        let template = TutorialYmlTemplate::new("test", "Test");
        let yml = template.generate();
        assert!(yml.contains("category:"));
        assert!(yml.contains("needs_node:"));
        assert!(yml.contains("description:"));
        assert!(yml.contains("type:"));
    }

    #[test]
    fn test_legacy_function() {
        let yml = generate_tutorial_yml("my-tutorial", "My Tutorial");
        assert!(yml.contains("name: My Tutorial"));
        assert!(yml.contains("slug: my-tutorial"));
    }
}
