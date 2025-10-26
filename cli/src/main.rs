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
        None => {
            // No subcommand, default to create
            handle_create(cli.slug, cli.skip_install, cli.no_git, cli.non_interactive).await?;
        }
        Some(Commands::Versions {
            recipe_slug,
            ci,
            show_source,
            validate,
        }) => {
            handle_versions(recipe_slug, ci, show_source, validate).await?;
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
            RecipeType::Sdk,
            "Polkadot SDK",
            "Runtime, pallets, and blockchain development",
        )
        .item(
            RecipeType::Contracts,
            "Smart Contracts",
            "ink! smart contract development",
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
                RecipeType::Sdk => "Polkadot SDK",
                RecipeType::Contracts => "Smart Contracts",
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
        .with_recipe_type(RecipeType::Sdk) // Default to SDK
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
