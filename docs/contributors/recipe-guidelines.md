# Recipe Guidelines

Standards and best practices for creating high-quality Polkadot Cookbook recipes.

## Table of Contents

- [Recipe Structure](#recipe-structure)
- [Content Guidelines](#content-guidelines)
- [Code Quality](#code-quality)
- [Documentation Standards](#documentation-standards)
- [Testing Requirements](#testing-requirements)
- [Metadata Requirements](#metadata-requirements)
- [Style Guide](#style-guide)

---

## Recipe Structure

### Required Files

Every recipe must include:

```
recipes/your-recipe/
├── README.md              # Main content (REQUIRED)
├── recipe.config.yml      # Metadata (REQUIRED)
├── src/                   # Source code (REQUIRED for code recipes)
└── tests/                 # Tests (REQUIRED)
```

### Optional Files

Include when appropriate:

```
recipes/your-recipe/
├── package.json           # npm dependencies (TypeScript)
├── tsconfig.json          # TypeScript config
├── vitest.config.ts       # Test config
├── Cargo.toml             # Rust dependencies
├── hardhat.config.ts      # Hardhat config (Solidity)
├── scripts/               # Helper scripts
└── assets/                # Images, diagrams
```

### Type-Specific Structure

**Polkadot SDK (Rust):**
```
recipes/your-recipe/
├── README.md
├── recipe.config.yml
├── Cargo.toml
├── src/
│   └── lib.rs             # Pallet implementation
└── tests/
    └── integration.test.rs
```

**Solidity Contracts:**
```
recipes/your-recipe/
├── README.md
├── recipe.config.yml
├── package.json
├── hardhat.config.ts
├── contracts/
│   └── YourContract.sol
├── scripts/
│   └── deploy.ts
└── test/
    └── YourContract.test.ts
```

**XCM Recipes:**
```
recipes/your-recipe/
├── README.md
├── recipe.config.yml
├── package.json
├── chopsticks.yml         # Chopsticks config
├── src/
│   └── xcm-transfer.ts
└── tests/
    └── transfer.test.ts
```

**TypeScript Interaction:**
```
recipes/your-recipe/
├── README.md
├── recipe.config.yml
├── package.json
├── tsconfig.json
├── src/
│   └── index.ts
└── tests/
    └── integration.test.ts
```

---

## Content Guidelines

### README.md Structure

Every recipe README must follow this structure:

```markdown
# Recipe Title

One-sentence description of what this recipe teaches.

## Prerequisites

- Required knowledge
- Required tools
- Required dependencies

## Learning Objectives

By completing this recipe, you will learn:
- Objective 1
- Objective 2
- Objective 3

## Overview

[Brief 2-3 paragraph overview of the concept]

## Steps

### 1. First Step

Explanation of what this step does...

[Code example]

Expected output...

### 2. Second Step

[Continue with numbered steps]

## Complete Example

[Full working code example]

## Testing

How to test this recipe...

## Expected Output

What you should see when running the code...

## Troubleshooting

### Common Issue 1
**Symptom:** Description
**Cause:** Explanation
**Solution:** How to fix

## Next Steps

- Related recipes
- Further reading
- Advanced topics

## References

- [External documentation links]
```

### Content Quality Standards

**Clear and Concise**
- Use simple language
- Avoid jargon or explain it when necessary
- One concept per paragraph
- Short sentences (prefer under 20 words)

**Beginner-Friendly**
- Assume minimal prior knowledge
- Define technical terms
- Explain why, not just what
- Link to prerequisite learning

**Actionable**
- Provide concrete examples
- Show exact commands to run
- Include expected output
- Explain error messages

**Complete**
- Cover the entire workflow
- Don't skip steps
- Include setup and teardown
- Provide troubleshooting

---

## Code Quality

### General Code Standards

✅ **DO:**
- Use meaningful variable names
- Add comments for complex logic
- Follow language-specific style guides
- Handle errors gracefully
- Write idiomatic code
- Keep functions small and focused

❌ **DON'T:**
- Use single-letter variables (except loop counters)
- Leave commented-out code
- Use magic numbers
- Ignore errors
- Write deeply nested code
- Include unused imports

### Rust Code

**Follow Rust conventions:**
```rust
// ✅ Good
pub fn calculate_total(items: &[Item]) -> Result<Balance, Error> {
    let mut total = Balance::zero();

    for item in items {
        total = total
            .checked_add(item.price)
            .ok_or(Error::Overflow)?;
    }

    Ok(total)
}

// ❌ Bad
pub fn calc(x: &[Item]) -> Balance {
    let mut t = 0;  // Unclear variable name
    for i in x {
        t = t + i.price;  // No error handling
    }
    t
}
```

**Formatting:**
```bash
# Always format before committing
cargo fmt --all

# Check clippy warnings
cargo clippy --all-targets --all-features -- -D warnings
```

**Error Handling:**
```rust
// ✅ Use proper error types
pub enum Error {
    InsufficientBalance,
    InvalidInput,
    Overflow,
}

// ✅ Propagate errors with ?
fn do_something() -> Result<(), Error> {
    let result = risky_operation()?;
    Ok(())
}

// ❌ Don't panic in production code
fn bad_function() {
    panic!("This should never happen");  // Bad!
}
```

### TypeScript Code

**Follow TypeScript conventions:**
```typescript
// ✅ Good
export async function transferTokens(
  api: ApiPromise,
  from: KeyringPair,
  to: string,
  amount: BN
): Promise<Hash> {
  const tx = api.tx.balances.transfer(to, amount);

  return new Promise((resolve, reject) => {
    tx.signAndSend(from, ({ status, events }) => {
      if (status.isInBlock) {
        resolve(status.asInBlock);
      }
    }).catch(reject);
  });
}

// ❌ Bad
export async function transfer(a, b, c, d) {  // Untyped parameters
  const tx = a.tx.balances.transfer(c, d);
  return tx.signAndSend(b);  // No error handling
}
```

**Use strict TypeScript:**
```json
// tsconfig.json
{
  "compilerOptions": {
    "strict": true,
    "noImplicitAny": true,
    "strictNullChecks": true
  }
}
```

### Solidity Code

**Follow Solidity conventions:**
```solidity
// ✅ Good
/// @notice Transfers tokens from sender to recipient
/// @param recipient The address receiving tokens
/// @param amount The amount to transfer
/// @return success Whether the transfer succeeded
function transfer(
    address recipient,
    uint256 amount
) external returns (bool success) {
    require(recipient != address(0), "Invalid recipient");
    require(balances[msg.sender] >= amount, "Insufficient balance");

    balances[msg.sender] -= amount;
    balances[recipient] += amount;

    emit Transfer(msg.sender, recipient, amount);
    return true;
}

// ❌ Bad
function transfer(address a, uint256 b) external returns (bool) {
    balances[msg.sender] -= b;  // No checks!
    balances[a] += b;
    return true;
}
```

**Security:**
- Use SafeMath or Solidity 0.8+ (overflow protection)
- Check for zero addresses
- Use ReentrancyGuard for external calls
- Validate all inputs
- Emit events for state changes

---

## Documentation Standards

### Code Comments

**Rust:**
```rust
/// Calculates the total balance for an account.
///
/// # Arguments
/// * `account` - The account to check
///
/// # Returns
/// The total balance or an error
pub fn get_balance(account: &AccountId) -> Result<Balance, Error> {
    // Implementation
}
```

**TypeScript:**
```typescript
/**
 * Fetches the current block number.
 *
 * @param api - The Polkadot API instance
 * @returns The current block number
 * @throws {Error} If the API is not connected
 */
export async function getCurrentBlock(api: ApiPromise): Promise<number> {
  // Implementation
}
```

**Solidity:**
```solidity
/// @notice Approves spender to withdraw from caller's account
/// @dev Implements ERC20 approve function
/// @param spender Address authorized to spend
/// @param amount Maximum amount they can spend
/// @return success Whether approval succeeded
function approve(address spender, uint256 amount)
    external
    returns (bool success)
{
    // Implementation
}
```

### Inline Explanations

Add explanations for complex operations:

```rust
// Calculate the era payout for validators
// Formula: total_issuance * inflation_rate / eras_per_year
let era_payout = total_issuance
    .saturating_mul(inflation_rate)
    .saturating_div(eras_per_year);
```

### README Code Examples

**Always include:**
- Setup/imports
- Full context
- Expected output
- Error handling

```typescript
// ✅ Complete example
import { ApiPromise, WsProvider } from '@polkadot/api';

async function main() {
  // Connect to node
  const provider = new WsProvider('wss://rpc.polkadot.io');
  const api = await ApiPromise.create({ provider });

  // Fetch chain name
  const chain = await api.rpc.system.chain();
  console.log(`Connected to chain: ${chain}`);

  // Always disconnect
  await api.disconnect();
}

main().catch(console.error);
```

**Expected output:**
```
Connected to chain: Polkadot
```

---

## Testing Requirements

### Test Coverage

**Minimum requirements:**
- All public functions tested
- Happy path scenarios
- Error cases
- Edge cases

**Rust:**
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transfer_success() {
        // Test successful transfer
    }

    #[test]
    fn test_transfer_insufficient_balance() {
        // Test error case
    }

    #[test]
    fn test_transfer_zero_amount() {
        // Test edge case
    }
}
```

**TypeScript (Vitest):**
```typescript
import { describe, it, expect, beforeAll, afterAll } from 'vitest';

describe('Token Transfer', () => {
  let api: ApiPromise;

  beforeAll(async () => {
    // Setup
  });

  afterAll(async () => {
    // Cleanup
  });

  it('should transfer tokens successfully', async () => {
    // Test implementation
  });

  it('should fail with insufficient balance', async () => {
    // Error test
  });
});
```

### Test Quality

✅ **Good tests:**
```typescript
it('should emit Transfer event with correct parameters', async () => {
  const tx = await contract.transfer(recipient, amount);
  const receipt = await tx.wait();

  const event = receipt.events?.find(e => e.event === 'Transfer');
  expect(event).toBeDefined();
  expect(event.args?.from).toBe(sender);
  expect(event.args?.to).toBe(recipient);
  expect(event.args?.amount).toBe(amount);
});
```

❌ **Bad tests:**
```typescript
it('should work', async () => {
  expect(true).toBe(true);  // Meaningless test
});
```

### Integration Tests

Test complete workflows:

```typescript
describe('Complete Transfer Workflow', () => {
  it('should transfer, verify balance, and emit event', async () => {
    // 1. Check initial balance
    const initialBalance = await api.query.balances.freeBalance(recipient);

    // 2. Execute transfer
    const hash = await transferTokens(api, alice, recipient, amount);

    // 3. Verify balance changed
    const finalBalance = await api.query.balances.freeBalance(recipient);
    expect(finalBalance.sub(initialBalance)).toEqual(amount);

    // 4. Verify event emitted
    const events = await api.query.system.events.at(hash);
    const transferEvent = events.find(e =>
      api.events.balances.Transfer.is(e.event)
    );
    expect(transferEvent).toBeDefined();
  });
});
```

---

## Metadata Requirements

### recipe.config.yml

All fields are required:

```yaml
title: "Your Recipe Title"
slug: "your-recipe-slug"
pathway: "runtime"  # runtime, contracts, basic-interaction, xcm, testing
difficulty: "beginner"  # beginner, intermediate, advanced
content_type: "tutorial"  # tutorial, guide
description: "One-sentence description of the recipe"
repository: "https://github.com/polkadot-developers/polkadot-cookbook"
type: "polkadot-sdk"  # polkadot-sdk, xcm, solidity, typescript
```

**Field guidelines:**

**title:**
- Clear and descriptive
- 3-8 words
- Title case
- Example: "Build a Custom Pallet with Events"

**slug:**
- Lowercase
- Hyphen-separated
- Match directory name
- Example: `build-custom-pallet-events`

**pathway:**
- `runtime` - Polkadot SDK runtime development
- `contracts` - Smart contract development
- `basic-interaction` - Basic blockchain interactions
- `xcm` - Cross-chain messaging
- `testing` - Testing strategies

**difficulty:**
- `beginner` - New to ecosystem
- `intermediate` - Some experience
- `advanced` - Production-ready complexity

**content_type:**
- `tutorial` - Step-by-step learning
- `guide` - Reference/how-to

**description:**
- One sentence
- Under 120 characters
- Explains what you'll learn
- Example: "Learn to build a custom pallet with events and error handling"

**type:**
- `polkadot-sdk` - Rust/Substrate
- `xcm` - Cross-chain messaging
- `solidity` - Solidity contracts
- `typescript` - TypeScript interactions


Only include if you need different versions than global defaults:

```yaml
versions:
  rust: "1.86"
  polkadot_omni_node: "0.6.0"
```


---

## Style Guide

### Writing Style

**Tone:**
- Friendly and encouraging
- Professional but approachable
- Direct and concise

**Voice:**
- Active voice preferred
- Second person ("you will learn")
- Present tense for instructions

**Examples:**

✅ **Good:**
> First, you'll create a new pallet. This pallet will store a single value.

❌ **Bad:**
> A new pallet should be created by the developer. The pallet would have been used to store a value.

### Formatting

**Headers:**
```markdown
# H1 - Recipe Title (only one)
## H2 - Major Sections
### H3 - Subsections
#### H4 - Minor Points (use sparingly)
```

**Lists:**
```markdown
Unordered list for non-sequential items:
- Item 1
- Item 2

Ordered list for sequential steps:
1. First step
2. Second step
```

**Code blocks:**
````markdown
```rust
// Always specify language
pub fn example() {
    // Code here
}
```

```bash
# Use bash for shell commands
cargo build
```

```typescript
// TypeScript examples
const value = await api.query.system.account(address);
```
````

**Emphasis:**
```markdown
Use **bold** for important terms
Use *italic* for emphasis
Use `code` for inline code, commands, file names
```

### Common Terms

Use consistent terminology:

| ✅ Preferred | ❌ Avoid |
|-------------|---------|
| pallet | module |
| extrinsic | transaction |
| runtime | chain runtime, on-chain logic |
| Polkadot SDK | Substrate, Substrate SDK |
| smart contract | contract (when clarity needed) |

---

## Checklist

Before submitting your recipe, verify:

### Structure
- [ ] README.md exists and follows template
- [ ] recipe.config.yml with all required fields
- [ ] src/ directory with implementation
- [ ] tests/ directory with comprehensive tests

### Content
- [ ] Clear title and description
- [ ] Prerequisites listed
- [ ] Learning objectives defined
- [ ] Step-by-step instructions
- [ ] Complete code examples
- [ ] Expected output shown
- [ ] Troubleshooting section

### Code Quality
- [ ] Follows language style guide
- [ ] Formatted (cargo fmt / prettier)
- [ ] Linted (clippy / eslint)
- [ ] Meaningful variable names
- [ ] Proper error handling
- [ ] Comments for complex logic

### Testing
- [ ] All code examples tested manually
- [ ] Automated tests pass
- [ ] Edge cases covered
- [ ] Error cases tested

### Documentation
- [ ] Code comments where needed
- [ ] README is clear and complete
- [ ] Links to external resources
- [ ] No broken links

### Validation
- [ ] `dot recipe validate` passes
- [ ] Pre-commit hooks pass
- [ ] CI tests pass

---

## Examples

Study these well-structured recipes:

- **Basic Pallet** - `recipes/basic-pallet/` - Simple, clear structure
- **Storage Operations** - `recipes/storage-operations/` - Good code examples
- **Events and Errors** - `recipes/events-and-errors/` - Comprehensive error handling

Browse all recipes:
```bash
dot recipe list
```

---

## Related Documentation

- **[Recipe Development Guide](recipe-development.md)** - Development best practices
- **[Testing Recipes Guide](testing-recipes.md)** - Testing strategies
- **[Contributor Workflow](workflow.md)** - Contribution process
- **[Commit Conventions](commit-conventions.md)** - Commit message format

---

[← Back to Contributors Guide](README.md)
