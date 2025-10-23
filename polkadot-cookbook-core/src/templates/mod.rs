/// Template generation for project scaffolding
pub mod justfile;
pub mod readme;
pub mod test;
pub mod tutorial_yml;
pub mod versions_yml;

pub use justfile::JustfileTemplate;
pub use readme::ReadmeTemplate;
pub use test::TestTemplate;
pub use tutorial_yml::TutorialYmlTemplate;
pub use versions_yml::VersionsYmlTemplate;

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
            name: "Polkadot SDK Tutorial".to_string(),
            description: "Template for Polkadot SDK tutorials with test setup".to_string(),
        },
        TemplateInfo {
            id: "contracts".to_string(),
            name: "Smart Contracts Tutorial".to_string(),
            description: "Template for smart contracts tutorials".to_string(),
        },
    ]
}
