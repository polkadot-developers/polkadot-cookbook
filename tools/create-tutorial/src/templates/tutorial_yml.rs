/// Generates the tutorial.config.yml metadata file
pub fn generate_tutorial_yml(slug: &str, title: &str) -> String {
    format!(r#"name: {}
slug: {}
category: polkadot-sdk-cookbook
needs_node: true
description: Replace with a short description.
type: sdk # or contracts
"#, title, slug)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tutorial_yml_includes_slug_and_title() {
        let yml = generate_tutorial_yml("my-tutorial", "My Tutorial");
        assert!(yml.contains("name: My Tutorial"));
        assert!(yml.contains("slug: my-tutorial"));
    }

    #[test]
    fn test_tutorial_yml_has_required_fields() {
        let yml = generate_tutorial_yml("test", "Test");
        assert!(yml.contains("category:"));
        assert!(yml.contains("needs_node:"));
        assert!(yml.contains("description:"));
        assert!(yml.contains("type:"));
    }
}
