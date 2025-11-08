# Troubleshooting FAQ

Common issues and solutions when developing Polkadot Cookbook recipes.

## Table of Contents

- [Setup and Installation](#setup-and-installation)
- [Recipe Creation](#recipe-creation)
- [Development Issues](#development-issues)
- [Testing Problems](#testing-problems)
- [Git and GitHub](#git-and-github)
- [CI/CD Failures](#cicd-failures)
- [Dependencies](#dependencies)

---

## Setup and Installation

### CLI Not Found After Installation

**Symptom:**
```bash
$ dot --version
bash: dot: command not found
```

**Causes:**
- Binary not in PATH
- Installation incomplete
- Binary not executable

**Solutions:**

```bash
# Check if dot exists
which dot

# If installed via cargo, add to PATH
export PATH="$PATH:$HOME/.cargo/bin"

# If built from source
export PATH="$PATH:/path/to/polkadot-cookbook/target/release"

# Make executable
chmod +x /path/to/dot

# Verify
dot --version
```

**Permanent fix (add to ~/.bashrc or ~/.zshrc):**
```bash
export PATH="$PATH:$HOME/.cargo/bin"
```

---

### Setup Command Shows Missing Tools

**Symptom:**
```bash
$ dot setup
❌ Rust toolchain: Not found
❌ Node.js: Not found
```

**Solutions:**

**Install Rust:**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
rustup update stable
```

**Install Node.js:**
```bash
# Using nvm (recommended)
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash
nvm install 20
nvm use 20

# Or download from nodejs.org
```

**Install Git:**
```bash
# macOS
xcode-select --install

# Ubuntu/Debian
sudo apt-get install git

# Fedora
sudo dnf install git
```

**Verify installation:**
```bash
dot setup
```

---

### GitHub CLI Authentication Fails

**Symptom:**
```bash
$ dot recipe submit
Error: GitHub CLI not authenticated
```

**Solution:**
```bash
# Check auth status
gh auth status

# Login
gh auth login

# Follow prompts:
# 1. Select: GitHub.com
# 2. Select: HTTPS
# 3. Authenticate via web browser

# Verify
gh auth status
```

---

## Recipe Creation

### Recipe Create Fails with "Directory Already Exists"

**Symptom:**
```bash
$ dot recipe create
Error: Recipe directory already exists: recipes/my-recipe
```

**Cause:** Recipe with same name already exists

**Solutions:**

```bash
# Option 1: Use different name
dot recipe create
# Enter different title

# Option 2: Remove existing directory
rm -rf recipes/my-recipe
dot recipe create

# Option 3: Continue working on existing recipe
cd recipes/my-recipe
```

---

### npm install Fails During Recipe Creation

**Symptom:**
```
Installing dependencies...
Error: npm install failed
```

**Causes:**
- Network issues
- npm not installed
- Incompatible Node.js version

**Solutions:**

```bash
# Check Node.js version
node --version  # Should be 18+ or 20+

# Update npm
npm install -g npm@latest

# Clear npm cache
npm cache clean --force

# Try manual installation
cd recipes/my-recipe
npm install

# Use --skip-install flag
dot recipe create --skip-install
cd recipes/my-recipe
npm install
```

---

### Git Branch Not Created

**Symptom:**
Recipe created but no git branch

**Cause:** Not in a git repository

**Solution:**
```bash
# Initialize git if needed
git init

# Or use --no-git flag
dot recipe create --no-git
```

---

## Development Issues

### Rust Compilation Errors

**Symptom:**
```bash
error[E0412]: cannot find type `T` in this scope
```

**Common causes and solutions:**

**Missing trait bounds:**
```rust
// ❌ Error
pub fn do_something<T>() { }

// ✅ Fixed
pub fn do_something<T: Config>() { }
```

**Missing imports:**
```rust
// ❌ Error
pub type MyStorage<T> = StorageValue<_, u32>;

// ✅ Fixed
use frame_support::pallet_prelude::*;
pub type MyStorage<T> = StorageValue<_, u32>;
```

**Generic parameter mismatch:**
```rust
// ❌ Error
impl<T> Pallet<T> { }

// ✅ Fixed
impl<T: Config> Pallet<T> { }
```

**General debugging:**
```bash
# Build with detailed errors
cargo build --verbose

# Check specific package
cargo check -p pallet-mymodule

# Expand macros to see generated code
cargo expand
```

---

### TypeScript Type Errors

**Symptom:**
```
error TS2345: Argument of type 'string' is not assignable to parameter of type 'BN'
```

**Solutions:**

```typescript
// ❌ Wrong
const amount = "1000";

// ✅ Correct
import { BN } from '@polkadot/util';
const amount = new BN(1000);

// Or for large numbers
const amount = 1_000_000_000_000n;  // BigInt
```

**Common fixes:**

```bash
# Check TypeScript version
npx tsc --version

# Install type definitions
npm install --save-dev @types/node

# Run type check
npx tsc --noEmit

# Fix with specific config
# Edit tsconfig.json
{
  "compilerOptions": {
    "strict": true,
    "esModuleInterop": true
  }
}
```

---

### Clippy Warnings

**Symptom:**
```bash
warning: this expression creates a reference which is immediately dereferenced
  --> src/lib.rs:42:20
```

**Solutions:**

```bash
# See all warnings
cargo clippy --all-targets --all-features

# Auto-fix where possible
cargo clippy --fix

# Suppress specific warning (use sparingly)
#[allow(clippy::needless_borrow)]

# Fix common issues:
```

**Common clippy fixes:**

```rust
// Needless borrow
// ❌ Warning
let value = calculate(&data);
// ✅ Fixed
let value = calculate(data);

// Unnecessary clone
// ❌ Warning
let copy = value.clone();
return copy;
// ✅ Fixed
return value;

// Redundant pattern matching
// ❌ Warning
if let Some(x) = maybe_value {
    Some(x)
} else {
    None
}
// ✅ Fixed
maybe_value
```

---

### Format Check Failures

**Symptom:**
```bash
error: rustfmt check failed
```

**Solution:**
```bash
# Format all code
cargo fmt --all

# Check what would change
cargo fmt --all -- --check

# Format specific file
rustfmt src/lib.rs
```

---

## Testing Problems

### Tests Fail with "Connection Refused"

**Symptom:**
```
Error: Connection refused (os error 111)
```

**Cause:** Node not running

**Solution:**

**For local development node:**
```bash
# Start development node
polkadot-omni-node --dev

# In another terminal, run tests
npm test
```

**For Chopsticks (XCM tests):**
```bash
# Start Chopsticks
npx @acala-network/chopsticks -c chopsticks.yml &

# Wait for initialization
sleep 10

# Run tests
npm test

# Cleanup
pkill -f chopsticks
```

**For mock provider (unit tests):**
```typescript
// Use mock provider instead of real connection
import { WsProvider } from '@polkadot/api';
import { MockProvider } from '@polkadot/api/test';

// For unit tests
const provider = new MockProvider();

// For integration tests
const provider = new WsProvider('ws://localhost:9944');
```

---

### Tests Timeout

**Symptom:**
```
Error: Test timeout exceeded (30000ms)
```

**Solutions:**

```typescript
// Increase timeout for specific test
it('long running test', async () => {
  // test code
}, 60000);  // 60 seconds

// Increase timeout in vitest.config.ts
export default defineConfig({
  test: {
    testTimeout: 60000,  // 60 seconds
  },
});

// Add timeout to beforeAll
beforeAll(async () => {
  // setup code
}, 60000);
```

---

### Flaky Tests

**Symptom:** Tests pass sometimes but fail randomly

**Common causes:**

**Race conditions:**
```typescript
// ❌ Flaky
it('should process transaction', async () => {
  await api.tx.transfer(...).signAndSend(alice);
  const balance = await api.query.system.account(bob);
  expect(balance).toBe(expected);  // May not be updated yet
});

// ✅ Fixed
it('should process transaction', async () => {
  await new Promise<void>((resolve) => {
    api.tx.transfer(...).signAndSend(alice, ({ status }) => {
      if (status.isInBlock) {
        resolve();
      }
    });
  });

  const balance = await api.query.system.account(bob);
  expect(balance).toBe(expected);
});
```

**Timing dependencies:**
```typescript
// ❌ Flaky
await sendTransaction();
await wait(1000);  // Arbitrary wait
verify();

// ✅ Fixed
await sendTransaction();
await waitForEvent('TransactionComplete');
verify();
```

**Solution: Add retries for flaky tests:**
```typescript
import { test } from 'vitest';

test.retry(3)('flaky test', async () => {
  // test code
});
```

---

### Test Isolation Issues

**Symptom:** Tests fail when run together but pass individually

**Cause:** Shared state between tests

**Solution:**

```typescript
// ✅ Reset state between tests
beforeEach(async () => {
  // Create fresh instances
  api = await ApiPromise.create({ provider });
  keyring = new Keyring({ type: 'sr25519' });
});

afterEach(async () => {
  // Cleanup
  await api?.disconnect();
});
```

```rust
// ✅ Rust: Use fresh test environment
#[test]
fn test_one() {
    new_test_ext().execute_with(|| {
        // Test code - fresh state
    });
}

#[test]
fn test_two() {
    new_test_ext().execute_with(|| {
        // Test code - fresh state
    });
}
```

---

## Git and GitHub

### Merge Conflicts

**Symptom:**
```bash
$ git merge origin/master
CONFLICT (content): Merge conflict in recipes/my-recipe/README.md
```

**Solution:**

```bash
# 1. Identify conflicts
git status

# 2. Open conflicting files
# Look for conflict markers:
# <<<<<<< HEAD
# Your changes
# =======
# Their changes
# >>>>>>> origin/master

# 3. Resolve conflicts manually
vim recipes/my-recipe/README.md

# 4. Mark as resolved
git add recipes/my-recipe/README.md

# 5. Complete merge
git commit

# 6. Push
git push
```

**Prevent conflicts:**
```bash
# Keep your branch updated
git fetch upstream
git rebase upstream/master

# Or merge frequently
git merge upstream/master
```

---

### Pre-commit Hook Failures

**Symptom:**
```bash
$ git commit -m "my change"
Running pre-commit hooks...
❌ cargo fmt check failed
```

**Solution:**

```bash
# Fix formatting
cargo fmt --all

# Fix clippy issues
cargo clippy --fix --all-targets

# Try commit again
git commit -m "my change"

# Or skip hooks (not recommended)
git commit --no-verify -m "my change"
```

---

### Push Rejected

**Symptom:**
```bash
$ git push
! [rejected]  recipe/my-recipe -> recipe/my-recipe (non-fast-forward)
```

**Causes:**
- Remote branch has commits you don't have
- Force-pushed to shared branch

**Solutions:**

```bash
# Pull and merge
git pull --rebase

# Resolve any conflicts
# Then push
git push

# Or force push (only if you're sure)
git push --force-with-lease
```

---

### PR Creation Fails

**Symptom:**
```bash
$ dot recipe submit
Error: Failed to create pull request
```

**Solutions:**

```bash
# Check gh auth
gh auth status

# Check git status
git status  # Ensure changes are committed

# Check remote
git remote -v  # Ensure fork exists

# Manual PR creation
git push -u origin recipe/my-recipe
gh pr create --title "feat(recipe): my recipe" --body "Description"
```

---

## CI/CD Failures

### GitHub Actions Tests Fail

**Symptom:** Tests pass locally but fail in CI

**Common causes:**

**Environment differences:**
```yaml
# Check CI environment
runs-on: ubuntu-latest  # Different from your macOS

# Match versions
- uses: actions-rs/toolchain@v1
  with:
    toolchain: 1.86  # Ensure same as local
```

**Missing dependencies:**
```yaml
# Add missing system dependencies
- name: Install dependencies
  run: |
    sudo apt-get update
    sudo apt-get install -y build-essential pkg-config libssl-dev
```

**Timing issues:**
```typescript
// Increase timeouts in CI
const timeout = process.env.CI ? 60000 : 30000;
```

---

### Coverage Check Fails

**Symptom:**
```
Error: Coverage 75% below threshold 80%
```

**Solution:**

```bash
# Check coverage locally
cargo tarpaulin --out Html
# Open tarpaulin-report.html to see uncovered lines

# Add tests for uncovered code
# Rerun coverage
cargo tarpaulin

# If coverage requirement is too high for your PR
# Comment in PR asking maintainers to review threshold
```

---

### Semantic Label Wrong

**Symptom:** PR has wrong `semantic:major/minor/patch` label

**Causes:**
- Commits don't follow conventional format
- Breaking change not marked

**Solutions:**

```bash
# Fix commit messages
git rebase -i HEAD~3
# Change commits to follow conventional format

# Mark breaking changes
git commit --amend -m "feat(recipe)!: breaking change"

# Push changes
git push --force-with-lease

# Or manually change label in GitHub UI
```

---

## Dependencies

### Version Mismatch

**Symptom:**
```
error: package `polkadot-sdk v1.15.0` cannot be built because it requires rustc 1.86 or newer
```

**Solution:**

```bash
# Check current version
rustc --version

# Update Rust
rustup update stable

# Or install specific version
rustup install 1.86
rustup default 1.86

# Verify
rustc --version
```

---

### Dependency Resolution Fails

**Symptom:**
```bash
$ cargo build
error: failed to resolve patches for `https://github.com/paritytech/polkadot-sdk`
```

**Solutions:**

```bash
# Update dependencies
cargo update

# Clear cache
cargo clean
rm -rf ~/.cargo/registry
rm -rf ~/.cargo/git

# Rebuild
cargo build

# Check Cargo.lock
git status Cargo.lock  # Ensure it's committed
```

---

### npm Dependency Conflicts

**Symptom:**
```
npm ERR! ERESOLVE unable to resolve dependency tree
```

**Solutions:**

```bash
# Try legacy peer deps
npm install --legacy-peer-deps

# Or force
npm install --force

# Clear cache
npm cache clean --force
rm -rf node_modules package-lock.json
npm install

# Check Node version
node --version  # Ensure 18+ or 20+
nvm use 20
```

---

## Getting Help

If you've tried these solutions and still have issues:

### 1. Search Existing Issues

```bash
# Search GitHub issues
gh issue list --search "your error message"
```

### 2. Check Documentation

- [Getting Started Guide](../getting-started/)
- [Recipe Development Guide](recipe-development.md)
- [Testing Guide](testing-recipes.md)
- [Workflows Guide](../maintainers/workflows.md)

### 3. Ask the Community

- [GitHub Discussions](https://github.com/polkadot-developers/polkadot-cookbook/discussions)
- [Discord Community](https://substrate.io/ecosystem/connect/)

### 4. Report a Bug

```bash
# Create detailed issue
gh issue create --title "Bug: description" --body "
## Description
What went wrong...

## Steps to Reproduce
1. Step one
2. Step two

## Expected Behavior
What should happen...

## Actual Behavior
What actually happens...

## Environment
- OS: macOS 13.0
- Rust: 1.86.0
- Node: 20.10.0
- CLI: 0.2.0

## Logs
[Paste relevant error logs]
"
```

---

## Related Documentation

- **[Contributor Workflow](workflow.md)** - Contribution process
- **[Recipe Development](recipe-development.md)** - Development best practices
- **[Testing Guide](testing-recipes.md)** - Testing strategies
- **[CLI Reference](../developers/cli-reference.md)** - CLI commands
- **[Workflows Guide](../maintainers/workflows.md)** - CI/CD details

---

[← Back to Contributors Guide](README.md)
