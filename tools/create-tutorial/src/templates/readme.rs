/// Generates the README.md template
pub fn generate_readme(slug: &str) -> String {
    format!(r#"# {}

Describe the goal, prerequisites, and step-by-step instructions for this tutorial.

## Prerequisites

- Rust `1.86+` (check with `rustc --version`)
- Node.js `20+` (check with `node --version`)
- Basic knowledge of Polkadot SDK

## Steps

1. **Setup environment**
   ```bash
   cd tutorials/{}
   npm install
   ```

2. **Build the project**
   ```bash
   # Add your build commands here
   ```

3. **Run tests**
   ```bash
   npm run test
   ```

## Testing

To run the end-to-end tests:

```bash
cd tutorials/{}
npm run test
```

## Next Steps

- Add your implementation code to `{}-code/`
- Write comprehensive tests in `tests/`
- Update this README with detailed instructions
"#, slug, slug, slug, slug)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_readme_includes_slug() {
        let readme = generate_readme("my-tutorial");
        assert!(readme.contains("# my-tutorial"));
        assert!(readme.contains("cd tutorials/my-tutorial"));
    }

    #[test]
    fn test_readme_has_prerequisites() {
        let readme = generate_readme("test");
        assert!(readme.contains("## Prerequisites"));
        assert!(readme.contains("Rust"));
        assert!(readme.contains("Node.js"));
    }

    #[test]
    fn test_readme_has_sections() {
        let readme = generate_readme("test");
        assert!(readme.contains("## Steps"));
        assert!(readme.contains("## Testing"));
        assert!(readme.contains("## Next Steps"));
    }
}
