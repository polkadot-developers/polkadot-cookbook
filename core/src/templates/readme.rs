/// README template generator
use super::Template;
use crate::config::slug_to_title;

/// Generates README.md content for a recipe
pub struct ReadmeTemplate {
    slug: String,
    title: String,
}

impl ReadmeTemplate {
    /// Create a new README template for the given recipe slug
    pub fn new(slug: impl Into<String>) -> Self {
        let slug = slug.into();
        let title = slug_to_title(&slug);
        Self { slug, title }
    }
}

impl Template for ReadmeTemplate {
    fn generate(&self) -> String {
        format!(
            r#"---
title: {}
description: Describe what this recipe teaches in one sentence.
difficulty: Beginner
content_type: tutorial
categories: Basics
---

# {}

Describe the goal, prerequisites, and step-by-step instructions for this recipe.

## Prerequisites

- Rust `1.86+` (check with `rustc --version`)
- Node.js `20+` (check with `node --version`)
- Basic knowledge of Polkadot SDK

## Steps

1. **Setup environment**
   ```bash
   cd recipes/{}
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
cd recipes/{}
npm run test
```

## Next Steps

- Add your implementation code to `src/`
- Write comprehensive tests in `tests/`
- Update this README with detailed instructions
"#,
            self.title, self.title, self.slug, self.slug
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_readme_includes_slug_and_frontmatter() {
        let template = ReadmeTemplate::new("my-recipe");
        let readme = template.generate();
        assert!(readme.contains("# My Recipe"));
        assert!(readme.contains("cd recipes/my-recipe"));
        assert!(readme.contains("---\ntitle: My Recipe"));
        assert!(readme.contains("difficulty:"));
        assert!(readme.contains("content_type:"));
    }

    #[test]
    fn test_readme_has_prerequisites() {
        let template = ReadmeTemplate::new("test");
        let readme = template.generate();
        assert!(readme.contains("## Prerequisites"));
        assert!(readme.contains("Rust"));
        assert!(readme.contains("Node.js"));
    }

    #[test]
    fn test_readme_has_sections() {
        let template = ReadmeTemplate::new("test");
        let readme = template.generate();
        assert!(readme.contains("## Steps"));
        assert!(readme.contains("## Testing"));
        assert!(readme.contains("## Next Steps"));
    }
}
