//! CLI wrapper for Polkadot Cookbook Core library
//!
//! This is a thin wrapper around the polkadot-cookbook-core library that provides
//! a command-line interface for creating and managing Polkadot Cookbook tutorials.

use anyhow::Result;
use clap::Parser;
use cliclack::{clear_screen, confirm, input, intro, note, outro, outro_cancel, spinner};
use polkadot_cookbook_core::{config::ProjectConfig, Scaffold};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "create-tutorial")]
#[command(about = "Create a new Polkadot Cookbook tutorial", long_about = None)]
#[command(version)]
struct Cli {
    /// Tutorial slug (e.g., "my-tutorial"). If not provided, will prompt interactively.
    #[arg(value_name = "SLUG")]
    slug: Option<String>,

    /// Skip npm install
    #[arg(long, default_value = "false")]
    skip_install: bool,

    /// Skip git branch creation
    #[arg(long, default_value = "false")]
    no_git: bool,

    /// Non-interactive mode (use defaults, require slug argument)
    #[arg(long, default_value = "false")]
    non_interactive: bool,
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

    // Non-interactive mode: require slug argument
    if cli.non_interactive {
        let slug = cli
            .slug
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Slug argument is required in non-interactive mode"))?;

        return run_non_interactive(slug, cli.skip_install, cli.no_git).await;
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
        "Let's create your new tutorial. This will scaffold the project structure,\ngenerate template files, and set up the testing environment."
    )?;

    // Get or prompt for slug
    let slug = if let Some(s) = cli.slug {
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
    let create_git_branch = if cli.no_git {
        false
    } else {
        confirm("Create a git branch for this tutorial?")
            .initial_value(true)
            .interact()?
    };

    // Prompt for npm install (only if not specified via flag)
    let skip_install = if cli.skip_install {
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
