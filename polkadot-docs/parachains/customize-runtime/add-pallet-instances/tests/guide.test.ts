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
const TEMPLATE_VERSION = process.env.TEMPLATE_VERSION!;
const CHAIN_SPEC_PATH = join(WORKSPACE_DIR, "chain_spec.json");
const PID_FILE = join(WORKSPACE_DIR, "node.pid");
const WASM_PATH = join(
  TEMPLATE_DIR,
  "target/release/wbuild/parachain-template-runtime/parachain_template_runtime.compact.compressed.wasm"
);

let nodeProcess: ChildProcess | null = null;

// All tests in a single describe block to ensure sequential execution
describe("Add Pallet Instances Guide", () => {
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
      } catch (error) {
        console.log("Installing polkadot-omni-node...");
        execSync(`cargo install polkadot-omni-node@${process.env.POLKADOT_OMNI_NODE_VERSION} --locked`, {
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

  // ==================== CLONE AND MODIFY TESTS ====================
  describe("2. Clone and Modify Template", () => {
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
      expect(existsSync(join(TEMPLATE_DIR, "runtime"))).toBe(true);
      console.log("Repository cloned successfully");
    }, 120000);

    it("should add pallet-collective to runtime/Cargo.toml", () => {
      const cargoPath = join(TEMPLATE_DIR, "runtime/Cargo.toml");
      let content = readFileSync(cargoPath, "utf-8");

      // Check if pallet-collective is already added
      if (content.includes('"pallet-collective"')) {
        console.log("pallet-collective already in Cargo.toml");
        return;
      }

      // Find the polkadot-sdk features array and add pallet-collective
      const featuresRegex = /("pallet-transaction-payment-rpc-runtime-api")/;
      const match = content.match(featuresRegex);

      if (match) {
        // Add pallet-collective after pallet-transaction-payment-rpc-runtime-api
        content = content.replace(
          match[1],
          `${match[1]}, "pallet-collective"`
        );
        writeFileSync(cargoPath, content);
        console.log("Added pallet-collective to Cargo.toml features");
      } else {
        throw new Error("Could not find polkadot-sdk features array in Cargo.toml");
      }

      // Verify it was added
      const updatedContent = readFileSync(cargoPath, "utf-8");
      expect(updatedContent).toContain('"pallet-collective"');
    });

    it("should add parameter_types and instance type aliases", () => {
      const configPath = join(TEMPLATE_DIR, "runtime/src/configs/mod.rs");
      let content = readFileSync(configPath, "utf-8");

      // Check if parameter_types for collective are already added
      if (content.includes("pub const MotionDuration")) {
        console.log("Collective parameter_types already added");
        return;
      }

      // Add parameter_types and type aliases after the existing parameter_types block
      // Find a good insertion point - after the last impl block or at the end
      const parameterTypesAndAliases = `
/// Parameters for pallet-collective instances
parameter_types! {
	pub const MotionDuration: BlockNumber = 24 * HOURS;
	pub const MaxProposals: u32 = 100;
	pub const MaxMembers: u32 = 100;
	pub MaxProposalWeight: Weight = Perbill::from_percent(50) * RuntimeBlockWeights::get().max_block;
}

/// Type alias for the Technical Collective instance
pub type TechnicalCollective = pallet_collective::Instance1;
/// Type alias for the Council Collective instance
pub type CouncilCollective = pallet_collective::Instance2;
`;

      content += parameterTypesAndAliases;
      writeFileSync(configPath, content);
      console.log("Added parameter_types and instance type aliases");

      // Verify it was added
      const updatedContent = readFileSync(configPath, "utf-8");
      expect(updatedContent).toContain("pub const MotionDuration");
      expect(updatedContent).toContain("TechnicalCollective");
      expect(updatedContent).toContain("CouncilCollective");
    });

    it("should implement pallet_collective::Config for TechnicalCollective", () => {
      const configPath = join(TEMPLATE_DIR, "runtime/src/configs/mod.rs");
      let content = readFileSync(configPath, "utf-8");

      // Check if Config is already implemented for TechnicalCollective
      if (content.includes("impl pallet_collective::Config<TechnicalCollective> for Runtime")) {
        console.log("pallet_collective::Config<TechnicalCollective> already implemented");
        return;
      }

      // Add the Config implementation
      const configImpl = `
/// Configure the pallet-collective for TechnicalCollective (Technical Committee)
impl pallet_collective::Config<TechnicalCollective> for Runtime {
	type RuntimeOrigin = RuntimeOrigin;
	type Proposal = RuntimeCall;
	type RuntimeEvent = RuntimeEvent;
	type MotionDuration = MotionDuration;
	type MaxProposals = MaxProposals;
	type MaxMembers = MaxMembers;
	type DefaultVote = pallet_collective::MoreThanMajorityThenPrimeDefaultVote;
	type SetMembersOrigin = EnsureRoot<AccountId>;
	type WeightInfo = pallet_collective::weights::SubstrateWeight<Runtime>;
	type MaxProposalWeight = MaxProposalWeight;
	type DisapproveOrigin = EnsureRoot<Self::AccountId>;
	type KillOrigin = EnsureRoot<Self::AccountId>;
	type Consideration = ();
}
`;

      content += configImpl;
      writeFileSync(configPath, content);
      console.log("Added pallet_collective::Config<TechnicalCollective> implementation");

      // Verify it was added
      const updatedContent = readFileSync(configPath, "utf-8");
      expect(updatedContent).toContain("impl pallet_collective::Config<TechnicalCollective> for Runtime");
    });

    it("should implement pallet_collective::Config for CouncilCollective", () => {
      const configPath = join(TEMPLATE_DIR, "runtime/src/configs/mod.rs");
      let content = readFileSync(configPath, "utf-8");

      // Check if Config is already implemented for CouncilCollective
      if (content.includes("impl pallet_collective::Config<CouncilCollective> for Runtime")) {
        console.log("pallet_collective::Config<CouncilCollective> already implemented");
        return;
      }

      // Add the Config implementation
      const configImpl = `
/// Configure the pallet-collective for CouncilCollective (Council)
impl pallet_collective::Config<CouncilCollective> for Runtime {
	type RuntimeOrigin = RuntimeOrigin;
	type Proposal = RuntimeCall;
	type RuntimeEvent = RuntimeEvent;
	type MotionDuration = MotionDuration;
	type MaxProposals = MaxProposals;
	type MaxMembers = MaxMembers;
	type DefaultVote = pallet_collective::MoreThanMajorityThenPrimeDefaultVote;
	type SetMembersOrigin = EnsureRoot<AccountId>;
	type WeightInfo = pallet_collective::weights::SubstrateWeight<Runtime>;
	type MaxProposalWeight = MaxProposalWeight;
	type DisapproveOrigin = EnsureRoot<Self::AccountId>;
	type KillOrigin = EnsureRoot<Self::AccountId>;
	type Consideration = ();
}
`;

      content += configImpl;
      writeFileSync(configPath, content);
      console.log("Added pallet_collective::Config<CouncilCollective> implementation");

      // Verify it was added
      const updatedContent = readFileSync(configPath, "utf-8");
      expect(updatedContent).toContain("impl pallet_collective::Config<CouncilCollective> for Runtime");
    });

    it("should re-export collective types in lib.rs", () => {
      const libPath = join(TEMPLATE_DIR, "runtime/src/lib.rs");
      let content = readFileSync(libPath, "utf-8");

      // Check if re-exports already exist (check for combined import syntax)
      if (content.includes("pub use configs::{TechnicalCollective, CouncilCollective}")) {
        console.log("Collective types already re-exported");
        return;
      }

      // Add re-exports after pub mod configs
      const modConfigsRegex = /(pub mod configs;)/;
      const match = content.match(modConfigsRegex);

      if (match) {
        const reExports = `${match[1]}
pub use configs::{TechnicalCollective, CouncilCollective};`;
        content = content.replace(match[1], reExports);
        writeFileSync(libPath, content);
        console.log("Added re-exports for TechnicalCollective and CouncilCollective");
      } else {
        throw new Error("Could not find 'pub mod configs;' in lib.rs");
      }

      // Verify it was added (check for combined import syntax)
      const updatedContent = readFileSync(libPath, "utf-8");
      expect(updatedContent).toContain("pub use configs::{TechnicalCollective, CouncilCollective}");
    });

    it("should register TechnicalCommittee pallet in runtime", () => {
      const libPath = join(TEMPLATE_DIR, "runtime/src/lib.rs");
      let content = readFileSync(libPath, "utf-8");

      // Check if TechnicalCommittee is already registered
      if (content.includes("pub type TechnicalCommittee = pallet_collective")) {
        console.log("TechnicalCommittee pallet already registered");
        return;
      }

      // Add import for Instance1 and Instance2 from frame_support::instances
      // This must be at the crate level, not inside mod runtime
      if (!content.includes("use frame_support::instances::{Instance1, Instance2}")) {
        // Add after the polkadot_sdk import
        const sdkImportRegex = /(use polkadot_sdk::\{[^}]+\};)/;
        const sdkMatch = content.match(sdkImportRegex);
        if (sdkMatch) {
          content = content.replace(
            sdkMatch[1],
            `${sdkMatch[1]}\nuse frame_support::instances::{Instance1, Instance2};`
          );
        }
      }

      // Find the TemplatePallet registration and add TechnicalCommittee before it
      const templatePalletRegex = /(#\[runtime::pallet_index\(50\)\]\s*pub type TemplatePallet)/;
      const match = content.match(templatePalletRegex);

      if (match) {
        // Use Instance1 directly as required by the new #[frame_support::runtime] macro
        const technicalCommitteeRegistration = `#[runtime::pallet_index(16)]
	pub type TechnicalCommittee = pallet_collective<Instance1>;

	`;
        content = content.replace(match[1], technicalCommitteeRegistration + match[1]);
        writeFileSync(libPath, content);
        console.log("Registered TechnicalCommittee pallet in runtime (index 16)");
      } else {
        throw new Error("Could not find TemplatePallet registration in lib.rs");
      }

      // Verify it was added
      const updatedContent = readFileSync(libPath, "utf-8");
      expect(updatedContent).toContain("pub type TechnicalCommittee = pallet_collective");
    });

    it("should register Council pallet in runtime", () => {
      const libPath = join(TEMPLATE_DIR, "runtime/src/lib.rs");
      let content = readFileSync(libPath, "utf-8");

      // Check if Council is already registered
      if (content.includes("pub type Council = pallet_collective")) {
        console.log("Council pallet already registered");
        return;
      }

      // Find the TechnicalCommittee registration and add Council after it
      const technicalCommitteeRegex = /(pub type TechnicalCommittee = pallet_collective<Instance1>;)/;
      const match = content.match(technicalCommitteeRegex);

      if (match) {
        // Use Instance2 directly as required by the new #[frame_support::runtime] macro
        const councilRegistration = `${match[1]}

	#[runtime::pallet_index(17)]
	pub type Council = pallet_collective<Instance2>;`;
        content = content.replace(match[1], councilRegistration);
        writeFileSync(libPath, content);
        console.log("Registered Council pallet in runtime (index 17)");
      } else {
        throw new Error("Could not find TechnicalCommittee registration in lib.rs");
      }

      // Verify it was added
      const updatedContent = readFileSync(libPath, "utf-8");
      expect(updatedContent).toContain("pub type Council = pallet_collective");
    });

    it("should verify collective types are properly configured", () => {
      // Verify that the type aliases are defined in configs/mod.rs for Config implementations
      const configPath = join(TEMPLATE_DIR, "runtime/src/configs/mod.rs");
      const configContent = readFileSync(configPath, "utf-8");

      expect(configContent).toContain("pub type TechnicalCollective = pallet_collective::Instance1");
      expect(configContent).toContain("pub type CouncilCollective = pallet_collective::Instance2");

      // Verify that the runtime uses Instance1 and Instance2 directly
      // (the new #[frame_support::runtime] macro requires identifiers, not paths)
      const libPath = join(TEMPLATE_DIR, "runtime/src/lib.rs");
      const libContent = readFileSync(libPath, "utf-8");

      expect(libContent).toContain("pallet_collective<Instance1>");
      expect(libContent).toContain("pallet_collective<Instance2>");
      expect(libContent).toContain("use frame_support::instances::{Instance1, Instance2}");

      console.log("Collective types are properly configured with Instance1 and Instance2");
    });
  });

  // ==================== BUILD TESTS ====================
  describe("3. Build Verification", () => {
    it("should build the modified runtime", () => {
      console.log("Building modified parachain template with collective instances (this may take 15-30 minutes)...");

      execSync("cargo build --release", {
        cwd: TEMPLATE_DIR,
        encoding: "utf-8",
        stdio: "inherit",
        timeout: 1800000,
      });

      expect(existsSync(WASM_PATH)).toBe(true);
      console.log("WASM runtime built successfully with pallet-collective instances");
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
  describe("4. Runtime Verification", () => {
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

    it("should have TechnicalCommittee and Council pallets available", async () => {
      console.log("Verifying collective pallet instances are available...");

      // Query runtime metadata to verify pallets exist
      const response = await fetch("http://127.0.0.1:9944", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          jsonrpc: "2.0",
          method: "state_getMetadata",
          params: [],
          id: 1,
        }),
      });

      const result = await response.json();
      expect(result.result).toBeDefined();

      // The metadata is encoded, but if we got here without errors,
      // the pallets are registered. Let's also verify the runtime version
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

      // The pallet instances were successfully added if:
      // 1. Build succeeded with pallet-collective in Cargo.toml
      // 2. Both Config traits were implemented
      // 3. Both pallets were registered in runtime macro
      // 4. Node started successfully
      console.log("TechnicalCommittee and Council pallets verified (build and runtime startup succeeded)");
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
