//! CLI wrapper for Polkadot Cookbook Core library
//!
//! This is a thin wrapper around the polkadot-cookbook-core library that provides
//! a command-line interface for creating and managing Polkadot Cookbook tutorials.

use anyhow::Result;
use clap::Parser;
use colored::*;
use polkadot_cookbook_core::{config::ProjectConfig, Scaffold};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "create-tutorial")]
#[command(about = "Create a new Polkadot Cookbook tutorial", long_about = None)]
#[command(version)]
struct Cli {
    /// Tutorial slug (e.g., "my-tutorial")
    #[arg(value_name = "SLUG")]
    slug: String,

    /// Skip npm install
    #[arg(long, default_value = "false")]
    skip_install: bool,

    /// Skip git branch creation
    #[arg(long, default_value = "false")]
    no_git: bool,
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

    println!("\n{}\n", "🚀 Polkadot Cookbook - Tutorial Creator".blue().bold());

    // Validate slug using core library
    if let Err(e) = polkadot_cookbook_core::config::validate_slug(&cli.slug) {
        eprintln!("{}", "❌ Invalid tutorial slug format!".red());
        eprintln!("{}", format!("Error: {}", e).red());
        eprintln!("{}", "ℹ️  Slug must be lowercase, with words separated by dashes.".cyan());
        eprintln!("{}", "ℹ️  Examples: \"my-tutorial\", \"add-nft-pallet\", \"zero-to-hero\"".cyan());
        std::process::exit(1);
    }

    // Validate working directory
    if let Err(e) = polkadot_cookbook_core::config::validate_working_directory() {
        eprintln!("{}", "❌ Invalid working directory!".red());
        eprintln!("{}", format!("Error: {}", e).red());
        eprintln!("{}", "ℹ️  Please run this command from the repository root.".cyan());
        std::process::exit(1);
    }

    println!("{}\n", format!("Creating tutorial: {}", cli.slug).cyan());

    // Create project configuration
    let config = ProjectConfig::new(&cli.slug)
        .with_destination(PathBuf::from("tutorials"))
        .with_git_init(!cli.no_git)
        .with_skip_install(cli.skip_install);

    // Create the project using the core library
    let scaffold = Scaffold::new();
    match scaffold.create_project(config).await {
        Ok(project_info) => {
            println!("\n{}", "============================================================".green());
            println!("{}", "🎉 Tutorial created successfully!".green());
            println!("{}", "============================================================\n".green());

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
            println!("     cd {} && npm test", project_info.project_path.display());
            println!();
            println!("{}", "  5. Update tutorial.config.yml metadata:".cyan());
            println!("     {}/tutorial.config.yml", project_info.project_path.display());
            println!();
            println!("{}", "  6. When ready, open a Pull Request:".cyan());
            println!("     git add -A");
            println!("     git commit -m \"feat(tutorial): add {}\"", project_info.slug);
            if let Some(branch) = project_info.git_branch {
                println!("     git push origin {}", branch);
            }
            println!();

            println!("{}", "📚 Need help? Check CONTRIBUTING.md or open an issue!\n".blue());
        }
        Err(e) => {
            eprintln!("{}", "❌ Failed to create tutorial!".red());
            eprintln!("{}", format!("Error: {}", e).red());
            std::process::exit(1);
        }
    }

    Ok(())
}
