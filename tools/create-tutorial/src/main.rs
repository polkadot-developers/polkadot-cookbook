use anyhow::Result;
use clap::Parser;
use colored::*;
use std::path::PathBuf;

use create_tutorial::{bootstrap, git, scaffold, validator};

#[derive(Parser)]
#[command(name = "create-tutorial")]
#[command(about = "Create a new Polkadot Cookbook tutorial", long_about = None)]
struct Cli {
    /// Tutorial slug (e.g., "my-tutorial")
    #[arg(value_name = "SLUG")]
    slug: String,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    println!("\n{}\n", "🚀 Polkadot Cookbook - Tutorial Creator".blue().bold());

    // Validate working directory
    validator::validate_working_directory()?;

    // Validate slug format
    if !validator::is_valid_slug(&cli.slug) {
        eprintln!("{}", "❌ Invalid tutorial slug format!".red());
        eprintln!("{}", "ℹ️  Slug must be lowercase, with words separated by dashes.".cyan());
        eprintln!("{}", "ℹ️  Examples: \"my-tutorial\", \"add-nft-pallet\", \"zero-to-hero\"".cyan());
        std::process::exit(1);
    }

    // Check if tutorial already exists
    let tutorial_dir = PathBuf::from("tutorials").join(&cli.slug);
    if tutorial_dir.exists() {
        eprintln!("{}", format!("❌ Tutorial \"{}\" already exists!", cli.slug).red());
        eprintln!("{}", format!("ℹ️  Directory: {}", tutorial_dir.display()).cyan());
        std::process::exit(1);
    }

    println!("{}\n", format!("Creating tutorial: {}", cli.slug).cyan());

    // Step 1: Create git branch
    git::create_git_branch(&cli.slug)?;

    // Step 2: Scaffold structure
    scaffold::scaffold_structure(&cli.slug)?;

    // Step 3: Bootstrap tests
    bootstrap::bootstrap_tests(&cli.slug)?;

    // Step 4: Verify setup
    scaffold::verify_setup(&cli.slug)?;

    // Success message
    scaffold::print_success_message(&cli.slug);

    Ok(())
}
