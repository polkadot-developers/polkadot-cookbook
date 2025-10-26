# Version Management

The Polkadot Cookbook SDK provides a comprehensive version management system that allows recipes to specify dependency versions while inheriting defaults from a global configuration.

## Overview

Version management is implemented in the `version` module of `polkadot-cookbook-core` and provides:

- **Global Configuration**: Default versions defined in the repository root `versions.yml`
- **Recipe Overrides**: Recipe-specific `versions.yml` files that override global settings
- **Merge Logic**: Automatic merging where recipe versions take precedence
- **Source Tracking**: Track whether a version came from global or recipe config

## Architecture

### Components

1. **Types** (`version::types`): Core data structures
   - `VersionSet`: HashMap of dependency names to version strings
   - `GlobalVersionConfig`: Global version configuration
   - `RecipeVersionConfig`: Recipe-specific version configuration
   - `ResolvedVersions`: Merged versions with source tracking
   - `VersionSource`: Enum indicating if version is from Global or Recipe

2. **Loader** (`version::loader`): YAML file loading and parsing
   - `VersionLoader::load_global()`: Load global versions.yml
   - `VersionLoader::load_recipe()`: Load recipe versions.yml

3. **Resolver** (`version::resolver`): Version merging logic
   - `VersionResolver::merge()`: Merge global and recipe versions
   - `VersionResolver::merge_optional()`: Handle optional recipe config

### High-Level API

The easiest way to use version management is through the high-level functions:

```rust
use polkadot_cookbook_core::version::{
    resolve_recipe_versions,
    load_global_versions,
};
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load global versions only
    let global = load_global_versions(Path::new(".")).await?;

    // Load and merge versions for a specific recipe
    let resolved = resolve_recipe_versions(
        Path::new("."),
        "my-recipe"
    ).await?;

    // Access versions
    if let Some(rust_version) = resolved.get("rust") {
        println!("Using Rust version: {}", rust_version);
    }

    Ok(())
}
```

## File Format

### Global versions.yml (Repository Root)

```yaml
versions:
  rust: "1.86"
  polkadot_omni_node: "0.5.0"
  chain_spec_builder: "10.0.0"
  frame_omni_bencher: "0.13.0"

metadata:
  schema_version: "1.0"
```

### Recipe versions.yml (Recipe Directory)

```yaml
# Recipe-specific version overrides
versions:
  polkadot_omni_node: "0.6.0"
  chain_spec_builder: "11.0.0"

metadata:
  schema_version: "1.0"
```

### Merge Behavior

With the above configurations, the resolved versions for the recipe would be:

- `rust`: `"1.86"` (from global)
- `polkadot_omni_node`: `"0.6.0"` (overridden by recipe)
- `chain_spec_builder`: `"11.0.0"` (overridden by recipe)
- `frame_omni_bencher`: `"0.13.0"` (from global)

## Usage Examples

### Example 1: Basic Version Resolution

```rust
use polkadot_cookbook_core::version::resolve_versions;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let repo_root = Path::new(".");
    let recipe_path = Some(Path::new("recipes/my-recipe"));

    let resolved = resolve_versions(repo_root, recipe_path).await?;

    println!("Resolved versions:");
    for (name, version) in &resolved.versions {
        println!("  {}: {}", name, version);
    }

    Ok(())
}
```

### Example 2: Source Tracking

```rust
use polkadot_cookbook_core::version::{resolve_recipe_versions, VersionSource};
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resolved = resolve_recipe_versions(
        Path::new("."),
        "my-recipe"
    ).await?;

    for (name, version) in &resolved.versions {
        let source = match resolved.get_source(name) {
            Some(VersionSource::Global) => "global",
            Some(VersionSource::Recipe) => "recipe override",
            None => "unknown",
        };
        println!("{}: {} ({})", name, version, source);
    }

    Ok(())
}
```

### Example 3: Using in CLI Tools

```rust
use polkadot_cookbook_core::version::resolve_recipe_versions;
use std::path::Path;

async fn install_dependencies(recipe_slug: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Resolve versions for the recipe
    let versions = resolve_recipe_versions(Path::new("."), recipe_slug).await?;

    // Use versions to install dependencies
    if let Some(rust_version) = versions.get("rust") {
        println!("Installing Rust {}", rust_version);
        // ... actual installation logic
    }

    if let Some(omni_node_version) = versions.get("polkadot_omni_node") {
        println!("Installing polkadot-omni-node {}", omni_node_version);
        // ... actual installation logic
    }

    Ok(())
}
```

### Example 4: Using in CI Workflows

```rust
use polkadot_cookbook_core::version::resolve_recipe_versions;
use std::path::Path;

async fn ci_setup(recipe_slug: &str) -> Result<(), Box<dyn std::error::Error>> {
    let versions = resolve_recipe_versions(Path::new("."), recipe_slug).await?;

    // Export versions as environment variables for CI
    for (name, version) in &versions.versions {
        let env_var = format!("VERSION_{}", name.to_uppercase());
        println!("export {}={}", env_var, version);
    }

    Ok(())
}
```

## CLI Commands

The Polkadot Cookbook CLI provides commands to manage and view versions:

### View Global Versions

```bash
create-recipe versions
```

Output:
```
📦 Global versions

  rust                1.86
  polkadot_omni_node  0.5.0
  chain_spec_builder  10.0.0
  frame_omni_bencher  0.13.0
```

### View Recipe-Specific Versions

```bash
create-recipe versions my-recipe
```

### Show Version Sources

Use `--show-source` to see whether versions come from global or recipe config:

```bash
create-recipe versions my-recipe --show-source
```

Output:
```
📦 Versions for recipe: my-recipe

  rust                1.86   (global)
  polkadot_omni_node  0.6.0  (recipe)
  chain_spec_builder  10.0.0 (global)
```

### CI/Automation Format

Use `--ci` flag to get output suitable for shell evaluation:

```bash
create-recipe versions my-recipe --ci
```

Output:
```
RUST=1.86
POLKADOT_OMNI_NODE=0.6.0
CHAIN_SPEC_BUILDER=10.0.0
FRAME_OMNI_BENCHER=0.13.0
```

Use in scripts:
```bash
eval $(create-recipe versions my-recipe --ci)
echo "Using Rust version: $RUST"
```

### Validate versions.yml

Use `--validate` flag to check for unknown version keys:

```bash
create-recipe versions my-recipe --validate
```

**Output (valid):**
```
✅ All version keys are valid!

Found 4 valid version keys:
  • rust
  • polkadot_omni_node
  • chain_spec_builder
  • frame_omni_bencher
```

**Output (warnings):**
```
⚠️  Validation warnings:

  • Unknown key: 'unknown_tool'

Known keys:
  • rust
  • polkadot_omni_node
  • chain_spec_builder
  • frame_omni_bencher

Note: Unknown keys will be ignored by the workflow.
```

## Recipe Scaffolding

When creating a new recipe using the CLI, a `versions.yml` template is automatically generated:

```bash
create-recipe create my-new-recipe
```

This creates `recipes/my-new-recipe/versions.yml` with commented examples:

```yaml
# Recipe-specific version overrides
# These versions will override the global versions.yml on a per-key basis.
# Uncomment and modify the versions you need to override for this recipe.

versions:
  # rust: "1.86"
  # polkadot_omni_node: "0.5.0"
  # chain_spec_builder: "10.0.0"
  # frame_omni_bencher: "0.13.0"

metadata:
  schema_version: "1.0"
```

## API Reference

### Main Functions

#### `resolve_versions(repo_root, recipe_path) -> Result<ResolvedVersions>`

Resolve versions for a recipe by merging global and recipe-specific configurations.

**Parameters:**
- `repo_root: &Path` - Path to repository root (where global versions.yml exists)
- `recipe_path: Option<&Path>` - Optional path to recipe directory

**Returns:** `Result<ResolvedVersions>` containing merged versions with source tracking

---

#### `resolve_recipe_versions(repo_root, recipe_slug) -> Result<ResolvedVersions>`

Convenience function that constructs the recipe path from a slug.

**Parameters:**
- `repo_root: &Path` - Path to repository root
- `recipe_slug: &str` - Recipe slug (e.g., "my-recipe")

**Returns:** `Result<ResolvedVersions>` containing merged versions

---

#### `load_global_versions(repo_root) -> Result<ResolvedVersions>`

Load only global versions without recipe overrides.

**Parameters:**
- `repo_root: &Path` - Path to repository root

**Returns:** `Result<ResolvedVersions>` containing global versions only

### Data Structures

#### `ResolvedVersions`

Contains merged version information with source tracking.

**Methods:**
- `get(&self, name: &str) -> Option<&String>` - Get version by dependency name
- `get_source(&self, name: &str) -> Option<&VersionSource>` - Get source of a version
- `contains(&self, name: &str) -> bool` - Check if dependency exists
- `dependencies(&self) -> Vec<&String>` - Get all dependency names

**Fields:**
- `versions: HashMap<String, String>` - The merged version set
- `sources: HashMap<String, VersionSource>` - Source of each version

#### `VersionSource`

Enum indicating the source of a version:

- `VersionSource::Global` - Version from global versions.yml
- `VersionSource::Recipe` - Version from recipe-specific versions.yml

## Testing

The version management module includes comprehensive tests:

```bash
# Run all tests
cargo test --package polkadot-cookbook-core

# Run only version module tests
cargo test --package polkadot-cookbook-core version::

# Run the example
cargo run --package polkadot-cookbook-core --example version_resolution
```

## Error Handling

All version management functions return `Result<T, CookbookError>`:

- `CookbookError::FileSystemError` - File not found or read error
- `CookbookError::ConfigError` - YAML parsing error

### CLI Error Messages

The CLI provides detailed error messages with troubleshooting hints:

**Example: Malformed YAML**
```bash
$ create-recipe versions my-recipe

❌ Failed to resolve versions!
Error: Failed to parse recipe versions YAML: found unexpected end of stream...

Possible causes:
  • Recipe directory doesn't exist
  • versions.yml has invalid YAML syntax
  • Global versions.yml is missing or invalid

Tip: Validate YAML syntax:
  yq eval recipes/my-recipe/versions.yml
```

**Example: Missing global versions.yml**
```bash
$ create-recipe versions

❌ Failed to load global versions!
Error: Global versions file not found: versions.yml

Possible causes:
  • Global versions.yml is missing
  • versions.yml has invalid YAML syntax

Tip: Validate YAML syntax:
  yq eval versions.yml
```

### SDK Error Handling

Example error handling in code:

```rust
use polkadot_cookbook_core::error::CookbookError;

match resolve_recipe_versions(repo_root, recipe_slug).await {
    Ok(versions) => {
        // Use versions
    }
    Err(CookbookError::FileSystemError { message, path }) => {
        eprintln!("File error: {} ({:?})", message, path);
    }
    Err(CookbookError::ConfigError(msg)) => {
        eprintln!("Config error: {}", msg);
    }
    Err(e) => {
        eprintln!("Unexpected error: {}", e);
    }
}
```

## Future Enhancements

Potential future improvements:

1. **Version Constraints**: Support version ranges (e.g., ">=1.86, <2.0")
2. **Validation**: Verify version strings against known formats
3. **Caching**: Cache loaded versions for performance
4. **Schema Validation**: Enforce schema_version compatibility
5. **CLI Commands**: Add CLI commands to view/validate versions
6. **Version History**: Track version changes over time

## Integration with CI

The version management system is integrated into CI workflows using the CLI:

```yaml
# Example GitHub Actions usage
- name: Setup Rust toolchain for CLI
  uses: dtolnay/rust-toolchain@stable

- name: Build polkadot-cookbook CLI
  run: cargo build --package polkadot-cookbook-cli --release

- name: Resolve tool versions using SDK
  id: resolve
  run: |
    # Use the polkadot-cookbook CLI to resolve versions
    # This uses the SDK's version management to merge global and recipe-specific versions
    eval $(./target/release/create-recipe versions ${{ matrix.slug }} --ci)
    echo "rust=$RUST" >> $GITHUB_OUTPUT
    echo "chain-spec-builder=$CHAIN_SPEC_BUILDER" >> $GITHUB_OUTPUT
    echo "omni-node=$POLKADOT_OMNI_NODE" >> $GITHUB_OUTPUT

- name: Use resolved versions
  run: |
    echo "Installing Rust ${{ steps.resolve.outputs.rust }}"
    # Install dependencies using resolved versions
```

See `.github/workflows/test-recipes.yml` for the complete implementation.

## Contributing

When adding new version-managed dependencies:

1. Add the dependency to the global `versions.yml`
2. Update the recipe template in `templates/versions_yml.rs`
3. Update this documentation
4. Add tests for the new dependency in the version module

## Related Documentation

- [Main README](../README.md)
- [CLI Documentation](../polkadot-cookbook-cli/README.md) (TODO)
- [API Documentation](https://docs.rs/polkadot-cookbook-core) (TODO)
