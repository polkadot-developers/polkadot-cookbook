# Recipe Config Schema

Complete schema reference for `recipe.config.yml`.

## Overview

Every recipe requires a `recipe.config.yml` file that defines its metadata and configuration. This file is used by:
- The CLI for recipe discovery and validation
- CI/CD workflows for testing and deployment
- The cookbook website for display and navigation

## File Location

```
recipes/your-recipe/recipe.config.yml
```

## Schema

### Complete Example

```yaml
title: "Build a Custom Pallet with Events"
slug: "custom-pallet-events"
pathway: "runtime"
difficulty: "intermediate"
content_type: "tutorial"
description: "Learn to build a custom Substrate pallet with events and error handling"
repository: "https://github.com/polkadot-developers/polkadot-cookbook"
type: "polkadot-sdk"
```

### Field Definitions

#### `title` (required)

**Type:** String
**Description:** Human-readable recipe title
**Constraints:**
- 3-80 characters
- Title case recommended
- Clear and descriptive
- Unique across recipes

**Examples:**
```yaml
✅ title: "Build a Custom Pallet with Events"
✅ title: "Deploy a Solidity Contract on AssetHub"
✅ title: "Transfer Assets via XCM"

❌ title: "Pallet"  # Too vague
❌ title: "how to build a pallet"  # Not title case
❌ title: "This is a Very Long Title That Exceeds Reasonable Length Limits"  # Too long
```

---

#### `slug` (required)

**Type:** String
**Description:** URL-safe identifier for the recipe
**Constraints:**
- Lowercase only
- Hyphen-separated words
- Match directory name
- Unique across recipes
- 3-60 characters
- Pattern: `^[a-z0-9]+(-[a-z0-9]+)*$`

**Examples:**
```yaml
✅ slug: "custom-pallet-events"
✅ slug: "deploy-solidity-contract"
✅ slug: "xcm-asset-transfer"

❌ slug: "Custom-Pallet"  # Not lowercase
❌ slug: "custom_pallet"  # Use hyphens not underscores
❌ slug: "custom pallet"  # No spaces
```

**Auto-generation:**
The CLI automatically generates slugs from titles:
```
"Build a Custom Pallet" → "build-a-custom-pallet"
"XCM Asset Transfer" → "xcm-asset-transfer"
```

---

#### `pathway` (required)

**Type:** String (enum)
**Description:** Recipe category/learning path
**Allowed values:**
- `runtime` - Polkadot SDK runtime development (pallets, runtimes)
- `contracts` - Smart contract development (Solidity, ink!)
- `basic-interaction` - Basic blockchain interactions
- `xcm` - Cross-chain messaging
- `testing` - Testing strategies and patterns

**Examples:**
```yaml
✅ pathway: "runtime"
✅ pathway: "contracts"
✅ pathway: "xcm"

❌ pathway: "Runtime"  # Must be lowercase
❌ pathway: "substrate"  # Not a valid pathway
```

**Usage:**
- Used for recipe organization on website
- Determines default file structure
- Filters recipes by category

---

#### `difficulty` (required)

**Type:** String (enum)
**Description:** Technical difficulty level
**Allowed values:**
- `beginner` - New to ecosystem, minimal prerequisites
- `intermediate` - Some experience required
- `advanced` - Complex, production-ready concepts

**Examples:**
```yaml
✅ difficulty: "beginner"
✅ difficulty: "intermediate"
✅ difficulty: "advanced"

❌ difficulty: "easy"  # Use "beginner"
❌ difficulty: "expert"  # Use "advanced"
```

**Guidelines:**

**Beginner:**
- New to Polkadot/Substrate
- Minimal blockchain knowledge required
- Basic programming concepts
- Step-by-step guidance
- Examples: "Your First Pallet", "Connect to a Node"

**Intermediate:**
- Some Polkadot/Substrate experience
- Understanding of blockchain concepts
- Comfortable with Rust or TypeScript
- Examples: "Custom Storage", "Event Handling"

**Advanced:**
- Deep Polkadot/Substrate knowledge
- Production-ready implementations
- Complex architectural patterns
- Examples: "Custom Consensus", "Runtime Migrations"

---

#### `content_type` (required)

**Type:** String (enum)
**Description:** Format and structure of content
**Allowed values:**
- `tutorial` - Step-by-step learning content
- `guide` - Reference/how-to guides

**Examples:**
```yaml
✅ content_type: "tutorial"
✅ content_type: "guide"

❌ content_type: "reference"  # Use "guide"
❌ content_type: "documentation"  # Use "guide"
```

**Differences:**

**Tutorial:**
- Sequential, numbered steps
- Learning-focused
- Builds incrementally
- Explains why and how
- Example: "Build a Custom Pallet" (step 1, 2, 3...)

**Guide:**
- Task-oriented
- Reference material
- Jump to specific sections
- Focus on how-to
- Example: "Pallet API Reference"

---

#### `description` (required)

**Type:** String
**Description:** One-sentence summary of what the recipe teaches
**Constraints:**
- 20-150 characters
- Complete sentence
- Starts with capital letter
- No period at end
- Clear and specific

**Examples:**
```yaml
✅ description: "Learn to build a custom Substrate pallet with events and error handling"
✅ description: "Deploy and interact with a Solidity contract on AssetHub"
✅ description: "Transfer assets between parachains using XCM"

❌ description: "Pallet tutorial"  # Too vague
❌ description: "Learn to build a custom Substrate pallet with events and error handling."  # Remove period
❌ description: "This recipe teaches you how to build..."  # Too verbose
```

**Guidelines:**
- Start with action verb: "Learn to...", "Build...", "Deploy...", "Transfer..."
- Be specific about what's covered
- Mention key technologies
- Focus on learning outcome

---

#### `repository` (required)

**Type:** String (URL)
**Description:** GitHub repository URL
**Constraints:**
- Valid HTTPS URL
- GitHub repository
- Standard value: `https://github.com/polkadot-developers/polkadot-cookbook`

**Examples:**
```yaml
✅ repository: "https://github.com/polkadot-developers/polkadot-cookbook"

❌ repository: "github.com/polkadot-developers/polkadot-cookbook"  # Missing https://
❌ repository: "polkadot-cookbook"  # Not a URL
```

**Note:** This field is currently uniform across all recipes but may support custom repositories in the future for community contributions.

---

#### `type` (required)

**Type:** String (enum)
**Description:** Recipe implementation type
**Allowed values:**
- `polkadot-sdk` - Rust/Substrate development
- `xcm` - XCM-specific recipes (may use TypeScript + Chopsticks)
- `solidity` - Solidity smart contracts
- `typescript` - TypeScript blockchain interactions

**Examples:**
```yaml
✅ type: "polkadot-sdk"
✅ type: "xcm"
✅ type: "solidity"
✅ type: "typescript"

❌ type: "rust"  # Use "polkadot-sdk"
❌ type: "substrate"  # Use "polkadot-sdk"
❌ type: "contract"  # Use "solidity" or specify language
```

**Usage:**
- Determines test workflow in CI
- Influences file structure
- Affects dependency management

**Type Details:**

**polkadot-sdk:**
- Rust-based recipes
- Uses Polkadot SDK
- Tested with `cargo test`
- Examples: Pallets, runtime modules

**xcm:**
- Cross-chain messaging
- Often uses TypeScript + Chopsticks
- Tested with blockchain simulation
- Examples: Asset transfers, remote execution

**solidity:**
- Solidity smart contracts
- Uses Hardhat
- Tested with `npm test`
- Examples: ERC20, DeFi contracts

**typescript:**
- TypeScript interactions
- Uses Polkadot.js API
- Tested with Vitest
- Examples: Account management, queries

---

## Validation

### Using the CLI

```bash
# Validate recipe config
dot recipe validate your-recipe-slug
```

### Validation Rules

The CLI validates:

1. **File exists:** `recipe.config.yml` present
2. **Valid YAML:** Proper YAML syntax
3. **Required fields:** All required fields present
4. **Field types:** Correct data types
5. **Enums:** Values match allowed options
6. **Constraints:** Length and format constraints
7. **Slug match:** Slug matches directory name

### Common Validation Errors

**Missing required field:**
```yaml
# ❌ Error: missing 'description' field
title: "My Recipe"
slug: "my-recipe"
pathway: "runtime"
difficulty: "beginner"
content_type: "tutorial"
repository: "https://github.com/polkadot-developers/polkadot-cookbook"
type: "polkadot-sdk"
```

**Invalid enum value:**
```yaml
# ❌ Error: invalid pathway
pathway: "substrate"  # Should be "runtime"
```

**Invalid slug format:**
```yaml
# ❌ Error: slug contains uppercase
slug: "My-Recipe"  # Should be "my-recipe"
```

**Slug doesn't match directory:**
```yaml
# Directory: recipes/my-custom-recipe/
# ❌ Error: slug doesn't match directory name
slug: "different-name"  # Should be "my-custom-recipe"
```

---

## Complete Schema Reference

### JSON Schema

For programmatic validation:

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "type": "object",
  "required": [
    "title",
    "slug",
    "pathway",
    "difficulty",
    "content_type",
    "description",
    "repository",
    "type"
  ],
  "properties": {
    "title": {
      "type": "string",
      "minLength": 3,
      "maxLength": 80
    },
    "slug": {
      "type": "string",
      "pattern": "^[a-z0-9]+(-[a-z0-9]+)*$",
      "minLength": 3,
      "maxLength": 60
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
    "repository": {
      "type": "string",
      "format": "uri",
      "pattern": "^https://github.com/"
    },
    "type": {
      "type": "string",
      "enum": ["polkadot-sdk", "xcm", "solidity", "typescript"]
    }
  },
  "additionalProperties": false
}
```

---

## Examples by Type

### Polkadot SDK Recipe

```yaml
title: "Build a Simple Storage Pallet"
slug: "simple-storage-pallet"
pathway: "runtime"
difficulty: "beginner"
content_type: "tutorial"
description: "Create a basic pallet with storage items and dispatchable functions"
repository: "https://github.com/polkadot-developers/polkadot-cookbook"
type: "polkadot-sdk"
```

### XCM Recipe

```yaml
title: "Teleport Assets Between Chains"
slug: "teleport-assets"
pathway: "xcm"
difficulty: "intermediate"
content_type: "tutorial"
description: "Transfer assets between relay chain and parachain using XCM teleport"
repository: "https://github.com/polkadot-developers/polkadot-cookbook"
type: "xcm"
```

### Solidity Recipe

```yaml
title: "Deploy an ERC20 Token"
slug: "erc20-token"
pathway: "contracts"
difficulty: "beginner"
content_type: "tutorial"
description: "Create and deploy a standard ERC20 token contract on AssetHub"
repository: "https://github.com/polkadot-developers/polkadot-cookbook"
type: "solidity"
```

### TypeScript Recipe

```yaml
title: "Query Account Balances"
slug: "query-account-balances"
pathway: "basic-interaction"
difficulty: "beginner"
content_type: "guide"
description: "Use Polkadot.js API to query and display account balance information"
repository: "https://github.com/polkadot-developers/polkadot-cookbook"
type: "typescript"
```

---

## Migration Guide

### Updating Existing Recipes

If you have an old recipe config, update it to match the schema:

**Old format (if any fields were different):**
```yaml
name: "My Recipe"  # Old field
category: "substrate"  # Old field
```

**New format:**
```yaml
title: "My Recipe"  # Renamed from 'name'
pathway: "runtime"  # Renamed from 'category', value changed
slug: "my-recipe"  # Added (auto-generate from title)
type: "polkadot-sdk"  # Added
# ... other required fields
```

**Steps:**
1. Rename fields as needed
2. Add missing required fields
3. Update enum values to match allowed values
4. Validate with `dot recipe validate`

---

## Related Documentation

- **[Recipe Guidelines](../contributors/recipe-guidelines.md)** - Recipe structure standards
- **[Manifest Schema](manifest-schema.md)** - Release manifest format
- **[SDK Guide](../developers/sdk-guide.md)** - Programmatic config access

---

[← Back to Reference Documentation](README.md)
