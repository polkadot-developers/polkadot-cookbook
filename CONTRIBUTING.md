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
  - [Step 1: Setup Your Environment](#step-1-setup-your-environment)
  - [Step 2: Create Recipe Structure](#step-2-create-recipe-structure)
  - [Step 3: Write Your Recipe](#step-3-write-your-recipe)
  - [Step 4: Test Your Recipe](#step-4-test-your-recipe)
  - [Step 5: Submit Your Recipe](#step-5-submit-your-recipe)
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

The primary way to contribute is by creating new recipes. Anyone can contribute a recipe by submitting a pull request - **no prior proposal or approval is required**. See the [Recipe Contribution Workflow](#recipe-contribution-workflow) section below.

**Note:** A recipe in the Polkadot Cookbook does not guarantee that it will be used in the official Polkadot Docs. Recipes are community-contributed resources that may be selected for documentation based on quality, relevance, and maintainability.

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

- **Rust** `1.81+` - [Install via rustup](https://rustup.rs)
- **Git** - [Download](https://git-scm.com/)

**Optional but recommended:**
- **Just** - Task runner for recipes - `cargo install just`

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
   cargo build --release --bin dot
   ```

5. **Verify your setup**:
   ```bash
   ./target/release/dot setup
   ```

   The `setup` command will check your environment and guide you through installing any missing dependencies.

6. **Run diagnostics** (optional):
   ```bash
   ./target/release/dot doctor
   ```

   The `doctor` command performs comprehensive environment checks and shows the status of all tools.

7. **Pre-commit hooks** (automatically installed):

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

**New Streamlined Process:** You can now contribute recipes directly via pull request without a prior proposal! The `dot recipe submit` command makes it easy to create PRs with a single command.

### Step 1: Setup Your Environment

If you haven't already completed the [Setting Up Your Development Environment](#setting-up-your-development-environment) steps, do that first.

**Sync with upstream**:
```bash
git fetch upstream
git checkout master
git merge upstream/master
```

> **Note**: You don't need to manually create a git branch - the CLI will do this for you automatically in the next step!

### Step 2: Create Recipe Structure

The CLI tool provides an interactive experience to create your recipe:

```bash
./target/release/dot my-pallet
```

Or use explicit command:

```bash
./target/release/dot recipe new my-pallet
```

**Interactive Prompts:**

The CLI will guide you through:
1. **Recipe slug** - Confirmed or prompted (lowercase, dashes only)
2. **Recipe type** - Choose between Polkadot SDK, Solidity, or XCM
3. **Recipe name** - Display name for your recipe
4. **Description** - Brief description of what the recipe teaches

**Non-Interactive Mode:**

For scripts or CI/CD:
```bash
./target/release/dot my-pallet --non-interactive
```

**What the CLI does:**
- ✓ Scaffolds the recipe directory structure
- ✓ Generates template files based on recipe type
- ✓ Creates recipe.config.yml with metadata
- ✓ Sets up testing infrastructure
- ✓ Shows next steps and commands

**Generated structure for Polkadot SDK recipes:**
```
recipes/my-pallet/
├── Cargo.toml            # Workspace configuration
├── recipe.config.yml     # Recipe metadata
├── README.md             # Recipe documentation
├── justfile              # Development commands (optional)
├── pallets/
│   └── template/         # Your pallet implementation
│       ├── Cargo.toml
│       └── src/
│           └── lib.rs    # Pallet code
└── tests/                # Mock runtime and tests
    ├── mock.rs           # Mock runtime for testing
    └── integration_test.rs
```

> **Tip**: After creation, you can use `dot recipe validate` to check your structure!

### Step 3: Write Your Recipe

1. **Write the recipe content** in `recipes/my-pallet/README.md`
   - Use clear, concise language
   - Include code examples with explanations
   - Add screenshots or diagrams where helpful
   - Provide step-by-step instructions

2. **Implement your pallet** in `recipes/my-pallet/pallets/template/src/lib.rs`
   - Follow FRAME best practices
   - Add storage items, events, errors, and dispatchables
   - Include inline comments for complex logic
   - Use meaningful variable and function names

3. **Add tests** in `recipes/my-pallet/tests/`
   - Create a mock runtime in `mock.rs`
   - Write integration tests in `integration_test.rs`
   - Test all pallet functionality

4. **Review and update your recipe configuration** in `recipe.config.yml`
   - The CLI pre-populated name, slug, and description from your input
   - Verify `type` is set to `polkadot-sdk`
   - Update the description if needed

See [Recipe Structure](#recipe-structure) for detailed requirements.

### Step 4: Test Your Recipe

1. **Use CLI testing tools**:
   ```bash
   # Test your recipe
   ./target/release/dot recipe test my-pallet

   # Validate structure
   ./target/release/dot recipe validate my-pallet

   # Run linting
   ./target/release/dot recipe lint my-pallet
   ```

2. **Or test directly with Cargo**:
   ```bash
   cd recipes/my-pallet

   # Run tests
   cargo test --all-features

   # Check formatting
   cargo fmt --all -- --check

   # Run clippy
   cargo clippy --all-features --all-targets -- -D warnings
   ```

3. **Verify test coverage**:
   - Test all dispatchable functions
   - Test error conditions
   - Test storage operations
   - Test events are emitted correctly

See [Testing Requirements](#testing-requirements) for details.

### Step 5: Submit Your Recipe

The easiest way to submit your recipe is using the built-in submit command:

#### Option 1: Using the CLI Submit Command (Recommended)

```bash
./target/release/dot recipe submit my-recipe
```

Or from within the recipe directory:
```bash
cd recipes/my-recipe
../../target/release/dot recipe submit
```

**What the submit command does:**
- ✓ Validates recipe structure
- ✓ Commits any uncommitted changes
- ✓ Pushes your branch to your fork
- ✓ Creates a pull request with auto-generated title and description
- ✓ Links to the PR URL when complete

**Authentication:**
The submit command uses the GitHub API and requires a GitHub token. It will automatically check:
1. `GITHUB_TOKEN` environment variable
2. GitHub CLI (`gh`) config file at `~/.config/gh/hosts.yml`

If neither is found, create a token at https://github.com/settings/tokens/new with `repo` scope.

**Optional flags:**
- `--title "Custom PR Title"` - Specify a custom PR title
- `--body "Custom description"` - Specify a custom PR description

#### Option 2: Manual Submission

If you prefer to submit manually:

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

3. **Create a Pull Request**:
   - Go to the [repository](https://github.com/polkadot-developers/polkadot-cookbook)
   - Click "New Pull Request"
   - Select your branch
   - Fill out the PR template
   - Use the GitHub web interface or: `gh pr create --title "feat(recipe): add my-recipe"`

#### After Submission

1. **Respond to review feedback**:
   - Address all reviewer comments
   - Push additional commits as needed
   - Request re-review when ready

2. **CI Checks**:
   - Wait for automated tests to pass (`.github/workflows/test-recipes.yml`)
   - Fix any failing tests or linting issues

**Note:** Recipes are reviewed for quality, accuracy, and adherence to guidelines. Not all submitted recipes may be merged, but feedback will be provided to help improve them.

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

#### pallets/ (Required for Polkadot SDK)

Pallet implementation code:
- Follow FRAME conventions
- Include inline comments
- Add documentation for public APIs
- Implement storage, events, errors, and dispatchables

#### tests/ (Required)

Integration tests with mock runtime:
- `mock.rs` - Mock runtime configuration
- `integration_test.rs` - Test cases for pallet functionality
- Test all dispatchable functions
- Test error conditions and edge cases

#### recipe.config.yml (Required)

Recipe metadata:
```yaml
name: My Pallet
slug: my-pallet
description: Brief description of the recipe
type: polkadot-sdk  # or solidity, xcm
```

#### Cargo.toml (Required for Polkadot SDK)

Workspace configuration:
- Define workspace members
- Specify Polkadot SDK dependencies
- Configure features (std, try-runtime)

### Testing Requirements

#### Framework

Polkadot SDK recipes **must** use:
- **Cargo test** - Standard Rust testing framework
- **Mock runtime** - For testing pallet functionality in isolation
- **Integration tests** - Test pallet behavior with mock runtime

#### Mock Runtime Pattern

**Required for all Polkadot SDK recipes.** Create a mock runtime in `tests/mock.rs`:

```rust
use frame_support::{
    derive_impl, parameter_types,
    traits::{ConstU32, ConstU64},
};
use sp_runtime::{traits::IdentityLookup, BuildStorage};

type Block = frame_system::mocking::MockBlock<Test>;

frame_support::construct_runtime!(
    pub struct Test {
        System: frame_system,
        TemplatePallet: pallet_template,
    }
);

#[derive_impl(frame_system::config_preludes::TestDefaultConfig)]
impl frame_system::Config for Test {
    type Block = Block;
    type AccountId = u64;
    type Lookup = IdentityLookup<Self::AccountId>;
}

impl pallet_template::Config for Test {}

pub fn new_test_ext() -> sp_io::TestExternalities {
    let t = frame_system::GenesisConfig::<Test>::default()
        .build_storage()
        .unwrap();
    t.into()
}
```

#### Integration Test Pattern

Create tests in `tests/integration_test.rs`:

```rust
use crate::mock::*;

#[test]
fn it_works_for_default_value() {
    new_test_ext().execute_with(|| {
        // Test your pallet functionality
        assert_ok!(TemplatePallet::do_something(RuntimeOrigin::signed(1), 42));
        // Verify storage
        assert_eq!(TemplatePallet::something(), Some(42));
    });
}

#[test]
fn correct_error_for_invalid_value() {
    new_test_ext().execute_with(|| {
        // Test error conditions
        assert_noop!(
            TemplatePallet::cause_error(RuntimeOrigin::signed(1)),
            Error::<Test>::NoneValue
        );
    });
}
```

#### Test Best Practices

1. **Test all dispatchables** - Cover success and error cases
2. **Test storage operations** - Verify reads and writes work correctly
3. **Test events** - Ensure events are emitted with correct data
4. **Test permissions** - Verify origin checks work properly
5. **Test edge cases** - Overflow, underflow, and boundary conditions
6. **Use descriptive test names** - Clear function names explaining what's tested

### Code Style

#### Rust (Polkadot SDK Recipes)

- Follow [Rust API guidelines](https://rust-lang.github.io/api-guidelines/)
- Follow [FRAME coding style](https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/reference_docs/frame_pallet_coupling/index.html)
- Run `cargo fmt --all` before committing (enforced by pre-commit hooks)
- Run `cargo clippy --all-features --all-targets -- -D warnings` and fix all issues (enforced by pre-commit hooks)
- Add documentation comments (`///`) for all public APIs
- Use meaningful error types and descriptive error variants
- Use `#[pallet::weight(...)]` for all dispatchables
- Implement proper genesis configuration when needed

#### TypeScript (Solidity/XCM Recipes - Coming Soon)

- Use TypeScript strict mode
- Follow [TypeScript style guide](https://google.github.io/styleguide/tsguide.html)
- Use ESLint and Prettier (configuration provided)
- Prefer `async/await` over callbacks
- Use explicit types for function parameters and return values

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
feat(recipe): add custom-pallet recipe
fix(recipe): correct storage operations in my-pallet
docs: update CONTRIBUTING.md with testing guidelines
test(custom-pallet): add integration tests for dispatchables
chore(cli): update dependencies
```

## Additional Resources

For more detailed information, see:

- **[Architecture](docs/architecture.md)** - System architecture and design
- **[Testing Guide](docs/testing.md)** - Testing workflows and CI/CD
- **[Workflows](docs/workflows.md)** - GitHub Actions and automation

## Getting Help

### Resources

- **Example Recipe**: `recipes/basic-pallet/` - Simple FRAME pallet example
- **[CLI Tool Documentation](cli/README.md)** - CLI tool commands and usage
- **[Core Library Documentation](core/README.md)** - SDK API reference
- **[Polkadot SDK Documentation](https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/index.html)**
- **[FRAME Documentation](https://paritytech.github.io/polkadot-sdk/master/frame_support/index.html)**

### Communication

- **Questions**: Open an [issue](https://github.com/polkadot-developers/polkadot-cookbook/issues)

---

Thank you for contributing to Polkadot Cookbook!
