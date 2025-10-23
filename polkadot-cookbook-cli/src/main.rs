//! CLI wrapper for Polkadot Cookbook Core library
//!
//! This is a thin wrapper around the polkadot-cookbook-core library that provides
//! a command-line interface for creating and managing Polkadot Cookbook tutorials.

use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::*;
use polkadot_cookbook_core::{
    config::ProjectConfig,
    version::{load_global_versions, resolve_tutorial_versions, VersionSource},
    Scaffold,
};
use std::path::{Path, PathBuf};

#[derive(Parser)]
#[command(name = "polkadot-cookbook")]
#[command(about = "Polkadot Cookbook CLI - Create and manage tutorials", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new tutorial
    Create {
        /// Tutorial slug (e.g., "my-tutorial")
        #[arg(value_name = "SLUG")]
        slug: String,

        /// Skip npm install
        #[arg(long, default_value = "false")]
        skip_install: bool,

        /// Skip git branch creation
        #[arg(long, default_value = "false")]
        no_git: bool,
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
        Commands::Create {
            slug,
            skip_install,
            no_git,
        } => {
            handle_create(slug, skip_install, no_git).await?;
        }
        Commands::Versions {
            tutorial_slug,
            ci,
            show_source,
            validate,
        } => {
            handle_versions(tutorial_slug, ci, show_source, validate).await?;
        }
    }

    Ok(())
}

async fn handle_create(slug: String, skip_install: bool, no_git: bool) -> Result<()> {
    println!(
        "\n{}\n",
        "🚀 Polkadot Cookbook - Tutorial Creator".blue().bold()
    );

    // Validate slug using core library
    if let Err(e) = polkadot_cookbook_core::config::validate_slug(&slug) {
        eprintln!("{}", "❌ Invalid tutorial slug format!".red());
        eprintln!("{}", format!("Error: {}", e).red());
        eprintln!(
            "{}",
            "ℹ️  Slug must be lowercase, with words separated by dashes.".cyan()
        );
        eprintln!(
            "{}",
            "ℹ️  Examples: \"my-tutorial\", \"add-nft-pallet\", \"zero-to-hero\"".cyan()
        );
        std::process::exit(1);
    }

    // Validate working directory
    if let Err(e) = polkadot_cookbook_core::config::validate_working_directory() {
        eprintln!("{}", "❌ Invalid working directory!".red());
        eprintln!("{}", format!("Error: {}", e).red());
        eprintln!(
            "{}",
            "ℹ️  Please run this command from the repository root.".cyan()
        );
        std::process::exit(1);
    }

    println!("{}\n", format!("Creating tutorial: {}", slug).cyan());

    // Create project configuration
    let config = ProjectConfig::new(&slug)
        .with_destination(PathBuf::from("tutorials"))
        .with_git_init(!no_git)
        .with_skip_install(skip_install);

    // Create the project using the core library
    let scaffold = Scaffold::new();
    match scaffold.create_project(config).await {
        Ok(project_info) => {
            println!(
                "\n{}",
                "============================================================".green()
            );
            println!("{}", "🎉 Tutorial created successfully!".green());
            println!(
                "{}\n",
                "============================================================".green()
            );

            println!("{}", "📝 Project Information:".yellow());
            println!("   Slug: {}", project_info.slug);
            println!("   Title: {}", project_info.title);
            println!("   Path: {}", project_info.project_path.display());
            if let Some(ref branch) = project_info.git_branch {
                println!("   Git Branch: {}", branch);
            }
            println!();

            println!("{}", "📝 Next Steps:".yellow());
            println!();
            println!("{}", "  1. Write your tutorial content:".cyan());
            println!("     {}/README.md", project_info.project_path.display());
            println!();
            println!("{}", "  2. Add your code implementation:".cyan());
            println!("     {}/src/", project_info.project_path.display());
            println!();
            println!("{}", "  3. Write comprehensive tests:".cyan());
            println!("     {}/tests/", project_info.project_path.display());
            println!();
            println!("{}", "  4. Run tests to verify:".cyan());
            println!(
                "     cd {} && npm test",
                project_info.project_path.display()
            );
            println!();
            println!("{}", "  5. Update tutorial.config.yml metadata:".cyan());
            println!(
                "     {}/tutorial.config.yml",
                project_info.project_path.display()
            );
            println!();
            println!("{}", "  6. When ready, open a Pull Request:".cyan());
            println!("     git add -A");
            println!(
                "     git commit -m \"feat(tutorial): add {}\"",
                project_info.slug
            );
            if let Some(branch) = project_info.git_branch {
                println!("     git push origin {}", branch);
            }
            println!();

            println!(
                "{}",
                "📚 Need help? Check CONTRIBUTING.md or open an issue!\n".blue()
            );
        }
        Err(e) => {
            eprintln!("{}", "❌ Failed to create tutorial!".red());
            eprintln!("{}", format!("Error: {}", e).red());
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
            eprintln!("{}", "❌ Invalid working directory!".red());
            eprintln!("{}", format!("Error: {}", e).red());
            eprintln!(
                "{}",
                "ℹ️  Please run this command from the repository root.".cyan()
            );
        } else {
            eprintln!("Error: {}", e);
        }
        std::process::exit(1);
    }

    // Resolve versions with better error handling
    let resolved = match &tutorial_slug {
        Some(slug) => match resolve_tutorial_versions(repo_root, slug).await {
            Ok(v) => v,
            Err(e) => {
                if !ci_format {
                    eprintln!("{}", "❌ Failed to resolve versions!".red());
                    eprintln!("{}", format!("Error: {}", e).red());
                    eprintln!();
                    eprintln!("{}", "Possible causes:".yellow());
                    eprintln!("  • Tutorial directory doesn't exist");
                    eprintln!("  • versions.yml has invalid YAML syntax");
                    eprintln!("  • Global versions.yml is missing or invalid");
                    eprintln!();
                    eprintln!("{}", "Tip: Validate YAML syntax:".cyan());
                    eprintln!("  yq eval tutorials/{}/versions.yml", slug);
                } else {
                    eprintln!("Error resolving versions: {}", e);
                }
                std::process::exit(1);
            }
        },
        None => match load_global_versions(repo_root).await {
            Ok(v) => v,
            Err(e) => {
                if !ci_format {
                    eprintln!("{}", "❌ Failed to load global versions!".red());
                    eprintln!("{}", format!("Error: {}", e).red());
                    eprintln!();
                    eprintln!("{}", "Possible causes:".yellow());
                    eprintln!("  • Global versions.yml is missing");
                    eprintln!("  • versions.yml has invalid YAML syntax");
                    eprintln!();
                    eprintln!("{}", "Tip: Validate YAML syntax:".cyan());
                    eprintln!("  yq eval versions.yml");
                } else {
                    eprintln!("Error loading global versions: {}", e);
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
            println!("{}", "✅ All version keys are valid!".green());
            println!();
            println!("Found {} valid version keys:", resolved.versions.len());
            for key in resolved.versions.keys() {
                println!("  • {}", key.green());
            }
        } else {
            println!("{}", "⚠️  Validation warnings:".yellow());
            println!();
            for key in &unknown_keys {
                println!("{}", format!("  • Unknown key: '{}'", key).yellow());
            }
            println!();
            println!("{}", "Known keys:".cyan());
            for key in &known_keys {
                println!("  • {}", key);
            }
            println!();
            println!(
                "{}",
                "Note: Unknown keys will be ignored by the workflow.".dimmed()
            );
        }

        return Ok(());
    }

    if ci_format {
        // Output in CI-friendly format: KEY=VALUE
        for (name, version) in &resolved.versions {
            // Convert to SCREAMING_SNAKE_CASE for environment variables
            let env_name = name.to_uppercase();
            println!("{}={}", env_name, version);
        }
    } else {
        // Human-readable format
        if let Some(slug) = tutorial_slug {
            println!(
                "\n{}",
                format!("📦 Versions for tutorial: {}", slug).cyan().bold()
            );
        } else {
            println!("\n{}", "📦 Global versions".cyan().bold());
        }
        println!();

        // Find the longest key name for alignment
        let max_len = resolved.versions.keys().map(|k| k.len()).max().unwrap_or(0);

        for (name, version) in &resolved.versions {
            if show_source {
                let source = match resolved.get_source(name) {
                    Some(VersionSource::Global) => "global".dimmed(),
                    Some(VersionSource::Tutorial) => "tutorial".yellow(),
                    None => "unknown".red(),
                };
                println!(
                    "  {:<width$}  {}  ({})",
                    name.green(),
                    version,
                    source,
                    width = max_len
                );
            } else {
                println!("  {:<width$}  {}", name.green(), version, width = max_len);
            }
        }
        println!();
    }

    Ok(())
}
