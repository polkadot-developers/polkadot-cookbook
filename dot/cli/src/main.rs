//! CLI wrapper for Polkadot Cookbook SDK
//!
//! This is a thin wrapper around the polkadot-cookbook-sdk library that provides
//! a command-line interface for creating and managing Polkadot projects.

use anyhow::Result;
use clap::{Parser, Subcommand};
use cliclack::{clear_screen, confirm, input, intro, note, outro, outro_cancel, select, spinner};
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
    after_help = "EXAMPLES:\n  # Create a new smart contract project (interactive)\n  dot create\n\n  # Create a project non-interactively\n  dot create --title \"My DeFi Protocol\" --pathway contracts --non-interactive\n\n  # Run tests on a project\n  dot test my-project\n\n  # Submit a project as a recipe to the cookbook\n  dot submit my-project"
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
    /// Submit a project as a pull request to polkadot-cookbook
    Submit {
        /// Project slug (defaults to current directory)
        #[arg(value_name = "SLUG")]
        slug: Option<String>,

        /// Title for the pull request
        #[arg(short, long)]
        title: Option<String>,

        /// Body/description for the pull request
        #[arg(short, long)]
        body: Option<String>,
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
        Commands::Submit { slug, title, body } => {
            handle_project_submit(slug, title, body).await?;
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

async fn handle_project_submit(
    slug: Option<String>,
    title: Option<String>,
    body: Option<String>,
) -> Result<()> {
    clear_screen()?;

    // Determine if we're in standalone mode or cookbook repo mode
    let is_standalone =
        !std::path::Path::new("recipes").exists() || !std::path::Path::new("Cargo.toml").exists();

    if is_standalone {
        handle_standalone_submit(slug, title, body).await
    } else {
        handle_cookbook_repo_submit(slug, title, body).await
    }
}

/// Handle submission from within the polkadot-cookbook repository
async fn handle_cookbook_repo_submit(
    slug: Option<String>,
    title: Option<String>,
    body: Option<String>,
) -> Result<()> {
    let project_path = get_project_path(slug.clone())?;
    let project_slug = project_path
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    intro(format!(
        "📤 Submit Project: {}",
        project_slug.polkadot_pink()
    ))?;

    // Run tests before allowing submission
    note(
        "Pre-submission Check",
        "Running tests to ensure project quality",
    )?;
    let tests_passed = run_project_tests(&project_path, false, false, false).await?;

    if !tests_passed {
        outro_cancel(
            "Tests must pass before submission.\n\n\
             Please fix the failing tests and try again:\n\
             dot test",
        )?;
        std::process::exit(1);
    }

    println!(
        "\n{}\n",
        "✅ Tests passed! Proceeding with submission..."
            .polkadot_pink()
            .bold()
    );

    // Validate lock files are present
    note(
        "Lock File Check",
        "Verifying required lock files are present",
    )?;
    let project_metadata =
        match polkadot_cookbook_sdk::config::ProjectMetadata::from_project_directory(&project_path)
            .await
        {
            Ok(metadata) => metadata,
            Err(e) => {
                outro_cancel(format!("Failed to detect project type: {e}"))?;
                std::process::exit(1);
            }
        };

    if let Err(e) = polkadot_cookbook_sdk::config::validate_lock_files(
        &project_path,
        &project_metadata.project_type,
    ) {
        outro_cancel(format!("{e}"))?;
        std::process::exit(1);
    }

    println!(
        "{}\n",
        "✅ All required lock files present".polkadot_pink().bold()
    );

    // Get GitHub token
    let github_token = match get_github_token() {
        Ok(token) => token,
        Err(e) => {
            outro_cancel(format!(
                "GitHub authentication required.\n\n\
                 Error: {e}\n\n\
                 Please set GITHUB_TOKEN environment variable:\n\
                 export GITHUB_TOKEN=ghp_your_token_here\n\n\
                 Or authenticate with GitHub CLI:\n\
                 gh auth login\n\n\
                 Create a token at: https://github.com/settings/tokens/new\n\
                 Required scopes: repo, workflow"
            ))?;
            std::process::exit(1);
        }
    };

    // Get repository info from git remote
    let (owner, repo) = match get_repo_info() {
        Ok(info) => info,
        Err(e) => {
            outro_cancel(format!(
                "Failed to detect repository information.\n\n\
                 Error: {e}\n\n\
                 Make sure you have a git remote configured:\n\
                 git remote add origin https://github.com/YOUR_USERNAME/polkadot-cookbook.git"
            ))?;
            std::process::exit(1);
        }
    };

    // Read recipe metadata from frontmatter
    let readme_path = project_path.join("README.md");
    let (project_name, project_desc) =
        match polkadot_cookbook_sdk::metadata::parse_frontmatter_from_file(&readme_path).await {
            Ok(frontmatter) => (frontmatter.title, frontmatter.description),
            Err(_) => (
                project_slug.clone(),
                "A new Polkadot Cookbook recipe".to_string(),
            ),
        };

    // Auto-detect project type
    let project_type =
        match polkadot_cookbook_sdk::metadata::detect_project_type(&project_path).await {
            Ok(t) => match t {
                polkadot_cookbook_sdk::config::ProjectType::PolkadotSdk => "Polkadot SDK",
                polkadot_cookbook_sdk::config::ProjectType::Solidity => "Solidity",
                polkadot_cookbook_sdk::config::ProjectType::Xcm => "XCM",
                polkadot_cookbook_sdk::config::ProjectType::Transactions => "Transactions",
                polkadot_cookbook_sdk::config::ProjectType::Networks => "Network Infrastructure",
            },
            Err(_) => "Unknown",
        };

    // Check git status
    let git_status = std::process::Command::new("git")
        .args(["status", "--porcelain"])
        .current_dir(&project_path)
        .output()?;

    let has_changes = !String::from_utf8_lossy(&git_status.stdout)
        .trim()
        .is_empty();

    // Get current branch
    let current_branch_output = std::process::Command::new("git")
        .args(["branch", "--show-current"])
        .output()?;
    let current_branch = String::from_utf8_lossy(&current_branch_output.stdout)
        .trim()
        .to_string();

    note(
        "Project Info",
        format!(
            "Name:        {}\nSlug:        {}\nType:        {}\nBranch:      {}\nChanges:     {}",
            project_name.polkadot_pink(),
            project_slug.polkadot_pink(),
            project_type,
            current_branch.polkadot_pink(),
            if has_changes {
                "Yes (uncommitted)".yellow().to_string()
            } else {
                "None".dimmed().to_string()
            }
        ),
    )?;

    // Generate default PR title and body
    let default_title = title.unwrap_or_else(|| format!("feat(recipe): add {project_slug}"));
    let default_body = body.unwrap_or_else(|| {
        format!(
            "## Summary\n\n\
             This PR adds a new {project_type} recipe: **{project_name}**\n\n\
             {project_desc}\n\n\
             ## Recipe Details\n\n\
             - **Type**: {project_type}\n\
             - **Slug**: `{project_slug}`\n\n\
             ## Testing\n\n\
             - [ ] All tests pass\n\
             - [ ] Code is properly formatted\n\
             - [ ] Documentation is complete\n\n\
             ## Notes\n\n\
             This recipe is ready for review and does not require a prior proposal issue. \
             The Polkadot Cookbook accepts direct recipe contributions via PR."
        )
    });

    // Show what will be done
    note(
        "Pull Request Preview",
        format!(
            "Title:\n{}\n\nDescription:\n{}",
            default_title.polkadot_pink(),
            default_body.dimmed()
        ),
    )?;

    // Confirm submission
    let should_continue = confirm("Continue with PR creation?".polkadot_pink().to_string())
        .initial_value(true)
        .interact()?;

    if !should_continue {
        outro_cancel("Recipe submission cancelled")?;
        std::process::exit(0);
    }

    // If there are uncommitted changes, commit them
    if has_changes {
        let should_commit = confirm("Commit uncommitted changes?".polkadot_pink().to_string())
            .initial_value(true)
            .interact()?;

        if should_commit {
            let sp = spinner();
            sp.start("Committing changes...");

            let commit_msg = format!("feat(recipe): add {project_slug}");
            let commit_output = std::process::Command::new("git")
                .args(["commit", "-am", &commit_msg])
                .current_dir(&project_path)
                .output()?;

            if !commit_output.status.success() {
                sp.stop("❌ Failed to commit changes");
                let stderr = String::from_utf8_lossy(&commit_output.stderr);
                note("Error", &stderr)?;
                outro_cancel("Commit failed")?;
                std::process::exit(1);
            }

            sp.stop("✅ Changes committed");
        } else {
            outro_cancel("Please commit your changes first and try again")?;
            std::process::exit(0);
        }
    }

    // Push the branch
    let sp = spinner();
    sp.start(format!(
        "Pushing branch {}...",
        current_branch.polkadot_pink()
    ));

    let push_output = std::process::Command::new("git")
        .args(["push", "origin", &current_branch, "--set-upstream"])
        .output()?;

    if !push_output.status.success() {
        sp.stop("❌ Failed to push branch");
        let stderr = String::from_utf8_lossy(&push_output.stderr);
        note("Error", &stderr)?;
        outro_cancel("Push failed")?;
        std::process::exit(1);
    }

    sp.stop(format!(
        "✅ Branch {} pushed",
        current_branch.polkadot_pink()
    ));

    // Create the PR using GitHub API (as draft)
    sp.start("Creating draft pull request...");

    let octocrab = octocrab::Octocrab::builder()
        .personal_token(github_token)
        .build()?;

    let pr_result = octocrab
        .pulls(&owner, &repo)
        .create(&default_title, &current_branch, "master")
        .body(&default_body)
        .draft(true)
        .send()
        .await;

    let pr = match pr_result {
        Ok(pr) => pr,
        Err(e) => {
            sp.stop("❌ Failed to create pull request");
            note("Error", format!("{e}"))?;
            outro_cancel(
                "PR creation failed. Please check:\n\
                 • Your GitHub token has 'repo' permissions\n\
                 • The branch has been pushed to your fork\n\
                 • You don't already have an open PR for this branch",
            )?;
            std::process::exit(1);
        }
    };

    let pr_url = pr
        .html_url
        .map(|u| u.to_string())
        .unwrap_or_else(|| format!("https://github.com/{}/{}/pull/{}", owner, repo, pr.number));
    sp.stop("✅ Draft pull request created!");

    note(
        "Next Steps",
        format!(
            "1. Edit the PR description at:\n   {}\n\n\
             2. When ready, mark the PR as \"Ready for review\"",
            pr_url.polkadot_pink()
        ),
    )?;

    outro(format!(
        "🎉 Draft PR created!\n\n\
         Please review and edit the PR description before marking it ready.\n\
         View your PR at: {}",
        pr_url.polkadot_pink()
    ))?;

    Ok(())
}

/// Handle submission from standalone project (outside cookbook repo)
async fn handle_standalone_submit(
    slug: Option<String>,
    title: Option<String>,
    body: Option<String>,
) -> Result<()> {
    // Get current directory as the project path
    let project_path = std::env::current_dir()?;
    let project_slug = slug.unwrap_or_else(|| {
        project_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("my-project")
            .to_string()
    });

    intro(format!(
        "📤 Submit Project: {}",
        project_slug.polkadot_pink()
    ))?;

    note(
        "Standalone Mode",
        "Detected standalone project. Will fork and clone polkadot-cookbook repo.",
    )?;

    // Run tests before allowing submission
    note(
        "Pre-submission Check",
        "Running tests to ensure project quality",
    )?;
    let tests_passed = run_project_tests(&project_path, false, false, false).await?;

    if !tests_passed {
        outro_cancel(
            "Tests must pass before submission.\n\n\
             Please fix the failing tests and try again:\n\
             dot test",
        )?;
        std::process::exit(1);
    }

    println!(
        "\n{}\n",
        "✅ Tests passed! Proceeding with submission..."
            .polkadot_pink()
            .bold()
    );

    // Validate lock files are present
    note(
        "Lock File Check",
        "Verifying required lock files are present",
    )?;
    let project_metadata =
        match polkadot_cookbook_sdk::config::ProjectMetadata::from_project_directory(&project_path)
            .await
        {
            Ok(metadata) => metadata,
            Err(e) => {
                outro_cancel(format!("Failed to detect project type: {e}"))?;
                std::process::exit(1);
            }
        };

    if let Err(e) = polkadot_cookbook_sdk::config::validate_lock_files(
        &project_path,
        &project_metadata.project_type,
    ) {
        outro_cancel(format!("{e}"))?;
        std::process::exit(1);
    }

    println!(
        "{}\n",
        "✅ All required lock files present".polkadot_pink().bold()
    );

    // Get GitHub token
    let github_token = match get_github_token() {
        Ok(token) => token,
        Err(e) => {
            outro_cancel(format!(
                "GitHub authentication required.\n\n\
                 Error: {e}\n\n\
                 Please set GITHUB_TOKEN environment variable:\n\
                 export GITHUB_TOKEN=ghp_your_token_here\n\n\
                 Or authenticate with GitHub CLI:\n\
                 gh auth login\n\n\
                 Create a token at: https://github.com/settings/tokens/new\n\
                 Required scopes: repo, workflow"
            ))?;
            std::process::exit(1);
        }
    };

    // Read project metadata from frontmatter
    let readme_path = project_path.join("README.md");
    let (project_name, project_desc) =
        match polkadot_cookbook_sdk::metadata::parse_frontmatter_from_file(&readme_path).await {
            Ok(frontmatter) => (frontmatter.title, frontmatter.description),
            Err(_) => (
                project_slug.clone(),
                "A new Polkadot Cookbook recipe".to_string(),
            ),
        };

    // Auto-detect project type
    let project_type =
        match polkadot_cookbook_sdk::metadata::detect_project_type(&project_path).await {
            Ok(t) => match t {
                polkadot_cookbook_sdk::config::ProjectType::PolkadotSdk => "Polkadot SDK",
                polkadot_cookbook_sdk::config::ProjectType::Solidity => "Solidity",
                polkadot_cookbook_sdk::config::ProjectType::Xcm => "XCM",
                polkadot_cookbook_sdk::config::ProjectType::Transactions => "Transactions",
                polkadot_cookbook_sdk::config::ProjectType::Networks => "Network Infrastructure",
            },
            Err(_) => "Unknown",
        };

    // Prompt for pathway to organize the project
    let pathway_question = "Which pathway does this recipe belong to?"
        .polkadot_pink()
        .to_string();
    let pathway = select(pathway_question)
        .item(
            ProjectPathway::Contracts,
            "Smart Contract",
            "Develop Solidity contracts for Polkadot",
        )
        .item(
            ProjectPathway::Transactions,
            "Chain Transactions",
            "Single-chain PAPI interactions with TypeScript",
        )
        .item(
            ProjectPathway::Xcm,
            "Cross-Chain Transactions",
            "Cross-chain messaging with XCM and Chopsticks",
        )
        .item(
            ProjectPathway::Networks,
            "Polkadot Networks",
            "Network infrastructure with Zombienet/Chopsticks",
        )
        .interact()?;

    let pathway_folder = pathway.to_folder_name();

    note(
        "Project Info",
        format!(
            "Name:        {}\nSlug:        {}\nType:        {}\nPathway:     {}",
            project_name.polkadot_pink(),
            project_slug.polkadot_pink(),
            project_type,
            pathway_folder.polkadot_pink(),
        ),
    )?;

    // Generate default PR title and body
    let default_title = title.unwrap_or_else(|| format!("feat(recipe): add {project_slug}"));
    let default_body = body.unwrap_or_else(|| {
        format!(
            "## Summary\n\n\
             This PR adds a new {project_type} recipe: **{project_name}**\n\n\
             {project_desc}\n\n\
             ## Recipe Details\n\n\
             - **Type**: {project_type}\n\
             - **Slug**: `{project_slug}`\n\n\
             ## Testing\n\n\
             - [ ] All tests pass\n\
             - [ ] Code is properly formatted\n\
             - [ ] Documentation is complete\n\n\
             ## Notes\n\n\
             This recipe is ready for review and does not require a prior proposal issue. \
             The Polkadot Cookbook accepts direct recipe contributions via PR."
        )
    });

    note(
        "Pull Request Preview",
        format!(
            "Title:\n{}\n\nDescription:\n{}",
            default_title.polkadot_pink(),
            default_body.dimmed()
        ),
    )?;

    // Confirm submission
    let should_continue = confirm("Continue with submission?".polkadot_pink().to_string())
        .initial_value(true)
        .interact()?;

    if !should_continue {
        outro_cancel("Submission cancelled")?;
        std::process::exit(0);
    }

    let sp = spinner();
    let octocrab = octocrab::Octocrab::builder()
        .personal_token(github_token.clone())
        .build()?;

    // Step 1: Fork polkadot-developers/polkadot-cookbook if not already forked
    sp.start("Forking polkadot-developers/polkadot-cookbook...");

    let fork_result = octocrab
        .repos("polkadot-developers", "polkadot-cookbook")
        .create_fork()
        .send()
        .await;

    let fork = match fork_result {
        Ok(fork) => {
            sp.stop("✅ Repository forked (or already exists)");
            fork
        }
        Err(e) => {
            // Fork might already exist, try to get the user's fork
            let user = match octocrab.current().user().await {
                Ok(u) => u,
                Err(_) => {
                    sp.stop("❌ Failed to fork repository");
                    outro_cancel(format!("Fork error: {e}"))?;
                    std::process::exit(1);
                }
            };

            match octocrab.repos(&user.login, "polkadot-cookbook").get().await {
                Ok(existing_fork) => {
                    sp.stop("✅ Using existing fork");
                    existing_fork
                }
                Err(_) => {
                    sp.stop("❌ Failed to fork repository");
                    outro_cancel(format!("Fork error: {e}"))?;
                    std::process::exit(1);
                }
            }
        }
    };

    let fork_owner = fork.owner.as_ref().unwrap().login.clone();
    let clone_url = fork.clone_url.as_ref().unwrap().to_string();

    // Step 2: Clone the fork to a temp directory
    sp.start("Cloning forked repository...");

    let temp_dir = tempfile::tempdir()?;
    let temp_path = temp_dir.path();

    let clone_output = std::process::Command::new("git")
        .args(["clone", &clone_url, temp_path.to_str().unwrap()])
        .output()?;

    if !clone_output.status.success() {
        sp.stop("❌ Failed to clone repository");
        let stderr = String::from_utf8_lossy(&clone_output.stderr);
        outro_cancel(format!("Clone error: {stderr}"))?;
        std::process::exit(1);
    }

    sp.stop("✅ Repository cloned");

    // Step 3: Copy project to recipes/{pathway}/{slug}/
    sp.start(format!(
        "Copying project to recipes/{}/{}...",
        pathway_folder, project_slug
    ));

    let recipes_dir = temp_path.join("recipes");
    let pathway_dir = recipes_dir.join(pathway_folder);
    let dest_dir = pathway_dir.join(&project_slug);

    // Create pathway directory if it doesn't exist
    tokio::fs::create_dir_all(&pathway_dir).await?;

    // Copy project directory
    copy_dir_recursive(&project_path, &dest_dir).await?;

    sp.stop(format!(
        "✅ Project copied to recipes/{}/{}",
        pathway_folder, project_slug
    ));

    // Step 4: Create branch, commit, and push
    let branch_name = format!("feat/recipe-{}", project_slug);

    sp.start(format!("Creating branch {}...", branch_name));

    // Create and checkout branch
    let checkout_output = std::process::Command::new("git")
        .args(["checkout", "-b", &branch_name])
        .current_dir(temp_path)
        .output()?;

    if !checkout_output.status.success() {
        sp.stop("❌ Failed to create branch");
        let stderr = String::from_utf8_lossy(&checkout_output.stderr);
        outro_cancel(format!("Branch creation error: {stderr}"))?;
        std::process::exit(1);
    }

    sp.stop(format!("✅ Branch {} created", branch_name));

    // Add files
    sp.start("Committing changes...");

    let add_output = std::process::Command::new("git")
        .args(["add", "."])
        .current_dir(temp_path)
        .output()?;

    if !add_output.status.success() {
        sp.stop("❌ Failed to add files");
        std::process::exit(1);
    }

    // Commit
    let commit_msg = format!("feat(recipe): add {}", project_slug);
    let commit_output = std::process::Command::new("git")
        .args(["commit", "-m", &commit_msg])
        .current_dir(temp_path)
        .output()?;

    if !commit_output.status.success() {
        sp.stop("❌ Failed to commit changes");
        let stderr = String::from_utf8_lossy(&commit_output.stderr);
        outro_cancel(format!("Commit error: {stderr}"))?;
        std::process::exit(1);
    }

    sp.stop("✅ Changes committed");

    // Push
    sp.start(format!("Pushing to {}...", fork_owner));

    let push_output = std::process::Command::new("git")
        .args(["push", "-u", "origin", &branch_name])
        .current_dir(temp_path)
        .output()?;

    if !push_output.status.success() {
        sp.stop("❌ Failed to push branch");
        let stderr = String::from_utf8_lossy(&push_output.stderr);
        outro_cancel(format!("Push error: {stderr}"))?;
        std::process::exit(1);
    }

    sp.stop(format!("✅ Pushed to {}/{}", fork_owner, branch_name));

    // Step 5: Create draft PR to polkadot-developers/polkadot-cookbook
    sp.start("Creating draft pull request...");

    let pr_result = octocrab
        .pulls("polkadot-developers", "polkadot-cookbook")
        .create(
            &default_title,
            format!("{}:{}", fork_owner, branch_name),
            "master",
        )
        .body(&default_body)
        .draft(true)
        .send()
        .await;

    let pr = match pr_result {
        Ok(pr) => pr,
        Err(e) => {
            sp.stop("❌ Failed to create pull request");
            note("Error", format!("{e}"))?;
            outro_cancel(
                "PR creation failed. Please check:\n\
                 • Your GitHub token has 'repo' permissions\n\
                 • You don't already have an open PR for this recipe",
            )?;
            std::process::exit(1);
        }
    };

    let pr_url = pr.html_url.map(|u| u.to_string()).unwrap_or_else(|| {
        format!(
            "https://github.com/polkadot-developers/polkadot-cookbook/pull/{}",
            pr.number
        )
    });

    sp.stop("✅ Draft pull request created!");

    note(
        "Next Steps",
        format!(
            "1. Edit the PR description at:\n   {}\n\n\
             2. When ready, mark the PR as \"Ready for review\"",
            pr_url.polkadot_pink()
        ),
    )?;

    outro(format!(
        "🎉 Draft PR created!\n\n\
         Please review and edit the PR description before marking it ready.\n\
         View your PR at: {}",
        pr_url.polkadot_pink()
    ))?;

    // Cleanup temp directory (automatically handled by Drop)
    Ok(())
}

/// Recursively copy a directory
fn copy_dir_recursive<'a>(
    src: &'a std::path::Path,
    dst: &'a std::path::Path,
) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<()>> + 'a>> {
    Box::pin(async move {
        tokio::fs::create_dir_all(dst).await?;

        let mut entries = tokio::fs::read_dir(src).await?;
        while let Some(entry) = entries.next_entry().await? {
            let file_type = entry.file_type().await?;
            let src_path = entry.path();
            let file_name = entry.file_name();
            let file_name_str = file_name.to_str().unwrap_or("");

            // Skip hidden files, .git directory, and node_modules
            if file_name_str.starts_with('.') || file_name_str == "node_modules" {
                continue;
            }

            let dst_path = dst.join(&file_name);

            if file_type.is_dir() {
                copy_dir_recursive(&src_path, &dst_path).await?;
            } else if file_type.is_file() {
                tokio::fs::copy(&src_path, &dst_path).await?;
            } else if file_type.is_symlink() {
                // For symlinks, try to copy the target file if it's a regular file
                if let Ok(metadata) = tokio::fs::metadata(&src_path).await {
                    if metadata.is_file() {
                        tokio::fs::copy(&src_path, &dst_path).await?;
                    }
                    // Skip symlinks to directories or other special files
                }
            }
            // Skip other special file types (sockets, devices, etc.)
        }

        Ok(())
    })
}

/// Get GitHub token from environment variable or gh CLI
fn get_github_token() -> Result<String> {
    // First try environment variable
    if let Ok(token) = std::env::var("GITHUB_TOKEN") {
        if !token.is_empty() {
            return Ok(token);
        }
    }

    // Try gh CLI auth token command
    let gh_output = std::process::Command::new("gh")
        .args(["auth", "token"])
        .output();

    if let Ok(output) = gh_output {
        if output.status.success() {
            let token = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !token.is_empty() {
                return Ok(token);
            }
        }
    }

    Err(anyhow::anyhow!("No GitHub token found"))
}

/// Extract repository owner and name from git remote URL
fn get_repo_info() -> Result<(String, String)> {
    let output = std::process::Command::new("git")
        .args(["remote", "get-url", "origin"])
        .output()?;

    if !output.status.success() {
        return Err(anyhow::anyhow!("Failed to get git remote URL"));
    }

    let remote_url = String::from_utf8_lossy(&output.stdout).trim().to_string();

    // Parse different URL formats:
    // - https://github.com/owner/repo.git
    // - git@github.com:owner/repo.git
    // - https://github.com/owner/repo

    let parts: Vec<&str> = if remote_url.contains("github.com:") {
        // SSH format: git@github.com:owner/repo.git
        remote_url.split("github.com:").collect()
    } else if remote_url.contains("github.com/") {
        // HTTPS format: https://github.com/owner/repo.git
        remote_url.split("github.com/").collect()
    } else {
        return Err(anyhow::anyhow!(
            "Unsupported git remote URL format: {remote_url}"
        ));
    };

    if parts.len() != 2 {
        return Err(anyhow::anyhow!(
            "Could not parse git remote URL: {remote_url}"
        ));
    }

    let repo_path = parts[1].trim_end_matches(".git");
    let repo_parts: Vec<&str> = repo_path.split('/').collect();

    if repo_parts.len() != 2 {
        return Err(anyhow::anyhow!(
            "Invalid repository path in remote URL: {repo_path}"
        ));
    }

    Ok((repo_parts[0].to_string(), repo_parts[1].to_string()))
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
