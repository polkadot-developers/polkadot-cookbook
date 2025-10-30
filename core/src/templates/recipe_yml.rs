/// Recipe configuration YAML template generator
use super::Template;
use crate::config::RecipeType;

/// Generates recipe.config.yml content for a recipe
pub struct RecipeYmlTemplate {
    slug: String,
    title: String,
    description: String,
    recipe_type: RecipeType,
    category: String,
    needs_node: bool,
}

impl RecipeYmlTemplate {
    /// Create a new recipe.config.yml template with all configuration fields
    pub fn new(
        slug: impl Into<String>,
        title: impl Into<String>,
        description: impl Into<String>,
        recipe_type: RecipeType,
        category: impl Into<String>,
        needs_node: bool,
    ) -> Self {
        Self {
            slug: slug.into(),
            title: title.into(),
            description: description.into(),
            recipe_type,
            category: category.into(),
            needs_node,
        }
    }
}

impl Template for RecipeYmlTemplate {
    fn generate(&self) -> String {
        let type_str = match self.recipe_type {
            RecipeType::PolkadotSdk => "polkadot-sdk",
            RecipeType::Solidity => "solidity",
            RecipeType::Xcm => "xcm",
            RecipeType::BasicInteraction => "basic-interaction",
            RecipeType::Testing => "testing",
        };

        format!(
            r#"name: {}
slug: {}
category: {}
needs_node: {}
description: {}
type: {}
"#,
            self.title, self.slug, self.category, self.needs_node, self.description, type_str
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recipe_yml_includes_slug_and_title() {
        use crate::config::RecipeType;
        let template = RecipeYmlTemplate::new(
            "my-recipe",
            "My Recipe",
            "A test recipe",
            RecipeType::PolkadotSdk,
            "test-category",
            true,
        );
        let yml = template.generate();
        assert!(yml.contains("name: My Recipe"));
        assert!(yml.contains("slug: my-recipe"));
        assert!(yml.contains("description: A test recipe"));
    }

    #[test]
    fn test_recipe_yml_has_required_fields() {
        use crate::config::RecipeType;
        let template = RecipeYmlTemplate::new(
            "test",
            "Test",
            "Test description",
            RecipeType::PolkadotSdk,
            "test-cat",
            true,
        );
        let yml = template.generate();
        assert!(yml.contains("category:"));
        assert!(yml.contains("needs_node:"));
        assert!(yml.contains("description:"));
        assert!(yml.contains("type:"));
    }
}
