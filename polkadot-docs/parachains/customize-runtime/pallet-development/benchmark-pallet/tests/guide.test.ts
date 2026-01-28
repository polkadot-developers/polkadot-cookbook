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

// Pallet Cargo.toml with runtime-benchmarks feature
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
runtime-benchmarks = [
    "frame/runtime-benchmarks",
]
std = [
    "codec/std",
    "scale-info/std",
    "frame/std",
]
`;

// Complete pallet lib.rs with WeightInfo and benchmarking module
const PALLET_LIB_RS = `#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub mod weights;
pub use weights::*;

#[frame::pallet]
pub mod pallet {
    use alloc::vec::Vec;
    use frame::prelude::*;
    use crate::weights::WeightInfo;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        #[pallet::constant]
        type CounterMaxValue: Get<u32>;

        /// Weight information for extrinsics in this pallet.
        type WeightInfo: WeightInfo;
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
        #[pallet::weight(T::WeightInfo::set_counter_value())]
        pub fn set_counter_value(origin: OriginFor<T>, new_value: u32) -> DispatchResult {
            ensure_root(origin)?;
            ensure!(new_value <= T::CounterMaxValue::get(), Error::<T>::CounterMaxValueExceeded);
            CounterValue::<T>::put(new_value);
            Self::deposit_event(Event::CounterValueSet { new_value });
            Ok(())
        }

        #[pallet::call_index(1)]
        #[pallet::weight(T::WeightInfo::increment())]
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
        #[pallet::weight(T::WeightInfo::decrement())]
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

// Weights module with WeightInfo trait
const WEIGHTS_RS = `use frame::prelude::*;

pub trait WeightInfo {
    fn set_counter_value() -> Weight;
    fn increment() -> Weight;
    fn decrement() -> Weight;
}

/// Weights for pallet_custom using the Substrate node and target hardware benchmark.
pub struct SubstrateWeight<T>(core::marker::PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
    fn set_counter_value() -> Weight {
        Weight::from_parts(10_000, 0)
            .saturating_add(T::DbWeight::get().writes(1))
    }
    fn increment() -> Weight {
        Weight::from_parts(15_000, 0)
            .saturating_add(T::DbWeight::get().reads(1))
            .saturating_add(T::DbWeight::get().writes(2))
    }
    fn decrement() -> Weight {
        Weight::from_parts(15_000, 0)
            .saturating_add(T::DbWeight::get().reads(1))
            .saturating_add(T::DbWeight::get().writes(2))
    }
}

// For testing
impl WeightInfo for () {
    fn set_counter_value() -> Weight {
        Weight::from_parts(10_000, 0)
    }
    fn increment() -> Weight {
        Weight::from_parts(15_000, 0)
    }
    fn decrement() -> Weight {
        Weight::from_parts(15_000, 0)
    }
}
`;

// Benchmarking module
const BENCHMARKING_RS = `#![cfg(feature = "runtime-benchmarks")]

use super::*;
use frame::deps::frame_benchmarking::v2::*;
use frame::benchmarking::prelude::RawOrigin;

#[benchmarks]
mod benchmarks {
    use super::*;

    #[benchmark]
    fn set_counter_value() {
        let new_value: u32 = 100;

        #[extrinsic_call]
        _(RawOrigin::Root, new_value);

        assert_eq!(pallet::CounterValue::<T>::get(), new_value);
    }

    #[benchmark]
    fn increment() {
        let caller: T::AccountId = whitelisted_caller();
        let amount: u32 = 50;

        #[extrinsic_call]
        _(RawOrigin::Signed(caller.clone()), amount);

        assert_eq!(pallet::CounterValue::<T>::get(), amount);
        assert_eq!(pallet::UserInteractions::<T>::get(&caller), 1);
    }

    #[benchmark]
    fn decrement() {
        pallet::CounterValue::<T>::put(100);

        let caller: T::AccountId = whitelisted_caller();
        let amount: u32 = 30;

        #[extrinsic_call]
        _(RawOrigin::Signed(caller.clone()), amount);

        assert_eq!(pallet::CounterValue::<T>::get(), 70);
        assert_eq!(pallet::UserInteractions::<T>::get(&caller), 1);
    }

    impl_benchmark_test_suite!(Pallet, crate::mock::new_test_ext(), crate::mock::Test);
}
`;

// Mock runtime with WeightInfo
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
    type WeightInfo = ();
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

// Unit tests (same as pallet-testing)
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
fn increment_works() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        let account = 1u64;
        assert_ok!(CustomPallet::increment(RuntimeOrigin::signed(account), 50));
        assert_eq!(crate::CounterValue::<Test>::get(), 50);
        assert_eq!(crate::UserInteractions::<Test>::get(account), 1);
    });
}

#[test]
fn decrement_works() {
    new_test_ext_with_counter(100).execute_with(|| {
        System::set_block_number(1);
        let account = 2u64;
        assert_ok!(CustomPallet::decrement(RuntimeOrigin::signed(account), 30));
        assert_eq!(crate::CounterValue::<Test>::get(), 70);
        assert_eq!(crate::UserInteractions::<Test>::get(account), 1);
    });
}
`;

// All tests in a single describe block to ensure sequential execution
describe("Benchmark Pallets Guide", () => {
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

  // ==================== CLONE AND CREATE PALLET ====================
  describe("2. Clone Template and Create Pallet", () => {
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
      if (!existsSync(PALLET_DIR)) {
        mkdirSync(join(PALLET_DIR, "src"), { recursive: true });
        console.log("Created pallet-custom directory");
      } else {
        console.log("pallet-custom directory already exists");
      }

      expect(existsSync(PALLET_DIR)).toBe(true);
    });

    it("should write pallet Cargo.toml with runtime-benchmarks feature", () => {
      const cargoPath = join(PALLET_DIR, "Cargo.toml");

      if (existsSync(cargoPath)) {
        const content = readFileSync(cargoPath, "utf-8");
        if (content.includes('name = "pallet-custom"') && content.includes("runtime-benchmarks")) {
          console.log("pallet-custom/Cargo.toml already configured with benchmarks");
          return;
        }
      }

      writeFileSync(cargoPath, PALLET_CARGO_TOML);
      console.log("Written pallet-custom/Cargo.toml with runtime-benchmarks feature");

      const content = readFileSync(cargoPath, "utf-8");
      expect(content).toContain('name = "pallet-custom"');
      expect(content).toContain("runtime-benchmarks");
      expect(content).toContain("frame/runtime-benchmarks");
    });

    it("should add pallet-custom to workspace members", () => {
      const cargoPath = join(TEMPLATE_DIR, "Cargo.toml");
      let content = readFileSync(cargoPath, "utf-8");

      if (content.includes('"pallets/pallet-custom"')) {
        console.log("pallet-custom already in workspace members");
        return;
      }

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

      const updatedContent = readFileSync(cargoPath, "utf-8");
      expect(updatedContent).toContain('"pallets/pallet-custom"');
    });

    it("should add pallet-custom to workspace dependencies", () => {
      const cargoPath = join(TEMPLATE_DIR, "Cargo.toml");
      let content = readFileSync(cargoPath, "utf-8");

      if (content.includes("pallet-custom = ")) {
        console.log("pallet-custom already in workspace dependencies");
        return;
      }

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

      const updatedContent = readFileSync(cargoPath, "utf-8");
      expect(updatedContent).toContain("pallet-custom = ");
    });
  });

  // ==================== CREATE PALLET FILES ====================
  describe("3. Create Pallet with Benchmarking Support", () => {
    it("should write lib.rs with WeightInfo and benchmarking module", () => {
      const libPath = join(PALLET_DIR, "src/lib.rs");

      if (existsSync(libPath)) {
        const content = readFileSync(libPath, "utf-8");
        if (content.includes("mod benchmarking;") && content.includes("WeightInfo")) {
          console.log("pallet-custom/src/lib.rs already has benchmarking support");
          return;
        }
      }

      writeFileSync(libPath, PALLET_LIB_RS);
      console.log("Written pallet-custom/src/lib.rs with benchmarking support");

      const content = readFileSync(libPath, "utf-8");
      expect(content).toContain('#[cfg(feature = "runtime-benchmarks")]');
      expect(content).toContain("mod benchmarking;");
      expect(content).toContain("pub mod weights;");
      expect(content).toContain("type WeightInfo: WeightInfo;");
      expect(content).toContain("T::WeightInfo::set_counter_value()");
    });

    it("should create weights.rs with WeightInfo trait", () => {
      const weightsPath = join(PALLET_DIR, "src/weights.rs");

      if (existsSync(weightsPath)) {
        const content = readFileSync(weightsPath, "utf-8");
        if (content.includes("pub trait WeightInfo") && content.includes("SubstrateWeight")) {
          console.log("pallet-custom/src/weights.rs already configured");
          return;
        }
      }

      writeFileSync(weightsPath, WEIGHTS_RS);
      console.log("Written pallet-custom/src/weights.rs");

      const content = readFileSync(weightsPath, "utf-8");
      expect(content).toContain("pub trait WeightInfo");
      expect(content).toContain("fn set_counter_value() -> Weight");
      expect(content).toContain("fn increment() -> Weight");
      expect(content).toContain("fn decrement() -> Weight");
      expect(content).toContain("pub struct SubstrateWeight");
      expect(content).toContain("impl WeightInfo for ()");
    });

    it("should create benchmarking.rs with benchmark functions", () => {
      const benchmarkingPath = join(PALLET_DIR, "src/benchmarking.rs");

      if (existsSync(benchmarkingPath)) {
        const content = readFileSync(benchmarkingPath, "utf-8");
        if (content.includes("#[benchmarks]") && content.includes("impl_benchmark_test_suite")) {
          console.log("pallet-custom/src/benchmarking.rs already configured");
          return;
        }
      }

      writeFileSync(benchmarkingPath, BENCHMARKING_RS);
      console.log("Written pallet-custom/src/benchmarking.rs");

      const content = readFileSync(benchmarkingPath, "utf-8");
      expect(content).toContain('#![cfg(feature = "runtime-benchmarks")]');
      expect(content).toContain("#[benchmarks]");
      expect(content).toContain("#[benchmark]");
      expect(content).toContain("fn set_counter_value()");
      expect(content).toContain("fn increment()");
      expect(content).toContain("fn decrement()");
      expect(content).toContain("#[extrinsic_call]");
      expect(content).toContain("impl_benchmark_test_suite!");
    });

    it("should create mock.rs with WeightInfo = ()", () => {
      const mockPath = join(PALLET_DIR, "src/mock.rs");

      if (existsSync(mockPath)) {
        const content = readFileSync(mockPath, "utf-8");
        if (content.includes("type WeightInfo = ()")) {
          console.log("pallet-custom/src/mock.rs already configured");
          return;
        }
      }

      writeFileSync(mockPath, MOCK_RS);
      console.log("Written pallet-custom/src/mock.rs");

      const content = readFileSync(mockPath, "utf-8");
      expect(content).toContain("type WeightInfo = ()");
    });

    it("should create tests.rs with unit tests", () => {
      const testsPath = join(PALLET_DIR, "src/tests.rs");

      if (existsSync(testsPath)) {
        const content = readFileSync(testsPath, "utf-8");
        if (content.includes("set_counter_value_works") && content.includes("increment_works")) {
          console.log("pallet-custom/src/tests.rs already configured");
          return;
        }
      }

      writeFileSync(testsPath, TESTS_RS);
      console.log("Written pallet-custom/src/tests.rs");

      const content = readFileSync(testsPath, "utf-8");
      expect(content).toContain("fn set_counter_value_works()");
      expect(content).toContain("fn increment_works()");
      expect(content).toContain("fn decrement_works()");
    });
  });

  // ==================== VERIFY STRUCTURE ====================
  describe("4. Verify Benchmarking Structure", () => {
    it("should have WeightInfo trait with all dispatchables", () => {
      const weightsPath = join(PALLET_DIR, "src/weights.rs");
      const content = readFileSync(weightsPath, "utf-8");

      expect(content).toContain("fn set_counter_value() -> Weight");
      expect(content).toContain("fn increment() -> Weight");
      expect(content).toContain("fn decrement() -> Weight");
      console.log("WeightInfo trait has all dispatchable weights");
    });

    it("should have benchmarks for all dispatchables", () => {
      const benchmarkingPath = join(PALLET_DIR, "src/benchmarking.rs");
      const content = readFileSync(benchmarkingPath, "utf-8");

      // Count #[benchmark] attributes
      const benchmarkCount = (content.match(/#\[benchmark\]/g) || []).length;
      expect(benchmarkCount).toBe(3);
      console.log(`Found ${benchmarkCount} benchmarks (set_counter_value, increment, decrement)`);
    });

    it("should have benchmark test suite macro", () => {
      const benchmarkingPath = join(PALLET_DIR, "src/benchmarking.rs");
      const content = readFileSync(benchmarkingPath, "utf-8");

      expect(content).toContain("impl_benchmark_test_suite!");
      expect(content).toContain("crate::mock::new_test_ext()");
      expect(content).toContain("crate::mock::Test");
      console.log("Benchmark test suite macro verified");
    });

    it("should use T::WeightInfo in pallet calls", () => {
      const libPath = join(PALLET_DIR, "src/lib.rs");
      const content = readFileSync(libPath, "utf-8");

      expect(content).toContain("#[pallet::weight(T::WeightInfo::set_counter_value())]");
      expect(content).toContain("#[pallet::weight(T::WeightInfo::increment())]");
      expect(content).toContain("#[pallet::weight(T::WeightInfo::decrement())]");
      console.log("Pallet calls use T::WeightInfo for weights");
    });
  });

  // ==================== BUILD AND TEST ====================
  describe("5. Build and Test Benchmarks", () => {
    it("should build the pallet without benchmarks", () => {
      console.log("Building pallet-custom...");

      execSync("cargo build --package pallet-custom", {
        cwd: TEMPLATE_DIR,
        encoding: "utf-8",
        stdio: "inherit",
        timeout: 600000,
      });

      console.log("pallet-custom built successfully");
    }, 600000);

    it("should run unit tests", () => {
      console.log("Running unit tests...");

      const result = execSync("cargo test --package pallet-custom --lib -- --nocapture", {
        cwd: TEMPLATE_DIR,
        encoding: "utf-8",
        timeout: 600000,
      });

      expect(result).toContain("test result: ok");
      console.log("Unit tests passed");
    }, 600000);

    it("should build with runtime-benchmarks feature", () => {
      console.log("Building pallet-custom with runtime-benchmarks feature...");

      execSync("cargo build --package pallet-custom --features runtime-benchmarks", {
        cwd: TEMPLATE_DIR,
        encoding: "utf-8",
        stdio: "inherit",
        timeout: 600000,
      });

      console.log("pallet-custom built with runtime-benchmarks feature");
    }, 600000);

    it("should run benchmark tests", () => {
      console.log("Running benchmark tests with cargo test --features runtime-benchmarks...");

      const result = execSync("cargo test --package pallet-custom --features runtime-benchmarks -- --nocapture", {
        cwd: TEMPLATE_DIR,
        encoding: "utf-8",
        timeout: 600000,
      });

      // The benchmark test suite generates tests like benchmark_set_counter_value, etc.
      expect(result).toContain("test result: ok");
      console.log("Benchmark tests passed");
    }, 600000);
  });
});
