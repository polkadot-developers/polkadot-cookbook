//! Example demonstrating version resolution functionality
//!
//! This example shows how to use the version management API to load and resolve
//! tutorial-specific version configurations.
//!
//! Run with:
//! ```bash
//! cargo run --example version_resolution
//! ```

use cliclack::{intro, note, outro, spinner};
use polkadot_cookbook_core::version::{
    load_global_versions, resolve_tutorial_versions, VersionSource,
};
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt().with_env_filter("info").init();

    intro("📦 Polkadot Cookbook Version Resolution Example")?;

    // Example 1: Load global versions only
    let sp = spinner();
    sp.start("Loading global versions...");
    let repo_root = Path::new(".");

    match load_global_versions(repo_root).await {
        Ok(global) => {
            sp.stop("✅ Global versions loaded");

            let mut output = String::new();
            for (name, version) in &global.versions {
                output.push_str(&format!("{}: {}\n", name, version));
            }
            note("Global Versions", output.trim_end())?;
        }
        Err(e) => {
            sp.stop(format!("⚠️  Failed to load: {}", e));
            note(
                "Note",
                "This is expected if not run from the repository root",
            )?;
        }
    }

    // Example 2: Resolve versions for a specific tutorial
    let tutorials_dir = repo_root.join("tutorials");
    if tutorials_dir.exists() {
        if let Ok(mut entries) = tokio::fs::read_dir(&tutorials_dir).await {
            if let Some(entry) = entries.next_entry().await? {
                let tutorial_name = entry.file_name();
                let tutorial_slug = tutorial_name.to_string_lossy().to_string();

                let sp = spinner();
                sp.start(format!("Resolving versions for '{}'...", tutorial_slug));

                match resolve_tutorial_versions(repo_root, &tutorial_slug).await {
                    Ok(resolved) => {
                        sp.stop("✅ Versions resolved with overrides");

                        let mut output = String::new();
                        for (name, version) in &resolved.versions {
                            let source = match resolved.get_source(name) {
                                Some(VersionSource::Global) => "global",
                                Some(VersionSource::Tutorial) => "tutorial override",
                                None => "unknown",
                            };
                            output.push_str(&format!("{}: {} ({})\n", name, version, source));
                        }
                        note(format!("Tutorial: {}", tutorial_slug), output.trim_end())?;
                    }
                    Err(e) => {
                        sp.stop(format!("⚠️  Failed: {}", e));
                    }
                }
            } else {
                note(
                    "Note",
                    format!("No tutorials found in {}", tutorials_dir.display()),
                )?;
            }
        }
    } else {
        note(
            "Note",
            format!("Tutorials directory not found: {}", tutorials_dir.display()),
        )?;
    }

    outro("🎉 Example complete! Check the CLI for more features.")?;

    Ok(())
}
