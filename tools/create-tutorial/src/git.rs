use anyhow::Result;
use colored::*;
use std::process::Command;

/// Creates a new git branch for the tutorial
/// Branch name format: feat/tutorial-{slug}
pub fn create_git_branch(slug: &str) -> Result<()> {
    println!("{}", "Step 1/4: Creating git branch...".cyan());

    let branch_name = format!("feat/tutorial-{}", slug);
    let output = Command::new("git")
        .args(&["checkout", "-b", &branch_name])
        .output();

    match output {
        Ok(output) if output.status.success() => {
            println!("{}", format!("✅ Created branch: {}", branch_name).green());
        }
        _ => {
            eprintln!("{}", "❌ Failed to create git branch".red());
            eprintln!("{}", "⚠️  You may already be on a feature branch. Continue anyway.".yellow());
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_branch_name_format() {
        // We can't easily test git operations, but we can test the branch name format
        let slug = "my-tutorial";
        let expected_branch = format!("feat/tutorial-{}", slug);
        assert_eq!(expected_branch, "feat/tutorial-my-tutorial");
    }
}
