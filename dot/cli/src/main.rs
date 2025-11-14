//! CLI wrapper for Polkadot Cookbook SDK
//!
//! This is a thin wrapper around the polkadot-cookbook-sdk library that provides
//! a command-line interface for creating and managing Polkadot Cookbook recipes.

use anyhow::Result;
use clap::{Parser, Subcommand};
use cliclack::{clear_screen, confirm, input, intro, note, outro, outro_cancel, select, spinner};
use colored::Colorize;
use polkadot_cookbook_sdk::{
    config::{ProjectConfig, RecipePathway, RecipeType},
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
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Recipe title (for non-interactive mode)
    #[arg(long, global = true)]
    title: Option<String>,

    /// Recipe pathway (for non-interactive mode): parachain, contracts, basic-interaction, xcm, testing, request-new
    #[arg(long, global = true)]
    pathway: Option<String>,

    /// Skip npm install
    #[arg(long, default_value = "false", global = true)]
    skip_install: bool,

    /// Skip git branch creation
    #[arg(long, default_value = "false", global = true)]
    no_git: bool,

    /// Pallet-only mode: no runtime, no PAPI (advanced users)
    #[arg(long, default_value = "false", global = true)]
    pallet_only: bool,

    /// Non-interactive mode (use defaults, require title argument)
    #[arg(long, default_value = "false", global = true)]
    non_interactive: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new project (interactive)
    Create,
    /// Run project tests
    Test {
        /// Project slug (defaults to current directory)
        #[arg(value_name = "SLUG")]
        slug: Option<String>,
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

    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "polkadot_cookbook_sdk=info".to_string()),
        )
        .init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Create => {
            handle_create(
                cli.title,
                cli.pathway,
                cli.skip_install,
                cli.no_git,
                cli.pallet_only,
                cli.non_interactive,
            )
            .await?;
        }
        Commands::Test { slug } => {
            handle_recipe_test(slug).await?;
        }
        Commands::Submit { slug, title, body } => {
            handle_recipe_submit(slug, title, body).await?;
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
    let intro_text = format!("{}", "Polkadot Cookbook".polkadot_pink().bold());
    intro(&intro_text)?;

    let note_title = "Recipe Setup".polkadot_pink().to_string();
    note(
        &note_title,
        "Let's create your new Polkadot recipe. This will scaffold the project structure,\ngenerate template files, and set up the testing environment.",
    )?;

    // Step 1: Ask for pathway first (so users know what they can build)
    let pathway_question = "What would you like to build?".polkadot_pink().to_string();
    let pathway: RecipePathway = select(&pathway_question)
        .item(
            RecipePathway::Parachain,
            "Custom Parachain (Polkadot SDK)",
            "Build a custom parachain with PAPI integration",
        )
        .item(
            RecipePathway::Contracts,
            "Smart Contract (Solidity)",
            "Build, test, and run Solidity smart contracts",
        )
        .item(
            RecipePathway::BasicInteraction,
            "Basic Interaction",
            "Single-chain transactions and state queries with PAPI",
        )
        .item(
            RecipePathway::Xcm,
            "Cross-chain Interaction (XCM)",
            "Cross-chain asset transfers and cross-chain calls with Chopsticks",
        )
        .item(
            RecipePathway::Testing,
            "Polkadot Network (Zombienet / Chopsticks)",
            "Run a Polkadot network locally",
        )
        .item(
            RecipePathway::RequestNew,
            "None of these - Request new template",
            "Don't see what you need? Request a new recipe template",
        )
        .interact()?;

    // Handle "Request New Template" selection
    if pathway == RecipePathway::RequestNew {
        outro_cancel(format!(
            "🎯 Request a New Recipe Template\n\n\
            We'd love to support your use case! Please create a GitHub issue:\n\n\
            {} {}\n\n\
            Include in your issue:\n\
            {} What kind of recipe you want to create\n\
            {} What technology/framework it involves\n\
            {} Example use cases\n\
            {} Any specific requirements\n\n\
            We'll review your request and add the template if it fits the cookbook!",
            "→".polkadot_pink(),
            "https://github.com/paritytech/polkadot-cookbook/issues/new"
                .polkadot_pink()
                .bold(),
            "•".polkadot_pink(),
            "•".polkadot_pink(),
            "•".polkadot_pink(),
            "•".polkadot_pink(),
        ))?;
        std::process::exit(0);
    }

    // Map pathway to recipe type (for template selection)
    let recipe_type = match pathway {
        RecipePathway::Parachain => RecipeType::PolkadotSdk,
        RecipePathway::Contracts => RecipeType::Solidity,
        RecipePathway::BasicInteraction => RecipeType::BasicInteraction,
        RecipePathway::Xcm => RecipeType::Xcm,
        RecipePathway::Testing => RecipeType::Testing,
        RecipePathway::RequestNew => {
            // This should never be reached since we exit above
            unreachable!("RequestNew pathway should have been handled before reaching here")
        }
    };

    // Interactive mode always creates full parachain
    // (pallet-only mode is only available via --pallet-only flag)

    // Check dependencies for the selected pathway
    check_dependencies_interactive(&pathway)?;

    // Step 2: Ask for title (now that user knows the pathway)
    let title_question = "What is your recipe title?".polkadot_pink().to_string();
    let hint_text = "(e.g., 'Custom NFT Pallet', 'Cross-Chain Asset Transfer')"
        .dimmed()
        .to_string();
    let title: String = input(format!("{title_question} {hint_text}"))
        .placeholder("My Recipe")
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

    // Step 3: Prompt for description
    let description_question = "Brief description".polkadot_pink().to_string();
    let hint_text = "(1-2 sentences, 120-160 characters for SEO)"
        .dimmed()
        .to_string();
    let description: String = input(format!("{description_question} {hint_text}"))
        .placeholder("Learn how to build a custom NFT pallet with minting, transfers, and storage")
        .default_input("")
        .interact()?;

    let description = if description.trim().is_empty() {
        "Replace with a short description.".to_string()
    } else {
        description.trim().to_string()
    };

    // Git branch creation is default (unless --no-git flag is used)
    let create_git_branch = !no_git;

    // Npm install is default (unless --skip-install flag is used)
    // Skip install flag is already set from CLI args

    // Calculate derived values for the summary
    let project_path = PathBuf::from(".").join(&slug);
    let branch_name = if create_git_branch {
        format!("feat/{slug}")
    } else {
        "(none)".to_string()
    };

    // Generate directory tree based on pathway
    let tree_structure = match pathway {
        RecipePathway::Parachain => {
            if pallet_only {
                format!(
                    "{}/\n\
                     ├── README.md               (Pallet development guide)\n\
                     ├── Cargo.toml              (Workspace config)\n\
                     ├── rust-toolchain.toml     (Rust version)\n\
                     └── pallets/\n\
                         └── template/\n\
                             ├── Cargo.toml      (Pallet dependencies)\n\
                             └── src/\n\
                                 ├── lib.rs      (Main pallet logic)\n\
                                 ├── mock.rs     (Test runtime)\n\
                                 ├── tests.rs    (Unit tests)\n\
                                 ├── benchmarking.rs\n\
                                 └── weights.rs",
                    slug
                )
            } else {
                format!(
                    "{}/\n\
                     ├── README.md               (Full tutorial)\n\
                     ├── Cargo.toml              (Workspace config)\n\
                     ├── package.json            (PAPI dependencies)\n\
                     ├── runtime/                (Parachain runtime)\n\
                     │   ├── Cargo.toml\n\
                     │   └── src/\n\
                     ├── node/                   (Node implementation)\n\
                     │   ├── Cargo.toml\n\
                     │   └── src/\n\
                     ├── pallets/                (Custom pallets)\n\
                     │   └── template/\n\
                     │       ├── Cargo.toml\n\
                     │       └── src/\n\
                     ├── scripts/                (Helper scripts)\n\
                     │   ├── generate-spec.sh\n\
                     │   └── start-dev-node.sh\n\
                     └── tests/                  (PAPI integration tests)\n\
                         └── template-pallet.test.ts",
                    slug
                )
            }
        }
        RecipePathway::Contracts => {
            format!(
                "{}/\n\
                 ├── README.md\n\
                 ├── package.json\n\
                 ├── hardhat.config.ts\n\
                 ├── contracts/\n\
                 │   └── Contract.sol\n\
                 ├── scripts/\n\
                 │   └── deploy.ts\n\
                 ├── tests/\n\
                 │   └── Contract.test.ts\n\
                 └── src/",
                slug
            )
        }
        RecipePathway::BasicInteraction => {
            format!(
                "{}/\n\
                 ├── README.md\n\
                 ├── package.json\n\
                 ├── tsconfig.json\n\
                 ├── vitest.config.ts\n\
                 ├── src/\n\
                 │   └── example.ts\n\
                 └── tests/\n\
                     └── example.test.ts",
                slug
            )
        }
        RecipePathway::Xcm => {
            format!(
                "{}/\n\
                 ├── README.md\n\
                 ├── package.json\n\
                 ├── chopsticks.yml\n\
                 ├── tsconfig.json\n\
                 ├── vitest.config.ts\n\
                 ├── src/\n\
                 │   └── xcm-helpers.ts\n\
                 └── tests/\n\
                     └── xcm.test.ts",
                slug
            )
        }
        RecipePathway::Testing => {
            format!(
                "{}/\n\
                 ├── README.md\n\
                 ├── package.json\n\
                 ├── configs/\n\
                 │   ├── chopsticks.yml\n\
                 │   └── network.toml\n\
                 └── tests/\n\
                     └── network.test.ts",
                slug
            )
        }
        RecipePathway::RequestNew => {
            unreachable!("RequestNew should have been handled before summary")
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
             Directory structure:\n\n{}",
            "Title:".polkadot_pink(),
            title.polkadot_pink().bold(),
            "Slug:".polkadot_pink(),
            slug.dimmed(),
            "Pathway:".polkadot_pink(),
            match pathway {
                RecipePathway::Parachain => {
                    if pallet_only {
                        "Custom Parachain (Pallet-only)"
                    } else {
                        "Custom Parachain"
                    }
                }
                RecipePathway::Contracts => "Smart Contract",
                RecipePathway::BasicInteraction => "Basic Interaction",
                RecipePathway::Xcm => "Cross-chain Interaction",
                RecipePathway::Testing => "Polkadot Network",
                RecipePathway::RequestNew => {
                    unreachable!("RequestNew should have been handled before summary")
                }
            },
            "Location:".polkadot_pink(),
            project_path.display(),
            "Git Branch:".polkadot_pink(),
            branch_name,
            tree_structure.dimmed()
        ),
    )?;

    let confirm_question = "Continue?".polkadot_pink().to_string();
    let should_continue = confirm(&confirm_question).initial_value(true).interact()?;

    if !should_continue {
        outro_cancel("Recipe creation cancelled")?;
        std::process::exit(0);
    }

    // Create project configuration
    let mut config = ProjectConfig::new(&slug)
        .with_title(&title)
        .with_destination(PathBuf::from("."))
        .with_git_init(create_git_branch)
        .with_skip_install(skip_install)
        .with_recipe_type(recipe_type)
        .with_description(description)
        .with_pathway(pathway);

    // Set parachain-specific flags
    config.pallet_only = pallet_only;

    // Create the project with spinner
    let sp = spinner();
    let spinner_msg = if skip_install {
        "Creating recipe project...".polkadot_pink().to_string()
    } else {
        "Creating recipe project (this may take ~30 seconds for npm install)..."
            .polkadot_pink()
            .to_string()
    };
    sp.start(&spinner_msg);

    let scaffold = Scaffold::new();

    // Create progress callback to update spinner
    use polkadot_cookbook_sdk::scaffold::ProgressCallback;
    let progress_callback: ProgressCallback = Box::new(move |msg: &str| {
        // Note: cliclack spinners don't support live message updates,
        // but we use debug logging instead of info to keep output clean
        tracing::debug!("Progress: {}", msg);
    });

    match scaffold
        .create_project(config, Some(&progress_callback))
        .await
    {
        Ok(project_info) => {
            sp.stop(format!(
                "{}",
                "✅ Recipe created successfully!".polkadot_pink()
            ));

            let project_title = "📦 Project Created".polkadot_pink().to_string();
            note(
                &project_title,
                format!(
                    "Slug:       {}\nTitle:      {}\nLocation:   {}\nGit Branch: {}",
                    project_info.slug.polkadot_pink(),
                    project_info.title.polkadot_pink(),
                    project_info
                        .project_path
                        .display()
                        .to_string()
                        .polkadot_pink(),
                    project_info.git_branch.as_deref().unwrap_or("(none)")
                ),
            )?;

            let steps_title = "📝 Next Steps".polkadot_pink().to_string();

            // Generate context-aware next steps based on recipe type and mode
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
            } else if matches!(pathway, RecipePathway::Parachain) {
                format!(
                    "{} Customize your pallet\n   {} {}\n\n\
                     {} Configure runtime\n   {} {}\n\n\
                     {} Write PAPI tests\n   {} {}\n\n\
                     {} Build and test\n   {} {}\n   {} {}",
                    "1.".polkadot_pink().bold(),
                    "→".dimmed(),
                    format!(
                        "{}/pallets/template/src/lib.rs",
                        project_info.project_path.display()
                    )
                    .polkadot_pink(),
                    "2.".polkadot_pink().bold(),
                    "→".dimmed(),
                    format!("{}/runtime/src/lib.rs", project_info.project_path.display())
                        .polkadot_pink(),
                    "3.".polkadot_pink().bold(),
                    "→".dimmed(),
                    format!("{}/tests/", project_info.project_path.display()).polkadot_pink(),
                    "4.".polkadot_pink().bold(),
                    "→".dimmed(),
                    format!("cd {} && cargo build", project_info.project_path.display())
                        .polkadot_pink(),
                    "→".dimmed(),
                    "npm test".polkadot_pink()
                )
            } else {
                // Default for other pathways
                format!(
                    "{} Add implementation\n   {} {}\n\n\
                     {} Write tests\n   {} {}\n\n\
                     {} Run tests\n   {} {}",
                    "1.".polkadot_pink().bold(),
                    "→".dimmed(),
                    format!("{}/src/", project_info.project_path.display()).polkadot_pink(),
                    "2.".polkadot_pink().bold(),
                    "→".dimmed(),
                    format!("{}/tests/", project_info.project_path.display()).polkadot_pink(),
                    "3.".polkadot_pink().bold(),
                    "→".dimmed(),
                    format!("cd {} && npm test", project_info.project_path.display())
                        .polkadot_pink()
                )
            };

            note(&steps_title, next_steps)?;

            if let Some(_branch) = project_info.git_branch {
                let git_title = "🔀 Ready to Submit?".polkadot_pink().to_string();
                note(
                    &git_title,
                    format!(
                        "{} Use the submit command to create a PR:\n   {}\n\n\
                         {} Or manually:\n\
                         {} {}\n\
                         {} {}\n\
                         {} {}",
                        "1.".polkadot_pink().bold(),
                        format!("./target/release/dot recipe submit {}", project_info.slug)
                            .polkadot_pink(),
                        "2.".polkadot_pink().bold(),
                        "   →".dimmed(),
                        "git add -A".polkadot_pink(),
                        "   →".dimmed(),
                        format!("git commit -m \"feat(recipe): add {}\"", project_info.slug)
                            .polkadot_pink(),
                        "   →".dimmed(),
                        "git push && gh pr create".polkadot_pink(),
                    ),
                )?;
            }

            let outro_msg = "🎉 All set! Happy coding! Check CONTRIBUTING.md for guidelines."
                .polkadot_pink()
                .to_string();
            outro(&outro_msg)?;
        }
        Err(e) => {
            sp.stop(format!("❌ Failed to create recipe: {e}"));
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
        eprintln!("❌ Invalid recipe title: {e}");
        eprintln!("Title must be properly formatted.");
        std::process::exit(1);
    }

    // Generate slug from title
    let slug = polkadot_cookbook_sdk::config::title_to_slug(title);

    // Parse pathway to recipe type
    let recipe_type = if let Some(p) = pathway {
        match p.as_str() {
            "parachain" => RecipeType::PolkadotSdk,
            "contracts" => RecipeType::Solidity,
            "basic-interaction" => RecipeType::BasicInteraction,
            "xcm" => RecipeType::Xcm,
            "testing" => RecipeType::Testing,
            "request-new" => {
                eprintln!("🎯 Request a New Recipe Template\n");
                eprintln!("We'd love to support your use case! Please create a GitHub issue:\n");
                eprintln!("→ https://github.com/paritytech/polkadot-cookbook/issues/new\n");
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
                    "Valid pathways: parachain, contracts, basic-interaction, xcm, testing, request-new"
                );
                std::process::exit(1);
            }
        }
    } else {
        RecipeType::PolkadotSdk // Default
    };

    // Title is already provided as input parameter

    // Determine pathway from recipe type
    let pathway_value = match recipe_type {
        RecipeType::PolkadotSdk => Some(RecipePathway::Parachain),
        RecipeType::Solidity => Some(RecipePathway::Contracts),
        RecipeType::BasicInteraction => Some(RecipePathway::BasicInteraction),
        RecipeType::Xcm => Some(RecipePathway::Xcm),
        RecipeType::Testing => Some(RecipePathway::Testing),
    };

    println!(
        "{} {} ({})",
        "Creating recipe:".polkadot_pink(),
        title.polkadot_pink().bold(),
        slug.dimmed()
    );

    // Create project configuration with provided or default values
    let mut config = ProjectConfig::new(&slug)
        .with_title(title)
        .with_destination(PathBuf::from("."))
        .with_git_init(!no_git)
        .with_skip_install(skip_install)
        .with_recipe_type(recipe_type)
        .with_description("Replace with a short description.".to_string());

    // Add optional fields if provided
    if let Some(p) = pathway_value {
        config = config.with_pathway(p);
    }

    // Set parachain-specific flags (non-interactive mode)
    config.pallet_only = pallet_only;

    // Create the project
    let scaffold = Scaffold::new();
    match scaffold.create_project(config, None).await {
        Ok(project_info) => {
            println!(
                "{}",
                "✅ Recipe created successfully!".polkadot_pink().bold()
            );
            println!(
                "{} {}",
                "Path:".polkadot_pink(),
                project_info.project_path.display()
            );
            if let Some(ref branch) = project_info.git_branch {
                println!("{} {}", "Git Branch:".polkadot_pink(), branch);
            }
        }
        Err(e) => {
            eprintln!("❌ Failed to create recipe: {e}");
            std::process::exit(1);
        }
    }

    Ok(())
}

async fn handle_recipe_test(slug: Option<String>) -> Result<()> {
    let recipe_path = get_recipe_path(slug)?;

    intro(format!(
        "🧪 Testing Recipe: {}",
        recipe_path
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .polkadot_pink()
    ))?;

    // Auto-detect recipe type from files
    let recipe_config = match polkadot_cookbook_sdk::config::RecipeConfig::from_recipe_directory(
        &recipe_path,
    )
    .await
    {
        Ok(config) => config,
        Err(e) => {
            outro_cancel(format!("Failed to detect recipe type: {e}"))?;
            std::process::exit(1);
        }
    };

    let is_polkadot_sdk = matches!(
        recipe_config.recipe_type,
        polkadot_cookbook_sdk::config::RecipeType::PolkadotSdk
    );

    if is_polkadot_sdk {
        note("Recipe Type", "Polkadot SDK (Rust)")?;

        println!("\n{}\n", "Running cargo test...".polkadot_pink().bold());

        let status = std::process::Command::new("cargo")
            .args(["test", "--all-features"])
            .current_dir(&recipe_path)
            .status()?;

        println!(); // Add spacing after test output

        if status.success() {
            println!("{}", "✅ All tests passed!".polkadot_pink().bold());
        } else {
            eprintln!("{}", "❌ Tests failed".red().bold());
            outro_cancel("Tests failed")?;
            std::process::exit(1);
        }
    } else {
        note("Recipe Type", "TypeScript")?;

        println!("\n{}\n", "Running npm test...".polkadot_pink().bold());

        let status = std::process::Command::new("npm")
            .args(["test"])
            .current_dir(&recipe_path)
            .status()?;

        println!(); // Add spacing after test output

        if status.success() {
            println!("{}", "✅ All tests passed!".polkadot_pink().bold());
        } else {
            eprintln!("{}", "❌ Tests failed".red().bold());
            outro_cancel("Tests failed")?;
            std::process::exit(1);
        }
    }

    outro("✅ Testing complete!")?;
    Ok(())
}

async fn handle_recipe_submit(
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
    let recipe_path = get_recipe_path(slug.clone())?;
    let recipe_slug = recipe_path
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    intro(format!("📤 Submit Recipe: {}", recipe_slug.polkadot_pink()))?;

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
    let readme_path = recipe_path.join("README.md");
    let (recipe_name, recipe_desc) =
        match polkadot_cookbook_sdk::metadata::parse_frontmatter_from_file(&readme_path).await {
            Ok(frontmatter) => (frontmatter.title, frontmatter.description),
            Err(_) => (
                recipe_slug.clone(),
                "A new Polkadot Cookbook recipe".to_string(),
            ),
        };

    // Auto-detect recipe type
    let recipe_type = match polkadot_cookbook_sdk::metadata::detect_recipe_type(&recipe_path).await
    {
        Ok(t) => match t {
            polkadot_cookbook_sdk::config::RecipeType::PolkadotSdk => "Polkadot SDK",
            polkadot_cookbook_sdk::config::RecipeType::Solidity => "Solidity",
            polkadot_cookbook_sdk::config::RecipeType::Xcm => "XCM",
            polkadot_cookbook_sdk::config::RecipeType::BasicInteraction => "Basic Interactions",
            polkadot_cookbook_sdk::config::RecipeType::Testing => "Testing Infrastructure",
        },
        Err(_) => "Unknown",
    };

    // Check git status
    let git_status = std::process::Command::new("git")
        .args(["status", "--porcelain"])
        .current_dir(&recipe_path)
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
        "Recipe Info",
        format!(
            "Name:        {}\nSlug:        {}\nType:        {}\nBranch:      {}\nChanges:     {}",
            recipe_name.polkadot_pink(),
            recipe_slug.polkadot_pink(),
            recipe_type,
            current_branch.polkadot_pink(),
            if has_changes {
                "Yes (uncommitted)".yellow().to_string()
            } else {
                "None".dimmed().to_string()
            }
        ),
    )?;

    // Generate default PR title and body
    let default_title = title.unwrap_or_else(|| format!("feat(recipe): add {recipe_slug}"));
    let default_body = body.unwrap_or_else(|| {
        format!(
            "## Summary\n\n\
             This PR adds a new {recipe_type} recipe: **{recipe_name}**\n\n\
             {recipe_desc}\n\n\
             ## Recipe Details\n\n\
             - **Type**: {recipe_type}\n\
             - **Slug**: `{recipe_slug}`\n\n\
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

            let commit_msg = format!("feat(recipe): add {recipe_slug}");
            let commit_output = std::process::Command::new("git")
                .args(["commit", "-am", &commit_msg])
                .current_dir(&recipe_path)
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

    // Create the PR using GitHub API
    sp.start("Creating pull request...");

    let octocrab = octocrab::Octocrab::builder()
        .personal_token(github_token)
        .build()?;

    let pr_result = octocrab
        .pulls(&owner, &repo)
        .create(&default_title, &current_branch, "master")
        .body(&default_body)
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
    sp.stop("✅ Pull request created!");

    note("Success", format!("PR URL: {}", pr_url.polkadot_pink()))?;

    outro(format!(
        "🎉 Recipe submitted successfully!\n\n\
         Your recipe will be reviewed by maintainers.\n\
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

    // Auto-detect recipe type
    let recipe_type = match polkadot_cookbook_sdk::metadata::detect_recipe_type(&project_path).await
    {
        Ok(t) => match t {
            polkadot_cookbook_sdk::config::RecipeType::PolkadotSdk => "Polkadot SDK",
            polkadot_cookbook_sdk::config::RecipeType::Solidity => "Solidity",
            polkadot_cookbook_sdk::config::RecipeType::Xcm => "XCM",
            polkadot_cookbook_sdk::config::RecipeType::BasicInteraction => "Basic Interactions",
            polkadot_cookbook_sdk::config::RecipeType::Testing => "Testing Infrastructure",
        },
        Err(_) => "Unknown",
    };

    note(
        "Project Info",
        format!(
            "Name:        {}\nSlug:        {}\nType:        {}",
            project_name.polkadot_pink(),
            project_slug.polkadot_pink(),
            recipe_type,
        ),
    )?;

    // Generate default PR title and body
    let default_title = title.unwrap_or_else(|| format!("feat(recipe): add {project_slug}"));
    let default_body = body.unwrap_or_else(|| {
        format!(
            "## Summary\n\n\
             This PR adds a new {recipe_type} recipe: **{project_name}**\n\n\
             {project_desc}\n\n\
             ## Recipe Details\n\n\
             - **Type**: {recipe_type}\n\
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

    // Step 1: Fork paritytech/polkadot-cookbook if not already forked
    sp.start("Forking paritytech/polkadot-cookbook...");

    let fork_result = octocrab
        .repos("paritytech", "polkadot-cookbook")
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

    // Step 3: Copy project to recipes/{slug}/
    sp.start(format!("Copying project to recipes/{}...", project_slug));

    let recipes_dir = temp_path.join("recipes");
    let dest_dir = recipes_dir.join(&project_slug);

    // Create recipes directory if it doesn't exist
    tokio::fs::create_dir_all(&recipes_dir).await?;

    // Copy project directory
    copy_dir_recursive(&project_path, &dest_dir).await?;

    sp.stop(format!("✅ Project copied to recipes/{}", project_slug));

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

    // Step 5: Create PR to paritytech/polkadot-cookbook
    sp.start("Creating pull request...");

    let pr_result = octocrab
        .pulls("paritytech", "polkadot-cookbook")
        .create(
            &default_title,
            format!("{}:{}", fork_owner, branch_name),
            "master",
        )
        .body(&default_body)
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
            "https://github.com/paritytech/polkadot-cookbook/pull/{}",
            pr.number
        )
    });

    sp.stop("✅ Pull request created!");

    note("Success", format!("PR URL: {}", pr_url.polkadot_pink()))?;

    outro(format!(
        "🎉 Recipe submitted successfully!\n\n\
         Your recipe will be reviewed by maintainers.\n\
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

            // Skip hidden files and .git directory
            if file_name
                .to_str()
                .map(|s| s.starts_with('.'))
                .unwrap_or(false)
            {
                continue;
            }

            let dst_path = dst.join(&file_name);

            if file_type.is_dir() {
                copy_dir_recursive(&src_path, &dst_path).await?;
            } else {
                tokio::fs::copy(&src_path, &dst_path).await?;
            }
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
fn check_dependencies_interactive(pathway: &RecipePathway) -> Result<()> {
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

fn get_recipe_path(slug: Option<String>) -> Result<PathBuf> {
    // Find repository root by looking for .git directory
    let mut current = std::env::current_dir()?;
    let mut repo_root = None;

    loop {
        if current.join(".git").exists() {
            repo_root = Some(current.clone());
            break;
        }
        if let Some(parent) = current.parent() {
            current = parent.to_path_buf();
        } else {
            break;
        }
    }

    let repo_root = match repo_root {
        Some(root) => root,
        None => {
            eprintln!("Not in a git repository");
            std::process::exit(1);
        }
    };

    if let Some(slug) = slug {
        let path = repo_root.join("recipes").join(&slug);
        if !path.exists() {
            eprintln!("Recipe not found: {slug}");
            std::process::exit(1);
        }
        Ok(path)
    } else {
        // Try to infer from current directory
        let current = std::env::current_dir()?;
        if current.parent().and_then(|p| p.file_name()) == Some(std::ffi::OsStr::new("recipes")) {
            Ok(current)
        } else {
            eprintln!("Please provide a recipe slug or run from within a recipe directory");
            std::process::exit(1);
        }
    }
}
