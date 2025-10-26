# Contributing to Polkadot Cookbook

Thank you for your interest in contributing to the Polkadot Cookbook! This project aims to provide high-quality, practical recipes for developers building on Polkadot.

## Table of Contents

- [How Can I Contribute?](#how-can-i-contribute)
  - [Reporting Bugs](#reporting-bugs)
  - [Suggesting Enhancements](#suggesting-enhancements)
  - [Contributing Recipes](#contributing-recipes)
  - [Improving Documentation](#improving-documentation)
- [Getting Started](#getting-started)
  - [Prerequisites](#prerequisites)
  - [Setting Up Your Development Environment](#setting-up-your-development-environment)
- [Recipe Contribution Workflow](#recipe-contribution-workflow)
  - [Step 1: Propose Your Recipe](#step-1-propose-your-recipe)
  - [Step 2: Setup Your Environment](#step-2-setup-your-environment)
  - [Step 3: Create Recipe Structure](#step-3-dot-structure)
  - [Step 4: Write Your Recipe](#step-4-write-your-recipe)
  - [Step 5: Test Your Recipe](#step-5-test-your-recipe)
  - [Step 6: Submit a Pull Request](#step-6-submit-a-pull-request)
- [Development Guidelines](#development-guidelines)
  - [Recipe Structure](#recipe-structure)
  - [Testing Requirements](#testing-requirements)
  - [Code Style](#code-style)
  - [Documentation Standards](#documentation-standards)
- [Advanced Topics](#advanced-topics)
  - [Recipe Configuration](#recipe-configuration)
  - [Justfiles and Scripts](#justfiles-and-scripts)
  - [CI/CD Pipeline](#cicd-pipeline)
- [Getting Help](#getting-help)

## How Can I Contribute?

### Reporting Bugs

If you find a bug in a recipe or the infrastructure:

1. **Check existing issues** to avoid duplicates
2. **Open a [new issue](https://github.com/polkadot-developers/polkadot-cookbook/issues/new/choose)** and select "Custom Blank Issue"
3. **Provide detailed information**:
   - Steps to reproduce
   - Expected vs actual behavior
   - Environment details (OS, Node version, Rust version)
   - Error messages or logs
   - Link to the recipe or file with the issue

### Suggesting Enhancements

We welcome suggestions for improvements:

1. **Open a [new issue](https://github.com/polkadot-developers/polkadot-cookbook/issues/new/choose)** and select "Custom Blank Issue"
2. **Describe the enhancement** clearly with a descriptive title
3. **Explain the use case** and benefits
4. **Provide examples** if applicable
5. **Tag appropriately** (enhancement, documentation, etc.)

### Contributing Recipes

The primary way to contribute is by creating new recipes. See the [Recipe Contribution Workflow](#recipe-contribution-workflow) section below.

#### Recipe Types: Tutorials vs Guides

When creating a recipe, you must choose between two types using the `content_type` field in your README frontmatter:

**Tutorial** (`content_type: tutorial`)
- **Complete journey from zero to working solution**
- Includes all setup steps (creating directories, cloning repos, installing dependencies)
- Assumes minimal prior setup or existing project
- Walks through every command and configuration needed
- **Example**: "Build Your First Parachain" - starts with `mkdir my-parachain`, guides through setup, configuration, testing, and deployment

**Guide** (`content_type: guide`)
- **Focused, actionable steps for a specific task**
- Assumes you already have a working project or environment
- Skips basic setup, jumps straight to the task at hand
- More concise, targeting experienced developers
- **Example**: "Add XCM Support to Your Parachain" - assumes you have a parachain project, shows only the XCM-specific configuration and code

**Key Question**: *Does the reader need to start from scratch?*
- If **yes** → Tutorial
- If **no** (they have an existing project) → Guide

### Improving Documentation

Documentation improvements are always welcome:

- Fix typos or unclear explanations
- Add missing documentation
- Improve code examples
- Update outdated information

Submit documentation changes via pull request following the same process as code contributions.

## Getting Started

### Prerequisites

Before contributing, ensure you have the following installed:

- **Node.js** `20+` - [Download](https://nodejs.org/)
- **npm** `10+` (comes with Node.js)
- **Rust** `1.86+` - [Install via rustup](https://rustup.rs)

### Setting Up Your Development Environment

1. **Fork the repository** on GitHub

2. **Clone your fork**:
   ```bash
   git clone https://github.com/YOUR_USERNAME/polkadot-cookbook.git
   cd polkadot-cookbook
   ```

3. **Add upstream remote**:
   ```bash
   git remote add upstream https://github.com/polkadot-developers/polkadot-cookbook.git
   ```

4. **Build the CLI tool** (first time only):
   ```bash
   cargo build --release
   ```

5. **Verify your setup**:
   ```bash
   ./target/release/dot --help
   ```

   You should see the CLI help with available commands. The CLI provides an interactive experience to guide you through recipe creation!

6. **Pre-commit hooks** (automatically installed):

   Git hooks are **automatically installed** when you run `cargo build` or `cargo test` via [cargo-husky](https://github.com/rhysd/cargo-husky).

   **No manual setup required!** The hooks run these checks before each commit:
   - ✅ `cargo fmt` - Formats Rust code (blocking)
   - ✅ `cargo clippy` - Lints Rust code (blocking)
   - ⚠️ Conventional commit message format (warning only)

   **Skip hooks**: If needed, use `git commit --no-verify` (use sparingly!)

   **Run checks manually**:
   ```bash
   cargo fmt --all
   cargo clippy --all-targets --all-features
   ```

   See [docs/pre-commit-hooks.md](docs/pre-commit-hooks.md) for more details.

## Recipe Contribution Workflow

### Step 1: Propose Your Recipe

**All recipes must be proposed and approved before starting work.**

1. Open a [new issue](https://github.com/polkadot-developers/polkadot-cookbook/issues/new/choose) and select "Recipe Proposal"

2. The template will guide you to provide:
   - **Summary**: What will users learn? (1-2 sentences)
   - **Audience**: Level (beginner/intermediate/advanced) and prerequisites
   - **Tools & Versions**: Key tools with versions you plan to use
   - **Outline**: High-level steps and expected result
   - **Type**: Polkadot SDK or Smart Contracts recipe
   - **Notes**: Any additional context for reviewers

3. **Wait for approval** and assignment of a recipe slug (e.g., `my-recipe`)

### Step 2: Setup Your Environment

**Sync with upstream**:
```bash
git fetch upstream
git checkout master
git merge upstream/master
```

> **Note**: You don't need to manually create a git branch - the CLI will do this for you automatically in the next step!

### Step 3: Create Recipe Structure

The CLI tool provides an interactive experience to create your recipe:

```bash
./target/release/dot create my-recipe
```

Or simply:

```bash
./target/release/dot my-recipe
```

**Interactive Prompts:**

The CLI will guide you through:
1. **Recipe slug** - Confirmed or prompted (lowercase, dashes only)
2. **Recipe type** - Choose between Polkadot SDK or Smart Contracts
3. **Description** - Optional description for your recipe
4. **Git branch** - Confirm creation of feature branch (default: yes)
5. **Dependencies** - Confirm npm install (default: yes)
6. **Summary** - Review configuration before creation

**Non-Interactive Mode:**

For scripts or CI/CD:
```bash
./target/release/dot create my-recipe --non-interactive
```

**Available Options:**
- `--skip-install` - Skip npm package installation
- `--no-git` - Skip automatic git branch creation
- `--non-interactive` - Skip prompts, use defaults

**Example:**
```bash
# Interactive (recommended for first-time users)
./target/release/dot my-recipe

# Non-interactive with custom options
./target/release/dot create my-recipe --skip-install --no-git --non-interactive
```

**What the CLI does:**
- ✓ Creates a feature branch `feat/my-recipe` (unless `--no-git`)
- ✓ Scaffolds the recipe directory structure
- ✓ Generates template files with your metadata
- ✓ Sets up testing infrastructure
- ✓ Installs npm dependencies (unless `--skip-install`)
- ✓ Shows next steps and git commands

**Generated structure**:
```
recipes/my-recipe/
├── recipe.config.yml    # Recipe metadata and configuration
├── README.md              # Recipe content (Markdown)
├── package.json           # npm dependencies
├── tsconfig.json          # TypeScript configuration
├── vitest.config.ts       # Test configuration
├── justfile               # Development commands (optional)
├── versions.yml           # Recipe-specific version overrides
├── .gitignore             # Git ignore file
├── src/                   # Recipe code
│   └── .gitkeep
├── tests/                 # End-to-end tests
│   └── my-recipe-e2e.test.ts
└── scripts/               # Helper scripts
    └── .gitkeep
```

> **Tip**: After creation, the CLI displays all the next steps and exact git commands you'll need!

### Step 4: Write Your Recipe

1. **Write the recipe content** in `recipes/my-recipe/README.md`
   - Use clear, concise language
   - Include code examples with explanations
   - Add screenshots or diagrams where helpful
   - Provide step-by-step instructions

2. **Add implementation code** under `recipes/my-recipe/src/`
   - Follow Polkadot SDK best practices
   - Include inline comments for complex logic
   - Use meaningful variable and function names

3. **Review and update your recipe configuration** in `recipe.config.yml`
   - The CLI pre-populated name, slug, and description from your input
   - Update the description if needed (or if you skipped it during creation)
   - Verify `needs_node` is correct for your recipe (default: `true`)
   - Update `type` field (`sdk` or `contracts`) to match your recipe type
   - Add or update category if needed

See [Recipe Structure](#recipe-structure) for detailed requirements.

### Step 5: Test Your Recipe

1. **Write end-to-end tests** in `recipes/my-recipe/tests/`
   - Use Vitest + @polkadot/api
   - Implement the [fast-skip pattern](#fast-skip-pattern) (required)
   - Test actual functionality, not just API connectivity

2. **Run tests locally**:
   ```bash
   cd recipes/my-recipe
   npm test
   ```

3. **Verify test behavior**:
   - Tests pass when node is running
   - Tests skip gracefully when no node is available

See [Testing Requirements](#testing-requirements) for details.

### Step 6: Submit a Pull Request

> **Note**: The CLI already created your feature branch `feat/my-recipe` if you didn't use `--no-git`. You just need to commit and push!

1. **Commit your changes**:
   ```bash
   git add .
   git commit -m "feat(recipe): add my-recipe"
   ```

   Follow [Conventional Commits](https://www.conventionalcommits.org/) format:
   - `feat(recipe)`: New recipe
   - `fix(recipe)`: Recipe bug fix
   - `docs`: Documentation updates
   - `test`: Test updates

2. **Push to your fork**:
   ```bash
   # The CLI creates branches with the pattern: feat/{recipe-slug}
   git push origin feat/my-recipe
   ```

   > **Tip**: The CLI output shows the exact git commands you need to run!

3. **Create a Pull Request**:
   - Go to the [repository](https://github.com/polkadot-developers/polkadot-cookbook)
   - Click "New Pull Request"
   - Select your branch
   - Fill out the PR template completely
   - Link the related proposal issue

4. **Respond to review feedback**:
   - Address all reviewer comments
   - Push additional commits as needed
   - Request re-review when ready

5. **CI Checks**:
   - Wait for automated tests to pass (`.github/workflows/test-recipes.yml`)
   - Fix any failing tests or linting issues

## Development Guidelines

### Recipe Structure

Each recipe must follow this structure:

#### README.md (Required)

The recipe content in Markdown format with YAML frontmatter:

```markdown
---
title: Recipe Title
description: Brief one-sentence description of what this recipe teaches
difficulty: Beginner  # or Intermediate, Advanced
content_type: tutorial  # or guide (see Recipe Types section)
categories: Category1, Category2
---

# Recipe Title

Brief description of what this recipe teaches.

## Prerequisites

- Required knowledge
- Tools needed
- Environment setup

## Learning Objectives

By the end of this recipe, you will:
- Objective 1
- Objective 2
- Objective 3

## Step 1: Title

Detailed instructions...

## Step 2: Title

Detailed instructions...

## Conclusion

Summary of what was learned.

## Next Steps

- Suggested follow-up recipes
- Additional resources
```

#### src/ (Required)

Recipe implementation code:
- Include inline comments
- Follow language-specific conventions

#### tests/ (Required)

End-to-end tests using Vitest + @polkadot/api:
- At least one test file
- Implement fast-skip pattern
- Test actual functionality

#### recipe.config.yml (Required)

Recipe metadata:
```yaml
name: My Recipe Title
slug: my-recipe
category: polkadot-sdk-cookbook  # or contracts-cookbook
needs_node: true                  # or false
description: Brief description
type: sdk                         # or contracts
```

### Testing Requirements

#### Framework

All recipes **must** use:
- **Vitest** for test framework
- **@polkadot/api** for blockchain interaction

#### Fast Skip Pattern

**Required for all tests.** Tests must gracefully skip when no node is available:

```typescript
import { describe, it, expect, beforeAll } from 'vitest';
import { ApiPromise, WsProvider } from '@polkadot/api';

describe('My Recipe Tests', () => {
  let api: ApiPromise | null = null;

  beforeAll(async () => {
    try {
      const provider = new WsProvider('ws://127.0.0.1:9944');
      const promise = ApiPromise.create({ provider });
      api = await Promise.race([
        promise,
        new Promise((_, reject) =>
          setTimeout(() => reject(new Error('timeout')), 2000)
        )
      ]) as ApiPromise;
    } catch (e) {
      console.log('⏭️  Skipping tests - no node running');
      api = null;
    }
  }, 5000);

  it('should connect to node', () => {
    if (!api) return; // Fast skip
    expect(api.isConnected).toBe(true);
  });

  it('should perform recipe operation', async () => {
    if (!api) return; // Fast skip

    // Your test logic
    const result = await api.query.system.account('...');
    expect(result).toBeDefined();
  });
});
```

#### Test Best Practices

1. **Fast skip for missing nodes** - Always check node availability
2. **Meaningful assertions** - Test actual outcomes, not just connectivity
3. **Clean up resources** - Disconnect APIs in `afterAll`
4. **Descriptive test names** - Use clear `describe` and `it` descriptions
5. **Appropriate timeouts** - 10-30s for blockchain operations
6. **Idempotent tests** - Runnable multiple times without side effects

### Code Style

#### TypeScript

- Use TypeScript strict mode
- Follow [TypeScript style guide](https://google.github.io/styleguide/tsguide.html)
- Use ESLint and Prettier (configuration provided)
- Prefer `async/await` over callbacks
- Use explicit types for function parameters and return values

#### Rust

- Follow [Rust API guidelines](https://rust-lang.github.io/api-guidelines/)
- Run `cargo fmt` before committing
- Run `cargo clippy` and fix warnings
- Add documentation comments (`///`) for public APIs
- Use meaningful error types

### Documentation Standards

#### Code Comments

- Explain **why**, not **what**
- Document complex algorithms or business logic

#### Recipe Writing

- Adhere to the [PaperMoon Style Guide](https://github.com/papermoonio/documentation-style-guide/blob/main/style-guide.md)
- Use clear, concise language
- Define technical terms on first use
- Include code examples with context
- Add visual aids (diagrams, screenshots) where helpful
- Test all commands and code snippets

#### Commit Messages

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>(<scope>): <description>

[optional body]

[optional footer]
```

**Types**:
- `feat`: New feature or recipe
- `fix`: Bug fix
- `docs`: Documentation only
- `test`: Tests only
- `refactor`: Code refactoring
- `chore`: Maintenance tasks

**Examples**:
```
feat(recipe): add zero-to-hero recipe
fix(recipe): correct chain spec generation in my-recipe
docs: update CONTRIBUTING.md with testing guidelines
test(zero-to-hero): add integration tests for pallets
```

## Additional Resources

For more detailed information, see:

- **[Architecture](docs/architecture.md)** - System architecture and design
- **[Testing Guide](docs/testing.md)** - Testing workflows and CI/CD
- **[Workflows](docs/workflows.md)** - GitHub Actions and automation

## Getting Help

### Resources

- **Example Recipe**: `recipes/zero-to-hero/`
- **[CLI Documentation](cli/)** - CLI tool reference
- **[SDK Documentation](core/)** - Core library API
- **[Polkadot Documentation](https://docs.polkadot.com)**

### Communication

- **Questions**: Open an [issue](https://github.com/polkadot-developers/polkadot-cookbook/issues)

---

Thank you for contributing to Polkadot Cookbook!
