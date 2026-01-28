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
// This is needed as the mock-runtime guide builds on top of the create-a-pallet guide
const PALLET_LIB_RS = `#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub use pallet::*;

#[cfg(test)]
mod mock;

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

// Mock runtime from the guide
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

// All tests in a single describe block to ensure sequential execution
describe("Mock Your Runtime Guide", () => {
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

  // ==================== MOCK RUNTIME SETUP TESTS ====================
  describe("3. Create Mock Runtime Module", () => {
    it("should write lib.rs with mock module declaration", () => {
      const libPath = join(PALLET_DIR, "src/lib.rs");

      // Check if already written with mock module
      if (existsSync(libPath)) {
        const content = readFileSync(libPath, "utf-8");
        if (content.includes("#[cfg(test)]") && content.includes("mod mock;")) {
          console.log("pallet-custom/src/lib.rs already has mock module declaration");
          return;
        }
      }

      writeFileSync(libPath, PALLET_LIB_RS);
      console.log("Written pallet-custom/src/lib.rs with mock module declaration");

      // Verify
      const content = readFileSync(libPath, "utf-8");
      expect(content).toContain("#[cfg(test)]");
      expect(content).toContain("mod mock;");
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
      expect(content).toContain("pub enum Test");
      expect(content).toContain("System: frame_system");
      expect(content).toContain("CustomPallet: pallet_custom");
    });

    it("should have frame_system::Config implementation", () => {
      const mockPath = join(PALLET_DIR, "src/mock.rs");
      const content = readFileSync(mockPath, "utf-8");

      expect(content).toContain("#[derive_impl(frame_system::config_preludes::TestDefaultConfig)]");
      expect(content).toContain("impl frame_system::Config for Test");
      expect(content).toContain("type Block = Block");
      expect(content).toContain("type AccountId = u64");
      expect(content).toContain("type Lookup = IdentityLookup<Self::AccountId>");
      console.log("frame_system::Config implementation verified");
    });

    it("should have pallet_custom::Config implementation", () => {
      const mockPath = join(PALLET_DIR, "src/mock.rs");
      const content = readFileSync(mockPath, "utf-8");

      expect(content).toContain("impl pallet_custom::Config for Test");
      expect(content).toContain("type RuntimeEvent = RuntimeEvent");
      expect(content).toContain("type CounterMaxValue = ConstU32<1000>");
      console.log("pallet_custom::Config implementation verified");
    });

    it("should have new_test_ext helper function", () => {
      const mockPath = join(PALLET_DIR, "src/mock.rs");
      const content = readFileSync(mockPath, "utf-8");

      expect(content).toContain("pub fn new_test_ext()");
      expect(content).toContain("sp_io::TestExternalities");
      expect(content).toContain("build_storage()");
      expect(content).toContain("assimilate_storage");
      console.log("new_test_ext helper function verified");
    });

    it("should have custom genesis helper functions", () => {
      const mockPath = join(PALLET_DIR, "src/mock.rs");
      const content = readFileSync(mockPath, "utf-8");

      expect(content).toContain("pub fn new_test_ext_with_counter");
      expect(content).toContain("pub fn new_test_ext_with_interactions");
      expect(content).toContain("initial_counter_value:");
      expect(content).toContain("initial_user_interactions:");
      console.log("Custom genesis helper functions verified");
    });
  });

  // ==================== BUILD AND VERIFY TESTS ====================
  describe("4. Build and Verify Mock Runtime", () => {
    it("should build the pallet with mock runtime", () => {
      console.log("Building pallet-custom with mock runtime...");

      execSync("cargo build --package pallet-custom", {
        cwd: TEMPLATE_DIR,
        encoding: "utf-8",
        stdio: "inherit",
        timeout: 600000,
      });

      console.log("pallet-custom built successfully");
    }, 600000);

    it("should compile mock runtime tests successfully", () => {
      console.log("Compiling mock runtime tests with cargo test --package pallet-custom --lib --no-run...");

      // This command compiles the tests without running them
      execSync("cargo test --package pallet-custom --lib --no-run", {
        cwd: TEMPLATE_DIR,
        encoding: "utf-8",
        stdio: "inherit",
        timeout: 600000,
      });

      console.log("Mock runtime tests compiled successfully");
    }, 600000);

    it("should pass cargo check for the pallet", () => {
      console.log("Running cargo check on pallet-custom...");

      execSync("cargo check --package pallet-custom", {
        cwd: TEMPLATE_DIR,
        encoding: "utf-8",
        stdio: "inherit",
        timeout: 300000,
      });

      console.log("cargo check passed");
    }, 300000);
  });
});
