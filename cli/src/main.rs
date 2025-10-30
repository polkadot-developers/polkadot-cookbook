//! CLI wrapper for Polkadot Cookbook Core library
//!
//! This is a thin wrapper around the polkadot-cookbook-core library that provides
//! a command-line interface for creating and managing Polkadot Cookbook recipes.

use anyhow::Result;
use clap::{Parser, Subcommand};
use cliclack::{clear_screen, confirm, input, intro, note, outro, outro_cancel, select, spinner};
use colored::Colorize;
use polkadot_cookbook_core::{
    config::{ContentType, Difficulty, ProjectConfig, RecipePathway, RecipeType},
    version::{load_global_versions, resolve_recipe_versions, VersionSource},
    Scaffold,
};
use std::path::{Path, PathBuf};

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
#[command(about = "Create and manage Polkadot Cookbook recipes", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Recipe slug (e.g., "my-recipe"). If not provided, will prompt interactively.
    /// Only used when no subcommand is provided (defaults to 'create')
    #[arg(value_name = "SLUG")]
    slug: Option<String>,

    /// Recipe title (for non-interactive mode)
    #[arg(long)]
    title: Option<String>,

    /// Recipe pathway (for non-interactive mode): runtime, contracts, basic-interaction, xcm, testing, request-new
    #[arg(long)]
    pathway: Option<String>,

    /// Difficulty level (for non-interactive mode): beginner, intermediate, advanced
    #[arg(long)]
    difficulty: Option<String>,

    /// Content type (for non-interactive mode): tutorial, guide
    #[arg(long, name = "content-type")]
    content_type: Option<String>,

    /// Skip npm install
    #[arg(long, default_value = "false", global = true)]
    skip_install: bool,

    /// Skip git branch creation
    #[arg(long, default_value = "false", global = true)]
    no_git: bool,

    /// Non-interactive mode (use defaults, require slug argument)
    #[arg(long, default_value = "false", global = true)]
    non_interactive: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new recipe (default command if none specified)
    Create {
        /// Recipe slug (e.g., "my-recipe")
        #[arg(value_name = "SLUG")]
        slug: Option<String>,
    },
    /// Setup development environment
    Setup,
    /// Check environment and diagnose issues
    Doctor,
    /// Manage recipes
    Recipe {
        #[command(subcommand)]
        command: RecipeCommands,
    },
    /// Manage and view dependency versions
    Versions {
        /// Recipe slug to resolve versions for (omit for global versions only)
        #[arg(value_name = "SLUG")]
        recipe_slug: Option<String>,

        /// Output format for CI/automation (key=value pairs)
        #[arg(long, default_value = "false")]
        ci: bool,

        /// Show version sources (global vs recipe override)
        #[arg(long, default_value = "false")]
        show_source: bool,

        /// Validate versions.yml syntax and warn about unknown keys
        #[arg(long, default_value = "false")]
        validate: bool,
    },
}

#[derive(Subcommand)]
enum RecipeCommands {
    /// Create a new recipe
    New {
        /// Recipe slug (e.g., "my-recipe")
        #[arg(value_name = "SLUG")]
        slug: Option<String>,
    },
    /// Run recipe tests
    Test {
        /// Recipe slug (defaults to current directory)
        #[arg(value_name = "SLUG")]
        slug: Option<String>,
    },
    /// Validate recipe structure
    Validate {
        /// Recipe slug (defaults to current directory)
        #[arg(value_name = "SLUG")]
        slug: Option<String>,
    },
    /// Run linters (clippy, fmt)
    Lint {
        /// Recipe slug (defaults to current directory)
        #[arg(value_name = "SLUG")]
        slug: Option<String>,
    },
    /// List all recipes
    List,
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
            std::env::var("RUST_LOG").unwrap_or_else(|_| "polkadot_cookbook_core=info".to_string()),
        )
        .init();

    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Create { slug: cmd_slug }) => {
            // Use subcommand slug or global slug
            let slug = cmd_slug.or(cli.slug);
            handle_create(
                slug,
                cli.title,
                cli.pathway,
                cli.difficulty,
                cli.content_type,
                cli.skip_install,
                cli.no_git,
                cli.non_interactive,
            )
            .await?;
        }
        Some(Commands::Setup) => {
            handle_setup().await?;
        }
        Some(Commands::Doctor) => {
            handle_doctor().await?;
        }
        Some(Commands::Recipe { command }) => match command {
            RecipeCommands::New { slug } => {
                handle_create(
                    slug,
                    cli.title,
                    cli.pathway,
                    cli.difficulty,
                    cli.content_type,
                    cli.skip_install,
                    cli.no_git,
                    cli.non_interactive,
                )
                .await?;
            }
            RecipeCommands::Test { slug } => {
                handle_recipe_test(slug).await?;
            }
            RecipeCommands::Validate { slug } => {
                handle_recipe_validate(slug).await?;
            }
            RecipeCommands::Lint { slug } => {
                handle_recipe_lint(slug).await?;
            }
            RecipeCommands::List => {
                handle_recipe_list().await?;
            }
            RecipeCommands::Submit { slug, title, body } => {
                handle_recipe_submit(slug, title, body).await?;
            }
        },
        Some(Commands::Versions {
            recipe_slug,
            ci,
            show_source,
            validate,
        }) => {
            handle_versions(recipe_slug, ci, show_source, validate).await?;
        }
        None => {
            // No subcommand, default to create
            handle_create(
                cli.slug,
                cli.title,
                cli.pathway,
                cli.difficulty,
                cli.content_type,
                cli.skip_install,
                cli.no_git,
                cli.non_interactive,
            )
            .await?;
        }
    }

    Ok(())
}

async fn handle_create(
    slug: Option<String>,
    title: Option<String>,
    pathway: Option<String>,
    difficulty: Option<String>,
    content_type: Option<String>,
    skip_install: bool,
    no_git: bool,
    non_interactive: bool,
) -> Result<()> {
    // Non-interactive mode: require slug argument
    if non_interactive {
        let slug = slug
            .ok_or_else(|| anyhow::anyhow!("Slug argument is required in non-interactive mode"))?;
        return run_non_interactive(
            &slug,
            title,
            pathway,
            difficulty,
            content_type,
            skip_install,
            no_git,
        )
        .await;
    }

    // Interactive mode with cliclack
    clear_screen()?;

    // Validate working directory first
    if let Err(e) = polkadot_cookbook_core::config::validate_working_directory() {
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
            "Deploy contracts using pallet-revive",
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

    // Step 2: Ask for title (now that user knows the pathway)
    let (title, slug) = if let Some(s) = slug.as_ref() {
        // Slug was provided via CLI arg - validate and derive title
        if let Err(e) = polkadot_cookbook_core::config::validate_slug(s) {
            outro_cancel(format!(
                "❌ Invalid recipe slug format: {}\n\n\
                Slugs must be:\n\
                • Lowercase letters only\n\
                • Words separated by dashes\n\n\
                Examples:\n\
                {} my-recipe\n\
                {} add-nft-pallet\n\
                {} zero-to-hero\n\n\
                Invalid:\n\
                {} MyRecipe\n\
                {} my_recipe",
                e,
                "✓".polkadot_pink(),
                "✓".polkadot_pink(),
                "✓".polkadot_pink(),
                "✗".dimmed(),
                "✗".dimmed()
            ))?;
            std::process::exit(1);
        }
        let title = polkadot_cookbook_core::config::slug_to_title(s);
        (title, s.clone())
    } else {
        // Interactive mode: ask for title
        let title_question = "What is your recipe title?".polkadot_pink().to_string();
        let hint_text = "(e.g., 'Custom NFT Pallet', 'Cross-Chain Asset Transfer')"
            .dimmed()
            .to_string();
        let title: String = input(format!("{title_question} {hint_text}"))
            .placeholder("My Recipe")
            .validate(|input: &String| {
                if input.trim().is_empty() {
                    Err("Title cannot be empty")
                } else if let Err(e) = polkadot_cookbook_core::config::validate_title(input) {
                    Err(Box::leak(e.to_string().into_boxed_str()) as &str)
                } else {
                    Ok(())
                }
            })
            .interact()?;

        // Generate suggested slug from title
        let suggested_slug = polkadot_cookbook_core::config::title_to_slug(&title);

        // Prompt for slug with suggestion pre-filled
        let slug_question = "Recipe slug".polkadot_pink().to_string();
        let slug_hint = "(lowercase, dashes only)".dimmed().to_string();
        let slug: String = input(format!("{slug_question} {slug_hint}"))
            .default_input(&suggested_slug)
            .validate(|input: &String| {
                if input.is_empty() {
                    Err("Slug cannot be empty")
                } else if let Err(e) = polkadot_cookbook_core::config::validate_slug(input) {
                    Err(Box::leak(e.to_string().into_boxed_str()) as &str)
                } else {
                    Ok(())
                }
            })
            .interact()?;

        (title.trim().to_string(), slug)
    };

    // Step 3: Prompt for difficulty level
    let difficulty_question = "What's the difficulty level?".polkadot_pink().to_string();
    let difficulty: Difficulty = select(&difficulty_question)
        .item(
            Difficulty::Beginner,
            "Beginner",
            "New to Polkadot or blockchain development",
        )
        .item(
            Difficulty::Intermediate,
            "Intermediate",
            "Familiar with Polkadot concepts and basic development",
        )
        .item(
            Difficulty::Advanced,
            "Advanced",
            "Expert-level topics and complex implementations",
        )
        .interact()?;

    // Step 4: Prompt for content type
    let content_type_question = "What type of content will this recipe include?"
        .polkadot_pink()
        .to_string();
    let content_type: ContentType = select(&content_type_question)
        .item(
            ContentType::Tutorial,
            "Tutorial",
            "Step-by-step guide from zero to working solution",
        )
        .item(
            ContentType::Guide,
            "Guide",
            "Focused, actionable steps for a specific task",
        )
        .interact()?;

    // Step 5: Prompt for description
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
             {:<16} {}\n\
             {:<16} {}\n\
             {:<16} {}\n\n\
             Files to create:\n\
             {} README.md\n\
             {} recipe.config.yml\n\
             {} Template files for {}",
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
            "Difficulty:".polkadot_pink(),
            match difficulty {
                Difficulty::Beginner => "Beginner",
                Difficulty::Intermediate => "Intermediate",
                Difficulty::Advanced => "Advanced",
            },
            "Content Type:".polkadot_pink(),
            match content_type {
                ContentType::Tutorial => "Tutorial",
                ContentType::Guide => "Guide",
            },
            "Location:".polkadot_pink(),
            project_path.display(),
            "Git Branch:".polkadot_pink(),
            branch_name,
            "•".polkadot_pink(),
            "•".polkadot_pink(),
            "•".polkadot_pink(),
            match pathway {
                RecipePathway::Runtime => "Polkadot SDK",
                RecipePathway::Contracts => "Solidity",
                RecipePathway::BasicInteraction => "Basic Interactions",
                RecipePathway::Xcm => "XCM",
                RecipePathway::Testing => "Testing",
                RecipePathway::RequestNew => {
                    unreachable!("RequestNew should have been handled before summary")
                }
            }
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
        .with_pathway(pathway)
        .with_content_type(content_type)
        .with_difficulty(difficulty);

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
                     {} Run tests\n   {} {}\n\n\
                     {} Update metadata\n   {} {}",
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
                        .polkadot_pink(),
                    "5.".polkadot_pink().bold(),
                    "→".dimmed(),
                    format!("{}/recipe.config.yml", project_info.project_path.display())
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
    slug: &str,
    title: Option<String>,
    pathway: Option<String>,
    difficulty: Option<String>,
    content_type: Option<String>,
    skip_install: bool,
    no_git: bool,
) -> Result<()> {
    // Validate slug
    if let Err(e) = polkadot_cookbook_core::config::validate_slug(slug) {
        eprintln!("❌ Invalid recipe slug format: {e}");
        eprintln!("Slug must be lowercase, with words separated by dashes.");
        eprintln!("Examples: \"my-recipe\", \"add-nft-pallet\", \"zero-to-hero\"");
        std::process::exit(1);
    }

    // Validate working directory
    if let Err(e) = polkadot_cookbook_core::config::validate_working_directory() {
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

    // Determine title
    let title = title.unwrap_or_else(|| polkadot_cookbook_core::config::slug_to_title(slug));

    // Parse difficulty
    let difficulty_level = if let Some(d) = difficulty {
        match d.as_str() {
            "beginner" => Some(Difficulty::Beginner),
            "intermediate" => Some(Difficulty::Intermediate),
            "advanced" => Some(Difficulty::Advanced),
            _ => {
                eprintln!("❌ Invalid difficulty: {d}");
                eprintln!("Valid difficulties: beginner, intermediate, advanced");
                std::process::exit(1);
            }
        }
    } else {
        None
    };

    // Parse content type
    let content_type_value = if let Some(ct) = content_type {
        match ct.as_str() {
            "tutorial" => Some(ContentType::Tutorial),
            "guide" => Some(ContentType::Guide),
            _ => {
                eprintln!("❌ Invalid content type: {ct}");
                eprintln!("Valid content types: tutorial, guide");
                std::process::exit(1);
            }
        }
    } else {
        None
    };

    // Determine pathway from recipe type
    let pathway_value = match recipe_type {
        RecipeType::PolkadotSdk => Some(RecipePathway::Runtime),
        RecipeType::Solidity => Some(RecipePathway::Contracts),
        RecipeType::BasicInteraction => Some(RecipePathway::BasicInteraction),
        RecipeType::Xcm => Some(RecipePathway::Xcm),
        RecipeType::Testing => Some(RecipePathway::Testing),
    };

    println!(
        "{} {}",
        "Creating recipe:".polkadot_pink(),
        slug.polkadot_pink().bold()
    );

    // Create project configuration with provided or default values
    let mut config = ProjectConfig::new(slug)
        .with_title(&title)
        .with_destination(PathBuf::from("recipes"))
        .with_git_init(!no_git)
        .with_skip_install(skip_install)
        .with_recipe_type(recipe_type)
        .with_description("Replace with a short description.".to_string());

    // Add optional fields if provided
    if let Some(p) = pathway_value {
        config = config.with_pathway(p);
    }
    if let Some(ct) = content_type_value {
        config = config.with_content_type(ct);
    }
    if let Some(d) = difficulty_level {
        config = config.with_difficulty(d);
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

async fn handle_versions(
    recipe_slug: Option<String>,
    ci_format: bool,
    show_source: bool,
    validate: bool,
) -> Result<()> {
    let repo_root = Path::new(".");

    // Validate working directory
    if let Err(e) = polkadot_cookbook_core::config::validate_working_directory() {
        if !ci_format {
            eprintln!("❌ Invalid working directory: {e}");
            eprintln!("Please run this command from the repository root.");
        } else {
            eprintln!("Error: {e}");
        }
        std::process::exit(1);
    }

    // Resolve versions with better error handling
    let resolved = match &recipe_slug {
        Some(slug) => match resolve_recipe_versions(repo_root, slug).await {
            Ok(v) => v,
            Err(e) => {
                if !ci_format {
                    eprintln!("❌ Failed to resolve versions: {e}");
                    eprintln!();
                    eprintln!("Possible causes:");
                    eprintln!("  • Recipe directory doesn't exist");
                    eprintln!("  • versions.yml has invalid YAML syntax");
                    eprintln!("  • Global versions.yml is missing or invalid");
                    eprintln!();
                    eprintln!("Tip: Validate YAML syntax:");
                    eprintln!("  yq eval recipes/{slug}/versions.yml");
                } else {
                    eprintln!("Error resolving versions: {e}");
                }
                std::process::exit(1);
            }
        },
        None => match load_global_versions(repo_root).await {
            Ok(v) => v,
            Err(e) => {
                if !ci_format {
                    eprintln!("❌ Failed to load global versions: {e}");
                    eprintln!();
                    eprintln!("Possible causes:");
                    eprintln!("  • Global versions.yml is missing");
                    eprintln!("  • versions.yml has invalid YAML syntax");
                    eprintln!();
                    eprintln!("Tip: Validate YAML syntax:");
                    eprintln!("  yq eval versions.yml");
                } else {
                    eprintln!("Error loading global versions: {e}");
                }
                std::process::exit(1);
            }
        },
    };

    // Validation mode
    if validate {
        let known_keys = vec![
            "rust",
            "polkadot_omni_node",
            "chain_spec_builder",
            "frame_omni_bencher",
        ];

        let mut has_warnings = false;
        let mut unknown_keys = Vec::new();

        for key in resolved.versions.keys() {
            if !known_keys.contains(&key.as_str()) {
                unknown_keys.push(key.clone());
                has_warnings = true;
            }
        }

        if !has_warnings {
            intro("Validation Result")?;

            let mut valid_keys = String::new();
            valid_keys.push_str(&format!(
                "Found {} valid version keys:\n\n",
                resolved.versions.len()
            ));
            for key in resolved.versions.keys() {
                valid_keys.push_str(&format!("• {}\n", key.polkadot_pink()));
            }

            note("✅", valid_keys.trim_end())?;
            outro("All version keys are valid!")?;
        } else {
            intro("Validation Warnings")?;

            let mut warnings_text = String::new();
            warnings_text.push_str("Unknown keys:\n\n");
            for key in &unknown_keys {
                warnings_text.push_str(&format!("• {}\n", key.yellow()));
            }
            warnings_text.push_str("\nKnown keys:\n\n");
            for key in &known_keys {
                warnings_text.push_str(&format!("• {}\n", key.polkadot_pink()));
            }
            warnings_text.push_str("\nNote: Unknown keys will be ignored by the workflow.");

            note("⚠️", warnings_text.trim_end())?;
            outro("Validation complete with warnings")?;
        }

        return Ok(());
    }

    if ci_format {
        // Output in CI-friendly format: KEY=VALUE
        for (name, version) in &resolved.versions {
            // Convert to SCREAMING_SNAKE_CASE for environment variables
            let env_name = name.to_uppercase();
            println!("{env_name}={version}");
        }
    } else {
        // Human-readable format with Polkadot colors using cliclack
        if let Some(slug) = &recipe_slug {
            let title = format!("Versions for recipe: {}", slug.polkadot_pink());
            intro(&title)?;
        } else {
            intro("Global versions")?;
        }

        // Build the versions content
        let mut versions_text = String::new();
        for (name, version) in &resolved.versions {
            if show_source {
                let source = match resolved.get_source(name) {
                    Some(VersionSource::Global) => "global".dimmed().to_string(),
                    Some(VersionSource::Recipe) => "recipe".polkadot_pink().to_string(),
                    None => "unknown".dimmed().to_string(),
                };
                versions_text.push_str(&format!(
                    "{}  {}  ({})\n",
                    name.polkadot_pink(),
                    version,
                    source
                ));
            } else {
                versions_text.push_str(&format!("{}  {}\n", name.polkadot_pink(), version));
            }
        }

        note("📦", versions_text.trim_end())?;
        outro("Done")?;
    }

    Ok(())
}

async fn handle_setup() -> Result<()> {
    clear_screen()?;
    intro(
        "🔧 Setup Development Environment"
            .polkadot_pink()
            .to_string(),
    )?;

    note(
        "Checking Dependencies",
        "Verifying your development environment...",
    )?;

    let sp = spinner();
    sp.start("Checking Rust installation...");

    // Check Rust
    let rust_check = std::process::Command::new("rustc")
        .arg("--version")
        .output();

    match rust_check {
        Ok(output) if output.status.success() => {
            let version = String::from_utf8_lossy(&output.stdout);
            sp.stop(format!("✅ {}", version.trim()));
        }
        _ => {
            sp.stop("❌ Rust not found");
            note("Install Rust", "Visit https://rustup.rs to install Rust")?;
            outro_cancel("Setup incomplete")?;
            std::process::exit(1);
        }
    }

    // Check Cargo
    sp.start("Checking Cargo...");
    let cargo_check = std::process::Command::new("cargo")
        .arg("--version")
        .output();

    match cargo_check {
        Ok(output) if output.status.success() => {
            let version = String::from_utf8_lossy(&output.stdout);
            sp.stop(format!("✅ {}", version.trim()));
        }
        _ => {
            sp.stop("❌ Cargo not found");
            outro_cancel("Setup incomplete")?;
            std::process::exit(1);
        }
    }

    // Check Just (optional)
    sp.start("Checking Just...");
    let just_check = std::process::Command::new("just").arg("--version").output();

    match just_check {
        Ok(output) if output.status.success() => {
            let version = String::from_utf8_lossy(&output.stdout);
            sp.stop(format!("✅ {}", version.trim()));
        }
        _ => {
            sp.stop("⚠️  Just not found (optional)");
            note(
                "Install Just",
                "Just is recommended for running recipe commands.\nInstall: cargo install just",
            )?;
        }
    }

    // Check Git
    sp.start("Checking Git...");
    let git_check = std::process::Command::new("git").arg("--version").output();

    match git_check {
        Ok(output) if output.status.success() => {
            let version = String::from_utf8_lossy(&output.stdout);
            sp.stop(format!("✅ {}", version.trim()));
        }
        _ => {
            sp.stop("❌ Git not found");
            note(
                "Install Git",
                "Git is required for version control.\nVisit https://git-scm.com",
            )?;
            outro_cancel("Setup incomplete")?;
            std::process::exit(1);
        }
    }

    outro("✅ Setup complete! You're ready to create recipes.")?;
    Ok(())
}

async fn handle_doctor() -> Result<()> {
    clear_screen()?;
    intro("🩺 Environment Diagnostics".polkadot_pink().to_string())?;

    let mut issues = Vec::new();
    let mut warnings = Vec::new();

    // Check Rust version
    if let Ok(output) = std::process::Command::new("rustc")
        .arg("--version")
        .output()
    {
        if output.status.success() {
            let version_str = String::from_utf8_lossy(&output.stdout);
            if let Some(version_part) = version_str.split_whitespace().nth(1) {
                note("✅ Rust", format!("Version: {version_part}"))?;
            }
        }
    } else {
        issues.push("Rust is not installed");
    }

    // Check Cargo
    if let Ok(output) = std::process::Command::new("cargo")
        .arg("--version")
        .output()
    {
        if output.status.success() {
            let version_str = String::from_utf8_lossy(&output.stdout);
            if let Some(version_part) = version_str.split_whitespace().nth(1) {
                note("✅ Cargo", format!("Version: {version_part}"))?;
            }
        }
    } else {
        issues.push("Cargo is not installed");
    }

    // Check Git
    if let Ok(output) = std::process::Command::new("git").arg("--version").output() {
        if output.status.success() {
            let version_str = String::from_utf8_lossy(&output.stdout);
            note("✅ Git", version_str.trim())?;
        }
    } else {
        issues.push("Git is not installed");
    }

    // Check Just (optional)
    if let Ok(output) = std::process::Command::new("just").arg("--version").output() {
        if output.status.success() {
            let version_str = String::from_utf8_lossy(&output.stdout);
            note("✅ Just", version_str.trim())?;
        }
    } else {
        warnings.push("Just is not installed (optional but recommended)");
    }

    // Check if we're in a git repository
    if let Ok(output) = std::process::Command::new("git")
        .args(["rev-parse", "--git-dir"])
        .output()
    {
        if output.status.success() {
            note("✅ Git Repository", "You're in a git repository")?;
        } else {
            warnings.push("Not in a git repository");
        }
    }

    // Check if recipes directory exists
    if Path::new("recipes").exists() {
        note("✅ Recipes Directory", "Found recipes/ directory")?;
    } else {
        warnings.push("recipes/ directory not found - you may not be in the repository root");
    }

    if !issues.is_empty() {
        note("❌ Issues Found", issues.join("\n"))?;
        outro_cancel("Please fix the issues above")?;
        std::process::exit(1);
    }

    if !warnings.is_empty() {
        note("⚠️  Warnings", warnings.join("\n"))?;
    }

    outro("✅ All checks passed!")?;
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

    // Check recipe type
    let config_path = recipe_path.join("recipe.config.yml");
    if !config_path.exists() {
        outro_cancel("recipe.config.yml not found")?;
        std::process::exit(1);
    }

    let config_content = std::fs::read_to_string(&config_path)?;
    let is_polkadot_sdk = config_content.contains("type: polkadot-sdk");

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

async fn handle_recipe_validate(slug: Option<String>) -> Result<()> {
    let recipe_path = get_recipe_path(slug)?;

    intro(format!(
        "🔍 Validating Recipe: {}",
        recipe_path
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .polkadot_pink()
    ))?;

    let mut errors = Vec::new();
    let mut warnings = Vec::new();

    // Check for required files
    let required_files = vec!["README.md", "recipe.config.yml"];
    for file in required_files {
        let file_path = recipe_path.join(file);
        if file_path.exists() {
            note(format!("✅ {file}"), "Found")?;
        } else {
            errors.push(format!("Missing required file: {file}"));
        }
    }

    // Check recipe.config.yml
    let config_path = recipe_path.join("recipe.config.yml");
    if config_path.exists() {
        match std::fs::read_to_string(&config_path) {
            Ok(content) => {
                // Check for required fields
                let required_fields = vec!["name:", "slug:", "type:", "description:"];
                for field in required_fields {
                    if !content.contains(field) {
                        errors.push(format!("recipe.config.yml missing field: {field}"));
                    }
                }
            }
            Err(e) => {
                errors.push(format!("Could not read recipe.config.yml: {e}"));
            }
        }
    }

    // Check for test directory
    let tests_path = recipe_path.join("tests");
    if !tests_path.exists() {
        warnings.push("No tests/ directory found");
    } else {
        note("✅ tests/", "Found")?;
    }

    // Check for Cargo.toml or package.json
    let has_cargo = recipe_path.join("Cargo.toml").exists();
    let has_package = recipe_path.join("package.json").exists();

    if !has_cargo && !has_package {
        errors.push("Neither Cargo.toml nor package.json found".to_string());
    } else if has_cargo {
        note("✅ Cargo.toml", "Found (Rust project)")?;
    } else if has_package {
        note("✅ package.json", "Found (TypeScript project)")?;
    }

    if !errors.is_empty() {
        note("❌ Errors", errors.join("\n"))?;
        outro_cancel("Validation failed")?;
        std::process::exit(1);
    }

    if !warnings.is_empty() {
        note("⚠️  Warnings", warnings.join("\n"))?;
    }

    outro("✅ Validation passed!")?;
    Ok(())
}

async fn handle_recipe_lint(slug: Option<String>) -> Result<()> {
    let recipe_path = get_recipe_path(slug)?;

    intro(format!(
        "🔧 Linting Recipe: {}",
        recipe_path
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .polkadot_pink()
    ))?;

    // Check if it's a Rust project
    let cargo_toml = recipe_path.join("Cargo.toml");
    if !cargo_toml.exists() {
        outro_cancel("Not a Rust project (Cargo.toml not found)")?;
        std::process::exit(1);
    }

    // Run cargo fmt --check
    let sp = spinner();
    sp.start("Checking formatting (cargo fmt)...");

    let fmt_output = std::process::Command::new("cargo")
        .args(["fmt", "--all", "--", "--check"])
        .current_dir(&recipe_path)
        .output()?;

    if fmt_output.status.success() {
        sp.stop("✅ Formatting check passed");
    } else {
        sp.stop("❌ Formatting issues found");
        note("Fix with", "cargo fmt --all")?;
    }

    // Run cargo clippy
    sp.start("Running clippy...");

    let clippy_output = std::process::Command::new("cargo")
        .args([
            "clippy",
            "--all-features",
            "--all-targets",
            "--",
            "-D",
            "warnings",
        ])
        .current_dir(&recipe_path)
        .output()?;

    if clippy_output.status.success() {
        sp.stop("✅ Clippy check passed");
        outro("✅ All lints passed!")?;
    } else {
        sp.stop("❌ Clippy found issues");
        let stderr = String::from_utf8_lossy(&clippy_output.stderr);
        note("Clippy Output", &stderr)?;
        outro_cancel("Linting failed")?;
        std::process::exit(1);
    }

    Ok(())
}

async fn handle_recipe_list() -> Result<()> {
    intro("📚 Available Recipes".polkadot_pink().to_string())?;

    let recipes_dir = Path::new("recipes");
    if !recipes_dir.exists() {
        outro_cancel("recipes/ directory not found")?;
        std::process::exit(1);
    }

    let mut recipes = Vec::new();

    if let Ok(entries) = std::fs::read_dir(recipes_dir) {
        for entry in entries.flatten() {
            if entry.path().is_dir() {
                let name = entry.file_name();
                let name_str = name.to_string_lossy().to_string();

                // Read recipe type from config
                let config_path = entry.path().join("recipe.config.yml");
                let recipe_type = if let Ok(content) = std::fs::read_to_string(&config_path) {
                    if content.contains("type: polkadot-sdk") {
                        "Polkadot SDK (Runtime Development)"
                    } else if content.contains("type: solidity") {
                        "Smart Contracts (Solidity)"
                    } else if content.contains("type: xcm") {
                        "Chain Interactions (Basic Interactions, Cross-Chain Interactions)"
                    } else {
                        "Unknown"
                    }
                } else {
                    "Unknown"
                };

                recipes.push((name_str, recipe_type.to_string()));
            }
        }
    }

    if recipes.is_empty() {
        note("No Recipes", "No recipes found in recipes/ directory")?;
    } else {
        recipes.sort_by(|a, b| a.0.cmp(&b.0));

        let mut output = String::new();
        for (name, recipe_type) in recipes {
            output.push_str(&format!("• {} ({})\n", name.polkadot_pink(), recipe_type));
        }

        note("Recipes", output.trim())?;
    }

    outro("Done")?;
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

    // Read recipe metadata
    let config_path = recipe_path.join("recipe.config.yml");
    let config_content = std::fs::read_to_string(&config_path)?;

    // Extract recipe name and description from config
    let recipe_name = config_content
        .lines()
        .find(|l| l.starts_with("name:"))
        .and_then(|l| l.split(':').nth(1))
        .map(|s| s.trim())
        .unwrap_or(&recipe_slug);

    let recipe_desc = config_content
        .lines()
        .find(|l| l.starts_with("description:"))
        .and_then(|l| l.split(':').nth(1))
        .map(|s| s.trim())
        .unwrap_or("A new Polkadot Cookbook recipe");

    let recipe_type = if config_content.contains("type: polkadot-sdk") {
        "Polkadot SDK"
    } else if config_content.contains("type: solidity") {
        "Solidity"
    } else if config_content.contains("type: xcm") {
        "XCM"
    } else {
        "Unknown"
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

/// Get GitHub token from environment variable or gh CLI config
fn get_github_token() -> Result<String> {
    // First try environment variable
    if let Ok(token) = std::env::var("GITHUB_TOKEN") {
        if !token.is_empty() {
            return Ok(token);
        }
    }

    // Try gh CLI config file
    if let Some(home) = std::env::var_os("HOME") {
        let gh_config_path = PathBuf::from(home).join(".config/gh/hosts.yml");
        if gh_config_path.exists() {
            if let Ok(content) = std::fs::read_to_string(&gh_config_path) {
                // Parse the YAML to find oauth_token
                for line in content.lines() {
                    if line.trim().starts_with("oauth_token:") {
                        if let Some(token) = line.split(':').nth(1) {
                            let token = token.trim().to_string();
                            if !token.is_empty() {
                                return Ok(token);
                            }
                        }
                    }
                }
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
