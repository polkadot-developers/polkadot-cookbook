/// Generates the justfile template
pub fn generate_justfile() -> &'static str {
    r#"default:
  @just --list

say-hello:
  echo "Hello, world!"
"#
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_justfile_contains_default() {
        let content = generate_justfile();
        assert!(content.contains("default:"));
        assert!(content.contains("@just --list"));
    }

    #[test]
    fn test_justfile_contains_say_hello() {
        let content = generate_justfile();
        assert!(content.contains("say-hello:"));
    }
}
