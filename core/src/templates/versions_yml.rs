use crate::templates::Template;

/// Template for generating tutorial-specific versions.yml
pub struct VersionsYmlTemplate;

impl Template for VersionsYmlTemplate {
    fn generate(&self) -> String {
        r#"# Tutorial-specific version overrides
# These versions will override the global versions.yml on a per-key basis.
# Uncomment and modify the versions you need to override for this tutorial.

versions:
  # rust: "1.86"
  # polkadot_omni_node: "0.5.0"
  # chain_spec_builder: "10.0.0"
  # frame_omni_bencher: "0.13.0"

metadata:
  schema_version: "1.0"
"#
        .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_versions_yml() {
        let template = VersionsYmlTemplate;
        let content = template.generate();

        assert!(content.contains("versions:"));
        assert!(content.contains("metadata:"));
        assert!(content.contains("schema_version"));
        assert!(content.contains("# rust:"));
        assert!(content.contains("# polkadot_omni_node:"));
    }

    #[test]
    fn test_versions_yml_is_valid_yaml() {
        let template = VersionsYmlTemplate;
        let content = template.generate();

        // Parse to ensure it's valid YAML (even with comments)
        let result = serde_yaml::from_str::<serde_yaml::Value>(&content);
        assert!(result.is_ok());
    }
}
