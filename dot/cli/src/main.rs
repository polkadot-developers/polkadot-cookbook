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
#[command(about = "Polkadot Cookbook CLI - Create and manage recipes", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Recipe title (for non-interactive mode)
    #[arg(long, global = true)]
    title: Option<String>,

    /// Recipe pathway (for non-interactive mode): runtime, contracts, basic-interaction, xcm, testing, request-new
    #[arg(long, global = true)]
    pathway: Option<String>,

    /// Skip npm install
    #[arg(long, default_value = "false", global = true)]
    skip_install: bool,

    /// Skip git branch creation
    #[arg(long, default_value = "false", global = true)]
    no_git: bool,

    /// Non-interactive mode (use defaults, require title argument)
    #[arg(long, default_value = "false", global = true)]
    non_interactive: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Create and manage recipes
    Recipe {
        #[command(subcommand)]
        command: RecipeCommands,
    },
}

#[derive(Subcommand)]
enum RecipeCommands {
    /// Create a new recipe (interactive)
    Create,
    /// Run recipe tests
    Test {
        /// Recipe slug (defaults to current directory)
        #[arg(value_name = "SLUG")]
        slug: Option<String>,
    },
    /// Submit a recipe as a pull request
    Submit {
        /// Recipe slug (defaults to current directory)
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
        Commands::Recipe { command } => match command {
            RecipeCommands::Create => {
                handle_create(
                    cli.title,
                    cli.pathway,
                    cli.skip_install,
                    cli.no_git,
                    cli.non_interactive,
                )
                .await?;
            }
            RecipeCommands::Test { slug } => {
                handle_recipe_test(slug).await?;
            }
            RecipeCommands::Submit { slug, title, body } => {
                handle_recipe_submit(slug, title, body).await?;
            }
        },
    }

    Ok(())
}

async fn handle_create(
    title: Option<String>,
    pathway: Option<String>,
    skip_install: bool,
    no_git: bool,
    non_interactive: bool,
) -> Result<()> {
    // Non-interactive mode: require title argument
    if non_interactive {
        let title = title.ok_or_else(|| {
            anyhow::anyhow!("Title argument (--title) is required in non-interactive mode")
        })?;
        return run_non_interactive(&title, pathway, skip_install, no_git).await;
    }

    // Interactive mode with cliclack
    clear_screen()?;

    // Validate working directory first
    if let Err(e) = polkadot_cookbook_sdk::config::validate_working_directory() {
        outro_cancel(format!(
            "❌ Invalid working directory: {e}\n\nPlease run this command from the repository root."
        ))?;
        std::process::exit(1);
    }

    // Add spacing before intro
    println!("\n");

    // Polkadot-themed intro
    let intro_text = format!("{}", "Polkadot Cookbook".polkadot_pink().bold());
    intro(&intro_text)?;

    let note_title = "Recipe Setup".polkadot_pink().to_string();
    note(
        &note_title,
        "Let's create your new recipe. This will scaffold the project structure,\ngenerate template files, and set up the testing environment.",
    )?;

    // Step 1: Ask for pathway first (so users know what they can build)
    let pathway_question = "What kind of recipe are you building?"
        .polkadot_pink()
        .to_string();
    let pathway: RecipePathway = select(&pathway_question)
        .item(
            RecipePathway::Runtime,
            "Runtime Development (Polkadot SDK)",
            "Build custom pallets and runtime logic with FRAME",
        )
        .item(
            RecipePathway::Contracts,
            "Smart Contracts (Solidity)",
            "Deploy Solidity contracts",
        )
        .item(
            RecipePathway::BasicInteraction,
            "Basic Interactions",
            "Single-chain transactions and state queries with PAPI",
        )
        .item(
            RecipePathway::Xcm,
            "XCM (Cross-Chain Messaging)",
            "Asset transfers and cross-chain calls with Chopsticks",
        )
        .item(
            RecipePathway::Testing,
            "Testing Infrastructure",
            "Zombienet and Chopsticks network configurations",
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
        RecipePathway::Runtime => RecipeType::PolkadotSdk,
        RecipePathway::Contracts => RecipeType::Solidity,
        RecipePathway::BasicInteraction => RecipeType::BasicInteraction,
        RecipePathway::Xcm => RecipeType::Xcm,
        RecipePathway::Testing => RecipeType::Testing,
        RecipePathway::RequestNew => {
            // This should never be reached since we exit above
            unreachable!("RequestNew pathway should have been handled before reaching here")
        }
    };

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

    // Generate suggested slug from title
    let suggested_slug = polkadot_cookbook_sdk::config::title_to_slug(&title);

    // Prompt for slug with suggestion pre-filled
    let slug_question = "Recipe slug".polkadot_pink().to_string();
    let slug_hint = "(lowercase, dashes only)".dimmed().to_string();
    let slug: String = input(format!("{slug_question} {slug_hint}"))
        .default_input(&suggested_slug)
        .validate(|input: &String| {
            if input.is_empty() {
                Err("Slug cannot be empty")
            } else if let Err(e) = polkadot_cookbook_sdk::config::validate_slug(input) {
                Err(Box::leak(e.to_string().into_boxed_str()) as &str)
            } else {
                Ok(())
            }
        })
        .interact()?;

    let title = title.trim().to_string();

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

    // Prompt for git branch creation (only if not specified via flag)
    let create_git_branch = if no_git {
        false
    } else {
        let git_question = "Create a git branch for this recipe?"
            .polkadot_pink()
            .to_string();
        confirm(&git_question).initial_value(true).interact()?
    };

    // Prompt for npm install (only if not specified via flag)
    let skip_install = if skip_install {
        true
    } else {
        let install_question = "Install npm dependencies (vitest, @polkadot/api, etc.)?"
            .polkadot_pink()
            .to_string();
        !confirm(&install_question).initial_value(true).interact()?
    };

    // Calculate derived values for the summary
    let project_path = PathBuf::from("recipes").join(&slug);
    let branch_name = if create_git_branch {
        format!("feat/{slug}")
    } else {
        "(none)".to_string()
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
             Files to create:\n\
             {} README.md (with frontmatter)\n\
             {} Template files",
            "Title:".polkadot_pink(),
            title.polkadot_pink().bold(),
            "Slug:".polkadot_pink(),
            slug.dimmed(),
            "Pathway:".polkadot_pink(),
            match pathway {
                RecipePathway::Runtime => "Runtime Development",
                RecipePathway::Contracts => "Smart Contracts",
                RecipePathway::BasicInteraction => "Basic Interactions",
                RecipePathway::Xcm => "XCM",
                RecipePathway::Testing => "Testing Infrastructure",
                RecipePathway::RequestNew => {
                    unreachable!("RequestNew should have been handled before summary")
                }
            },
            "Location:".polkadot_pink(),
            project_path.display(),
            "Git Branch:".polkadot_pink(),
            branch_name,
            "•".polkadot_pink(),
            "•".polkadot_pink()
        ),
    )?;

    let confirm_question = "Continue?".polkadot_pink().to_string();
    let should_continue = confirm(&confirm_question).initial_value(true).interact()?;

    if !should_continue {
        outro_cancel("Recipe creation cancelled")?;
        std::process::exit(0);
    }

    // Create project configuration
    let config = ProjectConfig::new(&slug)
        .with_title(&title)
        .with_destination(PathBuf::from("recipes"))
        .with_git_init(create_git_branch)
        .with_skip_install(skip_install)
        .with_recipe_type(recipe_type)
        .with_description(description)
        .with_pathway(pathway);

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
    match scaffold.create_project(config).await {
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
            note(
                &steps_title,
                format!(
                    "{} Write recipe content\n   {} {}\n\n\
                     {} Add implementation\n   {} {}\n\n\
                     {} Write tests\n   {} {}\n\n\
                     {} Run tests\n   {} {}",
                    "1.".polkadot_pink().bold(),
                    "→".dimmed(),
                    format!("{}/README.md", project_info.project_path.display()).polkadot_pink(),
                    "2.".polkadot_pink().bold(),
                    "→".dimmed(),
                    format!("{}/src/", project_info.project_path.display()).polkadot_pink(),
                    "3.".polkadot_pink().bold(),
                    "→".dimmed(),
                    format!("{}/tests/", project_info.project_path.display()).polkadot_pink(),
                    "4.".polkadot_pink().bold(),
                    "→".dimmed(),
                    format!("cd {} && npm test", project_info.project_path.display())
                        .polkadot_pink()
                ),
            )?;

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
) -> Result<()> {
    // Validate title
    if let Err(e) = polkadot_cookbook_sdk::config::validate_title(title) {
        eprintln!("❌ Invalid recipe title: {e}");
        eprintln!("Title must be properly formatted.");
        std::process::exit(1);
    }

    // Generate slug from title
    let slug = polkadot_cookbook_sdk::config::title_to_slug(title);

    // Validate working directory
    if let Err(e) = polkadot_cookbook_sdk::config::validate_working_directory() {
        eprintln!("❌ Invalid working directory: {e}");
        eprintln!("Please run this command from the repository root.");
        std::process::exit(1);
    }

    // Parse pathway to recipe type
    let recipe_type = if let Some(p) = pathway {
        match p.as_str() {
            "runtime" => RecipeType::PolkadotSdk,
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
                    "Valid pathways: runtime, contracts, basic-interaction, xcm, testing, request-new"
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
        RecipeType::PolkadotSdk => Some(RecipePathway::Runtime),
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
        .with_destination(PathBuf::from("recipes"))
        .with_git_init(!no_git)
        .with_skip_install(skip_install)
        .with_recipe_type(recipe_type)
        .with_description("Replace with a short description.".to_string());

    // Add optional fields if provided
    if let Some(p) = pathway_value {
        config = config.with_pathway(p);
    }

    // Create the project
    let scaffold = Scaffold::new();
    match scaffold.create_project(config).await {
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

        let sp = spinner();
        sp.start("Running cargo test...");

        let output = std::process::Command::new("cargo")
            .args(["test", "--all-features"])
            .current_dir(&recipe_path)
            .output()?;

        if output.status.success() {
            sp.stop("✅ All tests passed!");
            let stdout = String::from_utf8_lossy(&output.stdout);
            if !stdout.is_empty() {
                note("Test Output", &stdout)?;
            }
        } else {
            sp.stop("❌ Tests failed");
            let stderr = String::from_utf8_lossy(&output.stderr);
            note("Error Output", &stderr)?;
            outro_cancel("Tests failed")?;
            std::process::exit(1);
        }
    } else {
        note("Recipe Type", "TypeScript")?;

        let sp = spinner();
        sp.start("Running npm test...");

        let output = std::process::Command::new("npm")
            .args(["test"])
            .current_dir(&recipe_path)
            .output()?;

        if output.status.success() {
            sp.stop("✅ All tests passed!");
        } else {
            sp.stop("❌ Tests failed");
            let stderr = String::from_utf8_lossy(&output.stderr);
            note("Error Output", &stderr)?;
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
    let recipe_path = get_recipe_path(slug.clone())?;
    let recipe_slug = recipe_path
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    clear_screen()?;
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
