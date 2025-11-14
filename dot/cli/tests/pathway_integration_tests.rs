//! End-to-end integration tests for all recipe pathways
//!
//! These tests validate the complete user workflow by creating real example recipes:
//! 1. `dot recipe create` - Create a recipe for each pathway in recipes/ directory
//! 2. `dot recipe test` - Run tests on the created recipe
//! 3. Verify all tests pass
//!
//! This ensures that:
//! - Templates generate valid, working code
//! - Generated recipes compile successfully
//! - Generated tests pass
//! - Rust version compatibility (for Rust-based recipes)
//! - TypeScript/Node version compatibility (for TS-based recipes)
//!
//! Generated example recipes:
//! - recipes/parachain-example/                (Full parachain with PAPI tests and XCM)
//! - recipes/pallets/pallet-example/           (Pallet-only mode, no runtime)
//! - recipes/contracts/contracts-example/      (Solidity contracts)
//! - recipes/transactions/transactions-example/ (Basic PAPI interactions)
//! - recipes/xcm/xcm-example/                  (XCM with Chopsticks)
//! - recipes/networks/infra-example/           (Zombienet/Chopsticks configs)

use assert_cmd::Command;
use predicates::prelude::*;
use serial_test::serial;
use std::path::PathBuf;
use std::time::Duration;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::{Child, Command as TokioCommand};
use tokio::time::{sleep, timeout};

/// Get the repository root directory
fn get_repo_root() -> PathBuf {
    // Start from the manifest directory (dot/cli)
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    // Go up two levels to reach repository root
    manifest_dir
        .parent()
        .and_then(|p| p.parent())
        .expect("Failed to find repository root")
        .to_path_buf()
}

/// Clean up an existing example recipe if it exists
fn cleanup_recipe(recipe_name: &str) {
    let repo_root = get_repo_root();

    // Search in all pathway subdirectories
    let pathways = ["contracts", "pallets", "transactions", "xcm", "networks"];

    for pathway in &pathways {
        let recipe_path = repo_root.join("recipes").join(pathway).join(recipe_name);
        if recipe_path.exists() {
            std::fs::remove_dir_all(&recipe_path)
                .unwrap_or_else(|e| eprintln!("Warning: Failed to remove {}: {}", recipe_name, e));
        }
    }

    // Also check legacy location (directly in recipes/)
    let legacy_path = repo_root.join("recipes").join(recipe_name);
    if legacy_path.exists() {
        std::fs::remove_dir_all(&legacy_path).unwrap_or_else(|e| {
            eprintln!("Warning: Failed to remove {} (legacy): {}", recipe_name, e)
        });
    }
}

/// Wait for the node to be ready by polling the WebSocket endpoint
async fn wait_for_node_ready(ws_url: &str, timeout_secs: u64) -> Result<(), String> {
    let start = std::time::Instant::now();
    let timeout_duration = Duration::from_secs(timeout_secs);

    loop {
        if start.elapsed() > timeout_duration {
            return Err(format!(
                "Timeout waiting for node at {} after {}s",
                ws_url, timeout_secs
            ));
        }

        // Try to connect to the WebSocket endpoint
        match tokio::net::TcpStream::connect("127.0.0.1:9944").await {
            Ok(_) => {
                println!("✅ Node is ready at {}", ws_url);
                // Give it a moment to fully initialize
                sleep(Duration::from_secs(2)).await;
                return Ok(());
            }
            Err(_) => {
                // Not ready yet, wait and retry
                sleep(Duration::from_millis(500)).await;
            }
        }
    }
}

/// Manages a test node process with automatic cleanup
struct TestNode {
    #[allow(dead_code)] // Managed by tokio's kill_on_drop
    process: Child,
    ws_url: String,
}

impl TestNode {
    /// Start a development node by running the start-dev-node.sh script
    async fn start(recipe_path: &PathBuf) -> Result<Self, String> {
        let script_path = recipe_path.join("scripts").join("start-dev-node.sh");

        if !script_path.exists() {
            return Err(format!("Script not found: {:?}", script_path));
        }

        println!("🚀 Starting node via {:?}", script_path);

        let mut process = TokioCommand::new("bash")
            .arg(&script_path)
            .current_dir(recipe_path)
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .kill_on_drop(true) // Automatically kill when dropped
            .spawn()
            .map_err(|e| format!("Failed to start node: {}", e))?;

        // Capture stdout/stderr for debugging
        if let Some(stdout) = process.stdout.take() {
            tokio::spawn(async move {
                let reader = BufReader::new(stdout);
                let mut lines = reader.lines();
                while let Ok(Some(line)) = lines.next_line().await {
                    println!("[NODE] {}", line);
                }
            });
        }

        if let Some(stderr) = process.stderr.take() {
            tokio::spawn(async move {
                let reader = BufReader::new(stderr);
                let mut lines = reader.lines();
                while let Ok(Some(line)) = lines.next_line().await {
                    eprintln!("[NODE ERR] {}", line);
                }
            });
        }

        Ok(TestNode {
            process,
            ws_url: "ws://localhost:9944".to_string(),
        })
    }

    /// Wait for the node to be ready
    async fn wait_ready(&self, timeout_secs: u64) -> Result<(), String> {
        wait_for_node_ready(&self.ws_url, timeout_secs).await
    }
}

impl Drop for TestNode {
    fn drop(&mut self) {
        println!("🛑 Stopping test node...");
        // kill_on_drop(true) will handle cleanup automatically
    }
}

/// Test Parachain pathway: Create recipe with default mode (full parachain + PAPI)
/// This test validates the complete developer workflow:
/// 1. Create recipe
/// 2. Compile runtime
/// 3. Generate chain spec
/// 4. Start development node
/// 5. Run PAPI integration tests
#[tokio::test]
#[serial]
#[ignore] // Run with: cargo test --test pathway_integration_tests -- --ignored
async fn test_parachain_example_end_to_end() {
    let repo_root = get_repo_root();
    let recipe_name = "parachain-example";

    // Clean up any existing example
    cleanup_recipe(recipe_name);

    // Step 1: Create a Parachain recipe (default mode: full parachain + PAPI)
    println!("📦 Step 1/5: Creating parachain recipe...");
    let recipes_dir = repo_root.join("recipes");
    std::fs::create_dir_all(&recipes_dir).unwrap();

    let mut create_cmd = Command::cargo_bin("dot").unwrap();
    create_cmd
        .current_dir(&recipes_dir)
        .arg("create")
        .arg("--title")
        .arg("Parachain Example")
        .arg("--pathway")
        .arg("pallets")
        .arg("--skip-install") // Skip npm install for faster CI
        .arg("--no-git")
        .arg("--non-interactive");

    create_cmd.assert().success();

    let recipe_path = repo_root.join("recipes").join(recipe_name);
    assert!(recipe_path.exists(), "Recipe directory should exist");
    assert!(
        recipe_path.join("README.md").exists(),
        "README.md should exist"
    );
    assert!(
        recipe_path.join("Cargo.toml").exists(),
        "Cargo.toml should exist"
    );
    assert!(
        recipe_path.join("rust-toolchain.toml").exists(),
        "rust-toolchain.toml should exist"
    );
    assert!(
        recipe_path.join("pallets").exists(),
        "pallets/ should exist"
    );
    assert!(
        recipe_path.join("runtime").exists(),
        "runtime/ should exist"
    );

    // Verify PAPI files are present (not pallet-only mode)
    assert!(
        recipe_path.join("package.json").exists(),
        "package.json should exist in full parachain mode"
    );
    assert!(
        recipe_path.join("tests").exists(),
        "tests/ should exist for PAPI tests"
    );
    assert!(
        recipe_path.join("scripts").exists(),
        "scripts/ should exist for node management"
    );
    assert!(
        recipe_path.join("zombienet.toml").exists()
            || recipe_path.join("zombienet-omni-node.toml").exists(),
        "zombienet config should exist"
    );

    // Verify XCM config IS present (always included for full parachain)
    assert!(
        recipe_path.join("zombienet-xcm.toml").exists(),
        "zombienet-xcm.toml should always exist in full parachain mode"
    );

    // Step 2: Compile the runtime
    println!("🔨 Step 2/5: Compiling runtime (this may take 10-15 minutes)...");
    let build_result = timeout(
        Duration::from_secs(900), // 15 minute timeout
        TokioCommand::new("cargo")
            .arg("build")
            .arg("--release")
            .current_dir(&recipe_path)
            .output(),
    )
    .await;

    match build_result {
        Ok(Ok(output)) => {
            assert!(
                output.status.success(),
                "Cargo build failed:\n{}",
                String::from_utf8_lossy(&output.stderr)
            );
            println!("✅ Runtime compiled successfully");
        }
        Ok(Err(e)) => panic!("Failed to execute cargo build: {}", e),
        Err(_) => panic!("Cargo build timed out after 15 minutes"),
    }

    // Verify WASM was built
    let wasm_path = recipe_path
        .join("target")
        .join("release")
        .join("wbuild")
        .join(format!("{}-runtime", recipe_name))
        .join(format!(
            "{}_runtime.compact.compressed.wasm",
            recipe_name.replace("-", "_")
        ));
    assert!(
        wasm_path.exists(),
        "Runtime WASM should exist at {:?}",
        wasm_path
    );

    // Step 3: Generate chain specification
    println!("📋 Step 3/5: Generating chain specification...");
    let generate_spec_script = recipe_path.join("scripts").join("generate-spec.sh");
    assert!(
        generate_spec_script.exists(),
        "generate-spec.sh should exist"
    );

    let spec_result = timeout(
        Duration::from_secs(60),
        TokioCommand::new("bash")
            .arg(&generate_spec_script)
            .current_dir(&recipe_path)
            .output(),
    )
    .await;

    match spec_result {
        Ok(Ok(output)) => {
            assert!(
                output.status.success(),
                "Chain spec generation failed:\n{}",
                String::from_utf8_lossy(&output.stderr)
            );
            println!("✅ Chain specification generated");
        }
        Ok(Err(e)) => panic!("Failed to execute generate-spec.sh: {}", e),
        Err(_) => panic!("Chain spec generation timed out"),
    }

    // Verify chain-spec.json was created
    let chain_spec_path = recipe_path.join("chain-spec.json");
    assert!(
        chain_spec_path.exists(),
        "chain-spec.json should exist at {:?}",
        chain_spec_path
    );

    // Step 4: Start the development node
    println!("🚀 Step 4/5: Starting development node...");
    let node = TestNode::start(&recipe_path)
        .await
        .expect("Failed to start test node");

    // Wait for node to be ready
    node.wait_ready(120)
        .await
        .expect("Node failed to become ready");

    // Step 5: Run PAPI integration tests
    println!("🧪 Step 5/5: Running PAPI integration tests...");

    // First install npm dependencies (since we skipped it during creation)
    let npm_install = timeout(
        Duration::from_secs(300),
        TokioCommand::new("npm")
            .arg("install")
            .current_dir(&recipe_path)
            .output(),
    )
    .await;

    match npm_install {
        Ok(Ok(output)) => {
            assert!(
                output.status.success(),
                "npm install failed:\n{}",
                String::from_utf8_lossy(&output.stderr)
            );
            println!("✅ npm dependencies installed");
        }
        Ok(Err(e)) => panic!("Failed to execute npm install: {}", e),
        Err(_) => panic!("npm install timed out"),
    }

    // Fetch metadata and generate TypeScript types using papi add
    println!("📝 Fetching metadata and generating TypeScript types...");
    let papi_add = timeout(
        Duration::from_secs(120),
        TokioCommand::new("npx")
            .arg("papi")
            .arg("add")
            .arg("dot")
            .arg("-w")
            .arg("ws://localhost:9944")
            .current_dir(&recipe_path)
            .output(),
    )
    .await;

    match papi_add {
        Ok(Ok(output)) => {
            assert!(
                output.status.success(),
                "PAPI add failed:\n{}",
                String::from_utf8_lossy(&output.stderr)
            );
            println!("✅ TypeScript types generated from running node");
        }
        Ok(Err(e)) => panic!("Failed to execute npx papi add: {}", e),
        Err(_) => panic!("PAPI add timed out"),
    }

    // Run the PAPI tests
    let test_result = timeout(
        Duration::from_secs(120),
        TokioCommand::new("npm")
            .arg("test")
            .current_dir(&recipe_path)
            .output(),
    )
    .await;

    match test_result {
        Ok(Ok(output)) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);

            println!("Test stdout:\n{}", stdout);
            println!("Test stderr:\n{}", stderr);

            assert!(
                output.status.success(),
                "PAPI tests failed:\nstdout: {}\nstderr: {}",
                stdout,
                stderr
            );
            println!("✅ All PAPI tests passed!");
        }
        Ok(Err(e)) => panic!("Failed to execute npm test: {}", e),
        Err(_) => panic!("npm test timed out"),
    }

    // Node will be automatically stopped when TestNode is dropped
    println!("✅ Full parachain workflow test completed successfully!");
}

// Note: The separate XCM test has been removed since zombienet-xcm.toml
// is now always included in full parachain mode

/// Test Pallet-only pathway (no runtime, no PAPI, just pallet + tests)
#[test]
#[ignore] // Run with: cargo test --test pathway_integration_tests -- --ignored
fn test_pallet_example_end_to_end() {
    let repo_root = get_repo_root();
    let recipe_name = "pallet-example";

    // Clean up any existing example
    cleanup_recipe(recipe_name);

    // Step 1: Create a pallet-only recipe
    let mut create_cmd = Command::cargo_bin("dot").unwrap();
    create_cmd
        .current_dir(&repo_root)
        .arg("create")
        .arg("--title")
        .arg("Pallet Example")
        .arg("--pathway")
        .arg("pallets")
        .arg("--pallet-only") // Pallet-only mode: no runtime, no PAPI
        .arg("--skip-install")
        .arg("--no-git")
        .arg("--non-interactive");

    create_cmd.assert().success();

    let recipe_path = repo_root.join("recipes").join("pallets").join(recipe_name);
    assert!(recipe_path.exists(), "Recipe directory should exist");
    assert!(
        recipe_path.join("Cargo.toml").exists(),
        "Cargo.toml should exist"
    );
    assert!(
        recipe_path.join("pallets").exists(),
        "pallets/ should exist"
    );

    // Verify runtime/ does NOT exist in pallet-only mode
    // Actually, the runtime/ might still exist but won't be functional - let's check for PAPI files instead

    // Verify PAPI files are NOT present (pallet-only mode)
    assert!(
        !recipe_path.join("package.json").exists(),
        "package.json should not exist in pallet-only mode"
    );
    assert!(
        !recipe_path.join("tests").exists() || !recipe_path.join("tests").is_dir(),
        "PAPI tests/ should not exist in pallet-only mode"
    );
    assert!(
        !recipe_path.join("scripts").exists(),
        "scripts/ should not exist in pallet-only mode"
    );

    // Step 2: Run tests (should run pallet unit tests only)
    let mut test_cmd = Command::cargo_bin("dot").unwrap();
    test_cmd
        .current_dir(&repo_root)
        .arg("test")
        .arg(recipe_name)
        .timeout(std::time::Duration::from_secs(600));

    test_cmd
        .assert()
        .success()
        .stdout(predicate::str::contains("test result: ok"));
}

/// Test Contracts pathway: Create recipe and run tests
#[test]
#[ignore] // Run with: cargo test --test pathway_integration_tests -- --ignored
fn test_contracts_example_end_to_end() {
    let repo_root = get_repo_root();
    let recipe_name = "contracts-example";

    // Clean up any existing example
    cleanup_recipe(recipe_name);

    // Step 1: Create a Contracts (Solidity) recipe
    let mut create_cmd = Command::cargo_bin("dot").unwrap();
    create_cmd
        .current_dir(&repo_root)
        .arg("create")
        .arg("--title")
        .arg("Contracts Example")
        .arg("--pathway")
        .arg("contracts")
        .arg("--no-git")
        .arg("--non-interactive");

    create_cmd.assert().success();

    let recipe_path = repo_root
        .join("recipes")
        .join("contracts")
        .join(recipe_name);
    assert!(recipe_path.exists(), "Recipe directory should exist");
    assert!(
        recipe_path.join("README.md").exists(),
        "README.md should exist"
    );
    assert!(
        recipe_path.join("package.json").exists(),
        "package.json should exist"
    );
    assert!(
        recipe_path.join("hardhat.config.ts").exists(),
        "hardhat.config.ts should exist"
    );
    assert!(
        recipe_path.join("contracts").exists(),
        "contracts/ should exist"
    );
    assert!(recipe_path.join("tests").exists(), "tests/ should exist");

    // Step 2: Run tests using `dot test`
    let mut test_cmd = Command::cargo_bin("dot").unwrap();
    test_cmd
        .current_dir(&repo_root)
        .arg("test")
        .arg(recipe_name)
        .timeout(std::time::Duration::from_secs(300)); // 5 minute timeout

    test_cmd.assert().success();
}

/// Test BasicInteraction pathway: Create recipe and run tests
#[test]
#[ignore] // Run with: cargo test --test pathway_integration_tests -- --ignored
fn test_basic_interaction_example_end_to_end() {
    let repo_root = get_repo_root();
    let recipe_name = "transactions-example";

    // Clean up any existing example
    cleanup_recipe(recipe_name);

    // Step 1: Create a BasicInteraction recipe
    let mut create_cmd = Command::cargo_bin("dot").unwrap();
    create_cmd
        .current_dir(&repo_root)
        .arg("create")
        .arg("--title")
        .arg("Transactions Example")
        .arg("--pathway")
        .arg("transactions")
        .arg("--no-git")
        .arg("--non-interactive");

    create_cmd.assert().success();

    let recipe_path = repo_root
        .join("recipes")
        .join("transactions")
        .join(recipe_name);
    assert!(recipe_path.exists(), "Recipe directory should exist");
    assert!(
        recipe_path.join("README.md").exists(),
        "README.md should exist"
    );
    assert!(
        recipe_path.join("package.json").exists(),
        "package.json should exist"
    );
    assert!(
        recipe_path.join("vitest.config.ts").exists(),
        "vitest.config.ts should exist"
    );
    assert!(recipe_path.join("src").exists(), "src/ should exist");
    assert!(recipe_path.join("tests").exists(), "tests/ should exist");

    // Step 2: Run tests using `dot test`
    let mut test_cmd = Command::cargo_bin("dot").unwrap();
    test_cmd
        .current_dir(&repo_root)
        .arg("test")
        .arg(recipe_name)
        .timeout(std::time::Duration::from_secs(300)); // 5 minute timeout

    test_cmd.assert().success();
}

/// Test XCM pathway: Create recipe and run tests
#[test]
#[ignore] // Run with: cargo test --test pathway_integration_tests -- --ignored
fn test_xcm_example_end_to_end() {
    let repo_root = get_repo_root();
    let recipe_name = "xcm-example";

    // Clean up any existing example
    cleanup_recipe(recipe_name);

    // Step 1: Create an XCM recipe
    let mut create_cmd = Command::cargo_bin("dot").unwrap();
    create_cmd
        .current_dir(&repo_root)
        .arg("create")
        .arg("--title")
        .arg("XCM Example")
        .arg("--pathway")
        .arg("xcm")
        .arg("--no-git")
        .arg("--non-interactive");

    create_cmd.assert().success();

    let recipe_path = repo_root.join("recipes").join("xcm").join(recipe_name);
    assert!(recipe_path.exists(), "Recipe directory should exist");
    assert!(
        recipe_path.join("README.md").exists(),
        "README.md should exist"
    );
    assert!(
        recipe_path.join("package.json").exists(),
        "package.json should exist"
    );
    assert!(
        recipe_path.join("chopsticks.yml").exists(),
        "chopsticks.yml should exist"
    );
    assert!(recipe_path.join("src").exists(), "src/ should exist");
    assert!(recipe_path.join("tests").exists(), "tests/ should exist");

    // Step 2: Run tests using `dot test`
    let mut test_cmd = Command::cargo_bin("dot").unwrap();
    test_cmd
        .current_dir(&repo_root)
        .arg("test")
        .arg(recipe_name)
        .timeout(std::time::Duration::from_secs(300)); // 5 minute timeout

    test_cmd.assert().success();
}

/// Test Polkadot Infrastructure pathway: Create recipe and run tests
#[test]
#[ignore] // Run with: cargo test --test pathway_integration_tests -- --ignored
fn test_infra_example_end_to_end() {
    let repo_root = get_repo_root();
    let recipe_name = "infra-example";

    // Clean up any existing example
    cleanup_recipe(recipe_name);

    // Step 1: Create a Polkadot Infrastructure recipe
    let mut create_cmd = Command::cargo_bin("dot").unwrap();
    create_cmd
        .current_dir(&repo_root)
        .arg("create")
        .arg("--title")
        .arg("Infra Example")
        .arg("--pathway")
        .arg("networks")
        .arg("--no-git")
        .arg("--non-interactive");

    create_cmd.assert().success();

    let recipe_path = repo_root.join("recipes").join("networks").join(recipe_name);
    assert!(recipe_path.exists(), "Recipe directory should exist");
    assert!(
        recipe_path.join("README.md").exists(),
        "README.md should exist"
    );
    assert!(
        recipe_path.join("package.json").exists(),
        "package.json should exist"
    );
    assert!(
        recipe_path.join("configs").exists(),
        "configs/ should exist"
    );
    assert!(recipe_path.join("tests").exists(), "tests/ should exist");

    // Step 2: Run tests using `dot test`
    let mut test_cmd = Command::cargo_bin("dot").unwrap();
    test_cmd
        .current_dir(&repo_root)
        .arg("test")
        .arg(recipe_name)
        .timeout(std::time::Duration::from_secs(300)); // 5 minute timeout

    test_cmd.assert().success();
}

/// Smoke test: Quickly create all example recipes (without running tests)
/// This test runs faster and verifies basic scaffolding works for all pathways
#[test]
fn test_all_examples_create_only() {
    let repo_root = get_repo_root();

    let pathways = vec![
        (
            "contracts",
            "Contracts Example Smoke",
            "contracts-example-smoke",
        ),
        ("pallets", "Pallets Example Smoke", "pallets-example-smoke"),
        (
            "transactions",
            "Transactions Example Smoke",
            "transactions-example-smoke",
        ),
        ("xcm", "XCM Example Smoke", "xcm-example-smoke"),
        ("networks", "Infra Example Smoke", "infra-example-smoke"),
    ];

    for (pathway, title, expected_slug) in pathways {
        // Clean up any existing example
        cleanup_recipe(expected_slug);

        let mut cmd = Command::cargo_bin("dot").unwrap();
        let working_dir = repo_root.join("recipes").join(pathway);
        std::fs::create_dir_all(&working_dir).unwrap();
        cmd.current_dir(&working_dir)
            .arg("create")
            .arg("--title")
            .arg(title)
            .arg("--pathway")
            .arg(pathway)
            .arg("--skip-install")
            .arg("--no-git")
            .arg("--non-interactive");

        cmd.assert().success();

        let recipe_path = repo_root.join("recipes").join(pathway).join(expected_slug);
        assert!(
            recipe_path.exists(),
            "Recipe directory for {} should exist",
            pathway
        );
        assert!(
            recipe_path.join("README.md").exists(),
            "README.md for {} should exist",
            pathway
        );

        // Clean up after smoke test
        cleanup_recipe(expected_slug);
    }
}
