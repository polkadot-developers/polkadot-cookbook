import { describe, it, expect } from "vitest";
import { execSync } from "child_process";
import {
  existsSync,
  readFileSync,
  writeFileSync,
  mkdirSync,
} from "fs";
import { join } from "path";

const WORKSPACE_DIR = join(process.cwd(), ".test-workspace");
const TEMPLATE_DIR = join(WORKSPACE_DIR, "parachain-template");
const TEMPLATE_VERSION = "v0.0.4";
const PALLET_DIR = join(TEMPLATE_DIR, "pallets/pallet-custom");

// Complete pallet implementation from the create-a-pallet guide
const PALLET_LIB_RS = `#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[frame::pallet]
pub mod pallet {
    use alloc::vec::Vec;
    use frame::prelude::*;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        #[pallet::constant]
        type CounterMaxValue: Get<u32>;
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        CounterValueSet {
            new_value: u32,
        },
        CounterIncremented {
            new_value: u32,
            who: T::AccountId,
            amount: u32,
        },
        CounterDecremented {
            new_value: u32,
            who: T::AccountId,
            amount: u32,
        },
    }

    #[pallet::error]
    pub enum Error<T> {
        NoneValue,
        Overflow,
        Underflow,
        CounterMaxValueExceeded,
    }

    #[pallet::storage]
    pub type CounterValue<T> = StorageValue<_, u32, ValueQuery>;

    #[pallet::storage]
    pub type UserInteractions<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        u32,
        ValueQuery
    >;

    #[pallet::genesis_config]
    #[derive(DefaultNoBound)]
    pub struct GenesisConfig<T: Config> {
        pub initial_counter_value: u32,
        pub initial_user_interactions: Vec<(T::AccountId, u32)>,
    }

    #[pallet::genesis_build]
    impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
        fn build(&self) {
            CounterValue::<T>::put(self.initial_counter_value);
            for (account, count) in &self.initial_user_interactions {
                UserInteractions::<T>::insert(account, count);
            }
        }
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(0)]
        pub fn set_counter_value(origin: OriginFor<T>, new_value: u32) -> DispatchResult {
            ensure_root(origin)?;
            ensure!(new_value <= T::CounterMaxValue::get(), Error::<T>::CounterMaxValueExceeded);
            CounterValue::<T>::put(new_value);
            Self::deposit_event(Event::CounterValueSet { new_value });
            Ok(())
        }

        #[pallet::call_index(1)]
        #[pallet::weight(0)]
        pub fn increment(origin: OriginFor<T>, amount: u32) -> DispatchResult {
            let who = ensure_signed(origin)?;
            let current_value = CounterValue::<T>::get();
            let new_value = current_value.checked_add(amount).ok_or(Error::<T>::Overflow)?;
            ensure!(new_value <= T::CounterMaxValue::get(), Error::<T>::CounterMaxValueExceeded);
            CounterValue::<T>::put(new_value);
            UserInteractions::<T>::mutate(&who, |count| {
                *count = count.saturating_add(1);
            });
            Self::deposit_event(Event::CounterIncremented { new_value, who, amount });
            Ok(())
        }

        #[pallet::call_index(2)]
        #[pallet::weight(0)]
        pub fn decrement(origin: OriginFor<T>, amount: u32) -> DispatchResult {
            let who = ensure_signed(origin)?;
            let current_value = CounterValue::<T>::get();
            let new_value = current_value.checked_sub(amount).ok_or(Error::<T>::Underflow)?;
            CounterValue::<T>::put(new_value);
            UserInteractions::<T>::mutate(&who, |count| {
                *count = count.saturating_add(1);
            });
            Self::deposit_event(Event::CounterDecremented { new_value, who, amount });
            Ok(())
        }
    }
}
`;

const PALLET_CARGO_TOML = `[package]
name = "pallet-custom"
description = "A custom counter pallet for demonstration purposes."
version = "0.1.0"
license = "Unlicense"
authors.workspace = true
homepage.workspace = true
repository.workspace = true
edition.workspace = true
publish = false

[package.metadata.docs.rs]
targets = ["x86_64-unknown-linux-gnu"]

[dependencies]
codec = { features = ["derive"], workspace = true }
scale-info = { features = ["derive"], workspace = true }
frame = { features = ["experimental", "runtime"], workspace = true }

[features]
default = ["std"]
std = [
    "codec/std",
    "scale-info/std",
    "frame/std",
]
`;

// Mock runtime from the mock-runtime guide
const MOCK_RS = `use crate as pallet_custom;
use frame::{
    deps::{
        frame_support::{ derive_impl, traits::ConstU32 },
        sp_io,
        sp_runtime::{ traits::IdentityLookup, BuildStorage },
    },
    prelude::*,
};

type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame::deps::frame_support::construct_runtime!(
    pub enum Test
    {
        System: frame_system,
        CustomPallet: pallet_custom,
    }
);

#[derive_impl(frame_system::config_preludes::TestDefaultConfig)]
impl frame_system::Config for Test {
    type Block = Block;
    type AccountId = u64;
    type Lookup = IdentityLookup<Self::AccountId>;
}

impl pallet_custom::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type CounterMaxValue = ConstU32<1000>;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
    let mut t = frame_system::GenesisConfig::<Test>::default().build_storage().unwrap();

    (pallet_custom::GenesisConfig::<Test> {
        initial_counter_value: 0,
        initial_user_interactions: vec![],
    })
        .assimilate_storage(&mut t)
        .unwrap();

    t.into()
}

// Helper function to create a test externalities with a specific initial counter value
pub fn new_test_ext_with_counter(initial_value: u32) -> sp_io::TestExternalities {
    let mut t = frame_system::GenesisConfig::<Test>::default().build_storage().unwrap();

    (pallet_custom::GenesisConfig::<Test> {
        initial_counter_value: initial_value,
        initial_user_interactions: vec![],
    })
        .assimilate_storage(&mut t)
        .unwrap();

    t.into()
}

// Helper function to create a test externalities with initial user interactions
pub fn new_test_ext_with_interactions(
    initial_value: u32,
    interactions: Vec<(u64, u32)>
) -> sp_io::TestExternalities {
    let mut t = frame_system::GenesisConfig::<Test>::default().build_storage().unwrap();

    (pallet_custom::GenesisConfig::<Test> {
        initial_counter_value: initial_value,
        initial_user_interactions: interactions,
    })
        .assimilate_storage(&mut t)
        .unwrap();

    t.into()
}
`;

// Unit tests from the pallet-testing guide
const TESTS_RS = `use crate::{mock::*, Error, Event};
use frame::deps::frame_support::{assert_noop, assert_ok};
use frame::deps::sp_runtime::DispatchError;

#[test]
fn set_counter_value_works() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        assert_ok!(CustomPallet::set_counter_value(RuntimeOrigin::root(), 100));
        assert_eq!(crate::CounterValue::<Test>::get(), 100);
        System::assert_last_event(Event::CounterValueSet { new_value: 100 }.into());
    });
}

#[test]
fn set_counter_value_requires_root() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            CustomPallet::set_counter_value(RuntimeOrigin::signed(1), 100),
            DispatchError::BadOrigin
        );
    });
}

#[test]
fn set_counter_value_respects_max_value() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            CustomPallet::set_counter_value(RuntimeOrigin::root(), 1001),
            Error::<Test>::CounterMaxValueExceeded
        );
        assert_ok!(CustomPallet::set_counter_value(RuntimeOrigin::root(), 1000));
        assert_eq!(crate::CounterValue::<Test>::get(), 1000);
    });
}

#[test]
fn increment_works() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        let account = 1u64;
        assert_ok!(CustomPallet::increment(RuntimeOrigin::signed(account), 50));
        assert_eq!(crate::CounterValue::<Test>::get(), 50);
        System::assert_last_event(
            Event::CounterIncremented {
                new_value: 50,
                who: account,
                amount: 50,
            }
            .into(),
        );
        assert_eq!(crate::UserInteractions::<Test>::get(account), 1);
    });
}

#[test]
fn increment_tracks_multiple_interactions() {
    new_test_ext().execute_with(|| {
        let account = 1u64;
        assert_ok!(CustomPallet::increment(RuntimeOrigin::signed(account), 10));
        assert_ok!(CustomPallet::increment(RuntimeOrigin::signed(account), 20));
        assert_ok!(CustomPallet::increment(RuntimeOrigin::signed(account), 30));
        assert_eq!(crate::CounterValue::<Test>::get(), 60);
        assert_eq!(crate::UserInteractions::<Test>::get(account), 3);
    });
}

#[test]
fn increment_fails_on_overflow() {
    new_test_ext_with_counter(u32::MAX).execute_with(|| {
        assert_noop!(
            CustomPallet::increment(RuntimeOrigin::signed(1), 1),
            Error::<Test>::Overflow
        );
    });
}

#[test]
fn increment_respects_max_value() {
    new_test_ext_with_counter(950).execute_with(|| {
        assert_noop!(
            CustomPallet::increment(RuntimeOrigin::signed(1), 51),
            Error::<Test>::CounterMaxValueExceeded
        );
        assert_ok!(CustomPallet::increment(RuntimeOrigin::signed(1), 50));
        assert_eq!(crate::CounterValue::<Test>::get(), 1000);
    });
}

#[test]
fn decrement_works() {
    new_test_ext_with_counter(100).execute_with(|| {
        System::set_block_number(1);
        let account = 2u64;
        assert_ok!(CustomPallet::decrement(RuntimeOrigin::signed(account), 30));
        assert_eq!(crate::CounterValue::<Test>::get(), 70);
        System::assert_last_event(
            Event::CounterDecremented {
                new_value: 70,
                who: account,
                amount: 30,
            }
            .into(),
        );
        assert_eq!(crate::UserInteractions::<Test>::get(account), 1);
    });
}

#[test]
fn decrement_fails_on_underflow() {
    new_test_ext_with_counter(10).execute_with(|| {
        assert_noop!(
            CustomPallet::decrement(RuntimeOrigin::signed(1), 11),
            Error::<Test>::Underflow
        );
    });
}

#[test]
fn decrement_tracks_multiple_interactions() {
    new_test_ext_with_counter(100).execute_with(|| {
        let account = 3u64;
        assert_ok!(CustomPallet::decrement(RuntimeOrigin::signed(account), 10));
        assert_ok!(CustomPallet::decrement(RuntimeOrigin::signed(account), 20));
        assert_eq!(crate::CounterValue::<Test>::get(), 70);
        assert_eq!(crate::UserInteractions::<Test>::get(account), 2);
    });
}

#[test]
fn mixed_increment_and_decrement_works() {
    new_test_ext_with_counter(50).execute_with(|| {
        let account = 4u64;
        assert_ok!(CustomPallet::increment(RuntimeOrigin::signed(account), 25));
        assert_eq!(crate::CounterValue::<Test>::get(), 75);
        assert_ok!(CustomPallet::decrement(RuntimeOrigin::signed(account), 15));
        assert_eq!(crate::CounterValue::<Test>::get(), 60);
        assert_ok!(CustomPallet::increment(RuntimeOrigin::signed(account), 10));
        assert_eq!(crate::CounterValue::<Test>::get(), 70);
        assert_eq!(crate::UserInteractions::<Test>::get(account), 3);
    });
}

#[test]
fn different_users_tracked_separately() {
    new_test_ext().execute_with(|| {
        let account1 = 1u64;
        let account2 = 2u64;
        assert_ok!(CustomPallet::increment(RuntimeOrigin::signed(account1), 10));
        assert_ok!(CustomPallet::increment(RuntimeOrigin::signed(account1), 10));
        assert_ok!(CustomPallet::decrement(RuntimeOrigin::signed(account2), 5));
        assert_eq!(crate::CounterValue::<Test>::get(), 15);
        assert_eq!(crate::UserInteractions::<Test>::get(account1), 2);
        assert_eq!(crate::UserInteractions::<Test>::get(account2), 1);
    });
}

#[test]
fn genesis_config_works() {
    new_test_ext_with_interactions(42, vec![(1, 5), (2, 10)]).execute_with(|| {
        assert_eq!(crate::CounterValue::<Test>::get(), 42);
        assert_eq!(crate::UserInteractions::<Test>::get(1), 5);
        assert_eq!(crate::UserInteractions::<Test>::get(2), 10);
    });
}
`;

// All tests in a single describe block to ensure sequential execution
describe("Unit Test Pallets Guide", () => {
  // ==================== ENVIRONMENT TESTS ====================
  describe("1. Environment Prerequisites", () => {
    it("should have Rust installed", () => {
      const result = execSync("rustc --version", { encoding: "utf-8" });
      expect(result).toMatch(/rustc \d+\.\d+/);
      console.log(`Rust: ${result.trim()}`);
    });

    it("should have cargo installed", () => {
      const result = execSync("cargo --version", { encoding: "utf-8" });
      expect(result).toMatch(/cargo \d+\.\d+/);
      console.log(`Cargo: ${result.trim()}`);
    });

    it("should have wasm32-unknown-unknown target", () => {
      const targets = execSync("rustup target list --installed", {
        encoding: "utf-8",
      });
      expect(targets).toContain("wasm32-unknown-unknown");
      console.log("wasm32-unknown-unknown target: installed");
    });

    it("should have git installed", () => {
      const result = execSync("git --version", { encoding: "utf-8" });
      expect(result).toMatch(/git version/);
      console.log(`Git: ${result.trim()}`);
    });
  });

  // ==================== CLONE AND CREATE PALLET TESTS ====================
  describe("2. Clone Template and Create Pallet (Prerequisite)", () => {
    it("should clone the parachain template repository", () => {
      // Create workspace directory
      if (!existsSync(WORKSPACE_DIR)) {
        mkdirSync(WORKSPACE_DIR, { recursive: true });
      }

      if (existsSync(TEMPLATE_DIR)) {
        console.log(`Template already cloned, checking out ${TEMPLATE_VERSION}...`);
        execSync(`git fetch --tags && git checkout ${TEMPLATE_VERSION}`, { cwd: TEMPLATE_DIR, encoding: "utf-8" });
      } else {
        console.log(`Cloning polkadot-sdk-parachain-template ${TEMPLATE_VERSION}...`);
        execSync(
          `git clone --branch ${TEMPLATE_VERSION} https://github.com/paritytech/polkadot-sdk-parachain-template.git ${TEMPLATE_DIR}`,
          { encoding: "utf-8", stdio: "inherit" }
        );
      }

      expect(existsSync(join(TEMPLATE_DIR, "Cargo.toml"))).toBe(true);
      expect(existsSync(join(TEMPLATE_DIR, "pallets"))).toBe(true);
      console.log("Repository cloned successfully");
    }, 120000);

    it("should create a new pallet-custom directory", () => {
      // Create pallet directory structure
      if (!existsSync(PALLET_DIR)) {
        mkdirSync(join(PALLET_DIR, "src"), { recursive: true });
        console.log("Created pallet-custom directory");
      } else {
        console.log("pallet-custom directory already exists");
      }

      expect(existsSync(PALLET_DIR)).toBe(true);
    });

    it("should write pallet Cargo.toml", () => {
      const cargoPath = join(PALLET_DIR, "Cargo.toml");

      // Check if already written
      if (existsSync(cargoPath)) {
        const content = readFileSync(cargoPath, "utf-8");
        if (content.includes('name = "pallet-custom"')) {
          console.log("pallet-custom/Cargo.toml already configured");
          return;
        }
      }

      writeFileSync(cargoPath, PALLET_CARGO_TOML);
      console.log("Written pallet-custom/Cargo.toml");

      // Verify
      const content = readFileSync(cargoPath, "utf-8");
      expect(content).toContain('name = "pallet-custom"');
      expect(content).toContain("frame");
    });

    it("should add pallet-custom to workspace members", () => {
      const cargoPath = join(TEMPLATE_DIR, "Cargo.toml");
      let content = readFileSync(cargoPath, "utf-8");

      // Check if already added
      if (content.includes('"pallets/pallet-custom"')) {
        console.log("pallet-custom already in workspace members");
        return;
      }

      // Add to workspace members
      const membersRegex = /(members\s*=\s*\[\s*\n\s*"node",\s*\n\s*"pallets\/template",\s*\n\s*"runtime",)/;
      const match = content.match(membersRegex);

      if (match) {
        content = content.replace(
          match[1],
          `${match[1]}\n    "pallets/pallet-custom",`
        );
        writeFileSync(cargoPath, content);
        console.log("Added pallet-custom to workspace members");
      } else {
        throw new Error("Could not find members array in workspace Cargo.toml");
      }

      // Verify
      const updatedContent = readFileSync(cargoPath, "utf-8");
      expect(updatedContent).toContain('"pallets/pallet-custom"');
    });

    it("should add pallet-custom to workspace dependencies", () => {
      const cargoPath = join(TEMPLATE_DIR, "Cargo.toml");
      let content = readFileSync(cargoPath, "utf-8");

      // Check if already added
      if (content.includes("pallet-custom = ")) {
        console.log("pallet-custom already in workspace dependencies");
        return;
      }

      // Add to workspace dependencies after pallet-parachain-template
      const palletDepRegex = /(pallet-parachain-template\s*=\s*\{[^}]+\})/;
      const match = content.match(palletDepRegex);

      if (match) {
        content = content.replace(
          match[1],
          `${match[1]}\npallet-custom = { path = "./pallets/pallet-custom", default-features = false }`
        );
        writeFileSync(cargoPath, content);
        console.log("Added pallet-custom to workspace dependencies");
      } else {
        throw new Error("Could not find pallet-parachain-template in workspace dependencies");
      }

      // Verify
      const updatedContent = readFileSync(cargoPath, "utf-8");
      expect(updatedContent).toContain("pallet-custom = ");
    });
  });

  // ==================== CREATE PALLET FILES ====================
  describe("3. Create Pallet with Mock and Tests Modules", () => {
    it("should write lib.rs with mock and tests module declarations", () => {
      const libPath = join(PALLET_DIR, "src/lib.rs");

      // Check if already written with both modules
      if (existsSync(libPath)) {
        const content = readFileSync(libPath, "utf-8");
        if (content.includes("mod mock;") && content.includes("mod tests;")) {
          console.log("pallet-custom/src/lib.rs already has mock and tests module declarations");
          return;
        }
      }

      writeFileSync(libPath, PALLET_LIB_RS);
      console.log("Written pallet-custom/src/lib.rs with mock and tests module declarations");

      // Verify
      const content = readFileSync(libPath, "utf-8");
      expect(content).toContain("#[cfg(test)]");
      expect(content).toContain("mod mock;");
      expect(content).toContain("mod tests;");
      expect(content).toContain("pub struct Pallet<T>");
    });

    it("should create mock.rs with mock runtime configuration", () => {
      const mockPath = join(PALLET_DIR, "src/mock.rs");

      // Check if already written
      if (existsSync(mockPath)) {
        const content = readFileSync(mockPath, "utf-8");
        if (content.includes("construct_runtime!") && content.includes("new_test_ext")) {
          console.log("pallet-custom/src/mock.rs already configured");
          return;
        }
      }

      writeFileSync(mockPath, MOCK_RS);
      console.log("Written pallet-custom/src/mock.rs");

      // Verify mock runtime structure
      const content = readFileSync(mockPath, "utf-8");
      expect(content).toContain("use crate as pallet_custom");
      expect(content).toContain("construct_runtime!");
      expect(content).toContain("pub fn new_test_ext()");
      expect(content).toContain("pub fn new_test_ext_with_counter");
      expect(content).toContain("pub fn new_test_ext_with_interactions");
    });

    it("should create tests.rs with unit tests", () => {
      const testsPath = join(PALLET_DIR, "src/tests.rs");

      // Check if already written
      if (existsSync(testsPath)) {
        const content = readFileSync(testsPath, "utf-8");
        if (content.includes("set_counter_value_works") && content.includes("genesis_config_works")) {
          console.log("pallet-custom/src/tests.rs already configured");
          return;
        }
      }

      writeFileSync(testsPath, TESTS_RS);
      console.log("Written pallet-custom/src/tests.rs");

      // Verify test structure
      const content = readFileSync(testsPath, "utf-8");
      expect(content).toContain("use crate::{mock::*, Error, Event}");
      expect(content).toContain("assert_ok!");
      expect(content).toContain("assert_noop!");
    });
  });

  // ==================== VERIFY TEST STRUCTURE ====================
  describe("4. Verify Test Structure", () => {
    it("should have set_counter_value tests", () => {
      const testsPath = join(PALLET_DIR, "src/tests.rs");
      const content = readFileSync(testsPath, "utf-8");

      expect(content).toContain("fn set_counter_value_works()");
      expect(content).toContain("fn set_counter_value_requires_root()");
      expect(content).toContain("fn set_counter_value_respects_max_value()");
      console.log("set_counter_value tests verified");
    });

    it("should have increment tests", () => {
      const testsPath = join(PALLET_DIR, "src/tests.rs");
      const content = readFileSync(testsPath, "utf-8");

      expect(content).toContain("fn increment_works()");
      expect(content).toContain("fn increment_tracks_multiple_interactions()");
      expect(content).toContain("fn increment_fails_on_overflow()");
      expect(content).toContain("fn increment_respects_max_value()");
      console.log("increment tests verified");
    });

    it("should have decrement tests", () => {
      const testsPath = join(PALLET_DIR, "src/tests.rs");
      const content = readFileSync(testsPath, "utf-8");

      expect(content).toContain("fn decrement_works()");
      expect(content).toContain("fn decrement_fails_on_underflow()");
      expect(content).toContain("fn decrement_tracks_multiple_interactions()");
      console.log("decrement tests verified");
    });

    it("should have mixed operation and multi-user tests", () => {
      const testsPath = join(PALLET_DIR, "src/tests.rs");
      const content = readFileSync(testsPath, "utf-8");

      expect(content).toContain("fn mixed_increment_and_decrement_works()");
      expect(content).toContain("fn different_users_tracked_separately()");
      console.log("mixed operation and multi-user tests verified");
    });

    it("should have genesis config test", () => {
      const testsPath = join(PALLET_DIR, "src/tests.rs");
      const content = readFileSync(testsPath, "utf-8");

      expect(content).toContain("fn genesis_config_works()");
      expect(content).toContain("new_test_ext_with_interactions");
      console.log("genesis config test verified");
    });

    it("should use proper testing patterns", () => {
      const testsPath = join(PALLET_DIR, "src/tests.rs");
      const content = readFileSync(testsPath, "utf-8");

      // Check for proper imports
      expect(content).toContain("use frame::deps::frame_support::{assert_noop, assert_ok}");
      expect(content).toContain("use frame::deps::sp_runtime::DispatchError");

      // Check for event testing pattern
      expect(content).toContain("System::set_block_number(1)");
      expect(content).toContain("System::assert_last_event");

      console.log("Testing patterns verified");
    });
  });

  // ==================== RUN TESTS ====================
  describe("5. Build and Run Unit Tests", () => {
    it("should build the pallet", () => {
      console.log("Building pallet-custom...");

      execSync("cargo build --package pallet-custom", {
        cwd: TEMPLATE_DIR,
        encoding: "utf-8",
        stdio: "inherit",
        timeout: 600000,
      });

      console.log("pallet-custom built successfully");
    }, 600000);

    it("should run all unit tests successfully", () => {
      console.log("Running cargo test --package pallet-custom...");

      const result = execSync("cargo test --package pallet-custom -- --nocapture", {
        cwd: TEMPLATE_DIR,
        encoding: "utf-8",
        timeout: 600000,
      });

      console.log(result);

      // Verify all tests passed
      expect(result).toContain("test result: ok");
      expect(result).not.toContain("FAILED");

      // Count the tests
      const testMatch = result.match(/(\d+) passed/);
      if (testMatch) {
        console.log(`All ${testMatch[1]} unit tests passed`);
      }
    }, 600000);
  });
});
