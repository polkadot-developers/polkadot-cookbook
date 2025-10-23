//! CLI wrapper for Polkadot Cookbook Core library
//!
//! This is a thin wrapper around the polkadot-cookbook-core library that provides
//! a command-line interface for creating and managing Polkadot Cookbook tutorials.

use anyhow::Result;
use clap::{Parser, Subcommand};
use cliclack::{clear_screen, confirm, input, intro, note, outro, outro_cancel, spinner};
use polkadot_cookbook_core::{
    config::ProjectConfig,
    version::{load_global_versions, resolve_tutorial_versions, VersionSource},
    Scaffold,
};
use std::path::{Path, PathBuf};

#[derive(Parser)]
#[command(name = "create-tutorial")]
#[command(about = "Create and manage Polkadot Cookbook tutorials", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Tutorial slug (e.g., "my-tutorial"). If not provided, will prompt interactively.
    /// Only used when no subcommand is provided (defaults to 'create')
    #[arg(value_name = "SLUG", global = true)]
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
    /// Create a new tutorial (default command if none specified)
    Create {
        /// Tutorial slug (e.g., "my-tutorial")
        #[arg(value_name = "SLUG")]
        slug: Option<String>,
    },
    /// Manage and view dependency versions
    Versions {
        /// Tutorial slug to resolve versions for (omit for global versions only)
        #[arg(value_name = "SLUG")]
        tutorial_slug: Option<String>,

        /// Output format for CI/automation (key=value pairs)
        #[arg(long, default_value = "false")]
        ci: bool,

        /// Show version sources (global vs tutorial override)
        #[arg(long, default_value = "false")]
        show_source: bool,

        /// Validate versions.yml syntax and warn about unknown keys
        #[arg(long, default_value = "false")]
        validate: bool,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "polkadot_cookbook_core=info".to_string()),
        )
        .init();

    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Create { slug }) | None => {
            // Use subcommand slug or global slug
            let slug = slug.or(cli.slug);
            handle_create(slug, cli.skip_install, cli.no_git, cli.non_interactive).await?;
        }
        Some(Commands::Versions {
            tutorial_slug,
            ci,
            show_source,
            validate,
        }) => {
            handle_versions(tutorial_slug, ci, show_source, validate).await?;
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
        let slug = slug.ok_or_else(|| anyhow::anyhow!("Slug argument is required in non-interactive mode"))?;
        return run_non_interactive(&slug, skip_install, no_git).await;
    }

    // Interactive mode with cliclack
    clear_screen()?;
    intro("🚀 Polkadot Cookbook - Tutorial Creator")?;

    // Validate working directory first
    if let Err(e) = polkadot_cookbook_core::config::validate_working_directory() {
        outro_cancel(format!(
            "❌ Invalid working directory: {e}\n\nPlease run this command from the repository root."
        ))?;
        std::process::exit(1);
    }

    note(
        "Tutorial Setup",
        "Let's create your new tutorial. This will scaffold the project structure,\ngenerate template files, and set up the testing environment.",
    )?;

    // Get or prompt for slug
    let slug = if let Some(s) = slug {
        // Validate provided slug
        if let Err(e) = polkadot_cookbook_core::config::validate_slug(&s) {
            outro_cancel(format!(
                "❌ Invalid tutorial slug format: {e}\n\nSlug must be lowercase, with words separated by dashes.\nExamples: \"my-tutorial\", \"add-nft-pallet\", \"zero-to-hero\""
            ))?;
            std::process::exit(1);
        }
        s
    } else {
        // Prompt for slug
        let slug: String = input("What is your tutorial slug?")
            .placeholder("my-tutorial")
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

    // Prompt for git branch creation (only if not specified via flag)
    let create_git_branch = if no_git {
        false
    } else {
        confirm("Create a git branch for this tutorial?")
            .initial_value(true)
            .interact()?
    };

    // Prompt for npm install (only if not specified via flag)
    let skip_install = if skip_install {
        true
    } else {
        !confirm("Install npm dependencies (vitest, @polkadot/api, etc.)?")
            .initial_value(true)
            .interact()?
    };

    // Create project configuration
    let config = ProjectConfig::new(&slug)
        .with_destination(PathBuf::from("tutorials"))
        .with_git_init(create_git_branch)
        .with_skip_install(skip_install);

    // Create the project with spinner
    let sp = spinner();
    sp.start("Creating tutorial project...");

    let scaffold = Scaffold::new();
    match scaffold.create_project(config).await {
        Ok(project_info) => {
            sp.stop("✅ Tutorial scaffolding complete");

            println!();
            note(
                "📦 Project Created",
                format!(
                    "Slug:       {}\nTitle:      {}\nLocation:   {}\nGit Branch: {}",
                    project_info.slug,
                    project_info.title,
                    project_info.project_path.display(),
                    project_info.git_branch.as_deref().unwrap_or("(none)")
                ),
            )?;

            println!();
            note(
                "📝 Next Steps",
                format!(
                    "1. Write tutorial content\n   {}/README.md\n\n\
                     2. Add implementation code\n   {}/src/\n\n\
                     3. Write comprehensive tests\n   {}/tests/\n\n\
                     4. Run tests locally\n   cd {} && npm test\n\n\
                     5. Update metadata\n   {}/tutorial.config.yml",
                    project_info.project_path.display(),
                    project_info.project_path.display(),
                    project_info.project_path.display(),
                    project_info.project_path.display(),
                    project_info.project_path.display()
                ),
            )?;

            if let Some(branch) = project_info.git_branch {
                println!();
                note(
                    "🔀 Ready to Contribute?",
                    format!(
                        "git add -A\n\
                         git commit -m \"feat(tutorial): add {}\"\n\
                         git push origin {}\n\n\
                         Then open a Pull Request on GitHub!",
                        project_info.slug, branch
                    ),
                )?;
            }

            outro("🎉 All set! Happy coding! Check CONTRIBUTING.md for guidelines.")?;
        }
        Err(e) => {
            sp.stop(format!("❌ Failed to create tutorial: {e}"));
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
        eprintln!("❌ Invalid tutorial slug format: {e}");
        eprintln!("Slug must be lowercase, with words separated by dashes.");
        eprintln!("Examples: \"my-tutorial\", \"add-nft-pallet\", \"zero-to-hero\"");
        std::process::exit(1);
    }

    // Validate working directory
    if let Err(e) = polkadot_cookbook_core::config::validate_working_directory() {
        eprintln!("❌ Invalid working directory: {e}");
        eprintln!("Please run this command from the repository root.");
        std::process::exit(1);
    }

    println!("Creating tutorial: {slug}");

    // Create project configuration
    let config = ProjectConfig::new(slug)
        .with_destination(PathBuf::from("tutorials"))
        .with_git_init(!no_git)
        .with_skip_install(skip_install);

    // Create the project
    let scaffold = Scaffold::new();
    match scaffold.create_project(config).await {
        Ok(project_info) => {
            println!("✅ Tutorial created successfully!");
            println!("Path: {}", project_info.project_path.display());
            if let Some(ref branch) = project_info.git_branch {
                println!("Git Branch: {branch}");
            }
        }
        Err(e) => {
            eprintln!("❌ Failed to create tutorial: {e}");
            std::process::exit(1);
        }
    }

    Ok(())
}

async fn handle_versions(
    tutorial_slug: Option<String>,
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
    let resolved = match &tutorial_slug {
        Some(slug) => match resolve_tutorial_versions(repo_root, slug).await {
            Ok(v) => v,
            Err(e) => {
                if !ci_format {
                    eprintln!("❌ Failed to resolve versions: {e}");
                    eprintln!();
                    eprintln!("Possible causes:");
                    eprintln!("  • Tutorial directory doesn't exist");
                    eprintln!("  • versions.yml has invalid YAML syntax");
                    eprintln!("  • Global versions.yml is missing or invalid");
                    eprintln!();
                    eprintln!("Tip: Validate YAML syntax:");
                    eprintln!("  yq eval tutorials/{}/versions.yml", slug);
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
            println!("✅ All version keys are valid!");
            println!();
            println!("Found {} valid version keys:", resolved.versions.len());
            for key in resolved.versions.keys() {
                println!("  • {key}");
            }
        } else {
            println!("⚠️  Validation warnings:");
            println!();
            for key in &unknown_keys {
                println!("  • Unknown key: '{key}'");
            }
            println!();
            println!("Known keys:");
            for key in &known_keys {
                println!("  • {key}");
            }
            println!();
            println!("Note: Unknown keys will be ignored by the workflow.");
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
        // Human-readable format
        if let Some(slug) = tutorial_slug {
            println!();
            println!("📦 Versions for tutorial: {slug}");
        } else {
            println!();
            println!("📦 Global versions");
        }
        println!();

        // Find the longest key name for alignment
        let max_len = resolved.versions.keys().map(|k| k.len()).max().unwrap_or(0);

        for (name, version) in &resolved.versions {
            if show_source {
                let source = match resolved.get_source(name) {
                    Some(VersionSource::Global) => "global",
                    Some(VersionSource::Tutorial) => "tutorial",
                    None => "unknown",
                };
                println!("  {name:<max_len$}  {version}  ({source})");
            } else {
                println!("  {name:<max_len$}  {version}");
            }
        }
        println!();
    }

    Ok(())
}
