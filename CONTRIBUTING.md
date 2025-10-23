# Contributing to Polkadot Cookbook

Thank you for your interest in contributing to the Polkadot Cookbook! This project aims to provide high-quality, practical tutorials for developers building on Polkadot.

## Table of Contents

- [How Can I Contribute?](#how-can-i-contribute)
  - [Reporting Bugs](#reporting-bugs)
  - [Suggesting Enhancements](#suggesting-enhancements)
  - [Contributing Tutorials](#contributing-tutorials)
  - [Improving Documentation](#improving-documentation)
- [Getting Started](#getting-started)
  - [Prerequisites](#prerequisites)
  - [Setting Up Your Development Environment](#setting-up-your-development-environment)
- [Tutorial Contribution Workflow](#tutorial-contribution-workflow)
  - [Step 1: Propose Your Tutorial](#step-1-propose-your-tutorial)
  - [Step 2: Setup Your Environment](#step-2-setup-your-environment)
  - [Step 3: Create Tutorial Structure](#step-3-create-tutorial-structure)
  - [Step 4: Write Your Tutorial](#step-4-write-your-tutorial)
  - [Step 5: Test Your Tutorial](#step-5-test-your-tutorial)
  - [Step 6: Submit a Pull Request](#step-6-submit-a-pull-request)
- [Development Guidelines](#development-guidelines)
  - [Tutorial Structure](#tutorial-structure)
  - [Testing Requirements](#testing-requirements)
  - [Code Style](#code-style)
  - [Documentation Standards](#documentation-standards)
- [Advanced Topics](#advanced-topics)
  - [Tutorial Configuration](#tutorial-configuration)
  - [Justfiles and Scripts](#justfiles-and-scripts)
  - [CI/CD Pipeline](#cicd-pipeline)
- [Getting Help](#getting-help)

## How Can I Contribute?

### Reporting Bugs

If you find a bug in a tutorial or the infrastructure:

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

### Contributing Tutorials

The primary way to contribute is by creating new tutorials. See the [Tutorial Contribution Workflow](#tutorial-contribution-workflow) section below.

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
- **Git** - [Install](https://git-scm.com/downloads)
- **just** (optional, for running justfiles) - [Install](https://github.com/casey/just)

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

## Tutorial Contribution Workflow

### Step 1: Propose Your Tutorial

**All tutorials must be proposed and approved before starting work.**

1. Open a [new issue](https://github.com/polkadot-developers/polkadot-cookbook/issues/new?template=01-tutorial-proposal.md) using the "Tutorial Proposal" template

2. Provide the following information:
   - **Title**: Clear, descriptive title
   - **Learning Objectives**: What will readers learn?
   - **Target Audience**: Beginner, intermediate, or advanced?
   - **Prerequisites**: Required knowledge or setup
   - **Estimated Length**: How long will the tutorial take?
   - **Tools/Versions**: Specific tools or dependencies needed
   - **Outline**: High-level structure of the tutorial

3. **Wait for approval** and assignment of a tutorial slug (e.g., `my-tutorial`)

### Step 2: Setup Your Environment

1. **Sync with upstream**:
   ```bash
   git fetch upstream
   git checkout master
   git merge upstream/master
   ```

2. **Create a new branch**:
   ```bash
   git checkout -b feat/tutorial-my-tutorial
   ```

### Step 3: Create Tutorial Structure

Run the tutorial creation CLI with your approved slug:

```bash
cargo run --package polkadot-cookbook-cli -- my-tutorial
```

**Available options:**
- `--skip-install` - Skip npm package installation
- `--no-git` - Skip automatic git branch creation

**Example with options:**
```bash
cargo run --package polkadot-cookbook-cli -- my-tutorial --skip-install --no-git
```

This command will:
- Create a feature branch (unless `--no-git` is specified)
- Scaffold the tutorial directory structure
- Set up testing infrastructure
- Install dependencies (unless `--skip-install` is specified)
- Generate boilerplate files

**Generated structure**:
```
tutorials/my-tutorial/
├── tutorial.config.yml    # Tutorial metadata and configuration
├── README.md              # Tutorial content (Markdown)
├── package.json           # npm dependencies
├── tsconfig.json          # TypeScript configuration
├── vitest.config.ts       # Test configuration
├── justfile               # Development commands (optional)
├── src/                   # Tutorial code
└── tests/                 # End-to-end tests
```

### Step 4: Write Your Tutorial

1. **Write the tutorial content** in `tutorials/my-tutorial/README.md`
   - Use clear, concise language
   - Include code examples with explanations
   - Add screenshots or diagrams where helpful
   - Provide step-by-step instructions

2. **Add implementation code** under `tutorials/my-tutorial/src/`
   - Follow Polkadot SDK best practices
   - Include inline comments for complex logic
   - Use meaningful variable and function names

3. **Configure your tutorial** in `tutorial.config.yml`
   - Set accurate metadata (name, description, category)
   - Specify if a node is required (`needs_node`)
   - Configure build and runtime settings if applicable

See [Tutorial Structure](#tutorial-structure) for detailed requirements.

### Step 5: Test Your Tutorial

1. **Write end-to-end tests** in `tutorials/my-tutorial/tests/`
   - Use Vitest + @polkadot/api
   - Implement the [fast-skip pattern](#fast-skip-pattern) (required)
   - Test actual functionality, not just API connectivity

2. **Run tests locally**:
   ```bash
   cd tutorials/my-tutorial
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
   git commit -m "feat(tutorial): add my-tutorial"
   ```

   Follow [Conventional Commits](https://www.conventionalcommits.org/) format:
   - `feat(tutorial)`: New tutorial
   - `fix(tutorial)`: Tutorial bug fix
   - `docs`: Documentation updates
   - `test`: Test updates

2. **Push to your fork**:
   ```bash
   git push origin feat/tutorial-my-tutorial
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
   - Wait for automated tests to pass (`.github/workflows/test-tutorials.yml`)
   - Fix any failing tests or linting issues

## Development Guidelines

### Tutorial Structure

Each tutorial must follow this structure:

#### README.md (Required)

The tutorial content in Markdown format:

```markdown
# Tutorial Title

Brief description of what this tutorial teaches.

## Prerequisites

- Required knowledge
- Tools needed
- Environment setup

## Learning Objectives

By the end of this tutorial, you will:
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

- Suggested follow-up tutorials
- Additional resources
```

#### src/ (Required)

Tutorial implementation code:
- Include inline comments
- Follow language-specific conventions

#### tests/ (Required)

End-to-end tests using Vitest + @polkadot/api:
- At least one test file
- Implement fast-skip pattern
- Test actual functionality

#### tutorial.config.yml (Required)

Tutorial metadata:
```yaml
name: My Tutorial Title
slug: my-tutorial
category: polkadot-sdk-cookbook  # or contracts-cookbook
needs_node: true                  # or false
description: Brief description
type: sdk                         # or contracts
```

### Testing Requirements

#### Framework

All tutorials **must** use:
- **Vitest** for test framework
- **@polkadot/api** for blockchain interaction

#### Fast Skip Pattern

**Required for all tests.** Tests must gracefully skip when no node is available:

```typescript
import { describe, it, expect, beforeAll } from 'vitest';
import { ApiPromise, WsProvider } from '@polkadot/api';

describe('My Tutorial Tests', () => {
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

  it('should perform tutorial operation', async () => {
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

#### Tutorial Writing

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
- `feat`: New feature or tutorial
- `fix`: Bug fix
- `docs`: Documentation only
- `test`: Tests only
- `refactor`: Code refactoring
- `chore`: Maintenance tasks

**Examples**:
```
feat(tutorial): add zero-to-hero tutorial
fix(tutorial): correct chain spec generation in my-tutorial
docs: update CONTRIBUTING.md with testing guidelines
test(zero-to-hero): add integration tests for pallets
```

## Advanced Topics

### Tutorial Configuration

The `tutorial.config.yml` file controls how your tutorial is built and tested.

#### Basic Configuration

```yaml
name: My Tutorial Title
slug: my-tutorial
category: polkadot-sdk-cookbook
needs_node: true
description: Learn how to build a custom pallet
type: sdk
```

#### Advanced Configuration with Manifest

For tutorials with runtime/parachain code:

```yaml
name: My Tutorial Title
slug: my-tutorial
category: polkadot-sdk-cookbook
needs_node: true
description: Build a custom parachain
type: sdk

manifest:
  build:
    project_dir: src
    commands:
      - cargo build --release

  runtime:
    wasm_path: ./target/release/wbuild/my-runtime/my_runtime.compact.compressed.wasm

  network:
    relay_chain: paseo
    para_id: 1000

  tests:
    framework: vitest
    files:
      - tests/my-tutorial-e2e.test.ts

scripts_dir: scripts
zombienet_dir: zombienet
```

#### When to Update

Update `tutorial.config.yml` when:
- Changing build process or commands
- Modifying runtime WASM output paths
- Updating network configuration
- Changing test file locations
- Toggling `needs_node` flag

### Justfiles and Scripts

#### Tutorial Justfiles

Each tutorial can have a `justfile` for common development tasks:

```justfile
# List available commands
default:
  @just --list

# Setup Rust toolchain
setup-rust:
  rustup install 1.86.0
  rustup default 1.86.0

# Build the runtime
build:
  cd src && cargo build --release

# Run tests
test:
  npm test

# Start local node
start-node:
  ./scripts/start-node.sh
```

**Usage**:
```bash
cd tutorials/my-tutorial
just          # List commands
just build    # Run build
just test     # Run tests
```

#### Global vs Tutorial Scripts

**Global scripts** (`/scripts/`):
- Shared across multiple tutorials
- Common setup procedures
- Reusable utilities

**Tutorial scripts** (`/tutorials/<slug>/scripts/`):
- Tutorial-specific workflows
- Auto-generated versioned scripts (post-merge)
- Custom setup unique to this tutorial

**When to use which**:
- Use global scripts for common patterns
- Use tutorial scripts for unique workflows
- Propose migration to global if script is reused 3+ times

### CI/CD Pipeline

#### Automated Checks on PR

When you submit a PR, the following checks run automatically:

1. **Tutorial Tests** (`.github/workflows/test-tutorials.yml`)
   - Runs if new tutorial folder is added
   - Installs dependencies
   - Runs `npm test` for affected tutorials
   - Tests must pass or skip gracefully

2. **Build Verification**
   - Validates `tutorial.config.yml` syntax
   - Checks for required files
   - Verifies test files exist

#### Post-Merge Workflow

After your PR is merged, maintainers will:

1. **Generate versioned scripts** via `/generate-scripts` command
   - Creates pinned setup scripts in `tutorials/<slug>/scripts/`
   - Commits scripts to repository

2. **Create tutorial tag** in format `tutorial/<slug>/vYYYYMMDD-HHMMSS`
   - Enables stable snippet extraction for documentation

3. **Optionally create GitHub release** for major tutorials

#### Version Management

Tutorial versions are managed in `versions.yml`:

```yaml
versions:
  rust: "1.86.0"
  chain_spec_builder: "0.20.0"
  polkadot_omni_node: "0.4.1"

# Tutorial-specific overrides (optional)
my_tutorial:
  rust: "1.85.0"
```

Maintainers handle version updates. If your tutorial requires specific versions, note this in your proposal.

## SDK Architecture

The Polkadot Cookbook uses a modular SDK architecture consisting of two main components:

### Polkadot Cookbook Core (`polkadot-cookbook-core`)

The core library provides the business logic for tutorial creation and management. It can be used programmatically by other tools.

**Key modules:**
- `config` - Type-safe project and tutorial configuration
- `error` - Comprehensive error types with serialization support
- `git` - Async git operations
- `templates` - Template generation for scaffolding
- `scaffold` - Project creation and directory structure
- `bootstrap` - Test environment setup (npm, dependencies, config files)

**Features:**
- Async-first API using Tokio
- Structured logging with `tracing`
- Serializable errors for tooling integration
- Comprehensive test coverage (80%+)
- No terminal dependencies (pure library)

**Example programmatic usage:**
```rust
use polkadot_cookbook_core::{config::ProjectConfig, Scaffold};
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ProjectConfig::new("my-tutorial")
        .with_destination(PathBuf::from("./tutorials"))
        .with_git_init(true)
        .with_skip_install(false);

    let scaffold = Scaffold::new();
    let project_info = scaffold.create_project(config).await?;

    println!("Created: {}", project_info.project_path.display());
    Ok(())
}
```

For more information, see [`polkadot-cookbook-core/README.md`](polkadot-cookbook-core/README.md).

### Polkadot Cookbook CLI (`polkadot-cookbook-cli`)

A thin CLI wrapper around the core library that provides a command-line interface.

**Features:**
- Beautiful colored output
- Progress indicators
- Error handling with helpful messages
- Command-line flags for customization

**Usage:**
```bash
# Create a new tutorial
cargo run --package polkadot-cookbook-cli -- my-tutorial

# With options
cargo run --package polkadot-cookbook-cli -- my-tutorial --skip-install --no-git

# Show help
cargo run --package polkadot-cookbook-cli -- --help
```

### Why This Architecture?

The SDK architecture provides several benefits:

1. **Separation of Concerns**
   - Core library has zero UI/terminal dependencies
   - CLI is a thin presentation layer
   - Business logic is testable and reusable

2. **Programmatic Access**
   - Other tools can use the core library directly
   - IDE extensions can integrate the functionality
   - CI/CD pipelines can automate tutorial creation

3. **Better Testing**
   - Unit tests for business logic
   - Integration tests for workflows
   - CLI can be tested separately

4. **Easier Maintenance**
   - Clear module boundaries
   - Async-first for better performance
   - Structured logging for observability

### Contributing to the SDK

If you want to contribute to the SDK itself (not just tutorials):

1. **Core library changes** go in `polkadot-cookbook-core/`
   - Add features to appropriate modules
   - Write comprehensive tests
   - Use structured logging (`tracing`)
   - Ensure no terminal dependencies

2. **CLI changes** go in `polkadot-cookbook-cli/`
   - Keep it thin (mostly UI/formatting)
   - Delegate logic to core library
   - Use colored output for better UX

3. **Run tests**:
   ```bash
   # Test core library
   cargo test --package polkadot-cookbook-core

   # Test CLI
   cargo run --package polkadot-cookbook-cli -- test-project --skip-install --no-git

   # Test entire workspace
   cargo test --workspace
   ```

4. **Check formatting and lints**:
   ```bash
   cargo fmt --check
   cargo clippy --workspace -- -D warnings
   ```

## Getting Help

### Resources

- **Visual Guide**: [Tutorial Creation Workflow](docs/TUTORIAL_WORKFLOW.md)
- **Workflow Documentation**: [docs/WORKFLOWS.md](docs/WORKFLOWS.md)
- **Example Tutorial**: `tutorials/zero-to-hero/`
- **Polkadot Documentation**: [docs.polkadot.com](https://docs.polkadot.com)

### Communication

- **Questions**: Open an [issue](https://github.com/polkadot-developers/polkadot-cookbook/issues)

Thank you for contributing to Polkadot Cookbook!
