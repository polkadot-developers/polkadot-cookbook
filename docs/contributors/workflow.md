# Contributor Workflow

Step-by-step guide to contributing recipes to the Polkadot Cookbook.

## Overview

The contribution workflow follows these steps:

1. **Setup** - Fork, clone, and configure your environment
2. **Create** - Generate a new recipe or modify existing content
3. **Develop** - Write content, code, and tests
4. **Test** - Run tests locally
5. **Commit** - Use conventional commit messages
6. **Submit** - Create a pull request
7. **Review** - Respond to feedback
8. **Merge** - Maintainers merge your contribution

---

## Prerequisites

Before contributing, ensure you have:

- **Git** - Version control
- **GitHub Account** - For forking and PRs
- **GitHub CLI** (optional) - For easier PR submission
- **Development Tools** - Rust, Node.js, or both depending on recipe type
- **CLI Installed** - `dot` command-line tool

**Check your setup:**
```bash
dot setup
```

---

## Step 1: Fork and Clone

### Fork the Repository

1. Visit [polkadot-cookbook](https://github.com/polkadot-developers/polkadot-cookbook)
2. Click **Fork** in the top-right corner
3. Select your account as the destination

### Clone Your Fork

```bash
# Clone your fork
git clone https://github.com/YOUR_USERNAME/polkadot-cookbook.git
cd polkadot-cookbook

# Add upstream remote
git remote add upstream https://github.com/polkadot-developers/polkadot-cookbook.git

# Verify remotes
git remote -v
```

**Expected output:**
```
origin    https://github.com/YOUR_USERNAME/polkadot-cookbook.git (fetch)
origin    https://github.com/YOUR_USERNAME/polkadot-cookbook.git (push)
upstream  https://github.com/polkadot-developers/polkadot-cookbook.git (fetch)
upstream  https://github.com/polkadot-developers/polkadot-cookbook.git (push)
```

### Configure Git

```bash
# Set your identity
git config user.name "Your Name"
git config user.email "your.email@example.com"

# Verify configuration
git config --list
```

---

## Step 2: Create a Recipe

### Using the CLI (Recommended)

The CLI automatically creates a new branch for your recipe:

```bash
# Interactive mode
dot recipe create

# The CLI will:
# 1. Create recipes/your-recipe/
# 2. Generate scaffolded files
# 3. Create branch: recipe/your-recipe
# 4. Install dependencies
```

**Your branch is now:** `recipe/your-recipe`

### Manual Branch Creation

If you're modifying existing content instead of creating a new recipe:

```bash
# Update master first
git checkout master
git pull upstream master

# Create feature branch
git checkout -b feat/improve-documentation

# Or bug fix branch
git checkout -b fix/correct-example-code
```

**Branch naming conventions:**
- `recipe/<slug>` - New recipes (auto-created by CLI)
- `feat/<description>` - New features or enhancements
- `fix/<description>` - Bug fixes
- `docs/<description>` - Documentation improvements
- `refactor/<description>` - Code refactoring

---

## Step 3: Develop Your Recipe

### Recipe Structure

Your recipe directory should contain:

```
recipes/your-recipe/
├── README.md              # Main content (required)
├── recipe.config.yml      # Metadata (required)
├── src/                   # Source code
├── tests/                 # Tests
└── ...                    # Type-specific files
```

See [Recipe Guidelines](recipe-guidelines.md) for detailed structure requirements.

### Writing Guidelines

**README.md should include:**
- Clear title and description
- Prerequisites
- Learning objectives
- Step-by-step instructions
- Code examples with explanations
- Expected output
- Troubleshooting section

**See:** [Recipe Development Guide](recipe-development.md) for best practices.

### Version Management

If your recipe needs different dependency versions:

```yaml
versions:
  rust: "1.86"
  polkadot_omni_node: "0.6.0"
```

Only include versions that differ from global defaults.


---

## Step 4: Test Your Recipe

### Run Tests Locally

**TypeScript recipes:**
```bash
cd recipes/your-recipe
npm test
```

**Rust recipes:**
```bash
cd recipes/your-recipe
cargo test
cargo clippy --all-targets --all-features
cargo fmt --check
```

**XCM recipes:**
```bash
cd recipes/your-recipe
npm test  # Includes Chopsticks setup
```

### Validate Recipe Structure

```bash
# From repository root
```

**This checks:**
- Configuration file validity
- Required files present
- Version keys recognized
- YAML syntax correctness

### Manual Testing

Before submitting, manually test all examples:

1. **Copy code examples** from README
2. **Run them** in a test environment
3. **Verify output** matches documentation
4. **Test error cases** and troubleshooting steps

**Critical:** All code examples must work exactly as documented.

---

## Step 5: Commit Your Changes

### Stage Changes

```bash
# Review changes
git status

# Add specific files
git add recipes/your-recipe/README.md
git add recipes/your-recipe/recipe.config.yml

# Or add all changes
git add .
```

### Write Conventional Commits

All commits must follow [Conventional Commits](https://www.conventionalcommits.org/) format:

```
<type>(<scope>): <description>

[optional body]

[optional footer]
```

**Examples:**
```bash
# New recipe
git commit -m "feat(recipe): add custom pallet tutorial"

# Fix existing recipe
git commit -m "fix(recipe): correct storage example in basic-pallet"

# Update documentation
git commit -m "docs(recipe): add troubleshooting section"

# Breaking change
git commit -m "feat(recipe)!: redesign config format"
```

**Common types:**
- `feat` - New feature (triggers MINOR version bump)
- `fix` - Bug fix (triggers PATCH version bump)
- `docs` - Documentation only (no version bump)
- `test` - Adding tests (no version bump)
- `refactor` - Code refactoring (no version bump)
- `chore` - Maintenance tasks (no version bump)

**See:** [Commit Conventions Guide](commit-conventions.md) for complete documentation.

### Pre-commit Hooks

The repository uses automatic pre-commit hooks that check:

- **Rust formatting** (`cargo fmt --check`)
- **Clippy lints** (`cargo clippy`)
- **Commit message format** (conventional commits)

**If hooks fail:**
```bash
# Fix formatting
cargo fmt --all

# Fix clippy warnings
cargo clippy --fix

# Amend commit if needed
git add .
git commit --amend
```

**See:** [Pre-commit Hooks Guide](../automation/pre-commit-hooks.md)

---

## Step 6: Submit Your Recipe

### Push to Your Fork

```bash
# Push your branch
git push -u origin recipe/your-recipe
```

### Create Pull Request

**Option A: Using the CLI (Recommended)**
```bash
dot recipe submit
```

The CLI will:
1. Validate recipe structure
2. Push to your fork
3. Create PR with template
4. Apply appropriate labels

**Option B: Using GitHub CLI**
```bash
gh pr create --title "feat(recipe): add your recipe" --body "Description of your recipe"
```

**Option C: Using GitHub Web UI**
1. Visit your fork on GitHub
2. Click **Pull Request**
3. Select base: `master` ← compare: `recipe/your-recipe`
4. Fill in PR template
5. Click **Create Pull Request**

### Pull Request Template

Your PR description should include:

```markdown
## Summary
Brief description of what this recipe teaches.

## Type of Change
- [ ] New recipe
- [ ] Recipe improvement
- [ ] Bug fix
- [ ] Documentation update

## Checklist
- [ ] Tests pass locally
- [ ] Documentation is clear
- [ ] Examples work as documented
- [ ] Follows recipe guidelines
- [ ] Uses conventional commits

## Testing
How to test this recipe...
```

---

## Step 7: Respond to Review

### What Happens Next

1. **Automated checks run** - Tests, linting, validation
2. **Semantic label applied** - Based on commit analysis
3. **Maintainers review** - Code quality, documentation clarity
4. **Feedback provided** - Requested changes or approval

### Making Changes

When reviewers request changes:

```bash
# Make requested changes
vim recipes/your-recipe/README.md

# Commit changes
git add .
git commit -m "docs(recipe): address review feedback"

# Push to update PR
git push
```

The PR automatically updates with new commits.

### Responding to Comments

- **Be responsive** - Reply to review comments promptly
- **Ask questions** - If feedback is unclear, ask for clarification
- **Be open** - Accept constructive criticism
- **Explain decisions** - If you disagree, explain your reasoning

### Common Review Feedback

**"Add more explanation"**
```
Action: Expand complex sections with more detail
```

**"Tests are failing"**
```
Action: Fix failing tests and push fixes
```

**"Fix commit messages"**
```bash
# If you need to fix commit history
git rebase -i HEAD~3
# Edit commit messages in editor
git push --force-with-lease
```

**"Update version references"**
```
```

---

## Step 8: After Merge

### Your PR Gets Merged

Congratulations! Your contribution is now part of the cookbook.

**What happens:**
1. **PR merged** to master branch
2. **Automated workflows** run tests
3. **Semantic label** determines version bump
4. **Release scheduled** - Next weekly release or immediate (for breaking changes)

### Keep Your Fork Updated

```bash
# Switch to master
git checkout master

# Pull latest changes
git pull upstream master

# Update your fork
git push origin master

# Delete merged branch (optional)
git branch -d recipe/your-recipe
git push origin --delete recipe/your-recipe
```

### Next Contribution

Ready to contribute again? Start from Step 2!

```bash
# Update master first
git checkout master
git pull upstream master

# Create new recipe
dot recipe create
```

---

## Best Practices

### Before Starting

✅ **DO:**
- Check existing recipes for similar topics
- Read recipe guidelines thoroughly
- Set up your development environment properly
- Update your fork to latest master

❌ **DON'T:**
- Start coding without reading guidelines
- Work directly on master branch
- Skip testing your examples

### During Development

✅ **DO:**
- Test all examples manually
- Write clear, beginner-friendly explanations
- Include comprehensive error handling
- Commit frequently with meaningful messages
- Run tests before committing

❌ **DON'T:**
- Copy-paste code without understanding
- Skip writing tests
- Use hardcoded values
- Leave TODO comments in final code
- Commit untested code

### When Submitting

✅ **DO:**
- Fill out PR template completely
- Wait for CI checks to pass
- Respond to reviews promptly
- Be open to feedback

❌ **DON'T:**
- Submit WIP (work-in-progress) unless marked as draft
- Ignore failed CI checks
- Force-push without `--force-with-lease`
- Merge your own PRs (maintainer responsibility)

---

## Troubleshooting

### Merge Conflicts

**Symptom:** PR shows merge conflicts

**Solution:**
```bash
# Update your branch with latest master
git fetch upstream
git checkout recipe/your-recipe
git merge upstream/master

# Resolve conflicts in your editor
# Look for <<<<<<< markers

# Mark as resolved
git add .
git commit -m "fix: resolve merge conflicts"

# Push updates
git push
```

### CI Tests Failing

**Symptom:** GitHub Actions checks fail

**Solution:**
```bash
# Pull latest changes
git pull

# Run tests locally
npm test  # or cargo test

# Fix issues
# Commit fixes
git add .
git commit -m "fix: resolve test failures"
git push
```

### Fork Out of Sync

**Symptom:** Your fork is behind upstream

**Solution:**
```bash
# Fetch upstream changes
git fetch upstream

# Update master
git checkout master
git merge upstream/master

# Push to your fork
git push origin master
```

### Pre-commit Hook Failures

**Symptom:** Commit rejected by pre-commit hooks

**Solution:**
```bash
# Fix formatting issues
cargo fmt --all

# Fix clippy warnings
cargo clippy --fix --all-targets

# Retry commit
git add .
git commit -m "feat(recipe): your message"
```

---

## Quick Reference

### Essential Commands

```bash
# Setup
git clone https://github.com/YOUR_USERNAME/polkadot-cookbook.git
cd polkadot-cookbook
git remote add upstream https://github.com/polkadot-developers/polkadot-cookbook.git

# Create recipe
dot recipe create

# Test
npm test                 # TypeScript
cargo test              # Rust

# Commit
git add .
git commit -m "feat(recipe): description"

# Submit
git push -u origin branch-name
dot recipe submit

# Update fork
git checkout master
git pull upstream master
git push origin master
```

### Helpful Links

- **[First Recipe Tutorial](../getting-started/first-recipe.md)** - Step-by-step walkthrough
- **[Recipe Guidelines](recipe-guidelines.md)** - Structure and style requirements
- **[Recipe Development Guide](recipe-development.md)** - Best practices
- **[Testing Guide](testing-recipes.md)** - Testing strategies
- **[Commit Conventions](commit-conventions.md)** - Conventional commit format
- **[Troubleshooting FAQ](troubleshooting.md)** - Common issues and solutions

---

## Getting Help

### Resources

- **[GitHub Discussions](https://github.com/polkadot-developers/polkadot-cookbook/discussions)** - Ask questions
- **[Discord Community](https://substrate.io/ecosystem/connect/)** - Real-time chat
- **[GitHub Issues](https://github.com/polkadot-developers/polkadot-cookbook/issues)** - Report bugs
- **[Contributing Guide](../../CONTRIBUTING.md)** - General guidelines

### Maintainer Contact

If you need help from maintainers:
- Tag `@polkadot-developers/maintainers` in GitHub
- Ask in Discord #polkadot-cookbook channel
- Open a discussion on GitHub

---

## Summary

The contribution workflow:

1. ✅ Fork and clone repository
2. ✅ Create recipe with CLI or manual branch
3. ✅ Develop content following guidelines
4. ✅ Test thoroughly (automated and manual)
5. ✅ Commit with conventional format
6. ✅ Submit pull request
7. ✅ Respond to review feedback
8. ✅ Keep fork updated after merge

**Happy contributing!**

---

[← Back to Contributors Guide](README.md)
