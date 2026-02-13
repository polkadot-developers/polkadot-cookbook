---
layout: doc
title: "Your First Project"
---

# Your First Project

A step-by-step tutorial for creating your first Polkadot project.

## What You'll Learn

By the end of this tutorial, you'll know how to:
- Create a new project using the CLI
- Understand the generated file structure
- Run and test your project

## Prerequisites

Before starting, ensure you have:

1. **CLI Installed** - See [Installation Guide](installation.md)
2. **Git Installed** - Required for cloning upstream templates during project creation
3. **Git Configured** - Name and email set
4. **Network Access** - Required on first run to clone the upstream template (cached for subsequent runs)
5. **Development Tools** - Rust (for parachain projects) or Node.js (for other projects)

**Verify your setup:**
```bash
# Check CLI is installed
dot --version

# Check git configuration
git config user.name
git config user.email
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
3. Clone the upstream `polkadot-sdk-parachain-template` (cached after first run)
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
├── rust-toolchain.toml    # Rust version (e.g., 1.88)
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

### 3. Start Development Node

Launch your parachain locally:

```bash
npm run start:node
```

The node will:
- Start in development mode using `dev_chain_spec.json`
- Expose RPC at `ws://localhost:9944`
- Use Alice as the sudo account

### 4. Run PAPI Integration Tests

In a new terminal, run the test suite:

```bash
npm test
```

**What happens:** The test command automatically generates TypeScript types from your running node before running tests.

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

# In another terminal, run tests (types are auto-generated)
npm test

# If you need to manually regenerate types
npm run generate:types
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

## Step 6: Commit and Push to Your Repository

The project created by `dot create` is a **standalone project** that lives in your own repository, not inside the cookbook.

```bash
# Review changes
git status

# Add all files
git add .

# Commit with conventional commit format
git commit -m "feat: add my first parachain"

# Push to your own repository
git remote add origin https://github.com/YOUR_USERNAME/recipe-my-first-parachain.git
git push -u origin main

# Tag a release
git tag v1.0.0
git push --tags
```

---

## Step 7: Contribute to the Cookbook

To share your recipe with the community, add a **test harness** to the cookbook that verifies your external repository. See the [Contributing Guide](https://github.com/polkadot-developers/polkadot-cookbook/blob/master/CONTRIBUTING.md) for the full workflow:

1. Fork the cookbook repository
2. Add a test harness under `recipes/{pathway}/{your-recipe}/`
3. Test locally: `cd recipes/{pathway}/{your-recipe} && npm ci && npm test`
4. Open a pull request

Maintainers will review your PR. When they request changes:

```bash
# Make requested changes to the test harness
# Commit and push
git add .
git commit -m "fix(recipe): update version tag"
git push
```

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

### Chain Transactions (PAPI)

```bash
dot create
# Select: Chain Transactions
# Title: Query Chain State
```

**Generated structure:**
```
query-chain-state/
├── README.md
├── package.json
├── tsconfig.json
├── vitest.config.ts
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
- **[Contributing Guide](https://github.com/polkadot-developers/polkadot-cookbook/blob/master/CONTRIBUTING.md)** - Contribution workflow
- **[Architecture](../developers/architecture.md)** - How the cookbook works

### Explore Examples

- **[parachain-example](https://github.com/polkadot-developers/polkadot-cookbook/tree/master/recipes/parachains/parachain-example/)** - Full parachain with PAPI integration
- **[contracts-example](https://github.com/polkadot-developers/polkadot-cookbook/tree/master/recipes/contracts/contracts-example/)** - Solidity contracts
- **[transaction-example](https://github.com/polkadot-developers/polkadot-cookbook/tree/master/recipes/transactions/transaction-example/)** - PAPI interactions
- **[cross-chain-transaction-example](https://github.com/polkadot-developers/polkadot-cookbook/tree/master/recipes/cross-chain-transactions/cross-chain-transaction-example/)** - Cross-chain messaging

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
- No network access (required on first run to clone upstream template)
- Insufficient disk space

**Solution:**
```bash
# Check git configuration
git config user.name
git config user.email

# Ensure git is installed
git --version

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

# Run tests (types are auto-generated)
npm test

# If needed, manually regenerate types
npm run generate:types
```

---

## Summary

You've learned how to:
- Create a project with `dot create`
- Understand the generated file structure
- Build and test a parachain
- Run PAPI integration tests

**Ready for more?** Check out the [Recipe Development Guide](../contributors/recipe-development.md) for best practices and advanced techniques.

---

[← Back to Getting Started](README.md)
