/// Recipe configuration YAML template generator
use super::Template;

/// Generates recipe.config.yml content for a recipe
pub struct RecipeYmlTemplate {
    slug: String,
    title: String,
    description: String,
}

impl RecipeYmlTemplate {
    /// Create a new recipe.config.yml template with the given slug, title, and description
    pub fn new(slug: impl Into<String>, title: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            slug: slug.into(),
            title: title.into(),
            description: description.into(),
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
description: {}
type: sdk # or contracts
"#,
            self.title, self.slug, self.description
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recipe_yml_includes_slug_and_title() {
        let template = RecipeYmlTemplate::new("my-recipe", "My Recipe", "A test recipe");
        let yml = template.generate();
        assert!(yml.contains("name: My Recipe"));
        assert!(yml.contains("slug: my-recipe"));
        assert!(yml.contains("description: A test recipe"));
    }

    #[test]
    fn test_recipe_yml_has_required_fields() {
        let template = RecipeYmlTemplate::new("test", "Test", "Test description");
        let yml = template.generate();
        assert!(yml.contains("category:"));
        assert!(yml.contains("needs_node:"));
        assert!(yml.contains("description:"));
        assert!(yml.contains("type:"));
    }
}
