# Your First Recipe

A step-by-step tutorial for creating your first Polkadot Cookbook recipe.

## What You'll Learn

By the end of this tutorial, you'll know how to:
- Create a new recipe using the CLI
- Understand the generated file structure
- Customize the recipe configuration
- Write recipe content
- Run tests
- Submit your recipe for review

## Prerequisites

Before starting, ensure you have:

1. **CLI Installed** - See [Installation Guide](installation.md)
2. **Development Environment** - Run `dot setup` to verify
3. **Git Configured** - Name and email set
4. **GitHub Account** - For submitting your recipe

**Verify your setup:**
```bash
dot setup
```

You should see all checks passing.

---

## Step 1: Create Your Recipe

The CLI provides an interactive mode that guides you through recipe creation.

### Run the Create Command

```bash
dot recipe create
```

You'll be prompted for information about your recipe.

### Interactive Prompts

**1. Recipe Title**
```
? Enter recipe title: My First Pallet
```

Choose a clear, descriptive title. This will be used to generate the recipe slug (e.g., "My First Pallet" → `my-first-pallet`).

**2. Select Pathway**
```
? Select pathway:
  ❯ runtime - Polkadot SDK runtime development
    contracts - Smart contract development
    basic-interaction - Basic blockchain interactions
    xcm - Cross-chain messaging
    testing - Testing strategies and patterns
```

For this tutorial, select **runtime** (Polkadot SDK development).

**3. Select Difficulty**
```
? Select difficulty:
  ❯ beginner - Introductory recipes
    intermediate - Moderate complexity
    advanced - Complex, production-ready examples
```

Select **beginner** for your first recipe.

**4. Select Content Type**
```
? Select content type:
  ❯ tutorial - Step-by-step learning content
    guide - Reference/how-to guides
```

Select **tutorial** for step-by-step content.

### What Happens Next

The CLI will:
1. Create the recipe directory: `recipes/my-first-pallet/`
2. Generate scaffolded files
3. Install npm dependencies (TypeScript recipes)
4. Create a git branch (if in a git repository)

**Output:**
```
✨ Recipe created successfully!

📁 Location: recipes/my-first-pallet
🌿 Branch: recipe/my-first-pallet

Next steps:
1. cd recipes/my-first-pallet
2. Read the README.md for instructions
3. Start coding!
```

---

## Step 2: Explore the Generated Files

Navigate to your new recipe directory:

```bash
cd recipes/my-first-pallet
```

### File Structure

```
recipes/my-first-pallet/
├── README.md              # Recipe content and documentation
├── recipe.config.yml      # Metadata and configuration
├── versions.yml           # Dependency version overrides (optional)
├── package.json           # npm dependencies (TypeScript recipes)
├── tsconfig.json          # TypeScript configuration
├── vitest.config.ts       # Test configuration
├── src/                   # Implementation code
│   └── index.ts           # Main source file
└── tests/                 # Test files
    └── example.test.ts    # Example test
```

### Key Files Explained

#### `README.md`
This is the **main recipe content**. It contains:
- Title and description
- Prerequisites
- Learning objectives
- Step-by-step instructions
- Code examples
- Expected output
- Troubleshooting

The README is pre-scaffolded with a template structure.

#### `recipe.config.yml`
Recipe metadata used by the cookbook system:

```yaml
title: "My First Pallet"
slug: "my-first-pallet"
pathway: "runtime"
difficulty: "beginner"
content_type: "tutorial"
description: "Learn to build your first custom pallet"
repository: "https://github.com/polkadot-developers/polkadot-cookbook"
type: "polkadot-sdk"
```

**Important fields:**
- `type`: Recipe type (`polkadot-sdk`, `xcm`, `solidity`)
- `pathway`: Category for organization
- `difficulty`: Helps users find appropriate content

#### `versions.yml` (Optional)
Override global dependency versions for this recipe:

```yaml
versions:
  rust: "1.86"
  polkadot_omni_node: "0.6.0"  # Override global version
```

Only include if your recipe needs different versions than the global defaults.

For more on version management, see [Version Management Guide](../maintainers/version-management.md).

---

## Step 3: Customize Your Recipe

### Update the README

Open `README.md` and fill in the template sections:

```markdown
# My First Pallet

Learn how to build a custom pallet for the Polkadot SDK.

## Prerequisites

- Rust 1.86 or later
- Polkadot SDK basics
- Understanding of Substrate pallets

## Learning Objectives

By completing this recipe, you will learn:
- How to create a custom pallet
- How to define storage items
- How to implement dispatchable functions
- How to write pallet tests

## Steps

### 1. Create the Pallet Module

First, create the basic pallet structure...

[Continue with your content]
```

**Tips for good recipe content:**
- Start with clear learning objectives
- Break down complex tasks into small steps
- Include code examples with explanations
- Show expected output
- Add troubleshooting for common issues

### Update recipe.config.yml

Customize the description:

```yaml
description: "Build your first custom Substrate pallet with storage and dispatchable functions"
```

### Add Your Implementation

For a Polkadot SDK recipe, you might add:

**Pallet code** (in your project structure):
```rust
// Your pallet implementation
#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    pub type MyStorage<T> = StorageValue<_, u32>;

    // More pallet code...
}
```

**Test code** (in `tests/`):
```typescript
import { describe, it, expect } from 'vitest';

describe('My First Pallet', () => {
  it('should store and retrieve values', async () => {
    // Your test implementation
    expect(true).toBe(true);
  });
});
```

---

## Step 4: Run Tests

The recipe comes with a test setup. Run the tests to verify everything works:

### TypeScript/Vitest Tests

```bash
npm test
```

**Expected output:**
```
✓ tests/example.test.ts (1)
  ✓ My First Pallet (1)
    ✓ should store and retrieve values

Test Files  1 passed (1)
Tests  1 passed (1)
```

### Rust Tests (if applicable)

If your recipe includes Rust code:

```bash
cargo test
```

### Fix Failing Tests

If tests fail:
1. Read the error message carefully
2. Check your implementation matches the test expectations
3. Verify all dependencies are installed
4. Check version compatibility

---

## Step 5: Validate Your Recipe

Before submitting, validate your recipe structure:

```bash
# From repository root
dot recipe validate my-first-pallet
```

**This checks:**
- `recipe.config.yml` exists and is valid
- `README.md` exists
- `versions.yml` is valid (if present)
- All version keys are recognized

**Expected output:**
```
✅ Recipe validation passed!

Checks:
  ✓ recipe.config.yml exists
  ✓ README.md exists
  ✓ versions.yml is valid
  ✓ All version keys are known
```

### Common Validation Issues

**Invalid YAML syntax:**
```
❌ recipe.config.yml: YAML parse error
```
Fix: Check YAML indentation and syntax

**Unknown version key:**
```
❌ Unknown version key: my_custom_tool
```
Fix: Use only recognized version keys (see `dot versions --validate`)

**Missing required files:**
```
❌ README.md not found
```
Fix: Ensure README.md exists in recipe directory

---

## Step 6: Commit Your Changes

Your recipe was created on a git branch. Commit your changes:

```bash
# Review changes
git status

# Add all files
git add .

# Commit with conventional commit format
git commit -m "feat(recipe): add my first pallet tutorial"
```

**Important:** Use [conventional commit format](../contributors/commit-conventions.md):
```
feat(recipe): <description>
```

This ensures proper semantic versioning and changelog generation.

---

## Step 7: Submit Your Recipe

Once your recipe is complete and tested, submit it for review:

```bash
dot recipe submit
```

### What the Submit Command Does

1. **Validates** recipe structure
2. **Checks** git repository state
3. **Pushes** to your fork (creates fork if needed)
4. **Creates** pull request on GitHub

### Interactive Prompts

```
? Select recipe to submit: my-first-pallet
? Recipe validated successfully!
? Pushing to fork...
? Creating pull request...

✅ Pull request created!
https://github.com/polkadot-developers/polkadot-cookbook/pull/123
```

### Pull Request Checklist

Your PR will include a checklist:
- [ ] Recipe follows style guidelines
- [ ] Tests pass locally
- [ ] Documentation is clear
- [ ] Examples work as expected

Reviewers will check:
- Code quality
- Documentation clarity
- Test coverage
- Adherence to guidelines

---

## Step 8: Respond to Review

Maintainers will review your PR and may request changes:

### Making Changes

```bash
# Make requested changes
vim README.md

# Commit changes
git add .
git commit -m "docs(recipe): clarify installation steps"

# Push to update PR
git push
```

The PR will automatically update with your changes.

### Common Review Feedback

**"Add more explanation"**
- Expand on complex concepts
- Add intermediate steps
- Include visual examples

**"Tests are failing"**
- Run tests locally: `npm test`
- Fix failing tests
- Commit and push fixes

**"Fix formatting"**
- Run `cargo fmt` (Rust)
- Run `prettier` (TypeScript)
- Check markdown formatting

---

## Next Steps

Congratulations! You've created your first recipe. Here's what to explore next:

### Learn More About Recipes

- **[Recipe Development Guide](../contributors/recipe-development.md)** - Best practices
- **[Recipe Guidelines](../contributors/recipe-guidelines.md)** - Style and structure
- **[Testing Recipes](../contributors/testing-recipes.md)** - Testing strategies

### Explore the Cookbook

- **[Browse Existing Recipes](../../recipes/)** - Learn from examples
- **[CLI Reference](../developers/cli-reference.md)** - All CLI commands
- **[Architecture](../developers/architecture.md)** - How the cookbook works

### Contribute More

- **[Contributor Workflow](../contributors/workflow.md)** - Development process
- **[Commit Conventions](../contributors/commit-conventions.md)** - Commit message format
- **[Contributing Guide](../../CONTRIBUTING.md)** - General contribution guidelines

---

## Troubleshooting

### Recipe Creation Failed

**Symptom:** `dot recipe create` fails with error

**Common causes:**
- Git not configured
- Node.js not installed
- Insufficient disk space

**Solution:**
```bash
# Check setup
dot setup

# Fix any missing dependencies
```

### Tests Won't Run

**Symptom:** `npm test` fails with error

**Common causes:**
- Dependencies not installed
- Vitest not configured
- TypeScript errors

**Solution:**
```bash
# Reinstall dependencies
rm -rf node_modules
npm install

# Check TypeScript
npx tsc --noEmit

# Run tests verbosely
npm test -- --reporter=verbose
```

### Submit Command Fails

**Symptom:** `dot recipe submit` fails

**Common causes:**
- GitHub CLI not authenticated
- Uncommitted changes
- Invalid recipe structure

**Solution:**
```bash
# Check gh authentication
gh auth status

# Login if needed
gh auth login

# Commit all changes
git add .
git commit -m "feat(recipe): complete implementation"

# Validate recipe
dot recipe validate my-first-pallet
```

### Merge Conflicts

**Symptom:** PR has merge conflicts

**Solution:**
```bash
# Update your branch
git fetch origin
git merge origin/master

# Resolve conflicts
# Edit conflicting files
git add .
git commit -m "fix: resolve merge conflicts"

# Push updates
git push
```

---

## Tips for Success

### Content Quality

✅ **DO:**
- Write clear, step-by-step instructions
- Include code examples for every step
- Show expected output
- Add troubleshooting section
- Test all examples yourself

❌ **DON'T:**
- Skip steps assuming prior knowledge
- Use jargon without explanation
- Include untested code
- Leave TODOs in published content

### Code Quality

✅ **DO:**
- Follow Rust/TypeScript style guides
- Add comments for complex logic
- Write comprehensive tests
- Handle errors gracefully
- Use meaningful variable names

❌ **DON'T:**
- Leave commented-out code
- Use hardcoded values
- Skip error handling
- Write tests that always pass

### Documentation

✅ **DO:**
- Explain why, not just what
- Link to related resources
- Include prerequisites
- Show real-world use cases
- Keep it up-to-date

❌ **DON'T:**
- Copy-paste without understanding
- Use broken links
- Reference outdated versions
- Assume reader knowledge

---

## Example Recipe

Want to see a complete example? Check out these recipes:

- **Basic Pallet** - `recipes/basic-pallet/` - Simple pallet structure
- **Storage Operations** - `recipes/storage-operations/` - Working with storage
- **Events and Errors** - `recipes/events-and-errors/` - Error handling

Browse all recipes: `dot recipe list`

---

## Need Help?

- **[Troubleshooting Guide](../contributors/troubleshooting.md)** - Common issues and solutions
- **[Discord Community](https://substrate.io/ecosystem/connect/)** - Ask questions
- **[GitHub Discussions](https://github.com/polkadot-developers/polkadot-cookbook/discussions)** - Community help
- **[GitHub Issues](https://github.com/polkadot-developers/polkadot-cookbook/issues)** - Report bugs

---

## Summary

You've learned how to:
- ✅ Create a recipe with `dot recipe create`
- ✅ Understand the generated file structure
- ✅ Customize recipe content and configuration
- ✅ Write and run tests
- ✅ Validate your recipe
- ✅ Submit a pull request

**Next:** Dive deeper into [Recipe Development Guide](../contributors/recipe-development.md) to learn best practices and advanced techniques.

---

[← Back to Getting Started](README.md)
