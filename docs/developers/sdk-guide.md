# SDK Guide

Guide to using the Polkadot Cookbook SDK programmatically.

## Overview

The Polkadot Cookbook SDK (`polkadot-cookbook-sdk`) is a Rust library that provides:

- **Recipe Configuration** - Parse and validate `recipe.config.yml`
- **Version Management** - Resolve dependency versions
- **Recipe Scaffolding** - Generate new recipes programmatically
- **Validation** - Validate recipe structure and content

## Installation

### As a Dependency

Add to your `Cargo.toml`:

```toml
[dependencies]
polkadot-cookbook-sdk = "0.3"
```

### From Source

```bash
git clone https://github.com/polkadot-developers/polkadot-cookbook.git
cd polkadot-cookbook
cargo build --release -p polkadot-cookbook-sdk
```

---

## Quick Start

### Parse Recipe Configuration

```rust
use polkadot_cookbook_sdk::RecipeConfig;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load recipe config
    let config_path = Path::new("recipes/my-recipe/recipe.config.yml");
    let config = RecipeConfig::load(config_path)?;

    println!("Recipe: {}", config.title);
    println!("Pathway: {}", config.pathway);
    println!("Difficulty: {}", config.difficulty);

    Ok(())
}
```

### Resolve Versions

```rust
use polkadot_cookbook_sdk::VersionManager;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let recipe_path = Path::new("recipes/my-recipe");

    // Load and resolve versions
    let manager = VersionManager::new()?;
    let versions = manager.resolve_for_recipe(recipe_path)?;

    // Access specific versions
    if let Some(rust_version) = versions.get("rust") {
        println!("Rust version: {}", rust_version);
    }

    Ok(())
}
```

### Create Recipe

```rust
use polkadot_cookbook_sdk::RecipeScaffold;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let scaffold = RecipeScaffold::builder()
        .title("My Custom Recipe")
        .pathway("runtime")
        .difficulty("beginner")
        .content_type("tutorial")
        .build()?;

    // Generate recipe files
    scaffold.generate("recipes/my-custom-recipe")?;

    println!("Recipe created successfully!");

    Ok(())
}
```

---

## Core Modules

### Recipe Configuration

#### RecipeConfig

Represents a recipe's metadata from `recipe.config.yml`.

```rust
use polkadot_cookbook_sdk::RecipeConfig;
use std::path::Path;

// Load from file
let config = RecipeConfig::load("recipes/my-recipe/recipe.config.yml")?;

// Access fields
println!("Title: {}", config.title);
println!("Slug: {}", config.slug);
println!("Pathway: {}", config.pathway);
println!("Difficulty: {}", config.difficulty);
println!("Type: {}", config.recipe_type);
println!("Description: {}", config.description);

// Validate
config.validate()?;
```

**Fields:**

```rust
pub struct RecipeConfig {
    pub title: String,
    pub slug: String,
    pub pathway: String,
    pub difficulty: String,
    pub content_type: String,
    pub description: String,
    pub repository: String,
    pub recipe_type: String,
}
```

**Methods:**

```rust
impl RecipeConfig {
    /// Load config from file
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, Error>;

    /// Validate config fields
    pub fn validate(&self) -> Result<(), Error>;

    /// Save config to file
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), Error>;

    /// Generate slug from title
    pub fn generate_slug(title: &str) -> String;
}
```

**Example: Create and save config**

```rust
use polkadot_cookbook_sdk::RecipeConfig;

let config = RecipeConfig {
    title: "My Recipe".to_string(),
    slug: "my-recipe".to_string(),
    pathway: "runtime".to_string(),
    difficulty: "beginner".to_string(),
    content_type: "tutorial".to_string(),
    description: "A simple recipe".to_string(),
    repository: "https://github.com/polkadot-developers/polkadot-cookbook".to_string(),
    recipe_type: "polkadot-sdk".to_string(),
};

// Validate before saving
config.validate()?;

// Save to file
config.save("recipes/my-recipe/recipe.config.yml")?;
```

---

### Version Management

#### VersionManager

Manages global and recipe-specific version resolution.

```rust
use polkadot_cookbook_sdk::VersionManager;
use std::collections::HashMap;
use std::path::Path;

// Create version manager
let manager = VersionManager::new()?;

// Get global versions
let global_versions = manager.global_versions();
println!("Global Rust version: {}", global_versions.get("rust").unwrap());

// Resolve versions for specific recipe
let recipe_path = Path::new("recipes/my-recipe");
let resolved = manager.resolve_for_recipe(recipe_path)?;

// Check if version is overridden
if let Some(source) = manager.version_source("polkadot_omni_node", recipe_path)? {
    match source {
        VersionSource::Global => println!("Using global version"),
        VersionSource::Recipe => println!("Using recipe override"),
    }
}
```

**Methods:**

```rust
impl VersionManager {
    /// Create new version manager (loads global versions)
    pub fn new() -> Result<Self, Error>;

    /// Get global versions
    pub fn global_versions(&self) -> &HashMap<String, String>;

    /// Resolve versions for a recipe (merges global + recipe-specific)
    pub fn resolve_for_recipe<P: AsRef<Path>>(&self, recipe_path: P) -> Result<HashMap<String, String>, Error>;

    /// Get source of a version (global or recipe-specific)
    pub fn version_source<P: AsRef<Path>>(&self, key: &str, recipe_path: P) -> Result<VersionSource, Error>;

    /// Validate version keys
    pub fn validate_keys(&self, versions: &HashMap<String, String>) -> Result<(), Error>;
}
```

**VersionSource enum:**

```rust
pub enum VersionSource {
}
```

**Example: Compare versions**

```rust
use polkadot_cookbook_sdk::VersionManager;
use std::path::Path;

let manager = VersionManager::new()?;

let recipe1 = Path::new("recipes/recipe-one");
let recipe2 = Path::new("recipes/recipe-two");

let versions1 = manager.resolve_for_recipe(recipe1)?;
let versions2 = manager.resolve_for_recipe(recipe2)?;

// Compare Rust versions
if versions1.get("rust") != versions2.get("rust") {
    println!("Recipes use different Rust versions!");
}
```

**Example: Format for CI**

```rust
use polkadot_cookbook_sdk::VersionManager;

let manager = VersionManager::new()?;
let versions = manager.resolve_for_recipe("recipes/my-recipe")?;

// Output in CI format (KEY=VALUE)
for (key, value) in versions {
    let env_var = key.to_uppercase();
    println!("{}={}", env_var, value);
}

// Output:
// RUST=1.86
// POLKADOT_OMNI_NODE=0.5.0
// ...
```

---

### Recipe Scaffolding

#### RecipeScaffold

Generate new recipe structures programmatically.

```rust
use polkadot_cookbook_sdk::RecipeScaffold;

// Build scaffold configuration
let scaffold = RecipeScaffold::builder()
    .title("Advanced Pallet Development")
    .pathway("runtime")
    .difficulty("advanced")
    .content_type("tutorial")
    .description("Learn advanced pallet development techniques")
    .recipe_type("polkadot-sdk")
    .build()?;

// Generate recipe files
let recipe_path = "recipes/advanced-pallet";
scaffold.generate(recipe_path)?;

println!("Created recipe at: {}", recipe_path);
```

**Builder pattern:**

```rust
pub struct RecipeScaffoldBuilder {
    // Builder fields
}

impl RecipeScaffoldBuilder {
    pub fn title(mut self, title: impl Into<String>) -> Self;
    pub fn pathway(mut self, pathway: impl Into<String>) -> Self;
    pub fn difficulty(mut self, difficulty: impl Into<String>) -> Self;
    pub fn content_type(mut self, content_type: impl Into<String>) -> Self;
    pub fn description(mut self, description: impl Into<String>) -> Self;
    pub fn recipe_type(mut self, recipe_type: impl Into<String>) -> Self;
    pub fn build(self) -> Result<RecipeScaffold, Error>;
}
```

**Generated files:**

```rust
impl RecipeScaffold {
    /// Generate all recipe files
    pub fn generate<P: AsRef<Path>>(&self, output_path: P) -> Result<(), Error>;

    /// Generate only specific files
    pub fn generate_readme<P: AsRef<Path>>(&self, output_path: P) -> Result<(), Error>;
    pub fn generate_config<P: AsRef<Path>>(&self, output_path: P) -> Result<(), Error>;
    pub fn generate_package_json<P: AsRef<Path>>(&self, output_path: P) -> Result<(), Error>;
}
```

**Example: Custom template**

```rust
use polkadot_cookbook_sdk::RecipeScaffold;

let scaffold = RecipeScaffold::builder()
    .title("Custom Recipe")
    .pathway("runtime")
    .difficulty("beginner")
    .content_type("guide")
    .build()?;

// Generate with custom template
let template = include_str!("../templates/custom-readme.md");
scaffold.generate_with_template("recipes/custom-recipe", template)?;
```

---

### Validation

#### RecipeValidator

Validate recipe structure and content.

```rust
use polkadot_cookbook_sdk::RecipeValidator;
use std::path::Path;

let recipe_path = Path::new("recipes/my-recipe");
let validator = RecipeValidator::new(recipe_path);

// Run all validations
match validator.validate() {
    Ok(_) => println!("✅ Recipe validation passed!"),
    Err(errors) => {
        println!("❌ Validation failed:");
        for error in errors {
            println!("  - {}", error);
        }
    }
}
```

**Validation checks:**

```rust
impl RecipeValidator {
    /// Validate all aspects of recipe
    pub fn validate(&self) -> Result<(), Vec<ValidationError>>;

    /// Check individual aspects
    pub fn validate_config(&self) -> Result<(), ValidationError>;
    pub fn validate_readme(&self) -> Result<(), ValidationError>;
    pub fn validate_structure(&self) -> Result<(), ValidationError>;
    pub fn validate_versions(&self) -> Result<(), ValidationError>;
}
```

**ValidationError:**

```rust
pub enum ValidationError {
    MissingFile(String),
    InvalidYaml(String),
    InvalidField { field: String, reason: String },
    UnknownVersionKey(String),
}
```

**Example: Selective validation**

```rust
use polkadot_cookbook_sdk::RecipeValidator;

let validator = RecipeValidator::new("recipes/my-recipe");

// Validate only config
if let Err(e) = validator.validate_config() {
    eprintln!("Config error: {}", e);
}

// Validate only versions
if let Err(e) = validator.validate_versions() {
    eprintln!("Version error: {}", e);
}
```

---

## Error Handling

The SDK uses a custom `Error` type:

```rust
use polkadot_cookbook_sdk::Error;

pub enum Error {
    Io(std::io::Error),
    Yaml(serde_yaml::Error),
    Validation(String),
    NotFound(String),
}
```

**Usage:**

```rust
use polkadot_cookbook_sdk::{RecipeConfig, Error};

fn load_recipe(path: &str) -> Result<RecipeConfig, Error> {
    RecipeConfig::load(path)
}

fn main() {
    match load_recipe("recipes/my-recipe/recipe.config.yml") {
        Ok(config) => println!("Loaded: {}", config.title),
        Err(Error::NotFound(msg)) => eprintln!("Not found: {}", msg),
        Err(Error::Validation(msg)) => eprintln!("Invalid: {}", msg),
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

---

## Advanced Usage

### Batch Processing

Process multiple recipes:

```rust
use polkadot_cookbook_sdk::{RecipeConfig, VersionManager};
use std::fs;
use std::path::Path;

fn process_all_recipes() -> Result<(), Box<dyn std::error::Error>> {
    let recipes_dir = Path::new("recipes");
    let version_manager = VersionManager::new()?;

    for entry in fs::read_dir(recipes_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            // Load config
            let config_path = path.join("recipe.config.yml");
            if let Ok(config) = RecipeConfig::load(&config_path) {
                println!("Processing: {}", config.title);

                // Get versions
                let versions = version_manager.resolve_for_recipe(&path)?;
                println!("  Rust version: {}", versions.get("rust").unwrap());

                // Validate
                if let Err(e) = config.validate() {
                    eprintln!("  ⚠ Validation error: {}", e);
                }
            }
        }
    }

    Ok(())
}
```

### Custom Recipe Templates

Create recipes with custom templates:

```rust
use polkadot_cookbook_sdk::RecipeScaffold;
use std::fs;

fn create_custom_recipe() -> Result<(), Box<dyn std::error::Error>> {
    let scaffold = RecipeScaffold::builder()
        .title("My Custom Recipe")
        .pathway("runtime")
        .difficulty("beginner")
        .content_type("tutorial")
        .build()?;

    // Generate base structure
    scaffold.generate("recipes/custom-recipe")?;

    // Add custom files
    let custom_content = "# Custom content here";
    fs::write("recipes/custom-recipe/CUSTOM.md", custom_content)?;

    Ok(())
}
```

### Version Comparison

Compare versions across recipes:

```rust
use polkadot_cookbook_sdk::VersionManager;
use std::collections::HashMap;
use std::fs;

fn compare_versions() -> Result<(), Box<dyn std::error::Error>> {
    let manager = VersionManager::new()?;
    let mut recipes_versions: HashMap<String, HashMap<String, String>> = HashMap::new();

    // Collect versions from all recipes
    for entry in fs::read_dir("recipes")? {
        let path = entry?.path();
        if path.is_dir() {
            let recipe_name = path.file_name().unwrap().to_string_lossy().to_string();
            let versions = manager.resolve_for_recipe(&path)?;
            recipes_versions.insert(recipe_name, versions);
        }
    }

    // Find inconsistencies
    let mut rust_versions: HashMap<String, Vec<String>> = HashMap::new();
    for (recipe, versions) in recipes_versions {
        if let Some(rust_ver) = versions.get("rust") {
            rust_versions.entry(rust_ver.clone())
                .or_insert_with(Vec::new)
                .push(recipe);
        }
    }

    // Report
    println!("Rust version usage:");
    for (version, recipes) in rust_versions {
        println!("  {} used by: {:?}", version, recipes);
    }

    Ok(())
}
```

---

## API Reference

### Complete Type Definitions

```rust
// Recipe Configuration
pub struct RecipeConfig {
    pub title: String,
    pub slug: String,
    pub pathway: String,
    pub difficulty: String,
    pub content_type: String,
    pub description: String,
    pub repository: String,
    pub recipe_type: String,
}

// Version Management
pub struct VersionManager {
    global_versions: HashMap<String, String>,
    known_keys: Vec<String>,
}

pub enum VersionSource {
    Global,
    Recipe,
}

// Recipe Scaffolding
pub struct RecipeScaffold {
    config: RecipeConfig,
}

pub struct RecipeScaffoldBuilder { /* ... */ }

// Validation
pub struct RecipeValidator {
    recipe_path: PathBuf,
}

pub enum ValidationError {
    MissingFile(String),
    InvalidYaml(String),
    InvalidField { field: String, reason: String },
    UnknownVersionKey(String),
}

// Error Handling
pub enum Error {
    Io(std::io::Error),
    Yaml(serde_yaml::Error),
    Validation(String),
    NotFound(String),
}
```

---

## Testing

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recipe_config_load() {
        let config = RecipeConfig::load("tests/fixtures/recipe.config.yml").unwrap();
        assert_eq!(config.title, "Test Recipe");
        assert_eq!(config.pathway, "runtime");
    }

    #[test]
    fn test_version_resolution() {
        let manager = VersionManager::new().unwrap();
        let versions = manager.resolve_for_recipe("tests/fixtures/recipe").unwrap();
        assert!(versions.contains_key("rust"));
    }

    #[test]
    fn test_scaffold_generation() {
        let scaffold = RecipeScaffold::builder()
            .title("Test")
            .pathway("runtime")
            .difficulty("beginner")
            .content_type("tutorial")
            .build()
            .unwrap();

        let temp_dir = tempfile::tempdir().unwrap();
        scaffold.generate(temp_dir.path()).unwrap();

        assert!(temp_dir.path().join("README.md").exists());
        assert!(temp_dir.path().join("recipe.config.yml").exists());
    }
}
```

---

## Examples

### CLI Tool Integration

How the CLI uses the SDK:

```rust
use polkadot_cookbook_sdk::{RecipeScaffold, VersionManager};
use clap::Parser;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Parser)]
enum Command {
    Create {
        #[arg(long)]
        title: String,
        #[arg(long)]
        pathway: String,
    },
    Versions {
        slug: Option<String>,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Command::Create { title, pathway } => {
            let scaffold = RecipeScaffold::builder()
                .title(&title)
                .pathway(&pathway)
                .difficulty("beginner")
                .content_type("tutorial")
                .build()?;

            let slug = RecipeConfig::generate_slug(&title);
            scaffold.generate(format!("recipes/{}", slug))?;

            println!("✨ Recipe created: {}", slug);
        }

        Command::Versions { slug } => {
            let manager = VersionManager::new()?;

            let versions = if let Some(slug) = slug {
                manager.resolve_for_recipe(format!("recipes/{}", slug))?
            } else {
                manager.global_versions().clone()
            };

            for (key, value) in versions {
                println!("{}: {}", key, value);
            }
        }
    }

    Ok(())
}
```

---

## Related Documentation

- **[CLI Reference](cli-reference.md)** - Command-line interface
- **[Architecture](architecture.md)** - System design
- **[API Reference](api-reference.md)** - Complete API documentation

---

[← Back to Developers Guide](README.md)
