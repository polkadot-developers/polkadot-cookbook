/// Template generation for recipe scaffolding
/// README template for recipe documentation
pub mod readme;
/// Test file template
pub mod test;

pub use readme::ReadmeTemplate;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_available_templates() {
        let templates = list_available_templates();

        assert_eq!(templates.len(), 2);

        // Check SDK template
        assert_eq!(templates[0].id, "sdk");
        assert_eq!(templates[0].name, "Polkadot SDK Recipe");
        assert!(templates[0].description.contains("Polkadot SDK"));

        // Check contracts template
        assert_eq!(templates[1].id, "contracts");
        assert_eq!(templates[1].name, "Smart Contracts Recipe");
        assert!(templates[1].description.contains("smart contracts"));
    }

    #[test]
    fn test_template_info_clone() {
        let info = TemplateInfo {
            id: "test".to_string(),
            name: "Test Template".to_string(),
            description: "Test description".to_string(),
        };

        let cloned = info.clone();
        assert_eq!(cloned.id, "test");
        assert_eq!(cloned.name, "Test Template");
        assert_eq!(cloned.description, "Test description");
    }

    #[test]
    fn test_template_info_debug() {
        let info = TemplateInfo {
            id: "test".to_string(),
            name: "Test Template".to_string(),
            description: "Test description".to_string(),
        };

        let debug_str = format!("{:?}", info);
        assert!(debug_str.contains("test"));
        assert!(debug_str.contains("Test Template"));
    }
}
