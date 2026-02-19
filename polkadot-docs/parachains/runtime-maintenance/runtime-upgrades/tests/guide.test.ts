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
import { ApiPromise, WsProvider } from "@polkadot/api";
import { Keyring } from "@polkadot/keyring";

const PROJECT_DIR = process.cwd();
const WORKSPACE_DIR = join(PROJECT_DIR, ".test-workspace");
const TEMPLATE_DIR = join(WORKSPACE_DIR, "parachain-template");
const TEMPLATE_VERSION = process.env.TEMPLATE_VERSION!;
const POLKADOT_SDK_VERSION = process.env.POLKADOT_SDK_VERSION!;
const BIN_DIR = join(WORKSPACE_DIR, "bin");
const CHAIN_SPEC_PATH = join(WORKSPACE_DIR, "chain_spec.json");
const PID_FILE = join(WORKSPACE_DIR, "zombienet.pid");
const WASM_PATH = join(
  TEMPLATE_DIR,
  "target/release/wbuild/parachain-template-runtime/parachain_template_runtime.compact.compressed.wasm"
);
const PALLET_DIR = join(TEMPLATE_DIR, "pallets/pallet-custom");
const POLKADOT_BINARY = join(BIN_DIR, "polkadot");
const POLKADOT_PREPARE_WORKER = join(BIN_DIR, "polkadot-prepare-worker");
const POLKADOT_EXECUTE_WORKER = join(BIN_DIR, "polkadot-execute-worker");

// Parachain collator RPC port
const PARACHAIN_RPC_PORT = 9988;
const PARACHAIN_RPC_URL = `http://127.0.0.1:${PARACHAIN_RPC_PORT}`;
const PARACHAIN_WS_URL = `ws://127.0.0.1:${PARACHAIN_RPC_PORT}`;

let zombienetProcess: ChildProcess | null = null;

// Complete pallet implementation from the Create a Custom Pallet guide (prerequisite)
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

/** Helper: JSON-RPC call to the parachain node */
async function rpcCall(method: string, params: unknown[] = []): Promise<unknown> {
  const response = await fetch(PARACHAIN_RPC_URL, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ jsonrpc: "2.0", method, params, id: 1 }),
  });
  const json = (await response.json()) as { result?: unknown; error?: unknown };
  if (json.error) throw new Error(`RPC error: ${JSON.stringify(json.error)}`);
  return json.result;
}

// All tests in a single describe block to ensure sequential execution
describe("Runtime Upgrades Tutorial", () => {
  afterAll(async () => {
    await stopZombienet();
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
      } catch {
        console.log("Installing chain-spec-builder...");
        execSync(`cargo install staging-chain-spec-builder@${process.env.CHAIN_SPEC_BUILDER_VERSION} --locked`, {
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
      } catch {
        console.log("Installing polkadot-omni-node...");
        execSync(`cargo install polkadot-omni-node@${process.env.POLKADOT_OMNI_NODE_VERSION} --locked`, {
          stdio: "inherit",
        });
      }
    });

    it("should have Zombienet installed", () => {
      const result = execSync("zombienet version 2>&1 || zombienet --version 2>&1", {
        encoding: "utf-8",
      });
      expect(result.length).toBeGreaterThan(0);
      console.log(`Zombienet: ${result.trim()}`);
    });

    it("should have git installed", () => {
      const result = execSync("git --version", { encoding: "utf-8" });
      expect(result).toMatch(/git version/);
      console.log(`Git: ${result.trim()}`);
    });
  });

  // ==================== PREREQUISITE: CREATE CUSTOM PALLET ====================
  describe("2. Set Up Parachain with Custom Pallet (Prerequisite)", () => {
    it("should create workspace directory", () => {
      if (!existsSync(WORKSPACE_DIR)) {
        mkdirSync(WORKSPACE_DIR, { recursive: true });
      }
      expect(existsSync(WORKSPACE_DIR)).toBe(true);
    });

    it("should clone the parachain template repository", () => {
      if (existsSync(TEMPLATE_DIR)) {
        console.log(`Template already cloned, resetting to ${TEMPLATE_VERSION}...`);
        execSync(`git checkout -- . && git clean -fd -e target/ && git fetch --tags && git checkout ${TEMPLATE_VERSION}`, { cwd: TEMPLATE_DIR, encoding: "utf-8" });
      } else {
        console.log(`Cloning polkadot-sdk-parachain-template ${TEMPLATE_VERSION}...`);
        execSync(
          `git clone --branch ${TEMPLATE_VERSION} https://github.com/paritytech/polkadot-sdk-parachain-template.git ${TEMPLATE_DIR}`,
          { encoding: "utf-8", stdio: "inherit" }
        );
      }

      expect(existsSync(join(TEMPLATE_DIR, "Cargo.toml"))).toBe(true);
      console.log("Repository cloned successfully");
    }, 120000);

    it("should create the custom pallet directory", () => {
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
      if (existsSync(cargoPath) && readFileSync(cargoPath, "utf-8").includes('name = "pallet-custom"')) {
        console.log("pallet-custom/Cargo.toml already configured");
        return;
      }
      writeFileSync(cargoPath, PALLET_CARGO_TOML);
      expect(readFileSync(cargoPath, "utf-8")).toContain('name = "pallet-custom"');
      console.log("Written pallet-custom/Cargo.toml");
    });

    it("should write pallet lib.rs", () => {
      const libPath = join(PALLET_DIR, "src/lib.rs");
      if (existsSync(libPath) && readFileSync(libPath, "utf-8").includes("CounterValue")) {
        console.log("pallet-custom/src/lib.rs already configured");
        return;
      }
      writeFileSync(libPath, PALLET_LIB_RS);
      expect(readFileSync(libPath, "utf-8")).toContain("CounterValue");
      console.log("Written pallet-custom/src/lib.rs");
    });

    it("should add pallet-custom to workspace members", () => {
      const cargoPath = join(TEMPLATE_DIR, "Cargo.toml");
      let content = readFileSync(cargoPath, "utf-8");
      if (content.includes('"pallets/pallet-custom"')) {
        console.log("pallet-custom already in workspace members");
        return;
      }
      const membersRegex = /^(members\s*=\s*\[([^\]]*)\])/m;
      const match = content.match(membersRegex);
      if (!match) throw new Error("Could not find members array in workspace Cargo.toml");
      content = content.replace(match[1], match[1].replace(']', ', "pallets/pallet-custom"]'));
      writeFileSync(cargoPath, content);
      expect(readFileSync(cargoPath, "utf-8")).toContain('"pallets/pallet-custom"');
      console.log("Added pallet-custom to workspace members");
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
      if (!match) throw new Error("Could not find pallet-parachain-template in workspace dependencies");
      content = content.replace(
        match[1],
        `${match[1]}\npallet-custom = { path = "./pallets/pallet-custom", default-features = false }`
      );
      writeFileSync(cargoPath, content);
      expect(readFileSync(cargoPath, "utf-8")).toContain("pallet-custom = ");
      console.log("Added pallet-custom to workspace dependencies");
    });

    it("should add pallet-custom to runtime/Cargo.toml dependencies", () => {
      const cargoPath = join(TEMPLATE_DIR, "runtime/Cargo.toml");
      let content = readFileSync(cargoPath, "utf-8");
      if (content.includes("pallet-custom")) {
        console.log("pallet-custom already in runtime/Cargo.toml");
        return;
      }
      const palletTemplateRegex = /(pallet-parachain-template\.workspace\s*=\s*true)/;
      const match = content.match(palletTemplateRegex);
      if (!match) throw new Error("Could not find pallet-parachain-template.workspace in runtime/Cargo.toml");
      content = content.replace(match[1], `${match[1]}\npallet-custom.workspace = true`);
      writeFileSync(cargoPath, content);
      expect(readFileSync(cargoPath, "utf-8")).toContain("pallet-custom");
      console.log("Added pallet-custom to runtime/Cargo.toml");
    });

    it("should add pallet-custom/std to runtime features", () => {
      const cargoPath = join(TEMPLATE_DIR, "runtime/Cargo.toml");
      let content = readFileSync(cargoPath, "utf-8");
      if (content.includes('"pallet-custom/std"')) {
        console.log("pallet-custom/std already in runtime features");
        return;
      }
      const stdFeatureRegex = /(\t"pallet-parachain-template\/std",)/;
      const match = content.match(stdFeatureRegex);
      if (!match) throw new Error("Could not find pallet-parachain-template/std in runtime features");
      content = content.replace(match[1], `${match[1]}\n\t"pallet-custom/std",`);
      writeFileSync(cargoPath, content);
      expect(readFileSync(cargoPath, "utf-8")).toContain('"pallet-custom/std"');
      console.log("Added pallet-custom/std to runtime features");
    });

    it("should implement pallet_custom::Config for Runtime", () => {
      const configPath = join(TEMPLATE_DIR, "runtime/src/configs/mod.rs");
      let content = readFileSync(configPath, "utf-8");
      if (content.includes("impl pallet_custom::Config for Runtime")) {
        console.log("pallet_custom::Config already implemented");
        return;
      }
      content += `
/// Configure the custom counter pallet
impl pallet_custom::Config for Runtime {
\ttype RuntimeEvent = RuntimeEvent;
\ttype CounterMaxValue = ConstU32<1000>;
}
`;
      writeFileSync(configPath, content);
      expect(readFileSync(configPath, "utf-8")).toContain("impl pallet_custom::Config for Runtime");
      console.log("Added pallet_custom::Config implementation");
    });

    it("should register CustomPallet in runtime", () => {
      const libPath = join(TEMPLATE_DIR, "runtime/src/lib.rs");
      let content = readFileSync(libPath, "utf-8");
      if (content.includes("pub type CustomPallet = pallet_custom")) {
        console.log("CustomPallet already registered in runtime");
        return;
      }
      const templatePalletRegex = /(#\[runtime::pallet_index\(50\)\]\s*pub type TemplatePallet = pallet_parachain_template;)/;
      const match = content.match(templatePalletRegex);
      if (!match) throw new Error("Could not find TemplatePallet registration in lib.rs");
      content = content.replace(
        match[1],
        `${match[1]}\n\n\t#[runtime::pallet_index(51)]\n\tpub type CustomPallet = pallet_custom;`
      );
      writeFileSync(libPath, content);
      expect(readFileSync(libPath, "utf-8")).toContain("pub type CustomPallet = pallet_custom");
      console.log("Registered CustomPallet in runtime (index 51)");
    });
  });

  // ==================== INITIAL BUILD (spec_version = 1) ====================
  describe("3. Initial Build (spec_version = 1)", () => {
    it("should build the runtime with the custom pallet", () => {
      console.log("Building parachain template with custom pallet (this may take 15-30 minutes)...");
      execSync("cargo build --release", {
        cwd: TEMPLATE_DIR,
        encoding: "utf-8",
        stdio: "inherit",
        timeout: 1800000,
      });
      expect(existsSync(WASM_PATH)).toBe(true);
      console.log("Initial WASM runtime built successfully");
    }, 1800000);

    it("should have generated the WASM runtime", () => {
      expect(existsSync(WASM_PATH)).toBe(true);
      const stats = statSync(WASM_PATH);
      const sizeKB = Math.round(stats.size / 1024);
      console.log(`WASM runtime size: ${sizeKB} KB`);
      expect(stats.size).toBeGreaterThan(100000);
    });

    it("should generate chain specification", () => {
      console.log("Generating chain specification...");
      execSync(
        `chain-spec-builder create -t development \
          --relay-chain rococo-local \
          --para-id 1000 \
          --runtime ${WASM_PATH} \
          named-preset development`,
        { encoding: "utf-8", cwd: WORKSPACE_DIR }
      );
      expect(existsSync(CHAIN_SPEC_PATH)).toBe(true);
      const chainSpec = JSON.parse(readFileSync(CHAIN_SPEC_PATH, "utf-8"));
      expect(chainSpec.name).toBeDefined();
      console.log(`Chain spec generated: ${chainSpec.name}`);
    }, 60000);
  });

  // ==================== DOWNLOAD RELAY CHAIN BINARIES ====================
  describe("4. Download Relay Chain Binaries", () => {
    it("should download polkadot binaries", () => {
      if (!existsSync(BIN_DIR)) {
        mkdirSync(BIN_DIR, { recursive: true });
      }

      if (existsSync(POLKADOT_BINARY) && existsSync(POLKADOT_PREPARE_WORKER) && existsSync(POLKADOT_EXECUTE_WORKER)) {
        try {
          const version = execSync(`${POLKADOT_BINARY} --version 2>&1`, { encoding: "utf-8" });
          console.log(`Polkadot already downloaded: ${version.trim()}`);
          return;
        } catch {
          console.log("Existing binaries not executable, re-downloading...");
        }
      }

      const platform = process.platform;
      if (platform !== "linux") {
        console.log(`Platform ${platform} detected. Using zombienet setup...`);
        execSync(`zombienet setup polkadot -y`, {
          cwd: BIN_DIR,
          encoding: "utf-8",
          stdio: "inherit",
          timeout: 300000,
        });
      } else {
        console.log(`Downloading Polkadot ${POLKADOT_SDK_VERSION} binaries...`);
        const baseUrl = `https://github.com/paritytech/polkadot-sdk/releases/download/${POLKADOT_SDK_VERSION}`;
        for (const binary of ["polkadot", "polkadot-prepare-worker", "polkadot-execute-worker"]) {
          console.log(`Downloading ${binary}...`);
          execSync(`curl -L -o ${binary} ${baseUrl}/${binary}`, {
            cwd: BIN_DIR,
            encoding: "utf-8",
            stdio: "inherit",
            timeout: 300000,
          });
          execSync(`chmod +x ${binary}`, { cwd: BIN_DIR });
        }
      }

      expect(existsSync(POLKADOT_BINARY)).toBe(true);
      const version = execSync(`${POLKADOT_BINARY} --version 2>&1`, { encoding: "utf-8" });
      console.log(`Polkadot: ${version.trim()}`);
    }, 300000);
  });

  // ==================== SPAWN NETWORK WITH ZOMBIENET ====================
  describe("5. Spawn Network and Verify spec_version = 1", () => {
    it("should spawn the network with Zombienet", async () => {
      console.log("Spawning network with Zombienet...");

      const configPath = join(PROJECT_DIR, "configs", "network.toml");

      zombienetProcess = spawn(
        "zombienet",
        ["spawn", configPath, "--provider", "native"],
        {
          cwd: WORKSPACE_DIR,
          stdio: ["ignore", "pipe", "pipe"],
          detached: true,
        }
      );

      if (zombienetProcess.pid) {
        writeFileSync(PID_FILE, zombienetProcess.pid.toString());
        console.log(`Zombienet started with PID: ${zombienetProcess.pid}`);
      }

      zombienetProcess.stdout?.on("data", (data) => {
        const msg = data.toString();
        if (msg.includes("Network launched")) {
          console.log("Network launched successfully!");
        }
      });

      // Wait for the parachain RPC to be available
      const maxWaitTime = 120000;
      const startTime = Date.now();

      while (Date.now() - startTime < maxWaitTime) {
        try {
          const response = await fetch(PARACHAIN_RPC_URL, {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({ jsonrpc: "2.0", method: "system_health", params: [], id: 1 }),
          });
          if (response.ok) {
            console.log("Parachain RPC is ready!");
            return;
          }
        } catch {
          // Node not ready yet
        }
        await new Promise((resolve) => setTimeout(resolve, 2000));
      }
      throw new Error("Network failed to start within 2 minutes");
    }, 180000);

    it("should produce parachain blocks", async () => {
      console.log("Waiting for parachain to produce blocks...");

      let blockNumber = 0;
      for (let attempt = 1; attempt <= 10; attempt++) {
        await new Promise((resolve) => setTimeout(resolve, 12000));
        try {
          const result = (await rpcCall("chain_getHeader")) as { number: string };
          blockNumber = parseInt(result.number, 16);
          console.log(`Parachain block number (attempt ${attempt}): ${blockNumber}`);
          if (blockNumber > 0) break;
        } catch {
          console.log(`Attempt ${attempt}: parachain not ready yet`);
        }
      }
      expect(blockNumber).toBeGreaterThan(0);
    }, 180000);

    it("should have spec_version = 1", async () => {
      const result = (await rpcCall("state_getRuntimeVersion")) as {
        specName: string;
        specVersion: number;
      };
      expect(result.specName).toBe("parachain-template-runtime");
      expect(result.specVersion).toBe(1);
      console.log(`Runtime: ${result.specName} v${result.specVersion}`);
    }, 10000);
  });

  // ==================== ADD NEW FEATURE AND BUMP VERSION ====================
  describe("6. Add New Feature and Bump spec_version", () => {
    it("should add reset_counter function to pallet", () => {
      const libPath = join(PALLET_DIR, "src/lib.rs");
      let content = readFileSync(libPath, "utf-8");

      if (content.includes("reset_counter")) {
        console.log("reset_counter already added");
        return;
      }

      // Insert reset_counter inside the impl<T: Config> Pallet<T> block,
      // before the closing "    }\n}" (impl close + module close).
      const closingBraces = "    }\n}";
      const insertPoint = content.lastIndexOf(closingBraces);
      if (insertPoint === -1) throw new Error("Could not find insertion point for reset_counter");

      const resetFn = `
        /// Reset the counter to zero.
        ///
        /// The dispatch origin of this call must be _Root_.
        ///
        /// Emits \`CounterValueSet\` event when successful.
        #[pallet::call_index(3)]
        #[pallet::weight(0)]
        pub fn reset_counter(origin: OriginFor<T>) -> DispatchResult {
            ensure_root(origin)?;
            <CounterValue<T>>::put(0u32);
            Self::deposit_event(Event::CounterValueSet { new_value: 0 });
            Ok(())
        }
`;

      content = content.slice(0, insertPoint) + resetFn + content.slice(insertPoint);
      writeFileSync(libPath, content);

      const updated = readFileSync(libPath, "utf-8");
      expect(updated).toContain("reset_counter");
      expect(updated).toContain("call_index(3)");
      console.log("Added reset_counter function to pallet");
    });

    it("should bump spec_version from 1 to 2", () => {
      const libPath = join(TEMPLATE_DIR, "runtime/src/lib.rs");
      let content = readFileSync(libPath, "utf-8");

      expect(content).toMatch(/spec_version:\s*1/);

      content = content.replace(/spec_version:\s*1/, "spec_version: 2");
      writeFileSync(libPath, content);

      const updated = readFileSync(libPath, "utf-8");
      expect(updated).toMatch(/spec_version:\s*2/);
      console.log("Bumped spec_version from 1 to 2");
    });
  });

  // ==================== BUILD UPGRADED RUNTIME ====================
  describe("7. Build the Upgraded Runtime", () => {
    it("should build the runtime with spec_version = 2", () => {
      console.log("Building upgraded runtime (this may take several minutes)...");
      execSync("cargo build --release", {
        cwd: TEMPLATE_DIR,
        encoding: "utf-8",
        stdio: "inherit",
        timeout: 1800000,
      });
      expect(existsSync(WASM_PATH)).toBe(true);

      const stats = statSync(WASM_PATH);
      const sizeKB = Math.round(stats.size / 1024);
      console.log(`Upgraded WASM runtime size: ${sizeKB} KB`);
      expect(stats.size).toBeGreaterThan(100000);
    }, 1800000);
  });

  // ==================== SUBMIT RUNTIME UPGRADE ====================
  describe("8. Submit Runtime Upgrade", () => {
    it("should submit the upgrade via sudo + system.setCode", async () => {
      console.log("Submitting runtime upgrade via @polkadot/api...");

      const wasmBinary = readFileSync(WASM_PATH);
      const wasmHex = "0x" + wasmBinary.toString("hex");

      const wsProvider = new WsProvider(PARACHAIN_WS_URL);
      const api = await ApiPromise.create({ provider: wsProvider });

      const keyring = new Keyring({ type: "sr25519" });
      const alice = keyring.addFromUri("//Alice");

      // sudoUncheckedWeight bypasses block weight limits for the large setCode call
      const setCodeCall = api.tx.system.setCode(wasmHex);
      const sudoCall = api.tx.sudo.sudoUncheckedWeight(setCodeCall, { refTime: 0, proofSize: 0 });

      await new Promise<void>((resolve, reject) => {
        sudoCall.signAndSend(alice, ({ status, dispatchError }) => {
          if (dispatchError) {
            if (dispatchError.isModule) {
              const decoded = api.registry.findMetaError(dispatchError.asModule);
              reject(new Error(`${decoded.section}.${decoded.name}: ${decoded.docs.join(" ")}`));
            } else {
              reject(new Error(dispatchError.toString()));
            }
          }
          if (status.isInBlock) {
            console.log(`Upgrade included in block: ${status.asInBlock.toHex()}`);
            resolve();
          }
        });
      });

      await api.disconnect();

      // Wait for the upgrade to propagate
      console.log("Waiting for runtime upgrade to take effect...");
      await new Promise((resolve) => setTimeout(resolve, 24000));
    }, 120000);

    it("should have spec_version = 2 after upgrade", async () => {
      // Poll for the spec version change (may take a block or two)
      let specVersion = 0;
      for (let attempt = 1; attempt <= 5; attempt++) {
        const result = (await rpcCall("state_getRuntimeVersion")) as {
          specName: string;
          specVersion: number;
        };
        specVersion = result.specVersion;
        console.log(`spec_version (attempt ${attempt}): ${specVersion}`);
        if (specVersion === 2) break;
        await new Promise((resolve) => setTimeout(resolve, 12000));
      }
      expect(specVersion).toBe(2);
    }, 90000);
  });

  // ==================== POST-UPGRADE VERIFICATION ====================
  describe("9. Post-Upgrade Verification", () => {
    it("should continue producing blocks after upgrade", async () => {
      const before = (await rpcCall("chain_getHeader")) as { number: string };
      const blockBefore = parseInt(before.number, 16);

      await new Promise((resolve) => setTimeout(resolve, 24000));

      const after = (await rpcCall("chain_getHeader")) as { number: string };
      const blockAfter = parseInt(after.number, 16);

      console.log(`Blocks: ${blockBefore} -> ${blockAfter}`);
      expect(blockAfter).toBeGreaterThan(blockBefore);
    }, 60000);

    it("should respond to RPC queries", async () => {
      const name = (await rpcCall("system_name")) as string;
      expect(name).toBeDefined();
      console.log(`System name: ${name}`);

      const version = (await rpcCall("system_version")) as string;
      expect(version).toBeDefined();
      console.log(`System version: ${version}`);
    }, 10000);
  });
});

async function stopZombienet(): Promise<void> {
  console.log("Stopping Zombienet...");

  if (zombienetProcess && !zombienetProcess.killed) {
    try {
      process.kill(-zombienetProcess.pid!, "SIGTERM");
    } catch {
      zombienetProcess.kill("SIGTERM");
    }
    zombienetProcess = null;
  }

  if (existsSync(PID_FILE)) {
    try {
      const pid = parseInt(readFileSync(PID_FILE, "utf-8"));
      process.kill(-pid, "SIGTERM");
    } catch {
      // Process might already be dead
    }
    unlinkSync(PID_FILE);
  }

  try {
    execSync("pkill -f 'polkadot.*rococo-local' 2>/dev/null || true", { encoding: "utf-8" });
    execSync("pkill -f 'polkadot-omni-node' 2>/dev/null || true", { encoding: "utf-8" });
  } catch {
    // Ignore
  }

  await new Promise((resolve) => setTimeout(resolve, 2000));
  console.log("Zombienet stopped");
}
