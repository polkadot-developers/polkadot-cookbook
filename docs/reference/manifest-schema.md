# Manifest Schema

Complete schema reference for release `manifest.yml` files.

## Overview

The manifest file is an automatically generated inventory of all recipes included in a release. It is created by the release workflow and stored in `.github/releases/v<version>/manifest.yml`.

**Purpose:**
- Track which recipes are included in each release
- Provide metadata for each recipe
- Enable programmatic access to recipe inventory
- Support cookbook website and tooling

**Note:** Manifests are auto-generated. You should not manually edit them.

## File Location

```
.github/releases/v<version>/manifest.yml
```

**Examples:**
```
.github/releases/v0.3.0/manifest.yml
.github/releases/v1.0.0/manifest.yml
```

---

## Schema

### Structure

```yaml
version: "0.3.0"
release_date: "2024-01-15"
repository: "https://github.com/polkadot-developers/polkadot-cookbook"
recipes:
  - slug: "recipe-slug"
    title: "Recipe Title"
    pathway: "runtime"
    difficulty: "beginner"
    content_type: "tutorial"
    description: "Recipe description"
    type: "polkadot-sdk"
    path: "recipes/recipe-slug"
```

### Complete Example

```yaml
version: "0.3.0"
release_date: "2024-01-15"
repository: "https://github.com/polkadot-developers/polkadot-cookbook"

recipes:
  - slug: "basic-pallet"
    title: "Build a Basic Pallet"
    pathway: "runtime"
    difficulty: "beginner"
    content_type: "tutorial"
    description: "Learn to build your first Substrate pallet with storage and dispatchables"
    type: "polkadot-sdk"
    path: "recipes/basic-pallet"

  - slug: "xcm-asset-transfer"
    title: "Transfer Assets via XCM"
    pathway: "xcm"
    difficulty: "intermediate"
    content_type: "tutorial"
    description: "Transfer assets between relay chain and parachains using XCM"
    type: "xcm"
    path: "recipes/xcm-asset-transfer"

  - slug: "erc20-token"
    title: "Deploy an ERC20 Token"
    pathway: "contracts"
    difficulty: "beginner"
    content_type: "tutorial"
    description: "Create and deploy a standard ERC20 token contract"
    type: "solidity"
    path: "recipes/erc20-token"
```

---

## Top-Level Fields

### `version` (required)

**Type:** String (semantic version)
**Description:** Release version number
**Format:** `MAJOR.MINOR.PATCH`
**Example:** `"0.3.0"`

```yaml
version: "0.3.0"
```

**Versioning rules:**
- Follows semantic versioning
- Matches git tag (without `v` prefix)
- Automatically determined by commit analysis

---

### `release_date` (required)

**Type:** String (ISO 8601 date)
**Description:** Date of release
**Format:** `YYYY-MM-DD`
**Example:** `"2024-01-15"`

```yaml
release_date: "2024-01-15"
```

---

### `repository` (required)

**Type:** String (URL)
**Description:** GitHub repository URL
**Format:** HTTPS URL
**Example:** `"https://github.com/polkadot-developers/polkadot-cookbook"`

```yaml
repository: "https://github.com/polkadot-developers/polkadot-cookbook"
```

---

### `recipes` (required)

**Type:** Array of recipe objects
**Description:** List of all recipes in this release

Each recipe object contains:
- `slug` - Recipe identifier
- `title` - Recipe title
- `pathway` - Category
- `difficulty` - Difficulty level
- `content_type` - Tutorial or guide
- `description` - One-sentence description
- `type` - Implementation type
- `path` - Relative path to recipe directory

---

## Recipe Object Schema

### `slug` (required)

**Type:** String
**Description:** Unique recipe identifier
**Format:** Lowercase, hyphen-separated
**Example:** `"basic-pallet"`

```yaml
slug: "basic-pallet"
```

Matches recipe directory name and `recipe.config.yml` slug field.

---

### `title` (required)

**Type:** String
**Description:** Human-readable recipe title
**Example:** `"Build a Basic Pallet"`

```yaml
title: "Build a Basic Pallet"
```

Copied from `recipe.config.yml`.

---

### `pathway` (required)

**Type:** String (enum)
**Description:** Recipe category
**Allowed values:**
- `runtime` - Polkadot SDK runtime development
- `contracts` - Smart contract development
- `basic-interaction` - Basic blockchain interactions
- `xcm` - Cross-chain messaging
- `testing` - Testing strategies

```yaml
pathway: "runtime"
```

---

### `difficulty` (required)

**Type:** String (enum)
**Description:** Technical difficulty level
**Allowed values:**
- `beginner` - Introductory
- `intermediate` - Moderate complexity
- `advanced` - Complex, production-ready

```yaml
difficulty: "beginner"
```

---

### `content_type` (required)

**Type:** String (enum)
**Description:** Content format
**Allowed values:**
- `tutorial` - Step-by-step learning
- `guide` - Reference/how-to

```yaml
content_type: "tutorial"
```

---

### `description` (required)

**Type:** String
**Description:** One-sentence summary
**Example:** `"Learn to build your first Substrate pallet"`

```yaml
description: "Learn to build your first Substrate pallet with storage and dispatchables"
```

---

### `type` (required)

**Type:** String (enum)
**Description:** Implementation type
**Allowed values:**
- `polkadot-sdk` - Rust/Substrate
- `xcm` - XCM recipes
- `solidity` - Solidity contracts
- `typescript` - TypeScript interactions

```yaml
type: "polkadot-sdk"
```

---

### `path` (required)

**Type:** String
**Description:** Relative path to recipe directory from repository root
**Format:** `recipes/<slug>`
**Example:** `"recipes/basic-pallet"`

```yaml
path: "recipes/basic-pallet"
```

---

## Generation Process

### How Manifests Are Created

1. **Triggered by:** Weekly release workflow or breaking change release
2. **Process:**
   - Scans `recipes/` directory
   - Reads each `recipe.config.yml`
   - Validates recipe structure
   - Compiles into manifest
   - Saves to `.github/releases/v<version>/manifest.yml`
3. **Included recipes:** Only recipes with valid configuration

### Workflow

```yaml
# In GitHub Actions
- name: Generate manifest
  run: |
    ./scripts/generate-manifest.sh v0.3.0
```

**Script responsibilities:**
- Discover all recipe directories
- Parse recipe configurations
- Validate completeness
- Generate YAML manifest
- Create release notes

---

## Usage

### Programmatic Access

**Load and parse manifest:**

```rust
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
struct Manifest {
    version: String,
    release_date: String,
    repository: String,
    recipes: Vec<Recipe>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Recipe {
    slug: String,
    title: String,
    pathway: String,
    difficulty: String,
    content_type: String,
    description: String,
    #[serde(rename = "type")]
    recipe_type: String,
    path: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manifest_data = fs::read_to_string(".github/releases/v0.3.0/manifest.yml")?;
    let manifest: Manifest = serde_yaml::from_str(&manifest_data)?;

    println!("Release: v{}", manifest.version);
    println!("Recipes: {}", manifest.recipes.len());

    for recipe in manifest.recipes {
        println!("  - {} ({})", recipe.title, recipe.difficulty);
    }

    Ok(())
}
```

**TypeScript:**

```typescript
import * as fs from 'fs';
import * as yaml from 'js-yaml';

interface Manifest {
  version: string;
  release_date: string;
  repository: string;
  recipes: Recipe[];
}

interface Recipe {
  slug: string;
  title: string;
  pathway: string;
  difficulty: string;
  content_type: string;
  description: string;
  type: string;
  path: string;
}

const manifestData = fs.readFileSync('.github/releases/v0.3.0/manifest.yml', 'utf8');
const manifest = yaml.load(manifestData) as Manifest;

console.log(`Release: v${manifest.version}`);
console.log(`Recipes: ${manifest.recipes.length}`);

manifest.recipes.forEach(recipe => {
  console.log(`  - ${recipe.title} (${recipe.difficulty})`);
});
```

### Query Recipes

**Filter by pathway:**

```typescript
const runtimeRecipes = manifest.recipes.filter(r => r.pathway === 'runtime');
console.log(`Runtime recipes: ${runtimeRecipes.length}`);
```

**Filter by difficulty:**

```typescript
const beginnerRecipes = manifest.recipes.filter(r => r.difficulty === 'beginner');
console.log(`Beginner recipes: ${beginnerRecipes.length}`);
```

**Group by type:**

```typescript
const byType = manifest.recipes.reduce((acc, recipe) => {
  acc[recipe.type] = acc[recipe.type] || [];
  acc[recipe.type].push(recipe);
  return acc;
}, {} as Record<string, Recipe[]>);

Object.entries(byType).forEach(([type, recipes]) => {
  console.log(`${type}: ${recipes.length} recipes`);
});
```

---

## Schema Reference

### JSON Schema

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "type": "object",
  "required": ["version", "release_date", "repository", "recipes"],
  "properties": {
    "version": {
      "type": "string",
      "pattern": "^\\d+\\.\\d+\\.\\d+$"
    },
    "release_date": {
      "type": "string",
      "pattern": "^\\d{4}-\\d{2}-\\d{2}$"
    },
    "repository": {
      "type": "string",
      "format": "uri"
    },
    "recipes": {
      "type": "array",
      "items": {
        "type": "object",
        "required": [
          "slug",
          "title",
          "pathway",
          "difficulty",
          "content_type",
          "description",
          "type",
          "path"
        ],
        "properties": {
          "slug": {
            "type": "string",
            "pattern": "^[a-z0-9]+(-[a-z0-9]+)*$"
          },
          "title": {
            "type": "string",
            "minLength": 3,
            "maxLength": 80
          },
          "pathway": {
            "type": "string",
            "enum": ["runtime", "contracts", "basic-interaction", "xcm", "testing"]
          },
          "difficulty": {
            "type": "string",
            "enum": ["beginner", "intermediate", "advanced"]
          },
          "content_type": {
            "type": "string",
            "enum": ["tutorial", "guide"]
          },
          "description": {
            "type": "string",
            "minLength": 20,
            "maxLength": 150
          },
          "type": {
            "type": "string",
            "enum": ["polkadot-sdk", "xcm", "solidity", "typescript"]
          },
          "path": {
            "type": "string",
            "pattern": "^recipes/[a-z0-9-]+$"
          }
        }
      }
    }
  }
}
```

---

## Statistics and Analytics

### Recipe Count by Pathway

```typescript
const pathwayCounts = manifest.recipes.reduce((acc, recipe) => {
  acc[recipe.pathway] = (acc[recipe.pathway] || 0) + 1;
  return acc;
}, {} as Record<string, number>);

console.log('Recipes by pathway:');
Object.entries(pathwayCounts).forEach(([pathway, count]) => {
  console.log(`  ${pathway}: ${count}`);
});
```

### Difficulty Distribution

```typescript
const difficultyCounts = manifest.recipes.reduce((acc, recipe) => {
  acc[recipe.difficulty] = (acc[recipe.difficulty] || 0) + 1;
  return acc;
}, {} as Record<string, number>);

console.log('Recipes by difficulty:');
Object.entries(difficultyCounts).forEach(([difficulty, count]) => {
  console.log(`  ${difficulty}: ${count}`);
});
```

---

## Example Manifests

### Small Release

```yaml
version: "0.1.0"
release_date: "2024-01-01"
repository: "https://github.com/polkadot-developers/polkadot-cookbook"

recipes:
  - slug: "basic-pallet"
    title: "Build a Basic Pallet"
    pathway: "runtime"
    difficulty: "beginner"
    content_type: "tutorial"
    description: "Learn to build your first Substrate pallet"
    type: "polkadot-sdk"
    path: "recipes/basic-pallet"
```

### Large Release

```yaml
version: "1.0.0"
release_date: "2024-06-01"
repository: "https://github.com/polkadot-developers/polkadot-cookbook"

recipes:
  - slug: "basic-pallet"
    title: "Build a Basic Pallet"
    pathway: "runtime"
    difficulty: "beginner"
    content_type: "tutorial"
    description: "Learn to build your first Substrate pallet"
    type: "polkadot-sdk"
    path: "recipes/basic-pallet"

  - slug: "storage-operations"
    title: "Pallet Storage Operations"
    pathway: "runtime"
    difficulty: "intermediate"
    content_type: "guide"
    description: "Master storage operations in Substrate pallets"
    type: "polkadot-sdk"
    path: "recipes/storage-operations"

  - slug: "xcm-teleport"
    title: "Teleport Assets via XCM"
    pathway: "xcm"
    difficulty: "intermediate"
    content_type: "tutorial"
    description: "Transfer assets using XCM teleport"
    type: "xcm"
    path: "recipes/xcm-teleport"

  # ... many more recipes
```

---

## Validation

Manifests are automatically validated during generation:

**Checks:**
- All required fields present
- Enum values are valid
- Slugs are unique
- Paths exist
- Recipe configs are valid

**If validation fails:**
- Manifest generation fails
- Release workflow stops
- Error reported in GitHub Actions

---

## Related Documentation

- **[Recipe Config Schema](recipe-config-schema.md)** - Recipe metadata format
- **[Release Process](../maintainers/release-process.md)** - How releases work
- **[Workflows Guide](../maintainers/workflows.md)** - CI/CD workflows

---

[← Back to Reference Documentation](README.md)
