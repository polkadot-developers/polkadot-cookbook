---
layout: doc
title: "Recipe Development Guide"
---

# Recipe Development Guide

Best practices and advanced techniques for developing high-quality recipes.

> **Scope:** This guide covers developing your recipe's source code in **your own repository**. For adding the test harness to the cookbook, see the [Workflow Guide](workflow.md).

## Table of Contents

- [Development Environment](#development-environment)
- [Recipe Planning](#recipe-planning)
- [Content Development](#content-development)
- [Code Development](#code-development)
- [Testing Strategy](#testing-strategy)
- [Documentation](#documentation)
- [Common Patterns](#common-patterns)
- [Performance Considerations](#performance-considerations)
- [Security Best Practices](#security-best-practices)

---

## Development Environment

### Setup

Ensure you have all required tools:

```bash
# Verify environment

# Expected output shows all tools installed
```

### Recommended Tools

**Code Editors:**
- **VS Code** with extensions:
  - rust-analyzer (Rust)
  - ESLint (TypeScript)
  - Prettier (formatting)
  - Better TOML (config files)
  - YAML (config files)

**Rust Development:**
```bash
# Install Rust toolchain
rustup update stable
rustup component add rustfmt clippy

# Install cargo tools
cargo install cargo-watch  # Auto-rebuild on changes
cargo install cargo-expand  # View macro expansions
```

**TypeScript Development:**
```bash
# Global tools
npm install -g typescript vitest
```

### Workspace Setup

**Your recipe project** (in your own repo, outside the cookbook):

```bash
# Scaffold a new project
dot create

# Develop and test locally
cd my-recipe
npm install  # or cargo build
npm test
```

**Cookbook fork** (for adding your test harness later):

```bash
git clone https://github.com/YOUR_USERNAME/polkadot-cookbook.git
cd polkadot-cookbook
git remote add upstream https://github.com/polkadot-developers/polkadot-cookbook.git
```

---

## Recipe Planning

### Before You Code

**1. Research Existing Content**
```bash

# Check for similar topics
grep -r "your topic" recipes/*/README.md
```

**2. Define Learning Objectives**

Ask yourself:
- What will users learn?
- What prerequisites do they need?
- What can they build after completing this?

**Example objectives:**
```markdown
By completing this recipe, you will learn:
- How to create a custom pallet with storage
- How to implement dispatchable functions
- How to emit events and handle errors
- How to write integration tests for pallets
```

**3. Plan Your Structure**

Outline the recipe flow:
1. Concept explanation
2. Basic implementation
3. Add features incrementally
4. Complete example
5. Testing
6. Next steps

**4. Identify Dependencies**

Determine what versions you need:
```bash
# Check global versions

# Create recipe-specific versions if needed
```

---

## Content Development

### Writing Effective Tutorials

**Progression Model:**

1. **Introduce** - What and why
2. **Explain** - How it works
3. **Demonstrate** - Show example
4. **Practice** - User implements
5. **Reinforce** - Review key concepts

**Example structure:**

```markdown
## Understanding Storage in Pallets

Storage allows your pallet to persist data on-chain. Think of it
as a database that's replicated across all nodes.

### How Storage Works

When you declare storage with `#[pallet::storage]`, the Polkadot
SDK generates:
- A getter function
- A setter function
- Database storage handling

### Example: Simple Value Storage

Let's create a storage item that holds a single number:

[Code example]

This creates a storage item named `MyValue` that stores a `u32`.

### Try It Yourself

Now, add another storage item to store a string...
```

### Incremental Learning

Build complexity gradually:

**Step 1 - Simple:**
```rust
#[pallet::storage]
pub type MyValue<T> = StorageValue<_, u32>;
```

**Step 2 - Add functionality:**
```rust
#[pallet::storage]
pub type UserBalances<T: Config> = StorageMap<
    _,
    Blake2_128Concat,
    T::AccountId,
    Balance,
>;
```

**Step 3 - Complete feature:**
```rust
impl<T: Config> Pallet<T> {
    pub fn transfer(
        origin: OriginFor<T>,
        to: T::AccountId,
        amount: Balance,
    ) -> DispatchResult {
        // Full implementation
    }
}
```

### Clear Explanations

✅ **Good explanation:**
> The `Blake2_128Concat` hasher is used for storage maps. It provides
> a good balance between security and performance. The hash ensures
> even distribution of keys, while the concatenated portion allows
> iteration over the map.

❌ **Bad explanation:**
> Use `Blake2_128Concat` for your hasher.

### Code Examples

**Every code example should:**
- Be complete and runnable
- Include necessary imports
- Show expected output
- Explain key lines

**Example:**

````markdown
```rust
use frame_support::{pallet_prelude::*, weights::Weight};

#[frame_support::pallet]
pub mod pallet {
    use super::*;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>>
            + IsType<<Self as frame_system::Config>::RuntimeEvent>;
    }

    // Storage item for a simple counter
    #[pallet::storage]
    pub type Counter<T> = StorageValue<_, u32, ValueQuery>;

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Increment the counter by 1
        #[pallet::weight(Weight::from_parts(10_000, 0))]
        pub fn increment(origin: OriginFor<T>) -> DispatchResult {
            ensure_signed(origin)?;

            // Get current value (defaults to 0 if not set)
            let current = Counter::<T>::get();

            // Increment (with overflow check)
            let new_value = current.checked_add(1)
                .ok_or(Error::<T>::Overflow)?;

            // Store new value
            Counter::<T>::put(new_value);

            // Emit event
            Self::deposit_event(Event::Incremented { new_value });

            Ok(())
        }
    }
}
```

**Key points:**
- `ValueQuery` provides a default value (0) instead of Option
- `checked_add` prevents overflow errors
- `ensure_signed` verifies transaction is signed
- Event emitted for transparency
````

---

## Code Development

### Rust Development

**Pallet Development Workflow:**

```bash
# 1. Create basic structure
vim src/lib.rs

# 2. Auto-rebuild on changes
cargo watch -x build

# 3. Run tests continuously
cargo watch -x test

# 4. Check for issues
cargo clippy --all-targets --all-features
```

**Common Patterns:**

**Storage Items:**
```rust
// Single value
#[pallet::storage]
pub type SingleValue<T> = StorageValue<_, u32, ValueQuery>;

// Map: AccountId -> Balance
#[pallet::storage]
pub type Balances<T: Config> = StorageMap<
    _,
    Blake2_128Concat,
    T::AccountId,
    Balance,
    ValueQuery,
>;

// Double map: (AccountId, TokenId) -> Balance
#[pallet::storage]
pub type TokenBalances<T: Config> = StorageDoubleMap<
    _,
    Blake2_128Concat,
    T::AccountId,
    Blake2_128Concat,
    TokenId,
    Balance,
    ValueQuery,
>;
```

**Events:**
```rust
#[pallet::event]
#[pallet::generate_deposit(pub(super) fn deposit_event)]
pub enum Event<T: Config> {
    /// Token transferred
    Transferred {
        from: T::AccountId,
        to: T::AccountId,
        amount: Balance,
    },
    /// Token minted
    Minted {
        to: T::AccountId,
        amount: Balance,
    },
}
```

**Errors:**
```rust
#[pallet::error]
pub enum Error<T> {
    /// Insufficient balance for transfer
    InsufficientBalance,
    /// Arithmetic overflow occurred
    Overflow,
    /// Invalid input parameter
    InvalidInput,
}
```

**Dispatchable Functions:**
```rust
#[pallet::call]
impl<T: Config> Pallet<T> {
    #[pallet::weight(10_000)]
    pub fn transfer(
        origin: OriginFor<T>,
        to: T::AccountId,
        amount: Balance,
    ) -> DispatchResult {
        // 1. Verify caller
        let from = ensure_signed(origin)?;

        // 2. Validate inputs
        ensure!(amount > 0, Error::<T>::InvalidInput);

        // 3. Check balances
        let from_balance = Balances::<T>::get(&from);
        ensure!(from_balance >= amount, Error::<T>::InsufficientBalance);

        // 4. Update storage
        let new_from_balance = from_balance
            .checked_sub(amount)
            .ok_or(Error::<T>::Overflow)?;
        Balances::<T>::insert(&from, new_from_balance);

        let to_balance = Balances::<T>::get(&to);
        let new_to_balance = to_balance
            .checked_add(amount)
            .ok_or(Error::<T>::Overflow)?;
        Balances::<T>::insert(&to, new_to_balance);

        // 5. Emit event
        Self::deposit_event(Event::Transferred { from, to, amount });

        Ok(())
    }
}
```

### TypeScript Development

**Polkadot.js API Patterns:**

```typescript
import { ApiPromise, WsProvider } from '@polkadot/api';
import { KeyringPair } from '@polkadot/keyring/types';

/**
 * Connects to a Polkadot node
 */
export async function connect(endpoint: string): Promise<ApiPromise> {
  const provider = new WsProvider(endpoint);
  const api = await ApiPromise.create({ provider });

  await api.isReady;
  return api;
}

/**
 * Transfers tokens
 */
export async function transfer(
  api: ApiPromise,
  from: KeyringPair,
  to: string,
  amount: bigint
): Promise<string> {
  return new Promise((resolve, reject) => {
    api.tx.balances
      .transfer(to, amount)
      .signAndSend(from, ({ status, events }) => {
        if (status.isInBlock) {
          console.log(`Included in block: ${status.asInBlock}`);

          // Check for errors
          events.forEach(({ event }) => {
            if (api.events.system.ExtrinsicFailed.is(event)) {
              const [dispatchError] = event.data;
              let errorInfo;

              if (dispatchError.isModule) {
                const decoded = api.registry.findMetaError(
                  dispatchError.asModule
                );
                errorInfo = `${decoded.section}.${decoded.name}: ${decoded.docs}`;
              } else {
                errorInfo = dispatchError.toString();
              }

              reject(new Error(errorInfo));
            }
          });

          resolve(status.asInBlock.toString());
        }
      })
      .catch(reject);
  });
}

/**
 * Query account balance
 */
export async function getBalance(
  api: ApiPromise,
  address: string
): Promise<bigint> {
  const { data: { free } } = await api.query.system.account(address);
  return free.toBigInt();
}
```

### Solidity Development

**Contract Patterns:**

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

/**
 * @title SimpleToken
 * @dev Basic ERC20 implementation
 */
contract SimpleToken {
    mapping(address => uint256) private balances;
    mapping(address => mapping(address => uint256)) private allowances;

    uint256 private totalSupply_;
    string public name;
    string public symbol;
    uint8 public decimals;

    event Transfer(address indexed from, address indexed to, uint256 value);
    event Approval(address indexed owner, address indexed spender, uint256 value);

    constructor(string memory _name, string memory _symbol, uint8 _decimals) {
        name = _name;
        symbol = _symbol;
        decimals = _decimals;
    }

    /**
     * @dev Transfers tokens to a recipient
     * @param recipient The address receiving tokens
     * @param amount The amount to transfer
     * @return success Whether the transfer succeeded
     */
    function transfer(address recipient, uint256 amount)
        external
        returns (bool success)
    {
        require(recipient != address(0), "Transfer to zero address");
        require(balances[msg.sender] >= amount, "Insufficient balance");

        balances[msg.sender] -= amount;
        balances[recipient] += amount;

        emit Transfer(msg.sender, recipient, amount);
        return true;
    }

    /**
     * @dev Returns the balance of an account
     * @param account The account to query
     * @return The account balance
     */
    function balanceOf(address account) external view returns (uint256) {
        return balances[account];
    }

    /**
     * @dev Returns total token supply
     */
    function totalSupply() external view returns (uint256) {
        return totalSupply_;
    }

    /**
     * @dev Internal function to mint tokens
     */
    function _mint(address account, uint256 amount) internal {
        require(account != address(0), "Mint to zero address");

        totalSupply_ += amount;
        balances[account] += amount;

        emit Transfer(address(0), account, amount);
    }
}
```

---

## Testing Strategy

### Test Organization

```
tests/
├── unit/               # Unit tests
│   ├── storage.test.ts
│   └── logic.test.ts
├── integration/        # Integration tests
│   ├── workflow.test.ts
│   └── e2e.test.ts
└── fixtures/          # Test data
    └── accounts.ts
```

### Rust Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use frame_support::{assert_ok, assert_noop};

    type TestExternalities = sp_io::TestExternalities;

    fn new_test_ext() -> TestExternalities {
        let mut t = system::GenesisConfig::default()
            .build_storage::<Test>()
            .unwrap();

        t.into()
    }

    #[test]
    fn transfer_works() {
        new_test_ext().execute_with(|| {
            // Setup
            let from = 1;
            let to = 2;
            let amount = 100;

            Balances::<Test>::insert(from, 1000);

            // Execute
            assert_ok!(Pallet::<Test>::transfer(
                RuntimeOrigin::signed(from),
                to,
                amount
            ));

            // Verify
            assert_eq!(Balances::<Test>::get(from), 900);
            assert_eq!(Balances::<Test>::get(to), 100);
        });
    }

    #[test]
    fn transfer_fails_insufficient_balance() {
        new_test_ext().execute_with(|| {
            let from = 1;
            let to = 2;
            let amount = 1000;

            Balances::<Test>::insert(from, 100);

            assert_noop!(
                Pallet::<Test>::transfer(RuntimeOrigin::signed(from), to, amount),
                Error::<Test>::InsufficientBalance
            );
        });
    }
}
```

**See:** [Testing Recipes Guide](testing-recipes.md) for comprehensive testing strategies.

---

## Documentation

### Code Documentation

**Document all public APIs:**

```rust
/// Transfers tokens from the caller to another account.
///
/// # Arguments
/// * `origin` - The transaction origin (must be signed)
/// * `to` - The recipient account
/// * `amount` - The amount to transfer
///
/// # Errors
/// Returns `InsufficientBalance` if the caller doesn't have enough tokens.
/// Returns `Overflow` if the arithmetic overflows.
///
/// # Events
/// Emits `Transferred` event on success.
///
/// # Example
/// ```ignore
/// Pallet::transfer(origin, bob, 100)?;
/// ```
#[pallet::weight(10_000)]
pub fn transfer(
    origin: OriginFor<T>,
    to: T::AccountId,
    amount: Balance,
) -> DispatchResult {
    // Implementation
}
```

### README Documentation

Include comprehensive README:

```markdown
# Recipe Title

## Quick Start

[30-second overview and minimal example]

## Prerequisites

- Specific tool versions
- Required knowledge
- Setup requirements

## Learning Objectives

[Clear, measurable objectives]

## Detailed Guide

[Step-by-step instructions]

## API Reference

[If applicable, document public APIs]

## Troubleshooting

[Common issues and solutions]

## Further Reading

[Related recipes and external resources]
```

---

## Common Patterns

### Error Handling

**Always handle errors:**

```rust
// ✅ Good
let result = risky_operation()
    .checked_sub(value)
    .ok_or(Error::<T>::Overflow)?;

// ❌ Bad
let result = risky_operation() - value;  // Can panic!
```

### Resource Cleanup

**TypeScript:**
```typescript
export async function withApi<T>(
  endpoint: string,
  fn: (api: ApiPromise) => Promise<T>
): Promise<T> {
  const api = await connect(endpoint);
  try {
    return await fn(api);
  } finally {
    await api.disconnect();
  }
}
```

### Configuration

**Make recipes configurable:**

```typescript
export interface Config {
  endpoint: string;
  timeout: number;
  retries: number;
}

const DEFAULT_CONFIG: Config = {
  endpoint: 'wss://rpc.polkadot.io',
  timeout: 30000,
  retries: 3,
};

export function createClient(config: Partial<Config> = {}) {
  const finalConfig = { ...DEFAULT_CONFIG, ...config };
  // Use finalConfig
}
```

---

## Performance Considerations

### Rust Performance

**Use appropriate storage types:**
```rust
// Fast for single values
StorageValue<_, T>

// Fast lookups, O(1)
StorageMap<_, Blake2_128Concat, K, V>

// Avoid iteration in production
for (key, value) in StorageMap::<T>::iter() { }  // Expensive!
```

**Optimize weights:**
```rust
#[pallet::weight(T::DbWeight::get().reads_writes(2, 1))]
pub fn my_function(origin: OriginFor<T>) -> DispatchResult {
    // 2 reads, 1 write
}
```

### TypeScript Performance

**Batch operations:**
```typescript
// ✅ Good - batch queries
const [balance1, balance2] = await Promise.all([
  api.query.system.account(addr1),
  api.query.system.account(addr2),
]);

// ❌ Bad - sequential queries
const balance1 = await api.query.system.account(addr1);
const balance2 = await api.query.system.account(addr2);
```

---

## Security Best Practices

### Input Validation

```rust
pub fn transfer(
    origin: OriginFor<T>,
    to: T::AccountId,
    amount: Balance,
) -> DispatchResult {
    let from = ensure_signed(origin)?;

    // Validate inputs
    ensure!(amount > 0, Error::<T>::InvalidAmount);
    ensure!(to != from, Error::<T>::SelfTransfer);

    // Check authorization
    ensure!(
        Self::is_authorized(&from),
        Error::<T>::Unauthorized
    );

    // Proceed with logic
}
```

### Arithmetic Safety

```rust
// ✅ Use checked arithmetic
let result = value1
    .checked_add(value2)
    .ok_or(Error::<T>::Overflow)?;

// ✅ Or saturating arithmetic (where appropriate)
let result = value1.saturating_add(value2);

// ❌ Never use unchecked arithmetic in production
let result = value1 + value2;  // Can overflow!
```

### Access Control

```solidity
// Solidity access control
modifier onlyOwner() {
    require(msg.sender == owner, "Not authorized");
    _;
}

function sensitiveOperation() external onlyOwner {
    // Protected operation
}
```

---

## Related Documentation

- **[Recipe Guidelines](recipe-guidelines.md)** - Structure and style requirements
- **[Testing Recipes](testing-recipes.md)** - Comprehensive testing guide
- **[Contributor Workflow](workflow.md)** - Contribution process
- **[Troubleshooting](troubleshooting.md)** - Common issues and solutions

---

[← Back to Contributors Guide](README.md)
