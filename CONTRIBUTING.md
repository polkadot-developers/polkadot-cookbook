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
  - [Step 3: Create Recipe Structure](#step-3-create-recipe-structure)
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
2. **Use the bug report template** when creating a new issue
3. **Provide detailed information**:
   - Steps to reproduce
   - Expected vs actual behavior
   - Environment details (OS, Node version, Rust version)
   - Error messages or logs

### Suggesting Enhancements

We welcome suggestions for improvements:

1. **Open an issue** using the enhancement template
2. **Describe the enhancement** clearly
3. **Explain the use case** and benefits
4. **Provide examples** if applicable

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

4. **Build the Polkadot Cookbook SDK** (first time only):
   ```bash
   cargo build --workspace --release
   ```

5. **Verify your setup**:
   ```bash
   cargo run --package polkadot-cookbook-cli -- --help
   ```

## Recipe Contribution Workflow

### Step 1: Propose Your Recipe

**All recipes must be proposed and approved before starting work.**

1. Open a [new issue](https://github.com/polkadot-developers/polkadot-cookbook/issues/new?template=01-recipe-proposal.md) using the "Recipe Proposal" template

2. Provide the following information:
   - **Title**: Clear, descriptive title
   - **Learning Objectives**: What will readers learn?
   - **Target Audience**: Beginner, intermediate, or advanced?
   - **Prerequisites**: Required knowledge or setup
   - **Estimated Length**: How long will the recipe take?
   - **Tools/Versions**: Specific tools or dependencies needed
   - **Outline**: High-level structure of the recipe

3. **Wait for approval** and assignment of a recipe slug (e.g., `my-recipe`)

### Step 2: Setup Your Environment

1. **Sync with upstream**:
   ```bash
   git fetch upstream
   git checkout master
   git merge upstream/master
   ```

2. **Create a new branch**:
   ```bash
   git checkout -b feat/recipe-my-recipe
   ```

### Step 3: Create Recipe Structure

Run the recipe creation CLI with your approved slug:

```bash
cargo run --package polkadot-cookbook-cli -- my-recipe
```

**Available options:**
- `--skip-install` - Skip npm package installation
- `--no-git` - Skip automatic git branch creation

**Example with options:**
```bash
cargo run --package polkadot-cookbook-cli -- my-recipe --skip-install --no-git
```

This command will:
- Create a feature branch (unless `--no-git` is specified)
- Scaffold the recipe directory structure
- Set up testing infrastructure
- Install dependencies (unless `--skip-install` is specified)
- Generate boilerplate files

**Generated structure**:
```
recipes/my-recipe/
├── recipe.config.yml    # Recipe metadata and configuration
├── README.md              # Recipe content (Markdown)
├── package.json           # npm dependencies
├── tsconfig.json          # TypeScript configuration
├── vitest.config.ts       # Test configuration
├── justfile               # Development commands (optional)
├── src/                   # Recipe code
└── tests/                 # End-to-end tests
```

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

3. **Configure your recipe** in `recipe.config.yml`
   - Set accurate metadata (name, description, category)
   - Specify if a node is required (`needs_node`)
   - Configure build and runtime settings if applicable

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
   git push origin feat/recipe-my-recipe
   ```

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
- **[CLI Documentation](polkadot-cookbook-cli/)** - CLI tool reference
- **[SDK Documentation](polkadot-cookbook-core/)** - Core library API
- **[Polkadot Documentation](https://docs.polkadot.com)**

### Communication

- **Questions**: Open an [issue](https://github.com/polkadot-developers/polkadot-cookbook/issues)

---

Thank you for contributing to Polkadot Cookbook!
