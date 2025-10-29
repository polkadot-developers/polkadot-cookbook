//! CLI wrapper for Polkadot Cookbook Core library
//!
//! This is a thin wrapper around the polkadot-cookbook-core library that provides
//! a command-line interface for creating and managing Polkadot Cookbook recipes.

use anyhow::Result;
use clap::{Parser, Subcommand};
use cliclack::{clear_screen, confirm, input, intro, note, outro, outro_cancel, select, spinner};
use colored::Colorize;
use polkadot_cookbook_core::{
    config::{ProjectConfig, RecipeType},
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
            handle_create(slug, cli.skip_install, cli.no_git, cli.non_interactive).await?;
        }
        Some(Commands::Setup) => {
            handle_setup().await?;
        }
        Some(Commands::Doctor) => {
            handle_doctor().await?;
        }
        Some(Commands::Recipe { command }) => match command {
            RecipeCommands::New { slug } => {
                handle_create(slug, cli.skip_install, cli.no_git, cli.non_interactive).await?;
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
            handle_create(cli.slug, cli.skip_install, cli.no_git, cli.non_interactive).await?;
        }
    }

    Ok(())
}

async fn handle_create(
    slug: Option<String>,
    skip_install: bool,
    no_git: bool,
    non_interactive: bool,
) -> Result<()> {
    // Non-interactive mode: require slug argument
    if non_interactive {
        let slug = slug
            .ok_or_else(|| anyhow::anyhow!("Slug argument is required in non-interactive mode"))?;
        return run_non_interactive(&slug, skip_install, no_git).await;
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

    // Get or prompt for slug
    let slug = if let Some(s) = slug {
        // Validate provided slug
        if let Err(e) = polkadot_cookbook_core::config::validate_slug(&s) {
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
        s
    } else {
        // Prompt for slug with hint
        let question = "What is your recipe slug?".polkadot_pink().to_string();
        let hint_text = "(lowercase, dashes only)".dimmed().to_string();
        let slug: String = input(format!("{question} {hint_text}"))
            .placeholder("my-recipe")
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
        slug
    };

    // Prompt for recipe type
    let recipe_type_question = "What type of recipe?".polkadot_pink().to_string();
    let recipe_type: RecipeType = select(&recipe_type_question)
        .item(
            RecipeType::PolkadotSdk,
            "Polkadot SDK",
            "Runtime pallets and blockchain development with Rust",
        )
        .item(
            RecipeType::Solidity,
            "Solidity",
            "Smart contracts using pallet-revive",
        )
        .item(
            RecipeType::Xcm,
            "XCM",
            "Cross-chain interactions with Chopsticks",
        )
        .interact()?;

    // Prompt for optional description
    let description_question = "Recipe description".polkadot_pink().to_string();
    let hint_text = "(optional, press Enter to skip)".dimmed().to_string();
    let description: String = input(format!("{description_question} {hint_text}"))
        .placeholder("Learn how to...")
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
    let title = polkadot_cookbook_core::config::slug_to_title(&slug);
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
            "{:<16} {} ({})\n\
             {:<16} {}\n\
             {:<16} {}\n\
             {:<16} {}\n\n\
             Files to create:\n\
             {} README.md\n\
             {} recipe.config.yml\n\
             {} versions.yml\n\
             {} tests/e2e.test.ts\n\
             {} justfile",
            "Recipe:".polkadot_pink(),
            slug.polkadot_pink().bold(),
            title.dimmed(),
            "Type:".polkadot_pink(),
            match recipe_type {
                RecipeType::PolkadotSdk => "Polkadot SDK",
                RecipeType::Solidity => "Solidity",
                RecipeType::Xcm => "XCM",
            },
            "Location:".polkadot_pink(),
            project_path.display(),
            "Git Branch:".polkadot_pink(),
            branch_name,
            "•".polkadot_pink(),
            "•".polkadot_pink(),
            "•".polkadot_pink(),
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
        .with_destination(PathBuf::from("recipes"))
        .with_git_init(create_git_branch)
        .with_skip_install(skip_install)
        .with_recipe_type(recipe_type)
        .with_description(description);

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

            if let Some(branch) = project_info.git_branch {
                let git_title = "🔀 Ready to Contribute?".polkadot_pink().to_string();
                note(
                    &git_title,
                    format!(
                        "{} {}\n{} {}\n{} {}\n\n{} Then open a Pull Request on GitHub!",
                        "→".dimmed(),
                        "git add -A".polkadot_pink(),
                        "→".dimmed(),
                        format!("git commit -m \"feat(recipe): add {}\"", project_info.slug)
                            .polkadot_pink(),
                        "→".dimmed(),
                        format!("git push origin {branch}").polkadot_pink(),
                        "📌".polkadot_pink()
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
async fn run_non_interactive(slug: &str, skip_install: bool, no_git: bool) -> Result<()> {
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

    println!(
        "{} {}",
        "Creating recipe:".polkadot_pink(),
        slug.polkadot_pink().bold()
    );

    // Create project configuration with defaults
    let config = ProjectConfig::new(slug)
        .with_destination(PathBuf::from("recipes"))
        .with_git_init(!no_git)
        .with_skip_install(skip_install)
        .with_recipe_type(RecipeType::PolkadotSdk) // Default to Polkadot SDK
        .with_description("Replace with a short description.".to_string());

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
                        "Polkadot SDK"
                    } else if content.contains("type: solidity") {
                        "Solidity"
                    } else if content.contains("type: xcm") {
                        "XCM"
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
