//! Example demonstrating version resolution functionality
//!
//! This example shows how to use the version management API to load and resolve
//! tutorial-specific version configurations.
//!
//! Run with:
//! ```bash
//! cargo run --example version_resolution
//! ```

use polkadot_cookbook_core::version::{
    load_global_versions, resolve_tutorial_versions, VersionSource,
};
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt().with_env_filter("info").init();

    println!("=== Polkadot Cookbook Version Resolution Example ===\n");

    // Example 1: Load global versions only
    println!("1. Loading global versions...");
    let repo_root = Path::new(".");

    match load_global_versions(repo_root).await {
        Ok(global) => {
            println!("   ✓ Global versions loaded:");
            for (name, version) in &global.versions {
                println!("     - {}: {}", name, version);
            }
        }
        Err(e) => {
            println!("   ✗ Failed to load global versions: {}", e);
            println!("   (This is expected if not run from the repository root)");
        }
    }

    println!();

    // Example 2: Resolve versions for a specific tutorial
    println!("2. Resolving versions for a tutorial...");

    // Check if there are any tutorials
    let tutorials_dir = repo_root.join("tutorials");
    if tutorials_dir.exists() {
        if let Ok(mut entries) = tokio::fs::read_dir(&tutorials_dir).await {
            if let Some(entry) = entries.next_entry().await? {
                let tutorial_name = entry.file_name();
                let tutorial_slug = tutorial_name.to_string_lossy().to_string();

                println!("   Checking tutorial: {}", tutorial_slug);

                match resolve_tutorial_versions(repo_root, &tutorial_slug).await {
                    Ok(resolved) => {
                        println!("   ✓ Resolved versions:");
                        for (name, version) in &resolved.versions {
                            let source = match resolved.get_source(name) {
                                Some(VersionSource::Global) => "global",
                                Some(VersionSource::Tutorial) => "tutorial",
                                None => "unknown",
                            };
                            println!("     - {}: {} (from {})", name, version, source);
                        }
                    }
                    Err(e) => {
                        println!("   ✗ Failed to resolve versions: {}", e);
                    }
                }
            } else {
                println!("   No tutorials found in {}", tutorials_dir.display());
            }
        }
    } else {
        println!(
            "   Tutorials directory not found: {}",
            tutorials_dir.display()
        );
    }

    println!("\n=== Example Complete ===");

    Ok(())
}
