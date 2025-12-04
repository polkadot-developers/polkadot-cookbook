# Your First Project

A step-by-step tutorial for creating your first Polkadot project and submitting it as a recipe to the cookbook.

## What You'll Learn

By the end of this tutorial, you'll know how to:
- Create a new project using the CLI
- Understand the generated file structure
- Run and test your project
- Submit your project as a recipe to the cookbook

## Prerequisites

Before starting, ensure you have:

1. **CLI Installed** - See [Installation Guide](installation.md)
2. **Git Configured** - Name and email set
3. **GitHub CLI** - For submitting your project: `gh auth login`
4. **Development Tools** - Rust (for parachain projects) or Node.js (for other projects)

**Verify your setup:**
```bash
# Check CLI is installed
dot --version

# Check git configuration
git config user.name
git config user.email

# Check GitHub CLI
gh auth status
```

---

## Step 1: Create Your Project

The CLI provides an interactive mode that guides you through project creation.

### Run the Create Command

```bash
dot create
```

### Interactive Prompts

**1. Select Pathway**
```
? What would you like to build?
  ❯ Smart Contract (Solidity)
    Parachain (Polkadot SDK)
    Chain Transactions
    Cross-Chain Transactions (XCM)
    Polkadot Networks (Zombienet / Chopsticks)
    None of these - Request new template
```

For this tutorial, select **Parachain (Polkadot SDK)**.

**2. Enter Project Name**
```
? What is your project name? (e.g., 'my-parachain')
```

Enter your project name. This will be used to generate the project directory (e.g., "my-first-parachain" → `./my-first-parachain/`).

### What Happens Next

The CLI will:
1. Check dependencies (Rust, Node.js, etc.)
2. Create the project directory: `./my-first-parachain/`
3. Generate scaffolded files from the polkadot-sdk-parachain-template
4. Install npm dependencies (for PAPI testing)
5. Run tests to verify the setup
6. Initialize a git repository

**Output:**
```
✅ Project created successfully!
✅ Tests passed!

📦 Project Created
Location: ./my-first-parachain

📝 Next Steps
1. Build your parachain
   → cd ./my-first-parachain && cargo build --release

2. Start development node
   → ./my-first-parachain/scripts/start-dev-node.sh

3. Run integration tests
   → cd ./my-first-parachain && npm test

🎉 All set! To get started:

   cd ./my-first-parachain
```

---

## Step 2: Explore the Generated Files

Navigate to your new project directory:

```bash
cd my-first-parachain
```

### File Structure

**Full Parachain Project:**
```
my-first-parachain/
├── README.md              # Tutorial documentation
├── Cargo.toml             # Workspace configuration
├── rust-toolchain.toml    # Rust version (e.g., 1.86)
├── package.json           # PAPI dependencies
├── pallets/               # Custom FRAME pallets
│   └── template/
│       ├── src/
│       │   ├── lib.rs     # Your pallet logic
│       │   ├── tests.rs   # Unit tests (mock runtime)
│       │   └── benchmarking.rs
│       └── Cargo.toml
├── runtime/               # Parachain runtime
│   ├── src/lib.rs         # Runtime configuration
│   ├── Cargo.toml
│   └── build.rs
├── tests/                 # PAPI integration tests
│   └── template-pallet.test.ts
├── scripts/
│   ├── setup-zombienet-binaries.sh  # Setup zombienet binaries
│   └── start-dev-node.sh            # Start development node
├── dev_chain_spec.json    # Development chain specification
├── zombienet.toml         # Parachain node network config
└── zombienet-omni-node.toml  # Omni-node network config
```

### Key Files Explained

#### `README.md`
This is the **main recipe content**. It's pre-filled with a comprehensive tutorial covering:
- Overview and prerequisites
- What you'll learn
- Project structure
- Quick start guide
- Development workflow
- PAPI testing
- Zombienet multi-chain testing

**Your task:** Customize this for your specific use case.

#### `pallets/template/src/lib.rs`
Your custom pallet implementation:

```rust
#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
    }

    #[pallet::storage]
    pub type Something<T> = StorageValue<_, u32>;

    // Add your storage items, events, errors, and dispatchable functions
}
```

#### `runtime/src/lib.rs`
Runtime configuration with 12+ essential pallets:
- System pallets (frame-system, cumulus-pallet-parachain-system)
- Consensus (pallet-aura)
- Accounts & tokens (pallet-balances, pallet-transaction-payment)
- Governance (pallet-sudo - dev only)
- Your custom pallet (pallet-template)

#### `tests/template-pallet.test.ts`
PAPI integration tests:

```typescript
import { createClient } from 'polkadot-api';
import { getWsProvider } from 'polkadot-api/ws-provider/node';

// Tests run against a live node
describe('Template Pallet', () => {
  it('should query template pallet storage', async () => {
    const something = await api.query.TemplatePallet.Something.getValue();
    expect(something).toBeDefined();
  });
});
```

---

## Step 3: Customize Your Project

### Update Your Pallet

Open `pallets/template/src/lib.rs` and customize the pallet:

```rust
#[pallet::storage]
pub type MyCounter<T> = StorageValue<_, u32, ValueQuery>;

#[pallet::call]
impl<T: Config> Pallet<T> {
    pub fn increment(origin: OriginFor<T>) -> DispatchResult {
        let who = ensure_signed(origin)?;

        MyCounter::<T>::mutate(|count| {
            *count = count.saturating_add(1);
        });

        Self::deposit_event(Event::CounterIncremented {
            value: MyCounter::<T>::get(),
            who
        });

        Ok(())
    }
}

#[pallet::event]
#[pallet::generate_deposit(pub(super) fn deposit_event)]
pub enum Event<T: Config> {
    CounterIncremented { value: u32, who: T::AccountId },
}
```

### Update the README

Customize the tutorial in `README.md`:

```markdown
# My First Parachain

> Learn to build a counter pallet with PAPI integration testing.

## What You'll Build

A simple counter pallet that:
- Stores a counter value on-chain
- Increments the counter via extrinsic
- Emits events on counter updates
- Includes comprehensive TypeScript tests

[Continue with your specific content...]
```

**Tips for good recipe content:**
- Start with clear learning objectives
- Break down complex tasks into small steps
- Include code examples with explanations
- Show expected output
- Add troubleshooting for common issues

---

## Step 4: Build and Test

### 1. Run Unit Tests

Test your pallet logic with the mock runtime:

```bash
cargo test --package pallet-my-first-parachain
```

**Expected output:**
```
running 3 tests
test tests::it_works_for_default_value ... ok
test tests::increment_works ... ok
test tests::multiple_increments_work ... ok

test result: ok. 3 passed; 0 failed; 0 ignored
```

### 2. Build the Runtime

Compile your runtime to WebAssembly:

```bash
npm run build:runtime
# or: cargo build --release
```

This takes 10-15 minutes on first build. The runtime WASM will be at:
```
target/release/wbuild/my-first-parachain-runtime/my_first_parachain_runtime.compact.compressed.wasm
```

### 3. Generate Chain Specification

Create a chain specification from your runtime:

```bash
npm run generate:spec
```

This creates `chain-spec.json` with your genesis state.

### 4. Start Development Node

Launch your parachain locally:

```bash
npm run start:node
```

The node will:
- Start in development mode
- Expose RPC at `ws://localhost:9944`
- Use Alice as the sudo account

### 5. Run PAPI Integration Tests

In a new terminal, run the test suite:

```bash
npm test
```

**Expected output:**
```
✓ tests/template-pallet.test.ts
  ✓ should connect to the chain
  ✓ should query template pallet storage
  ✓ should increment counter

Test Files  1 passed (1)
Tests  3 passed (3)
```

### Troubleshooting

**Cargo build fails:**
```bash
# Clear cache and rebuild
cargo clean
cargo build --release
```

**Node won't start:**
```bash
# Kill existing node
pkill -f polkadot-omni-node

# Ensure polkadot-omni-node is installed
cargo install polkadot-omni-node
```

**PAPI tests fail:**
```bash
# Ensure node is running
npm run start:node

# In another terminal, regenerate types
npm run generate:types

# Run tests
npm test
```

---

## Step 5: Test with Zombienet (Optional)

Your project includes zombienet configurations for testing in a multi-node environment.

### Setup Binaries (One-time)

```bash
npm run setup:zombienet
```

This installs:
- `polkadot` (relay chain)
- `polkadot-omni-node` (parachain collator)

### Launch Network with Omni Node (Recommended)

```bash
npm run zombienet:omni
```

This spawns:
- 2 relay chain validators (Alice, Bob)
- 1 parachain collator running your runtime with polkadot-omni-node

### Or Use Custom Parachain Node (Advanced)

```bash
npm run zombienet:node
```

This uses your custom-built parachain node binary instead of the omni node.

---

## Step 6: Commit Your Changes

Your project was created on a git branch. Commit your changes:

```bash
# Review changes
git status

# Add all files
git add .

# Commit with conventional commit format
git commit -m "feat(recipe): add my first parachain tutorial"
```

**Important:** Use [conventional commit format](../contributors/commit-conventions.md):
```
feat(recipe): <description>
```

---

## Step 7: Submit Your Project

Once your project is complete and tested, submit it for review:

```bash
dot submit my-first-parachain
```

### What the Submit Command Does

1. **Runs tests** to validate your code
2. **Validates** that required lock files are present (Cargo.lock and/or package-lock.json)
3. **Validates** recipe structure
4. **Checks** git repository state
5. **Pushes** to your fork (creates fork if needed)
6. **Creates** pull request on GitHub

### Pull Request Checklist

Your PR will be reviewed for:
- ✅ Code quality and correctness
- ✅ Documentation clarity
- ✅ Test coverage
- ✅ Working examples
- ✅ Lock files committed (Cargo.lock and/or package-lock.json)
- ✅ Adherence to guidelines

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
- Include troubleshooting

**"Tests are failing"**
- Run tests locally: `npm test`
- Fix failing tests
- Commit and push fixes

**"Fix formatting"**
- Run `cargo fmt` (Rust)
- Check markdown formatting

---

## Other Recipe Types

### Smart Contract (Solidity)

```bash
dot create
# Select: Smart Contract (Solidity)
# Title: My First Contract
```

**Generated structure:**
```
my-first-contract/
├── README.md
├── package.json
├── hardhat.config.ts
├── contracts/          # Solidity contracts
├── tests/              # Contract tests
└── scripts/            # Deployment scripts
```

**Quick start:**
```bash
cd my-first-contract
npm run compile      # Compile contracts
npm test            # Run tests
```

### Basic Interaction (PAPI)

```bash
dot create
# Select: Basic Interaction
# Title: Query Chain State
```

**Generated structure:**
```
query-chain-state/
├── README.md
├── package.json
├── src/               # Implementation
└── tests/             # Tests
```

**Quick start:**
```bash
cd query-chain-state
npm test
```

---

## Advanced: Pallet-Only Mode

For advanced users building just a pallet (no runtime):

```bash
dot create --title "My Pallet" --pathway pallets --pallet-only --non-interactive
```

**Generated structure:**
```
my-pallet/
├── README.md
├── Cargo.toml
├── rust-toolchain.toml
└── pallets/
    └── template/
        └── src/
            ├── lib.rs
            ├── mock.rs    # Mock runtime for testing
            └── tests.rs   # Unit tests
```

**Testing:**
```bash
cd my-pallet
cargo test
```

**Note:** Pallet-only mode excludes runtime, node, and PAPI tests.

---

## Next Steps

Congratulations! You've created your first project. Here's what to explore next:

### Learn More

- **[CLI Reference](../developers/cli-reference.md)** - All CLI commands
- **[Contributing Guide](../../CONTRIBUTING.md)** - Contribution workflow
- **[Architecture](../developers/architecture.md)** - How the cookbook works

### Explore Examples

- **[parachain-example](../../recipes/parachain-example/)** - Full parachain with XCM
- **[contracts-example](../../recipes/contracts-example/)** - Solidity contracts
- **[transaction-example](../../recipes/transaction-example/)** - PAPI interactions
- **[cross-chain-transaction-example](../../recipes/cross-chain-transactions/cross-chain-transaction-example/)** - Cross-chain messaging

### Contribute More

- **[Recipe Guidelines](../contributors/recipe-guidelines.md)** - Style and structure
- **[Testing Recipes](../contributors/testing-recipes.md)** - Testing strategies
- **[Commit Conventions](../contributors/commit-conventions.md)** - Commit format

---

## Troubleshooting

### Project Creation Failed

**Symptom:** `dot create` fails with error

**Common causes:**
- Git not configured
- Missing dependencies (Rust, Node.js)
- Insufficient disk space

**Solution:**
```bash
# Check git configuration
git config user.name
git config user.email

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Node.js
# Visit https://nodejs.org/
```

### Runtime Build Fails

**Symptom:** `cargo build --release` fails

**Common causes:**
- Rust version mismatch
- Dependency conflicts
- Out of memory

**Solution:**
```bash
# Use correct Rust version
rustup update

# Build with fewer parallel jobs (if memory limited)
cargo build --release -j 1

# Clear cache and retry
cargo clean
cargo build --release
```

### PAPI Tests Fail

**Symptom:** `npm test` fails

**Common causes:**
- Node not running
- Wrong metadata
- Port already in use

**Solution:**
```bash
# Ensure node is running
pkill -f polkadot-omni-node
npm run start:node

# Regenerate types
npm run generate:types

# Run tests
npm test
```

### GitHub Authentication Failed

**Symptom:** `dot submit` fails with auth error

**Solution:**
```bash
# Check authentication
gh auth status

# Login if needed
gh auth login

# Verify token
gh auth token
```

---

## Summary

You've learned how to:
- ✅ Create a project with `dot create`
- ✅ Understand the generated file structure
- ✅ Build and test a parachain
- ✅ Run PAPI integration tests
- ✅ Submit a pull request

**Ready for more?** Check out the [Recipe Development Guide](../contributors/recipe-development.md) for best practices and advanced techniques.

---

[← Back to Getting Started](README.md)
