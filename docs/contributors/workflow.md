# Contributor Workflow

Step-by-step guide to contributing recipes to the Polkadot Cookbook.

## Overview

Recipes live in external GitHub repositories. The cookbook contains **test harnesses** that clone, build, and verify each recipe. Contributing a recipe means adding a test harness that points to your external repo.

The contribution workflow follows these steps:

1. **Setup** - Fork, clone, and configure your environment
2. **Create** - Add a test harness directory for your recipe
3. **Develop** - Build your recipe in your own external repository
4. **Test** - Run the test harness locally
5. **Commit** - Use conventional commit messages
6. **Submit** - Push and create a pull request
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

## Step 2: Create a Test Harness (Your Cookbook Contribution)

Each recipe in the cookbook is a test harness that clones and verifies an external repository. To add a new recipe, create a test harness directory.

### Create a Branch

```bash
# Update master first
git checkout master
git pull upstream master

# Create a branch for your recipe
git checkout -b recipe/my-recipe-name
```

### Set Up the Test Harness Directory

Create the standard test harness structure under `recipes/{pathway}/{your-recipe}/`. Use an existing recipe as a template — for example, copy from [`recipes/contracts/contracts-example/`](../../recipes/contracts/contracts-example/):

```
recipes/{pathway}/{your-recipe}/
├── package.json           # vitest + @types/node + typescript
├── package-lock.json      # Locked dependencies (run npm install to generate)
├── vitest.config.ts       # Vitest config
├── tsconfig.json          # TypeScript config
├── .gitignore             # Ignore cloned repo dir, node_modules
├── README.md              # Description + link to external repo
└── tests/
    └── recipe.test.ts     # Clone → install → build → test
```

### What Goes in `tests/recipe.test.ts`

The test file clones your external repo at a pinned version, installs dependencies, builds, and runs tests:

```typescript
import { describe, it, expect } from "vitest";
import { execSync } from "child_process";
import { existsSync } from "fs";
import { join } from "path";

const PROJECT_DIR = process.cwd();
const REPO_URL = "https://github.com/YOUR_USERNAME/recipe-your-recipe";
const REPO_VERSION = "v1.0.0";
const REPO_DIR = join(PROJECT_DIR, "recipe-your-recipe");

describe("Your Recipe", () => {
  it("should clone the repository", () => {
    if (!existsSync(REPO_DIR)) {
      execSync(`git clone --branch ${REPO_VERSION} ${REPO_URL}`, {
        cwd: PROJECT_DIR, encoding: "utf-8", stdio: "inherit",
      });
    }
    expect(existsSync(REPO_DIR)).toBe(true);
  }, 120000);

  it("should install dependencies", () => {
    execSync("npm ci", { cwd: REPO_DIR, stdio: "inherit" });
    expect(existsSync(join(REPO_DIR, "node_modules"))).toBe(true);
  }, 120000);

  it("should build", () => {
    execSync("npm run build", { cwd: REPO_DIR, stdio: "inherit" });
  }, 120000);

  it("should pass tests", () => {
    execSync("npm test", { cwd: REPO_DIR, stdio: "inherit" });
  }, 120000);
});
```

### README.md Frontmatter

Include metadata at the top of your test harness README:

```markdown
---
title: "Your Recipe Title"
description: "Brief description of what the recipe does"
source_repo: "https://github.com/YOUR_USERNAME/recipe-your-recipe"
---
```

**Branch naming conventions:**
- `recipe/<slug>` - New recipes
- `feat/<description>` - New features or enhancements
- `fix/<description>` - Bug fixes
- `docs/<description>` - Documentation improvements
- `refactor/<description>` - Code refactoring

---

## Step 3: Develop Your Recipe (In Your Own Repository)

### Recipe Code Lives in Your Own Repository

The actual recipe code (source, tests, dependencies) lives in your own external GitHub repository — not inside the cookbook. You can use `dot create` to scaffold a project locally:

```bash
dot create
```

Develop and test your project in this standalone directory. When it's ready, push it to your own GitHub repository and tag a release:

```bash
git tag v1.0.0
git push --tags
```

### What You Contribute to the Cookbook

Your cookbook contribution is the **test harness** — a lightweight directory under `recipes/` that verifies your external repo works correctly. The test harness does not contain recipe source code; it only clones, builds, and tests your external repository.

See [Recipe Guidelines](recipe-guidelines.md) for quality standards.


---

## Step 4: Test Your Test Harness

### Run the Test Harness Locally

From the cookbook repository root:

```bash
cd recipes/{pathway}/{your-recipe}
npm ci
npm test
```

This will:
1. Clone your external repository at the pinned version tag
2. Install the recipe's dependencies
3. Build the project
4. Run the recipe's test suite

### Verify Everything Passes

Before submitting, ensure:
- The test harness clones and builds successfully
- All tests in the external repo pass
- The `README.md` links to the correct source repository
- The version tag in `recipe.test.ts` matches an existing tag in your external repo

---

## Step 5: Commit Your Changes

### Stage Changes

```bash
# Review changes
git status

# Add specific files
git add recipes/{pathway}/your-recipe/

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

## Step 6: Submit Your Pull Request

### Push to Your Fork

```bash
# Push your branch
git push -u origin your-branch-name
```

### Create Pull Request

**Option A: Using GitHub CLI**
```bash
gh pr create --title "feat(recipe): add your project" --body "Description of your project"
```

**Option B: Using GitHub Web UI**
1. Visit your fork on GitHub
2. Click **Pull Request**
3. Select base: `master` <- compare: `your-branch-name`
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

# Create new project
dot create
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

# Scaffold a new project (in your own repo)
dot create

# Test your test harness
cd recipes/{pathway}/{your-recipe}
npm ci && npm test

# Commit
git add .
git commit -m "feat(recipe): add my-recipe test harness"

# Submit
git push -u origin branch-name
gh pr create --title "feat(recipe): add my-recipe" --body "Description"

# Update fork
git checkout master
git pull upstream master
git push origin master
```

### Helpful Links

- **[First Project Tutorial](../getting-started/first-project.md)** - Step-by-step walkthrough
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

1. Fork and clone the cookbook repository
2. Develop your recipe in your own external repository
3. Add a test harness under `recipes/{pathway}/{your-recipe}/`
4. Test the harness locally (`npm ci && npm test`)
5. Commit with conventional format
6. Push and create a pull request
7. Respond to review feedback
8. Keep fork updated after merge

**Happy contributing!**

---

[← Back to Contributors Guide](README.md)
