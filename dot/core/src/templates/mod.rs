/// Template generation for recipe scaffolding
/// README template for recipe documentation
pub mod readme;
/// Recipe configuration YAML template
pub mod recipe_yml;
/// Test file template
pub mod test;

pub use readme::ReadmeTemplate;
pub use recipe_yml::RecipeYmlTemplate;
pub use test::TestTemplate;

/// Template trait for all template generators
pub trait Template {
    /// Generate the template content
    fn generate(&self) -> String;
}

/// Template metadata
#[derive(Debug, Clone)]
pub struct TemplateInfo {
    /// Template identifier
    pub id: String,
    /// Human-readable name
    pub name: String,
    /// Template description
    pub description: String,
}

/// Get information about available templates
pub fn list_available_templates() -> Vec<TemplateInfo> {
    vec![
        TemplateInfo {
            id: "sdk".to_string(),
            name: "Polkadot SDK Recipe".to_string(),
            description: "Template for Polkadot SDK recipes with test setup".to_string(),
        },
        TemplateInfo {
            id: "contracts".to_string(),
            name: "Smart Contracts Recipe".to_string(),
            description: "Template for smart contracts recipes".to_string(),
        },
    ]
}
