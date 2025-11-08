# Versions Schema

Complete schema reference for `versions.yml` files.

## Overview

Version files define dependency versions used in recipes. There are two levels:

1. **Global versions** (`versions.yml` at repository root) - Default versions
2. **Recipe versions** (`recipes/<slug>/versions.yml`) - Recipe-specific overrides

The system automatically merges these, with recipe versions taking precedence.

## File Locations

### Global Versions

```
versions.yml
```

Defines default versions for all recipes.

### Recipe Versions (Optional)

```
recipes/your-recipe/versions.yml
```

Overrides global versions for specific recipe.

---

## Schema

### Structure

```yaml
versions:
  key_name: "version_string"
  another_key: "version_string"
```

### Example

**Global (`versions.yml`):**
```yaml
versions:
  rust: "1.86"
  polkadot_omni_node: "0.5.0"
  chain_spec_builder: "10.0.0"
  frame_omni_bencher: "0.13.0"
  polkadot_sdk: "stable2409"
  polkadot_js_api: "14.3.1"
  hardhat: "2.19.0"
  chopsticks: "0.12.0"
```

**Recipe override (`recipes/my-recipe/versions.yml`):**
```yaml
versions:
  polkadot_omni_node: "0.6.0"  # Override global version
  rust: "1.86"  # Keep same as global (optional)
```

---

## Recognized Version Keys

### Core Tools

#### `rust`
**Description:** Rust toolchain version
**Format:** `MAJOR.MINOR` or `MAJOR.MINOR.PATCH`
**Example:** `"1.86"` or `"1.86.0"`
**Used by:** All Polkadot SDK recipes

```yaml
versions:
  rust: "1.86"
```

---

#### `polkadot_omni_node`
**Description:** Polkadot Omni Node version
**Format:** Semantic version
**Example:** `"0.5.0"`
**Used by:** Runtime recipes for local development

```yaml
versions:
  polkadot_omni_node: "0.5.0"
```

---

#### `chain_spec_builder`
**Description:** Chain Spec Builder tool version
**Format:** Semantic version
**Example:** `"10.0.0"`
**Used by:** Recipes creating custom chain specifications

```yaml
versions:
  chain_spec_builder: "10.0.0"
```

---

#### `frame_omni_bencher`
**Description:** FRAME Omni Bencher tool version
**Format:** Semantic version
**Example:** `"0.13.0"`
**Used by:** Recipes with runtime benchmarking

```yaml
versions:
  frame_omni_bencher: "0.13.0"
```

---

### SDK and Libraries

#### `polkadot_sdk`
**Description:** Polkadot SDK version/tag
**Format:** Version tag or branch name
**Examples:**
- `"stable2409"` - Stable release tag
- `"1.15.0"` - Semantic version
- `"master"` - Branch name (not recommended for recipes)

```yaml
versions:
  polkadot_sdk: "stable2409"
```

**Version naming:**
- `stableYYMM` - Stable monthly releases (recommended)
- `x.y.z` - Specific semantic version
- Avoid using `master` or development branches in recipes

---

#### `polkadot_js_api`
**Description:** Polkadot.js API npm package version
**Format:** Semantic version
**Example:** `"14.3.1"`
**Used by:** TypeScript recipes

```yaml
versions:
  polkadot_js_api: "14.3.1"
```

---

### Testing and Development Tools

#### `hardhat`
**Description:** Hardhat Ethereum development environment version
**Format:** Semantic version
**Example:** `"2.19.0"`
**Used by:** Solidity recipes

```yaml
versions:
  hardhat: "2.19.0"
```

---

#### `chopsticks`
**Description:** Chopsticks blockchain simulator version
**Format:** Semantic version
**Example:** `"0.12.0"`
**Used by:** XCM recipes

```yaml
versions:
  chopsticks: "0.12.0"
```

---

## Version Resolution

### Merge Behavior

When a recipe has its own `versions.yml`, the system merges it with global versions:

**Global (`versions.yml`):**
```yaml
versions:
  rust: "1.86"
  polkadot_omni_node: "0.5.0"
  polkadot_sdk: "stable2409"
```

**Recipe (`recipes/my-recipe/versions.yml`):**
```yaml
versions:
  polkadot_omni_node: "0.6.0"  # Override
```

**Resolved for recipe:**
```yaml
versions:
  rust: "1.86"  # From global
  polkadot_omni_node: "0.6.0"  # From recipe (override)
  polkadot_sdk: "stable2409"  # From global
```

### Resolution Algorithm

1. Load global versions from `versions.yml`
2. If recipe has `versions.yml`:
   - Load recipe versions
   - Merge with global (recipe wins on conflicts)
3. Return merged versions

### Checking Version Source

```bash
# Show where each version comes from
dot versions my-recipe --show-source

# Output:
# 📦 Versions for recipe: my-recipe
#
#   rust                1.86   (global)
#   polkadot_omni_node  0.6.0  (recipe)
#   polkadot_sdk        stable2409 (global)
```

---

## Usage

### View Versions

```bash
# Global versions
dot versions

# Recipe-specific versions
dot versions my-recipe

# With source information
dot versions my-recipe --show-source

# CI format (environment variables)
dot versions my-recipe --ci
```

### Validate Versions

```bash
# Validate version keys
dot versions my-recipe --validate
```

**This checks:**
- All keys are recognized
- No typos in key names
- YAML syntax is correct

**Output:**
```
✅ All version keys are valid!

Found 3 valid version keys:
  • rust
  • polkadot_omni_node
  • polkadot_sdk
```

### CI Integration

```yaml
# GitHub Actions example
- name: Resolve versions
  run: |
    eval $(./target/release/dot versions ${{ matrix.slug }} --ci)
    echo "Using Rust $RUST"
    echo "Using Polkadot SDK $POLKADOT_SDK"
```

**Output format (--ci):**
```bash
RUST=1.86
POLKADOT_OMNI_NODE=0.6.0
POLKADOT_SDK=stable2409
```

---

## When to Override Versions

### ✅ Override When:

**Testing new versions:**
```yaml
# Test recipe with newer SDK version
versions:
  polkadot_sdk: "stable2410"  # Newer than global
```

**Backward compatibility:**
```yaml
# Recipe requires older version
versions:
  rust: "1.85"  # Older Rust for compatibility
```

**Specific tool requirements:**
```yaml
# Recipe needs specific tool version
versions:
  chopsticks: "0.11.0"  # Specific Chopsticks version
```

### ❌ Don't Override When:

**No real need:**
```yaml
# ❌ Unnecessary override
versions:
  rust: "1.86"  # Same as global - just omit this
```

**Following global is fine:**
```yaml
# ✅ Just don't create versions.yml
# Recipe will use global versions automatically
```

**Breaking changes without reason:**
```yaml
# ❌ Don't override just to be different
versions:
  polkadot_omni_node: "0.4.0"  # Older version for no reason
```

---

## Version String Formats

### Semantic Versioning

Most tools use semantic versioning: `MAJOR.MINOR.PATCH`

```yaml
versions:
  polkadot_omni_node: "0.5.0"
  chopsticks: "0.12.0"
  hardhat: "2.19.0"
```

**Rules:**
- Always use quotes: `"0.5.0"` not `0.5.0`
- Include all three parts: `"0.5.0"` not `"0.5"`
- No prefixes: `"0.5.0"` not `"v0.5.0"`

### Rust Versions

Rust versions can be `MAJOR.MINOR` or `MAJOR.MINOR.PATCH`:

```yaml
versions:
  rust: "1.86"  # Recommended (allows any patch version)
  # OR
  rust: "1.86.0"  # Specific patch version
```

### SDK Tags

Polkadot SDK uses special tag formats:

```yaml
versions:
  polkadot_sdk: "stable2409"  # Stable monthly release (YYMM format)
  # OR
  polkadot_sdk: "1.15.0"  # Specific version
```

**Recommended:** Use `stableYYMM` tags for recipes.

---

## Validation

### Valid Keys

Only recognized keys are allowed. The CLI validates against a known list:

```bash
dot versions my-recipe --validate
```

**Recognized keys:**
- `rust`
- `polkadot_omni_node`
- `chain_spec_builder`
- `frame_omni_bencher`
- `polkadot_sdk`
- `polkadot_js_api`
- `hardhat`
- `chopsticks`

### Common Validation Errors

**Unknown key:**
```yaml
# ❌ Error: unknown key
versions:
  rustc: "1.86"  # Should be 'rust'
```

```
❌ Validation failed!

Unknown version keys:
  • rustc

Did you mean:
  • rust
```

**Invalid YAML:**
```yaml
# ❌ Error: invalid YAML syntax
versions:
rust: "1.86"  # Missing indentation
```

```
❌ YAML parse error at line 2
```

**Wrong format:**
```yaml
# ❌ Error: version should be string
versions:
  rust: 1.86  # Missing quotes
```

```
❌ Version values must be strings
```

---

## Examples

### Minimal Recipe Override

```yaml
# Only override what's needed
versions:
  polkadot_sdk: "stable2410"
```

### Multiple Overrides

```yaml
versions:
  rust: "1.87"
  polkadot_omni_node: "0.6.0"
  polkadot_sdk: "stable2410"
```

### XCM Recipe

```yaml
versions:
  chopsticks: "0.12.0"
  polkadot_js_api: "14.3.1"
```

### Solidity Recipe

```yaml
versions:
  hardhat: "2.19.0"
```

### Complete Example

**Global `versions.yml`:**
```yaml
versions:
  # Rust toolchain
  rust: "1.86"

  # Polkadot tools
  polkadot_omni_node: "0.5.0"
  chain_spec_builder: "10.0.0"
  frame_omni_bencher: "0.13.0"

  # SDK
  polkadot_sdk: "stable2409"

  # JavaScript/TypeScript
  polkadot_js_api: "14.3.1"

  # Contract development
  hardhat: "2.19.0"

  # Testing
  chopsticks: "0.12.0"
```

**Recipe `recipes/advanced-pallet/versions.yml`:**
```yaml
versions:
  # Use newer SDK for testing
  polkadot_sdk: "stable2410"

  # Use newer Rust for new features
  rust: "1.87"

  # All other versions inherited from global
```

---

## Schema Reference

### JSON Schema

For programmatic validation:

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "type": "object",
  "required": ["versions"],
  "properties": {
    "versions": {
      "type": "object",
      "patternProperties": {
        "^[a-z_]+$": {
          "type": "string",
          "minLength": 1
        }
      },
      "additionalProperties": false
    }
  },
  "additionalProperties": false
}
```

### TypeScript Type

```typescript
interface VersionsFile {
  versions: {
    [key: string]: string;
  };
}

// Example
const versions: VersionsFile = {
  versions: {
    rust: "1.86",
    polkadot_omni_node: "0.5.0",
  },
};
```

### Rust Type

```rust
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct VersionsFile {
    pub versions: HashMap<String, String>,
}
```

---

## Best Practices

### ✅ DO:

**Use global defaults when possible:**
```yaml
# No versions.yml needed if global is fine
```

**Override only when necessary:**
```yaml
versions:
  polkadot_sdk: "stable2410"  # Testing new version
```

**Document why you override:**
```yaml
# recipes/my-recipe/versions.yml
versions:
  # Using older SDK due to API compatibility
  polkadot_sdk: "stable2408"
```

**Keep versions in sync across similar recipes:**
```yaml
# If multiple recipes need same override, consider updating global
```

### ❌ DON'T:

**Don't duplicate global values:**
```yaml
# ❌ Unnecessary
versions:
  rust: "1.86"  # Same as global - just omit
```

**Don't use development versions:**
```yaml
# ❌ Unstable
versions:
  polkadot_sdk: "master"  # Use stable tags
```

**Don't override without testing:**
```yaml
# ❌ Test first
versions:
  polkadot_sdk: "stable2410"  # Ensure recipe works with this version
```

---

## Troubleshooting

### Version Not Applied

**Problem:** Recipe still uses global version

**Solutions:**
```bash
# Check if override is correctly loaded
dot versions my-recipe --show-source

# Verify YAML syntax
dot versions my-recipe --validate

# Check file location
ls recipes/my-recipe/versions.yml
```

### Unknown Key Error

**Problem:** `Unknown version key: my_tool`

**Solution:**
```yaml
# Use only recognized keys
# Check spelling and format
versions:
  rust: "1.86"  # ✅ Correct
  # rustc: "1.86"  # ❌ Wrong key name
```

### CI Using Wrong Version

**Problem:** CI workflow uses different version than local

**Solutions:**
```bash
# Ensure versions.yml is committed
git status recipes/my-recipe/versions.yml

# Verify CI resolution
dot versions my-recipe --ci
```

---

## Related Documentation

- **[Version Management Guide](../maintainers/version-management.md)** - Complete version system documentation
- **[Recipe Config Schema](recipe-config-schema.md)** - Recipe metadata format
- **[SDK Guide](../developers/sdk-guide.md)** - Programmatic version access
- **[Version Workflows](../automation/version-workflows.md)** - CI/CD integration

---

[← Back to Reference Documentation](README.md)
