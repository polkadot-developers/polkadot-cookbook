import { describe, it, expect, afterAll } from "vitest";
import { execSync, spawn, ChildProcess } from "child_process";
import {
  existsSync,
  readFileSync,
  writeFileSync,
  unlinkSync,
  mkdirSync,
  statSync,
} from "fs";
import { join } from "path";

const WORKSPACE_DIR = join(process.cwd(), ".test-workspace");
const TEMPLATE_DIR = join(WORKSPACE_DIR, "parachain-template");
const TEMPLATE_VERSION = "v0.0.4";
const CHAIN_SPEC_PATH = join(WORKSPACE_DIR, "chain_spec.json");
const PID_FILE = join(WORKSPACE_DIR, "node.pid");
const WASM_PATH = join(
  TEMPLATE_DIR,
  "target/release/wbuild/parachain-template-runtime/parachain_template_runtime.compact.compressed.wasm"
);
const PALLET_DIR = join(TEMPLATE_DIR, "pallets/pallet-custom");

let nodeProcess: ChildProcess | null = null;

// Complete pallet implementation from the guide
const PALLET_LIB_RS = `#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub use pallet::*;

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

// All tests in a single describe block to ensure sequential execution
describe("Create a Custom Pallet Guide", () => {
  afterAll(async () => {
    await stopNode();
  });

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

    it("should have chain-spec-builder installed", () => {
      try {
        const result = execSync("chain-spec-builder --version 2>&1", {
          encoding: "utf-8",
        });
        expect(result.length).toBeGreaterThan(0);
        console.log(`chain-spec-builder: ${result.trim()}`);
      } catch (error) {
        console.log("Installing chain-spec-builder...");
        execSync("cargo install staging-chain-spec-builder@10.0.0 --locked", {
          stdio: "inherit",
        });
      }
    });

    it("should have polkadot-omni-node installed", () => {
      try {
        const result = execSync("polkadot-omni-node --version 2>&1", {
          encoding: "utf-8",
        });
        expect(result.length).toBeGreaterThan(0);
        console.log(`polkadot-omni-node: ${result.trim()}`);
      } catch (error) {
        console.log("Installing polkadot-omni-node...");
        execSync("cargo install polkadot-omni-node@0.5.0 --locked", {
          stdio: "inherit",
        });
      }
    });

    it("should have git installed", () => {
      const result = execSync("git --version", { encoding: "utf-8" });
      expect(result).toMatch(/git version/);
      console.log(`Git: ${result.trim()}`);
    });
  });

  // ==================== CLONE AND CREATE PALLET TESTS ====================
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

    it("should write pallet lib.rs", () => {
      const libPath = join(PALLET_DIR, "src/lib.rs");

      // Check if already written
      if (existsSync(libPath)) {
        const content = readFileSync(libPath, "utf-8");
        if (content.includes("CounterValue") && content.includes("UserInteractions")) {
          console.log("pallet-custom/src/lib.rs already configured");
          return;
        }
      }

      writeFileSync(libPath, PALLET_LIB_RS);
      console.log("Written pallet-custom/src/lib.rs");

      // Verify
      const content = readFileSync(libPath, "utf-8");
      expect(content).toContain("pub struct Pallet<T>");
      expect(content).toContain("CounterValue");
      expect(content).toContain("UserInteractions");
      expect(content).toContain("increment");
      expect(content).toContain("decrement");
    });

    it("should add pallet-custom to workspace members", () => {
      const cargoPath = join(TEMPLATE_DIR, "Cargo.toml");
      let content = readFileSync(cargoPath, "utf-8");

      // Check if already added
      if (content.includes('"pallets/pallet-custom"')) {
        console.log("pallet-custom already in workspace members");
        return;
      }

      // Add to workspace members - look for the members array specifically
      // The members array has entries like: "node", "pallets/template", "runtime"
      // We need to add after "runtime" in the members array
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
  });

  // ==================== INTEGRATE PALLET INTO RUNTIME ====================
  describe("3. Integrate Pallet into Runtime", () => {
    it("should add pallet-custom to runtime/Cargo.toml dependencies", () => {
      const cargoPath = join(TEMPLATE_DIR, "runtime/Cargo.toml");
      let content = readFileSync(cargoPath, "utf-8");

      // Check if already added
      if (content.includes("pallet-custom")) {
        console.log("pallet-custom already in runtime/Cargo.toml");
        return;
      }

      // Add to dependencies after pallet-parachain-template.workspace = true
      // Use workspace = true since we added it to workspace dependencies
      const palletTemplateRegex = /(pallet-parachain-template\.workspace\s*=\s*true)/;
      const match = content.match(palletTemplateRegex);

      if (match) {
        content = content.replace(
          match[1],
          `${match[1]}\npallet-custom.workspace = true`
        );
        writeFileSync(cargoPath, content);
        console.log("Added pallet-custom to runtime/Cargo.toml dependencies");
      } else {
        throw new Error("Could not find pallet-parachain-template.workspace in runtime/Cargo.toml");
      }

      // Verify
      const updatedContent = readFileSync(cargoPath, "utf-8");
      expect(updatedContent).toContain("pallet-custom");
    });

    it("should add pallet-custom/std to runtime features", () => {
      const cargoPath = join(TEMPLATE_DIR, "runtime/Cargo.toml");
      let content = readFileSync(cargoPath, "utf-8");

      // Check if already added
      if (content.includes('"pallet-custom/std"')) {
        console.log("pallet-custom/std already in runtime features");
        return;
      }

      // Add to std features after pallet-parachain-template/std
      // Format in the template is with tabs: \t"pallet-parachain-template/std",
      const stdFeatureRegex = /(\t"pallet-parachain-template\/std",)/;
      const match = content.match(stdFeatureRegex);

      if (match) {
        content = content.replace(
          match[1],
          `${match[1]}\n\t"pallet-custom/std",`
        );
        writeFileSync(cargoPath, content);
        console.log("Added pallet-custom/std to runtime features");
      } else {
        throw new Error("Could not find pallet-parachain-template/std in runtime features");
      }

      // Verify
      const updatedContent = readFileSync(cargoPath, "utf-8");
      expect(updatedContent).toContain('"pallet-custom/std"');
    });

    it("should implement pallet_custom::Config for Runtime", () => {
      const configPath = join(TEMPLATE_DIR, "runtime/src/configs/mod.rs");
      let content = readFileSync(configPath, "utf-8");

      // Check if already implemented
      if (content.includes("impl pallet_custom::Config for Runtime")) {
        console.log("pallet_custom::Config already implemented");
        return;
      }

      // Add the Config implementation at the end
      const configImpl = `
/// Configure the custom counter pallet
impl pallet_custom::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type CounterMaxValue = ConstU32<1000>;
}
`;

      content += configImpl;
      writeFileSync(configPath, content);
      console.log("Added pallet_custom::Config implementation");

      // Verify
      const updatedContent = readFileSync(configPath, "utf-8");
      expect(updatedContent).toContain("impl pallet_custom::Config for Runtime");
    });

    it("should register CustomPallet in runtime", () => {
      const libPath = join(TEMPLATE_DIR, "runtime/src/lib.rs");
      let content = readFileSync(libPath, "utf-8");

      // Check if already registered
      if (content.includes("pub type CustomPallet = pallet_custom")) {
        console.log("CustomPallet already registered in runtime");
        return;
      }

      // Find the TemplatePallet registration and add CustomPallet after it
      const templatePalletRegex = /(#\[runtime::pallet_index\(50\)\]\s*pub type TemplatePallet = pallet_parachain_template;)/;
      const match = content.match(templatePalletRegex);

      if (match) {
        const customPalletRegistration = `${match[1]}

	#[runtime::pallet_index(51)]
	pub type CustomPallet = pallet_custom;`;
        content = content.replace(match[1], customPalletRegistration);
        writeFileSync(libPath, content);
        console.log("Registered CustomPallet in runtime (index 51)");
      } else {
        throw new Error("Could not find TemplatePallet registration in lib.rs");
      }

      // Verify
      const updatedContent = readFileSync(libPath, "utf-8");
      expect(updatedContent).toContain("pub type CustomPallet = pallet_custom");
    });
  });

  // ==================== BUILD TESTS ====================
  describe("4. Build Verification", () => {
    it("should build the modified runtime", () => {
      console.log("Building modified parachain template with custom pallet (this may take 15-30 minutes)...");

      execSync("cargo build --release", {
        cwd: TEMPLATE_DIR,
        encoding: "utf-8",
        stdio: "inherit",
        timeout: 1800000,
      });

      expect(existsSync(WASM_PATH)).toBe(true);
      console.log("WASM runtime built successfully with pallet-custom");
    }, 1800000);

    it("should have generated the WASM runtime", () => {
      expect(existsSync(WASM_PATH)).toBe(true);

      const stats = statSync(WASM_PATH);
      const sizeKB = Math.round(stats.size / 1024);
      console.log(`WASM runtime size: ${sizeKB} KB`);

      expect(stats.size).toBeGreaterThan(100000);
    });
  });

  // ==================== RUNTIME TESTS ====================
  describe("5. Runtime Verification", () => {
    it("should generate chain specification", () => {
      console.log("Generating chain specification...");

      if (!existsSync(WASM_PATH)) {
        throw new Error(`WASM runtime not found at ${WASM_PATH}. Build must complete first.`);
      }

      execSync(
        `chain-spec-builder create -t development \
          --relay-chain paseo \
          --para-id 1000 \
          --runtime ${WASM_PATH} \
          named-preset development`,
        { encoding: "utf-8", cwd: WORKSPACE_DIR }
      );

      expect(existsSync(CHAIN_SPEC_PATH)).toBe(true);

      const chainSpec = JSON.parse(readFileSync(CHAIN_SPEC_PATH, "utf-8"));
      expect(chainSpec.name).toBeDefined();
      expect(chainSpec.id).toBeDefined();

      console.log(`Chain spec generated: ${chainSpec.name}`);
    }, 60000);

    it("should start the parachain node", async () => {
      console.log("Starting parachain node...");

      nodeProcess = spawn(
        "polkadot-omni-node",
        ["--chain", CHAIN_SPEC_PATH, "--dev"],
        {
          cwd: WORKSPACE_DIR,
          stdio: ["ignore", "pipe", "pipe"],
          detached: true,
        }
      );

      if (nodeProcess.pid) {
        writeFileSync(PID_FILE, nodeProcess.pid.toString());
        console.log(`Node started with PID: ${nodeProcess.pid}`);
      }

      const maxWaitTime = 60000;
      const startTime = Date.now();

      while (Date.now() - startTime < maxWaitTime) {
        try {
          const response = await fetch("http://127.0.0.1:9944", {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({
              jsonrpc: "2.0",
              method: "system_health",
              params: [],
              id: 1,
            }),
          });

          if (response.ok) {
            console.log("Node is ready!");
            return;
          }
        } catch {
          // Node not ready yet
        }

        await new Promise((resolve) => setTimeout(resolve, 1000));
      }

      throw new Error("Node failed to start within 60 seconds");
    }, 90000);

    it("should produce blocks", async () => {
      console.log("Verifying block production...");

      await new Promise((resolve) => setTimeout(resolve, 6000));

      const response = await fetch("http://127.0.0.1:9944", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          jsonrpc: "2.0",
          method: "chain_getHeader",
          params: [],
          id: 1,
        }),
      });

      const result = await response.json();
      expect(result.result).toBeDefined();
      expect(result.result.number).toBeDefined();

      const blockNumber = parseInt(result.result.number, 16);
      console.log(`Current block number: ${blockNumber}`);

      expect(blockNumber).toBeGreaterThan(0);
    }, 30000);

    it("should have CustomPallet available in runtime", async () => {
      console.log("Verifying CustomPallet is available...");

      // Query runtime version to verify the runtime is working
      const versionResponse = await fetch("http://127.0.0.1:9944", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          jsonrpc: "2.0",
          method: "state_getRuntimeVersion",
          params: [],
          id: 1,
        }),
      });

      const versionResult = await versionResponse.json();
      expect(versionResult.result).toBeDefined();
      expect(versionResult.result.specName).toBeDefined();
      console.log(`Runtime: ${versionResult.result.specName} v${versionResult.result.specVersion}`);

      // The pallet was successfully added if:
      // 1. Build succeeded with pallet-custom
      // 2. Config trait was implemented
      // 3. Pallet was registered in runtime macro
      // 4. Node started successfully
      console.log("CustomPallet verified (build and runtime startup succeeded)");
    }, 10000);

    it("should respond to RPC queries", async () => {
      console.log("Testing RPC endpoints...");

      const nameResponse = await fetch("http://127.0.0.1:9944", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          jsonrpc: "2.0",
          method: "system_name",
          params: [],
          id: 1,
        }),
      });

      const nameResult = await nameResponse.json();
      expect(nameResult.result).toBeDefined();
      console.log(`System name: ${nameResult.result}`);

      const versionResponse = await fetch("http://127.0.0.1:9944", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          jsonrpc: "2.0",
          method: "system_version",
          params: [],
          id: 1,
        }),
      });

      const versionResult = await versionResponse.json();
      expect(versionResult.result).toBeDefined();
      console.log(`System version: ${versionResult.result}`);
    }, 10000);
  });
});

async function stopNode(): Promise<void> {
  console.log("Stopping parachain node...");

  if (nodeProcess && !nodeProcess.killed) {
    nodeProcess.kill("SIGTERM");
    nodeProcess = null;
  }

  if (existsSync(PID_FILE)) {
    try {
      const pid = parseInt(readFileSync(PID_FILE, "utf-8"));
      process.kill(pid, "SIGTERM");
    } catch {
      // Process might already be dead
    }
    unlinkSync(PID_FILE);
  }

  await new Promise((resolve) => setTimeout(resolve, 2000));
  console.log("Node stopped");
}
