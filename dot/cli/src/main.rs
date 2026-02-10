//! CLI wrapper for Polkadot Cookbook SDK
//!
//! This is a thin wrapper around the polkadot-cookbook-sdk library that provides
//! a command-line interface for creating and managing Polkadot projects.

use anyhow::Result;
use clap::{Parser, Subcommand};
use cliclack::{clear_screen, confirm, input, intro, note, outro, outro_cancel, select};
use colored::Colorize;
use polkadot_cookbook_sdk::{
    config::{ProjectConfig, ProjectPathway, ProjectType},
    dependencies::check_pathway_dependencies,
    Scaffold,
};
use std::path::PathBuf;

/// Polkadot brand pink color (#E6007A)
trait PolkadotColor {
    fn polkadot_pink(&self) -> colored::ColoredString;
}

impl PolkadotColor for &str {
    fn polkadot_pink(&self) -> colored::ColoredString {
        self.truecolor(230, 0, 122)
    }
}

impl PolkadotColor for String {
    fn polkadot_pink(&self) -> colored::ColoredString {
        self.truecolor(230, 0, 122)
    }
}

#[derive(Parser)]
#[command(name = "dot")]
#[command(about = "dot CLI - a command-line tool for Polkadot development", long_about = None)]
#[command(
    after_help = "EXAMPLES:\n  # Create a new smart contract project (interactive)\n  dot create\n\n  # Create a project non-interactively\n  dot create --title \"My DeFi Protocol\" --pathway contracts --non-interactive\n\n  # Run tests on a project\n  dot test my-project"
)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new project (interactive)
    Create {
        /// Project title (for non-interactive mode)
        #[arg(long)]
        title: Option<String>,

        /// Project pathway (for non-interactive mode): contracts, pallets, transactions, xcm, networks
        #[arg(long)]
        pathway: Option<String>,

        /// Skip npm install
        #[arg(long, default_value = "false")]
        skip_install: bool,

        /// Skip git repository initialization
        #[arg(long, default_value = "false")]
        no_git: bool,

        /// Pallet-only mode: no runtime, no PAPI (advanced users)
        #[arg(long, default_value = "false")]
        pallet_only: bool,

        /// Non-interactive mode (use defaults, require title argument)
        #[arg(long, default_value = "false")]
        non_interactive: bool,
    },
    /// Create a new smart contract project
    Contract {
        /// Project title
        #[arg(long)]
        title: Option<String>,

        /// Skip npm install
        #[arg(long, default_value = "false")]
        skip_install: bool,

        /// Skip git repository initialization
        #[arg(long, default_value = "false")]
        no_git: bool,

        /// Non-interactive mode (use defaults, require title argument)
        #[arg(long, default_value = "false")]
        non_interactive: bool,
    },
    /// Create a new parachain project
    Parachain {
        /// Project title
        #[arg(long)]
        title: Option<String>,

        /// Skip npm install
        #[arg(long, default_value = "false")]
        skip_install: bool,

        /// Skip git repository initialization
        #[arg(long, default_value = "false")]
        no_git: bool,

        /// Pallet-only mode: no runtime, no PAPI (advanced users)
        #[arg(long, default_value = "false")]
        pallet_only: bool,

        /// Non-interactive mode (use defaults, require title argument)
        #[arg(long, default_value = "false")]
        non_interactive: bool,
    },
    /// Run project tests
    Test {
        /// Project path (defaults to current directory)
        #[arg(value_name = "PATH")]
        path: Option<String>,

        /// Run only Rust tests (cargo test)
        #[arg(long)]
        rust: bool,

        /// Run only TypeScript tests (npm test)
        #[arg(long)]
        ts: bool,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    // Force colored output
    colored::control::set_override(true);

    // Initialize tracing (default to warn to keep CLI output clean)
    tracing_subscriber::fmt()
        .with_env_filter(std::env::var("RUST_LOG").unwrap_or_else(|_| "warn".to_string()))
        .init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Create {
            title,
            pathway,
            skip_install,
            no_git,
            pallet_only,
            non_interactive,
        } => {
            handle_create(
                title,
                pathway,
                skip_install,
                no_git,
                pallet_only,
                non_interactive,
            )
            .await?;
        }
        Commands::Contract {
            title,
            skip_install,
            no_git,
            non_interactive,
        } => {
            handle_create(
                title,
                Some("contracts".to_string()),
                skip_install,
                no_git,
                false, // pallet_only not applicable for contracts
                non_interactive,
            )
            .await?;
        }
        Commands::Parachain {
            title,
            skip_install,
            no_git,
            pallet_only,
            non_interactive,
        } => {
            handle_create(
                title,
                Some("pallets".to_string()),
                skip_install,
                no_git,
                pallet_only,
                non_interactive,
            )
            .await?;
        }
        Commands::Test { path, rust, ts } => {
            handle_project_test(path, rust, ts).await?;
        }
    }

    Ok(())
}

async fn handle_create(
    title: Option<String>,
    pathway: Option<String>,
    skip_install: bool,
    no_git: bool,
    pallet_only: bool,
    non_interactive: bool,
) -> Result<()> {
    // Non-interactive mode: require title argument
    if non_interactive {
        let title = title.ok_or_else(|| {
            anyhow::anyhow!("Title argument (--title) is required in non-interactive mode")
        })?;
        return run_non_interactive(&title, pathway, skip_install, no_git, pallet_only).await;
    }

    // Interactive mode with cliclack
    clear_screen()?;

    // Add spacing before intro
    println!("\n");

    // Polkadot-themed intro
    let intro_text = format!("{}", "dot CLI".polkadot_pink().bold());
    intro(&intro_text)?;

    let note_title = "Polkadot Project Setup".polkadot_pink().to_string();
    note(
        &note_title,
        "Let's create your new Polkadot project. This will scaffold the project structure,\ngenerate template files, and set up the testing environment.",
    )?;

    // Step 1: Determine pathway - either from argument or by asking user
    let pathway: ProjectPathway = if let Some(pathway_str) = pathway {
        // Pathway provided via argument (from alias commands like `dot contract`)
        // Parse the pathway string to ProjectPathway
        match pathway_str.as_str() {
            "contracts" => ProjectPathway::Contracts,
            "pallets" => ProjectPathway::Pallets,
            "transactions" => ProjectPathway::Transactions,
            "xcm" => ProjectPathway::Xcm,
            "networks" => ProjectPathway::Networks,
            _ => {
                outro_cancel(format!(
                    "Invalid pathway: '{}'. Valid options: contracts, pallets, transactions, xcm, networks",
                    pathway_str
                ))?;
                std::process::exit(1);
            }
        }
    } else {
        // No pathway provided - ask user interactively
        let pathway_question = "What would you like to build?".polkadot_pink().to_string();
        select(&pathway_question)
            .item(
                ProjectPathway::Contracts,
                "Smart Contract (Solidity)",
                "Build, test, and run Solidity smart contracts",
            )
            .item(
                ProjectPathway::Pallets,
                "Parachain (Polkadot SDK)",
                "Build a full parachain with custom pallets and PAPI integration",
            )
            .item(
                ProjectPathway::Transactions,
                "Chain Transactions",
                "Single-chain transactions and state queries with PAPI",
            )
            .item(
                ProjectPathway::Xcm,
                "Cross-Chain Transactions (XCM)",
                "Cross-chain asset transfers and cross-chain calls with Chopsticks",
            )
            .item(
                ProjectPathway::Networks,
                "Polkadot Networks (Zombienet / Chopsticks)",
                "Run Polkadot networks locally for testing",
            )
            .interact()?
    };

    // Map pathway to project type (for template selection)
    let project_type = match pathway {
        ProjectPathway::Pallets => ProjectType::PolkadotSdk,
        ProjectPathway::Contracts => ProjectType::Solidity,
        ProjectPathway::Transactions => ProjectType::Transactions,
        ProjectPathway::Xcm => ProjectType::Xcm,
        ProjectPathway::Networks => ProjectType::Networks,
    };

    // Interactive mode always creates full parachain
    // (pallet-only mode is only available via --pallet-only flag)

    // Check dependencies for the selected pathway
    check_dependencies_interactive(&pathway)?;

    // Step 2: Ask for project name (now that user knows the pathway)
    let name_question = "What is your project name?".polkadot_pink().to_string();
    let hint_text = "(e.g., 'my-parachain')".dimmed().to_string();
    let title: String = input(format!("{name_question} {hint_text}"))
        .placeholder("my-project")
        .validate(|input: &String| {
            if input.trim().is_empty() {
                Err("Title cannot be empty")
            } else if let Err(e) = polkadot_cookbook_sdk::config::validate_title(input) {
                Err(Box::leak(e.to_string().into_boxed_str()) as &str)
            } else {
                Ok(())
            }
        })
        .interact()?;

    // Auto-generate slug from title
    let title = title.trim().to_string();
    let slug = polkadot_cookbook_sdk::config::title_to_slug(&title);

    // Git initialization is default (unless --no-git flag is used)
    let init_git = !no_git;

    // Npm install is default (unless --skip-install flag is used)
    // Skip install flag is already set from CLI args

    // Calculate derived values for the summary
    let project_path = PathBuf::from(".").join(&slug);

    // Generate directory tree based on pathway
    let tree_structure = match pathway {
        ProjectPathway::Pallets => {
            if pallet_only {
                format!(
                    "{}/\n\
                     ├── README.md                    Pallet development guide and usage\n\
                     ├── Cargo.toml                   Rust workspace configuration\n\
                     ├── rust-toolchain.toml          Specifies Rust version for consistency\n\
                     └── pallets/                     Custom pallet directory\n\
                         └── template/                Example pallet implementation\n\
                             ├── Cargo.toml           Pallet dependencies\n\
                             └── src/                 Pallet source code\n\
                                 ├── lib.rs           Main pallet logic and extrinsics\n\
                                 ├── mock.rs          Mock runtime for testing\n\
                                 ├── tests.rs         Unit and integration tests\n\
                                 ├── benchmarking.rs  Performance benchmarks\n\
                                 └── weights.rs       Weight calculations for extrinsics",
                    slug
                )
            } else {
                format!(
                    "{}/\n\
                     ├── README.md                        Project documentation and guide\n\
                     ├── Cargo.toml                       Rust workspace configuration\n\
                     ├── rust-toolchain.toml              Specifies Rust version for consistency\n\
                     ├── LICENSE                          Project license (MIT/Apache-2.0)\n\
                     ├── Dockerfile                       Container image definition\n\
                     │\n\
                     ├── package.json                     Node.js dependencies for PAPI tests\n\
                     ├── tsconfig.json                    TypeScript compiler configuration\n\
                     ├── vitest.config.ts                 Test framework configuration\n\
                     │\n\
                     ├── runtime/                         Parachain runtime implementation\n\
                     │   ├── Cargo.toml                   Runtime dependencies and features\n\
                     │   ├── build.rs                     Build script for WASM compilation\n\
                     │   └── src/                         Runtime logic (pallets, configs)\n\
                     │\n\
                     ├── node/                            Node binary implementation\n\
                     │   ├── Cargo.toml                   Node dependencies\n\
                     │   ├── build.rs                     Node build script\n\
                     │   └── src/                         CLI, RPC, and service logic\n\
                     │\n\
                     ├── pallets/                         Custom pallets for your chain\n\
                     │   └── template/                    Example pallet template\n\
                     │       ├── Cargo.toml               Pallet dependencies\n\
                     │       └── src/                     Pallet logic, tests, benchmarks\n\
                     │\n\
                     ├── scripts/                         Utility scripts for development\n\
                     │   ├── setup-zombienet-binaries.sh  Download zombienet binaries\n\
                     │   └── start-dev-node.sh            Start local development node\n\
                     │\n\
                     ├── tests/                           Integration tests using PAPI\n\
                     │   └── template-pallet.test.ts      Example pallet tests\n\
                     │\n\
                     ├── chopsticks.yml                   Chopsticks config for local testing\n\
                     ├── dev_chain_spec.json              Development chain specification\n\
                     ├── zombienet.toml                   Parachain node network config\n\
                     └── zombienet-omni-node.toml         Omni-node network configuration\n\n\
                     ─────────────────────────────────────────────────────────────────────────────\n\n\
                     Based on: polkadot-sdk-parachain-template\n\
                     Polkadot SDK: v2503.0.1\n\
                     Release: https://github.com/paritytech/polkadot-sdk/releases/tag/polkadot-v2503.0.1",
                    slug
                )
            }
        }
        ProjectPathway::Contracts => {
            format!(
                "{}/\n\
                 ├── README.md               Project documentation and guide\n\
                 ├── package.json            Dependencies for Hardhat and testing\n\
                 ├── hardhat.config.ts       Hardhat configuration for Ethereum tooling\n\
                 ├── contracts/              Solidity smart contracts\n\
                 │   └── Contract.sol        Example smart contract\n\
                 ├── scripts/                Deployment and interaction scripts\n\
                 │   └── deploy.ts           Contract deployment script\n\
                 ├── tests/                  Contract tests\n\
                 │   └── Contract.test.ts    Example contract tests\n\
                 └── src/                    Additional TypeScript source files",
                slug
            )
        }
        ProjectPathway::Transactions => {
            format!(
                "{}/\n\
                 ├── README.md            Project documentation and guide\n\
                 ├── package.json         PAPI and testing dependencies\n\
                 ├── tsconfig.json        TypeScript compiler configuration\n\
                 ├── vitest.config.ts     Test framework configuration\n\
                 ├── src/                 Transaction building and helpers\n\
                 │   └── example.ts       Example transaction implementation\n\
                 └── tests/               Integration tests\n\
                     └── example.test.ts  Example transaction tests",
                slug
            )
        }
        ProjectPathway::Xcm => {
            format!(
                "{}/\n\
                 ├── README.md             Project documentation and guide\n\
                 ├── package.json          PAPI and Chopsticks dependencies\n\
                 ├── chopsticks.yml        Local multi-chain testing configuration\n\
                 ├── tsconfig.json         TypeScript compiler configuration\n\
                 ├── vitest.config.ts      Test framework configuration\n\
                 ├── src/                  XCM message building helpers\n\
                 │   └── xcm-helpers.ts    XCM utility functions\n\
                 └── tests/                Cross-chain interaction tests\n\
                     └── xcm.test.ts       XCM transfer tests",
                slug
            )
        }
        ProjectPathway::Networks => {
            format!(
                "{}/\n\
                 ├── README.md               Project documentation and guide\n\
                 ├── package.json            Testing dependencies\n\
                 ├── configs/                Network configuration files\n\
                 │   ├── chopsticks.yml      Chopsticks multi-chain setup\n\
                 │   └── network.toml        Zombienet network specification\n\
                 └── tests/                  Network infrastructure tests\n\
                     └── network.test.ts     Network connectivity tests",
                slug
            )
        }
    };

    // Show configuration summary and get confirmation
    let summary_title = "Configuration Summary".polkadot_pink().to_string();
    note(
        &summary_title,
        format!(
            "{:<16} {}\n\
             {:<16} {}\n\
             {:<16} {}\n\
             {:<16} {}\n\
             {:<16} {}\n\n\
             ─────────────────────────────────────────────────────────────────────────────\n\n\
             Directory structure:\n\n\n{}",
            "Title:".polkadot_pink(),
            title.polkadot_pink().bold(),
            "Slug:".polkadot_pink(),
            slug.dimmed(),
            "Pathway:".polkadot_pink(),
            match pathway {
                ProjectPathway::Pallets => {
                    if pallet_only {
                        "Custom Pallet (Pallet-only)"
                    } else {
                        "Custom Pallet"
                    }
                }
                ProjectPathway::Contracts => "Smart Contract",
                ProjectPathway::Transactions => "Chain Transactions",
                ProjectPathway::Xcm => "Cross-Chain Transactions",
                ProjectPathway::Networks => "Polkadot Networks",
            },
            "Location:".polkadot_pink(),
            project_path.display(),
            "Git Init:".polkadot_pink(),
            if init_git { "Yes" } else { "No" },
            tree_structure.dimmed()
        ),
    )?;

    let confirm_question = "Continue?".polkadot_pink().to_string();
    let should_continue = confirm(&confirm_question).initial_value(true).interact()?;

    if !should_continue {
        outro_cancel("Project creation cancelled")?;
        std::process::exit(0);
    }

    // Create project configuration
    let mut config = ProjectConfig::new(&slug)
        .with_title(&title)
        .with_destination(PathBuf::from("."))
        .with_git_init(init_git)
        .with_skip_install(skip_install)
        .with_project_type(project_type)
        .with_description("Replace with a short description.".to_string())
        .with_pathway(pathway);

    // Set parachain-specific flags
    config.pallet_only = pallet_only;

    // Create the project with progress indication
    let scaffold = Scaffold::new();

    // Create progress callback that prints status updates
    use polkadot_cookbook_sdk::scaffold::ProgressCallback;
    let progress_callback: ProgressCallback = Box::new(move |msg: &str| {
        println!("{} {}", "→".polkadot_pink(), msg);
    });

    println!("{}", "📁 Creating project...".polkadot_pink());

    match scaffold
        .create_project(config, Some(&progress_callback))
        .await
    {
        Ok(project_info) => {
            println!("{}", "✅ Project created successfully!".polkadot_pink());

            // Run tests to verify the setup works
            let should_run_tests = if pallet_only {
                println!(
                    "\n{}",
                    "Running cargo test to verify setup...".polkadot_pink()
                );
                true
            } else if matches!(pathway, ProjectPathway::Pallets) {
                // Skip tests for full parachain - requires lengthy compilation
                false
            } else {
                println!(
                    "\n{}",
                    "Running npm test to verify setup...".polkadot_pink()
                );
                true
            };

            if should_run_tests {
                let test_result = if pallet_only {
                    std::process::Command::new("cargo")
                        .arg("test")
                        .current_dir(&project_info.project_path)
                        .stdout(std::process::Stdio::inherit())
                        .stderr(std::process::Stdio::inherit())
                        .status()
                } else {
                    std::process::Command::new("npm")
                        .arg("test")
                        .current_dir(&project_info.project_path)
                        .stdout(std::process::Stdio::inherit())
                        .stderr(std::process::Stdio::inherit())
                        .status()
                };

                match test_result {
                    Ok(status) if status.success() => {
                        println!("{}", "✅ Tests passed!".polkadot_pink());
                    }
                    Ok(_) => {
                        println!("{}", "⚠️  Tests failed - check your setup".yellow());
                    }
                    Err(e) => {
                        println!("{}", format!("⚠️  Could not run tests: {}", e).yellow());
                    }
                }
            }

            let project_title = "📦 Project Created".polkadot_pink().to_string();
            note(
                &project_title,
                format!(
                    "Location:   {}",
                    project_info
                        .project_path
                        .display()
                        .to_string()
                        .polkadot_pink(),
                ),
            )?;

            let steps_title = "📝 Next Steps".polkadot_pink().to_string();

            // Generate context-aware next steps based on project type and mode
            let next_steps = if pallet_only {
                format!(
                    "{} Implement your pallet\n   {} {}\n\n\
                     {} Write unit tests\n   {} {}\n\n\
                     {} Build and test\n   {} {}",
                    "1.".polkadot_pink().bold(),
                    "→".dimmed(),
                    format!(
                        "{}/pallets/template/src/lib.rs",
                        project_info.project_path.display()
                    )
                    .polkadot_pink(),
                    "2.".polkadot_pink().bold(),
                    "→".dimmed(),
                    format!(
                        "{}/pallets/template/src/tests.rs",
                        project_info.project_path.display()
                    )
                    .polkadot_pink(),
                    "3.".polkadot_pink().bold(),
                    "→".dimmed(),
                    format!("cd {} && cargo test", project_info.project_path.display())
                        .polkadot_pink()
                )
            } else if matches!(pathway, ProjectPathway::Pallets) {
                format!(
                    "{} Build your parachain\n   {} {}\n\n\
                     {} Start development node\n   {} {}\n\n\
                     {} Run integration tests\n   {} {}",
                    "1.".polkadot_pink().bold(),
                    "→".dimmed(),
                    format!(
                        "cd {} && cargo build --release",
                        project_info.project_path.display()
                    )
                    .polkadot_pink(),
                    "2.".polkadot_pink().bold(),
                    "→".dimmed(),
                    format!(
                        "{}/scripts/start-dev-node.sh",
                        project_info.project_path.display()
                    )
                    .polkadot_pink(),
                    "3.".polkadot_pink().bold(),
                    "→".dimmed(),
                    format!("cd {} && npm test", project_info.project_path.display())
                        .polkadot_pink()
                )
            } else {
                // Default for other pathways
                format!(
                    "{} Change to project directory\n   {} {}\n\n\
                     {} Add implementation\n   {} {}\n\n\
                     {} Write tests\n   {} {}\n\n\
                     {} Run tests\n   {} {}",
                    "1.".polkadot_pink().bold(),
                    "→".dimmed(),
                    format!("cd {}", project_info.project_path.display()).polkadot_pink(),
                    "2.".polkadot_pink().bold(),
                    "→".dimmed(),
                    "src/".polkadot_pink(),
                    "3.".polkadot_pink().bold(),
                    "→".dimmed(),
                    "tests/".polkadot_pink(),
                    "4.".polkadot_pink().bold(),
                    "→".dimmed(),
                    "npm test".polkadot_pink()
                )
            };

            note(&steps_title, next_steps)?;

            outro("🎉 All set!".polkadot_pink().to_string())?;
        }
        Err(e) => {
            println!("{}", format!("❌ Failed to create project: {e}").red());
            outro_cancel(format!("Error: {e}"))?;
            std::process::exit(1);
        }
    }

    Ok(())
}

/// Run in non-interactive mode (for CI/CD or scripting)
async fn run_non_interactive(
    title: &str,
    pathway: Option<String>,
    skip_install: bool,
    no_git: bool,
    pallet_only: bool,
) -> Result<()> {
    // Validate title
    if let Err(e) = polkadot_cookbook_sdk::config::validate_title(title) {
        eprintln!("❌ Invalid project title: {e}");
        eprintln!("Title must be properly formatted.");
        std::process::exit(1);
    }

    // Generate slug from title
    let slug = polkadot_cookbook_sdk::config::title_to_slug(title);

    // Parse pathway to project type
    let project_type = if let Some(p) = pathway {
        match p.as_str() {
            "pallets" => ProjectType::PolkadotSdk,
            "contracts" => ProjectType::Solidity,
            "transactions" => ProjectType::Transactions,
            "xcm" => ProjectType::Xcm,
            "networks" => ProjectType::Networks,
            "request-new" => {
                eprintln!("🎯 Request a New Recipe Template\n");
                eprintln!("We'd love to support your use case! Please create a GitHub issue:\n");
                eprintln!(
                    "→ https://github.com/polkadot-developers/polkadot-cookbook/issues/new\n"
                );
                eprintln!("Include in your issue:");
                eprintln!("• What kind of recipe you want to create");
                eprintln!("• What technology/framework it involves");
                eprintln!("• Example use cases");
                eprintln!("• Any specific requirements\n");
                eprintln!(
                    "We'll review your request and add the template if it fits the cookbook!"
                );
                std::process::exit(0);
            }
            _ => {
                eprintln!("❌ Invalid pathway: {p}");
                eprintln!(
                    "Valid pathways: contracts, pallets, transactions, xcm, networks, request-new"
                );
                std::process::exit(1);
            }
        }
    } else {
        ProjectType::PolkadotSdk // Default
    };

    // Title is already provided as input parameter

    // Determine pathway from project type
    let pathway_value = match project_type {
        ProjectType::PolkadotSdk => Some(ProjectPathway::Pallets),
        ProjectType::Solidity => Some(ProjectPathway::Contracts),
        ProjectType::Transactions => Some(ProjectPathway::Transactions),
        ProjectType::Xcm => Some(ProjectPathway::Xcm),
        ProjectType::Networks => Some(ProjectPathway::Networks),
    };

    println!(
        "{} {} ({})",
        "Creating project:".polkadot_pink(),
        title.polkadot_pink().bold(),
        slug.dimmed()
    );

    // Create project configuration with provided or default values
    let mut config = ProjectConfig::new(&slug)
        .with_title(title)
        .with_destination(PathBuf::from("."))
        .with_git_init(!no_git)
        .with_skip_install(skip_install)
        .with_project_type(project_type)
        .with_description("Replace with a short description.".to_string());

    // Add optional fields if provided
    if let Some(p) = pathway_value {
        config = config.with_pathway(p);
    }

    // Set parachain-specific flags (non-interactive mode)
    config.pallet_only = pallet_only;

    // Create the project with progress callback
    let scaffold = Scaffold::new();

    // Create progress callback that prints status updates
    use polkadot_cookbook_sdk::scaffold::ProgressCallback;
    let progress_callback: ProgressCallback = Box::new(move |msg: &str| {
        println!("{} {}", "→".polkadot_pink(), msg);
    });

    match scaffold
        .create_project(config, Some(&progress_callback))
        .await
    {
        Ok(project_info) => {
            println!(
                "{}",
                "✅ Project created successfully!".polkadot_pink().bold()
            );
            println!(
                "{} {}",
                "Path:".polkadot_pink(),
                project_info.project_path.display()
            );
            println!(
                "{} {}",
                "Git Init:".polkadot_pink(),
                if project_info.git_initialized {
                    "Yes"
                } else {
                    "No"
                }
            );

            // Print next steps
            println!();
            println!("{}", "📝 Next Steps:".polkadot_pink().bold());
            if pallet_only {
                println!(
                    "  {} Change to project directory: {}",
                    "1.".polkadot_pink(),
                    format!("cd {}", project_info.project_path.display()).polkadot_pink()
                );
                println!(
                    "  {} Add pallet implementation: {}",
                    "2.".polkadot_pink(),
                    "pallets/template/src/lib.rs".polkadot_pink()
                );
                println!(
                    "  {} Run tests: {}",
                    "3.".polkadot_pink(),
                    "cargo test".polkadot_pink()
                );
            } else if matches!(project_type, ProjectType::PolkadotSdk) {
                println!(
                    "  {} Change to project directory: {}",
                    "1.".polkadot_pink(),
                    format!("cd {}", project_info.project_path.display()).polkadot_pink()
                );
                println!(
                    "  {} Build and run your parachain: {}",
                    "2.".polkadot_pink(),
                    "cargo build --release".polkadot_pink()
                );
                println!(
                    "  {} Run tests: {}",
                    "3.".polkadot_pink(),
                    "npm test".polkadot_pink()
                );
            } else {
                println!(
                    "  {} Change to project directory: {}",
                    "1.".polkadot_pink(),
                    format!("cd {}", project_info.project_path.display()).polkadot_pink()
                );
                println!(
                    "  {} Add implementation: {}",
                    "2.".polkadot_pink(),
                    "src/".polkadot_pink()
                );
                println!(
                    "  {} Write tests: {}",
                    "3.".polkadot_pink(),
                    "tests/".polkadot_pink()
                );
                println!(
                    "  {} Run tests: {}",
                    "4.".polkadot_pink(),
                    "npm test".polkadot_pink()
                );
            }
        }
        Err(e) => {
            eprintln!("❌ Failed to create project: {e}");
            std::process::exit(1);
        }
    }

    Ok(())
}

/// Run tests for a project and return whether they passed
async fn run_project_tests(
    project_path: &std::path::Path,
    show_notes: bool,
    rust_only: bool,
    ts_only: bool,
) -> Result<bool> {
    let has_cargo = project_path.join("Cargo.toml").exists();
    let has_package_json = project_path.join("package.json").exists();

    // Determine which tests to run
    let run_rust = has_cargo && !ts_only;
    let run_ts = has_package_json && !rust_only;

    if !run_rust && !run_ts {
        eprintln!("No tests to run. Project has neither Cargo.toml nor package.json");
        return Ok(false);
    }

    if show_notes {
        let test_types: Vec<&str> = [
            if run_rust {
                Some("Rust (cargo test)")
            } else {
                None
            },
            if run_ts {
                Some("TypeScript (npm test)")
            } else {
                None
            },
        ]
        .into_iter()
        .flatten()
        .collect();
        note("Running Tests", test_types.join(" + "))?;
    }

    let mut all_passed = true;

    // Run Rust tests
    if run_rust {
        println!("\n{}\n", "Running cargo test...".polkadot_pink().bold());

        let status = std::process::Command::new("cargo")
            .args(["test", "--all-features"])
            .current_dir(project_path)
            .status()?;

        println!();

        if status.success() {
            println!("{}", "✅ Rust tests passed!".polkadot_pink().bold());
        } else {
            eprintln!("{}", "❌ Rust tests failed".red().bold());
            all_passed = false;
        }
    }

    // Run TypeScript tests
    if run_ts {
        println!("\n{}\n", "Running npm test...".polkadot_pink().bold());

        let status = std::process::Command::new("npm")
            .args(["test"])
            .current_dir(project_path)
            .status()?;

        println!();

        if status.success() {
            println!("{}", "✅ TypeScript tests passed!".polkadot_pink().bold());
        } else {
            eprintln!("{}", "❌ TypeScript tests failed".red().bold());
            all_passed = false;
        }
    }

    Ok(all_passed)
}

async fn handle_project_test(path: Option<String>, rust_only: bool, ts_only: bool) -> Result<()> {
    let project_path = get_project_path(path)?;

    intro(format!(
        "🧪 Testing Project: {}",
        project_path
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .polkadot_pink()
    ))?;

    let tests_passed = run_project_tests(&project_path, true, rust_only, ts_only).await?;

    if !tests_passed {
        outro_cancel("Tests failed")?;
        std::process::exit(1);
    }

    outro("✅ Testing complete!")?;
    Ok(())
}

/// Check dependencies for a pathway and prompt user if any are missing
fn check_dependencies_interactive(pathway: &ProjectPathway) -> Result<()> {
    let results = check_pathway_dependencies(pathway);

    let missing: Vec<_> = results.iter().filter(|r| !r.installed).collect();

    if missing.is_empty() {
        return Ok(());
    }

    // Show missing dependencies
    let mut message = String::from("⚠️  Missing dependencies:\n\n");

    for result in &missing {
        let dep = &result.dependency;
        message.push_str(&format!("  ✗ {}\n", dep.name.polkadot_pink()));
    }

    message.push_str("\nInstallation instructions:\n\n");

    for result in &missing {
        let dep = &result.dependency;
        message.push_str(&format!("• {}\n", dep.name.bold()));
        message.push_str(&format!("  {}\n", dep.install_instructions));
        if !dep.install_url.is_empty() {
            message.push_str(&format!("  More info: {}\n", dep.install_url.dimmed()));
        }
        message.push('\n');
    }

    note("Dependencies", message.trim())?;

    let should_continue = confirm("Continue without all dependencies? (setup may fail)")
        .initial_value(false)
        .interact()?;

    if !should_continue {
        outro_cancel("Please install missing dependencies and try again")?;
        std::process::exit(1);
    }

    Ok(())
}

fn get_project_path(slug: Option<String>) -> Result<PathBuf> {
    if let Some(path_str) = slug {
        // Path/slug provided - treat as relative or absolute path
        let path = PathBuf::from(&path_str);

        // Check if it's a valid project directory
        if path.exists() && is_project_directory(&path) {
            return Ok(path.canonicalize().unwrap_or(path));
        }

        // Try as relative path from current directory
        let current = std::env::current_dir()?;
        let relative_path = current.join(&path_str);
        if relative_path.exists() && is_project_directory(&relative_path) {
            return Ok(relative_path.canonicalize().unwrap_or(relative_path));
        }

        eprintln!("Project not found: {path_str}");
        eprintln!("Make sure the path exists and contains a package.json or Cargo.toml");
        std::process::exit(1);
    } else {
        // No path provided - check if current directory is a project
        let current = std::env::current_dir()?;

        if is_project_directory(&current) {
            return Ok(current);
        }

        eprintln!("Not in a project directory.");
        eprintln!(
            "Either run from within a project directory or specify the path: dot test <path>"
        );
        std::process::exit(1);
    }
}

/// Check if a directory looks like a project (has package.json or Cargo.toml)
fn is_project_directory(path: &std::path::Path) -> bool {
    path.join("package.json").exists() || path.join("Cargo.toml").exists()
}
