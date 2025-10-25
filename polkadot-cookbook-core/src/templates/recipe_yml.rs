/// Recipe configuration YAML template generator
use super::Template;

/// Generates recipe.config.yml content for a recipe
pub struct RecipeYmlTemplate {
    slug: String,
    title: String,
}

impl RecipeYmlTemplate {
    pub fn new(slug: impl Into<String>, title: impl Into<String>) -> Self {
        Self {
            slug: slug.into(),
            title: title.into(),
        }
    }
}

impl Template for RecipeYmlTemplate {
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
pub fn generate_recipe_yml(slug: &str, title: &str) -> String {
    RecipeYmlTemplate::new(slug, title).generate()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recipe_yml_includes_slug_and_title() {
        let template = RecipeYmlTemplate::new("my-recipe", "My Recipe");
        let yml = template.generate();
        assert!(yml.contains("name: My Recipe"));
        assert!(yml.contains("slug: my-recipe"));
    }

    #[test]
    fn test_recipe_yml_has_required_fields() {
        let template = RecipeYmlTemplate::new("test", "Test");
        let yml = template.generate();
        assert!(yml.contains("category:"));
        assert!(yml.contains("needs_node:"));
        assert!(yml.contains("description:"));
        assert!(yml.contains("type:"));
    }

    #[test]
    fn test_legacy_function() {
        let yml = generate_recipe_yml("my-recipe", "My Recipe");
        assert!(yml.contains("name: My Recipe"));
        assert!(yml.contains("slug: my-recipe"));
    }
}
