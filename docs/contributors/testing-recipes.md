---
layout: doc
title: "Testing Recipes Guide"
---

# Testing Recipes Guide

Comprehensive guide to testing Polkadot Cookbook recipes across all types.

> **Scope:** This guide covers testing your recipe's source code in **your external repository**. For running the cookbook's test harnesses (which clone and verify external repos), see the [Workflow Guide](workflow.md#step-4-test-your-test-harness).

## Table of Contents

- [Testing Philosophy](#testing-philosophy)
- [Test Types](#test-types)
- [Rust Testing](#rust-testing)
- [TypeScript Testing](#typescript-testing)
- [Solidity Testing](#solidity-testing)
- [XCM Testing](#xcm-testing)
- [Integration Testing](#integration-testing)
- [CI/CD Testing](#cicd-testing)
- [Troubleshooting Tests](#troubleshooting-tests)

---

## Testing Philosophy

### Why Test?

**Quality assurance:**
- Verify code works as documented
- Catch regressions early
- Enable confident refactoring
- Document expected behavior

**User confidence:**
- Users trust tested recipes
- Examples are guaranteed to work
- Troubleshooting is easier

### Testing Principles

**1. Test What You Document**

Every code example in your README must have a test:

```markdown
<!-- In README.md -->
```rust
let value = Counter::<T>::get();
Counter::<T>::put(value + 1);
```
```

```rust
// In tests/integration.rs
#[test]
fn increment_counter_works() {
    new_test_ext().execute_with(|| {
        let value = Counter::<Test>::get();
        Counter::<Test>::put(value + 1);
        assert_eq!(Counter::<Test>::get(), 1);
    });
}
```

**2. Test Edge Cases**

Don't just test the happy path:

```rust
#[test]
fn transfer_works() { /* happy path */ }

#[test]
fn transfer_fails_insufficient_balance() { /* error case */ }

#[test]
fn transfer_fails_overflow() { /* edge case */ }

#[test]
fn transfer_zero_amount() { /* boundary case */ }
```

**3. Tests Should Be Reliable**

- No flaky tests
- No external dependencies (unless mocked)
- Deterministic results
- Fast execution

---

## Test Types

### Unit Tests

Test individual functions in isolation:

```rust
#[test]
fn calculate_fee_works() {
    let amount = 1000;
    let fee = calculate_fee(amount);
    assert_eq!(fee, 10);  // 1% fee
}
```

**When to use:** Testing pure functions, calculations, logic

### Integration Tests

Test multiple components working together:

```rust
#[test]
fn full_transfer_workflow() {
    new_test_ext().execute_with(|| {
        // Setup accounts
        // Execute transfer
        // Verify balances updated
        // Verify events emitted
    });
}
```

**When to use:** Testing complete workflows, user scenarios

### End-to-End Tests

Test complete system with real dependencies:

```typescript
it('should deploy contract and execute transaction', async () => {
  // Deploy contract to testnet
  // Execute real transaction
  // Verify on-chain state
});
```

**When to use:** Final validation before release

---

## Rust Testing

### Substrate Pallet Tests

**Test Setup:**

```rust
use crate as pallet_mymodule;
use frame_support::{
    assert_ok, assert_noop, parameter_types, traits::ConstU32,
};
use sp_core::H256;
use sp_runtime::{
    testing::Header,
    traits::{BlakeTwo256, IdentityLookup},
};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

frame_support::construct_runtime!(
    pub enum Test where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system,
        MyModule: pallet_mymodule,
    }
);

parameter_types! {
    pub const BlockHashCount: u64 = 250;
}

impl frame_system::Config for Test {
    type BaseCallFilter = frame_support::traits::Everything;
    type BlockWeights = ();
    type BlockLength = ();
    type DbWeight = ();
    type RuntimeOrigin = RuntimeOrigin;
    type RuntimeCall = RuntimeCall;
    type Index = u64;
    type BlockNumber = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = u64;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type RuntimeEvent = RuntimeEvent;
    type BlockHashCount = BlockHashCount;
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = ();
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = ();
    type OnSetCode = ();
    type MaxConsumers = ConstU32<16>;
}

impl pallet_mymodule::Config for Test {
    type RuntimeEvent = RuntimeEvent;
}

pub fn new_test_ext() -> sp_io::TestExternalities {
    let mut t = frame_system::GenesisConfig::default()
        .build_storage::<Test>()
        .unwrap();

    // Configure initial state
    pallet_mymodule::GenesisConfig::<Test> {
        initial_value: 42,
    }
    .assimilate_storage(&mut t)
    .unwrap();

    t.into()
}
```

**Testing Storage:**

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn storage_works() {
        new_test_ext().execute_with(|| {
            // Initially empty
            assert_eq!(MyStorage::<Test>::get(), None);

            // Insert value
            MyStorage::<Test>::put(42);
            assert_eq!(MyStorage::<Test>::get(), Some(42));

            // Update value
            MyStorage::<Test>::put(100);
            assert_eq!(MyStorage::<Test>::get(), Some(100));

            // Remove value
            MyStorage::<Test>::kill();
            assert_eq!(MyStorage::<Test>::get(), None);
        });
    }

    #[test]
    fn storage_map_works() {
        new_test_ext().execute_with(|| {
            let account = 1;

            // Check default
            assert_eq!(Balances::<Test>::get(account), 0);

            // Insert
            Balances::<Test>::insert(account, 1000);
            assert_eq!(Balances::<Test>::get(account), 1000);

            // Update
            Balances::<Test>::mutate(account, |balance| {
                *balance = balance.saturating_add(500);
            });
            assert_eq!(Balances::<Test>::get(account), 1500);

            // Remove
            Balances::<Test>::remove(account);
            assert_eq!(Balances::<Test>::get(account), 0);
        });
    }
}
```

**Testing Dispatchables:**

```rust
#[test]
fn transfer_works() {
    new_test_ext().execute_with(|| {
        let from = 1;
        let to = 2;
        let amount = 100;

        // Setup initial balance
        Balances::<Test>::insert(from, 1000);

        // Execute transfer
        assert_ok!(MyModule::transfer(
            RuntimeOrigin::signed(from),
            to,
            amount
        ));

        // Verify balances
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

        // Setup insufficient balance
        Balances::<Test>::insert(from, 100);

        // Attempt transfer
        assert_noop!(
            MyModule::transfer(RuntimeOrigin::signed(from), to, amount),
            Error::<Test>::InsufficientBalance
        );

        // Verify balances unchanged
        assert_eq!(Balances::<Test>::get(from), 100);
        assert_eq!(Balances::<Test>::get(to), 0);
    });
}

#[test]
fn transfer_fails_unsigned() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            MyModule::transfer(RuntimeOrigin::none(), 2, 100),
            sp_runtime::DispatchError::BadOrigin
        );
    });
}
```

**Testing Events:**

```rust
#[test]
fn transfer_emits_event() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);  // Events not emitted at block 0

        let from = 1;
        let to = 2;
        let amount = 100;

        Balances::<Test>::insert(from, 1000);

        assert_ok!(MyModule::transfer(
            RuntimeOrigin::signed(from),
            to,
            amount
        ));

        // Check event emitted
        System::assert_has_event(
            Event::Transferred { from, to, amount }.into()
        );

        // Or check last event
        System::assert_last_event(
            Event::Transferred { from, to, amount }.into()
        );
    });
}
```

**Testing Errors:**

```rust
#[test]
fn errors_work() {
    new_test_ext().execute_with(|| {
        // Test each error condition
        assert_noop!(
            MyModule::do_something(RuntimeOrigin::signed(1)),
            Error::<Test>::NotAuthorized
        );

        assert_noop!(
            MyModule::do_invalid(RuntimeOrigin::signed(1)),
            Error::<Test>::InvalidInput
        );
    });
}
```

### Running Rust Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run with output
cargo test -- --nocapture

# Run with logging
RUST_LOG=debug cargo test

# Run tests in specific module
cargo test pallet::tests

# Show test coverage
cargo tarpaulin --out Html
```

---

## TypeScript Testing

### Vitest Setup

**vitest.config.ts:**

```typescript
import { defineConfig } from 'vitest/config';

export default defineConfig({
  test: {
    globals: true,
    environment: 'node',
    setupFiles: ['./tests/setup.ts'],
    testTimeout: 30000,  // 30 seconds for blockchain tests
    hookTimeout: 30000,
  },
});
```

**tests/setup.ts:**

```typescript
import { beforeAll, afterAll } from 'vitest';

beforeAll(() => {
  console.log('Test suite starting...');
});

afterAll(() => {
  console.log('Test suite completed');
});
```

### Polkadot.js API Tests

**Connection Tests:**

```typescript
import { describe, it, expect, beforeAll, afterAll } from 'vitest';
import { ApiPromise, WsProvider } from '@polkadot/api';

describe('Polkadot API Connection', () => {
  let api: ApiPromise;

  beforeAll(async () => {
    const provider = new WsProvider('ws://localhost:9944');
    api = await ApiPromise.create({ provider });
  });

  afterAll(async () => {
    await api?.disconnect();
  });

  it('should connect to node', async () => {
    expect(api.isConnected).toBe(true);
  });

  it('should fetch chain name', async () => {
    const chain = await api.rpc.system.chain();
    expect(chain.toString()).toBeTruthy();
  });

  it('should fetch latest block', async () => {
    const header = await api.rpc.chain.getHeader();
    expect(header.number.toNumber()).toBeGreaterThan(0);
  });
});
```

**Query Tests:**

```typescript
describe('Balance Queries', () => {
  it('should fetch account balance', async () => {
    const alice = '5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY';

    const { data: { free } } = await api.query.system.account(alice);

    expect(free.toBigInt()).toBeGreaterThan(0n);
  });

  it('should handle non-existent account', async () => {
    const nobody = '5C4hrfjw9DjXZTzV3MwzrrAr9P1MJhSrvWGWqi1eSuyUpnhM';

    const { data: { free } } = await api.query.system.account(nobody);

    expect(free.toBigInt()).toBe(0n);
  });
});
```

**Transaction Tests:**

```typescript
import { Keyring } from '@polkadot/keyring';

describe('Token Transfer', () => {
  let api: ApiPromise;
  let keyring: Keyring;
  let alice: KeyringPair;
  let bob: KeyringPair;

  beforeAll(async () => {
    const provider = new WsProvider('ws://localhost:9944');
    api = await ApiPromise.create({ provider });

    keyring = new Keyring({ type: 'sr25519' });
    alice = keyring.addFromUri('//Alice');
    bob = keyring.addFromUri('//Bob');
  });

  afterAll(async () => {
    await api?.disconnect();
  });

  it('should transfer tokens', async () => {
    const amount = 1_000_000_000_000n; // 1 token

    // Get initial balances
    const { data: { free: initialBob } } = await api.query.system.account(bob.address);

    // Execute transfer
    await new Promise<void>((resolve, reject) => {
      api.tx.balances
        .transfer(bob.address, amount)
        .signAndSend(alice, ({ status, dispatchError }) => {
          if (status.isInBlock) {
            if (dispatchError) {
              reject(new Error('Transaction failed'));
            } else {
              resolve();
            }
          }
        })
        .catch(reject);
    });

    // Verify balance changed
    const { data: { free: finalBob } } = await api.query.system.account(bob.address);
    expect(finalBob.toBigInt()).toBe(initialBob.toBigInt() + amount);
  });

  it('should fail with insufficient balance', async () => {
    const amount = 1_000_000_000_000_000_000n; // Very large amount

    await expect(async () => {
      await new Promise((resolve, reject) => {
        api.tx.balances
          .transfer(bob.address, amount)
          .signAndSend(alice, ({ status, dispatchError }) => {
            if (status.isInBlock) {
              if (dispatchError) {
                if (dispatchError.isModule) {
                  const decoded = api.registry.findMetaError(
                    dispatchError.asModule
                  );
                  reject(new Error(`${decoded.section}.${decoded.name}`));
                }
              } else {
                resolve(null);
              }
            }
          })
          .catch(reject);
      });
    }).rejects.toThrow();
  });
});
```

### Running TypeScript Tests

```bash
# Run all tests
npm test

# Run specific test file
npm test balance.test.ts

# Run with coverage
npm test -- --coverage

# Watch mode
npm test -- --watch

# Verbose output
npm test -- --reporter=verbose
```

---

## Solidity Testing

### Hardhat Test Setup

Tests run against the pallet-revive dev node via the ETH-RPC adapter at `http://127.0.0.1:8545`. Start the dev node and ETH-RPC adapter before running tests.

**hardhat.config.ts:**

```typescript
import { HardhatUserConfig } from 'hardhat/config';
import '@nomicfoundation/hardhat-toolbox';

const config: HardhatUserConfig = {
  solidity: '0.8.24',
  networks: {
    localhost: {
      url: 'http://127.0.0.1:8545',
    },
  },
};

export default config;
```

**Contract Tests:**

```typescript
import { expect } from 'chai';
import { ethers } from 'hardhat';
import { SimpleToken } from '../typechain-types';
import { SignerWithAddress } from '@nomicfoundation/hardhat-ethers/signers';

describe('SimpleToken', () => {
  let token: SimpleToken;
  let owner: SignerWithAddress;
  let alice: SignerWithAddress;
  let bob: SignerWithAddress;

  beforeEach(async () => {
    [owner, alice, bob] = await ethers.getSigners();

    const TokenFactory = await ethers.getContractFactory('SimpleToken');
    token = await TokenFactory.deploy('Test Token', 'TEST', 18);
    await token.waitForDeployment();
  });

  describe('Deployment', () => {
    it('should set the correct name', async () => {
      expect(await token.name()).to.equal('Test Token');
    });

    it('should set the correct symbol', async () => {
      expect(await token.symbol()).to.equal('TEST');
    });

    it('should set the correct decimals', async () => {
      expect(await token.decimals()).to.equal(18);
    });
  });

  describe('Transfer', () => {
    beforeEach(async () => {
      // Mint tokens to alice
      await token.mint(alice.address, ethers.parseEther('1000'));
    });

    it('should transfer tokens', async () => {
      const amount = ethers.parseEther('100');

      await expect(
        token.connect(alice).transfer(bob.address, amount)
      ).to.changeTokenBalances(
        token,
        [alice, bob],
        [-amount, amount]
      );
    });

    it('should emit Transfer event', async () => {
      const amount = ethers.parseEther('100');

      await expect(
        token.connect(alice).transfer(bob.address, amount)
      )
        .to.emit(token, 'Transfer')
        .withArgs(alice.address, bob.address, amount);
    });

    it('should fail with insufficient balance', async () => {
      const amount = ethers.parseEther('10000');

      await expect(
        token.connect(alice).transfer(bob.address, amount)
      ).to.be.revertedWith('Insufficient balance');
    });

    it('should fail transfer to zero address', async () => {
      const amount = ethers.parseEther('100');

      await expect(
        token.connect(alice).transfer(ethers.ZeroAddress, amount)
      ).to.be.revertedWith('Transfer to zero address');
    });
  });

  describe('Approval', () => {
    it('should approve spender', async () => {
      const amount = ethers.parseEther('100');

      await expect(
        token.connect(alice).approve(bob.address, amount)
      )
        .to.emit(token, 'Approval')
        .withArgs(alice.address, bob.address, amount);

      expect(
        await token.allowance(alice.address, bob.address)
      ).to.equal(amount);
    });

    it('should allow transferFrom after approval', async () => {
      await token.mint(alice.address, ethers.parseEther('1000'));

      const amount = ethers.parseEther('100');
      await token.connect(alice).approve(bob.address, amount);

      await expect(
        token.connect(bob).transferFrom(alice.address, bob.address, amount)
      ).to.changeTokenBalances(
        token,
        [alice, bob],
        [-amount, amount]
      );
    });
  });
});
```

### Running Solidity Tests

Start the pallet-revive dev node and ETH-RPC adapter before running tests:

```bash
# Terminal 1: Start the dev node
revive-dev-node --dev

# Terminal 2: Start the ETH-RPC adapter
eth-rpc --dev
```

Then run tests against the running node:

```bash
# Run all tests
npx hardhat test --network localhost

# Run specific test
npx hardhat test --network localhost test/SimpleToken.test.ts

# With gas reporting
REPORT_GAS=true npx hardhat test --network localhost
```

---

## XCM Testing

### Chopsticks Setup

**chopsticks.yml:**

```yaml
endpoint: wss://polkadot-rpc.dwellir.com
mock-signature-host: true
db: ./db.sqlite
block: 12000000

import-storage:
  System:
    Account:
      - - '5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY'  # Alice
        - providers: 1
          data:
            free: '1000000000000000'  # 1M DOT
```

**XCM Tests:**

```typescript
import { describe, it, expect, beforeAll, afterAll } from 'vitest';
import { ApiPromise, WsProvider } from '@polkadot/api';
import { Keyring } from '@polkadot/keyring';

describe('XCM Transfer', () => {
  let relayApi: ApiPromise;
  let paraApi: ApiPromise;
  let keyring: Keyring;
  let alice: KeyringPair;

  beforeAll(async () => {
    // Connect to Chopsticks instances
    relayApi = await ApiPromise.create({
      provider: new WsProvider('ws://localhost:8000'),
    });

    paraApi = await ApiPromise.create({
      provider: new WsProvider('ws://localhost:8001'),
    });

    keyring = new Keyring({ type: 'sr25519' });
    alice = keyring.addFromUri('//Alice');
  }, 60000);  // 60 second timeout for setup

  afterAll(async () => {
    await relayApi?.disconnect();
    await paraApi?.disconnect();
  });

  it('should teleport assets to parachain', async () => {
    const amount = 1_000_000_000_000n;  // 1 DOT
    const dest = { V3: { parents: 0, interior: { X1: { Parachain: 1000 } } } };
    const beneficiary = {
      V3: {
        parents: 0,
        interior: {
          X1: {
            AccountId32: {
              network: null,
              id: alice.publicKey,
            },
          },
        },
      },
    };

    const assets = {
      V3: [
        {
          id: { Concrete: { parents: 0, interior: 'Here' } },
          fun: { Fungible: amount },
        },
      ],
    };

    // Execute teleport
    await new Promise<void>((resolve, reject) => {
      relayApi.tx.xcmPallet
        .limitedTeleportAssets(dest, beneficiary, assets, 0, 'Unlimited')
        .signAndSend(alice, ({ status, dispatchError }) => {
          if (status.isInBlock) {
            if (dispatchError) {
              reject(new Error('Teleport failed'));
            } else {
              resolve();
            }
          }
        })
        .catch(reject);
    });

    // Wait for XCM execution
    await new Promise(resolve => setTimeout(resolve, 12000));  // 2 blocks

    // Verify balance on parachain
    const { data: { free } } = await paraApi.query.system.account(alice.address);
    expect(free.toBigInt()).toBeGreaterThan(0n);
  }, 30000);
});
```

### Running XCM Tests

```bash
# Start Chopsticks in background
npx @acala-network/chopsticks -c chopsticks.yml &

# Wait for initialization
sleep 10

# Run tests
npm test

# Cleanup
pkill -f chopsticks
```

---

## Integration Testing

### Multi-Step Workflows

```typescript
describe('Complete Transfer Workflow', () => {
  it('should complete end-to-end transfer', async () => {
    // 1. Connect
    const api = await connect('ws://localhost:9944');

    // 2. Get initial balance
    const initialBalance = await getBalance(api, bob.address);

    // 3. Execute transfer
    const amount = 1_000_000_000_000n;
    await transfer(api, alice, bob.address, amount);

    // 4. Verify balance
    const finalBalance = await getBalance(api, bob.address);
    expect(finalBalance).toBe(initialBalance + amount);

    // 5. Cleanup
    await api.disconnect();
  });
});
```

---

## CI/CD Testing

### GitHub Actions

Tests run automatically in CI:

```yaml
# .github/workflows/test.yml
name: Test

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3

      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable

      - name: Run tests
        run: cargo test

      - name: Run clippy
        run: cargo clippy --all-targets --all-features
```

**See:** [Workflows Guide](../maintainers/workflows.md) for complete CI/CD documentation.

---

## Troubleshooting Tests

### Common Issues

**Tests timeout:**
```typescript
// Increase timeout
it('long running test', async () => {
  // test code
}, 60000);  // 60 second timeout
```

**Flaky tests:**
```typescript
// Add retries
test.retry(3)('flaky test', async () => {
  // test code
});
```

**Connection failures:**
```typescript
// Add connection retry logic
async function connectWithRetry(endpoint: string, retries = 3): Promise<ApiPromise> {
  for (let i = 0; i < retries; i++) {
    try {
      const provider = new WsProvider(endpoint);
      return await ApiPromise.create({ provider });
    } catch (error) {
      if (i === retries - 1) throw error;
      await new Promise(resolve => setTimeout(resolve, 1000 * (i + 1)));
    }
  }
  throw new Error('Failed to connect');
}
```

---

## Best Practices

✅ **DO:**
- Test all documented examples
- Cover happy path and error cases
- Use descriptive test names
- Clean up resources (disconnect APIs)
- Make tests deterministic
- Run tests locally before pushing

❌ **DON'T:**
- Skip testing edge cases
- Leave commented-out tests
- Use real networks for tests
- Hard-code values (use constants)
- Ignore test failures

---

## Related Documentation

- **[Recipe Development Guide](recipe-development.md)** - Development best practices
- **[Recipe Guidelines](recipe-guidelines.md)** - Structure requirements
- **[Workflows Guide](../maintainers/workflows.md)** - Complete CI/CD reference

---

[← Back to Contributors Guide](README.md)
