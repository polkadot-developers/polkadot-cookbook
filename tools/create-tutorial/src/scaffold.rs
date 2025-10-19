use anyhow::Result;
use colored::*;
use std::fs;
use std::path::PathBuf;

use crate::templates;
use crate::validator::slug_to_title;

/// Scaffolds the complete tutorial directory structure
pub fn scaffold_structure(slug: &str) -> Result<()> {
    println!("\n{}", "Step 2/4: Scaffolding tutorial structure...".cyan());

    let tutorial_dir = PathBuf::from("tutorials").join(slug);

    create_directories(&tutorial_dir, slug)?;

    create_files(&tutorial_dir, slug)?;

    print_success(slug);

    Ok(())
}

fn create_directories(tutorial_dir: &PathBuf, slug: &str) -> Result<()> {
    fs::create_dir_all(tutorial_dir.join("tests"))?;
    fs::create_dir_all(tutorial_dir.join("scripts"))?;
    fs::create_dir_all(tutorial_dir.join("src"))?;
    Ok(())
}

fn create_files(tutorial_dir: &PathBuf, slug: &str) -> Result<()> {
    // Create justfile
    fs::write(tutorial_dir.join("justfile"), templates::generate_justfile())?;

    // Create example test
    let test_content = templates::generate_test(slug);
    fs::write(
        tutorial_dir.join("tests").join(format!("{}-e2e.test.ts", slug)),
        test_content,
    )?;

    // Create tutorial.config.yml
    let title = slug_to_title(slug);
    let tutorial_yml_content = templates::generate_tutorial_yml(slug, &title);
    fs::write(tutorial_dir.join("tutorial.config.yml"), tutorial_yml_content)?;

    // Create README.md
    let readme_content = templates::generate_readme(slug);
    fs::write(tutorial_dir.join("README.md"), readme_content)?;

    // Create .gitkeep in scripts/
    fs::write(tutorial_dir.join("scripts").join(".gitkeep"), "")?;

    // Create .gitignore
    let gitignore_content = r#"node_modules/
dist/
*.log
.DS_Store
coverage/
"#;
    fs::write(tutorial_dir.join(".gitignore"), gitignore_content)?;

    Ok(())
}

fn print_success(slug: &str) {
    println!("{}", "✅ Scaffolded folder structure".green());
    println!("{}", format!("  - tutorials/{}/README.md", slug).cyan());
    println!("{}", format!("  - tutorials/{}/tutorial.config.yml", slug).cyan());
    println!("{}", format!("  - tutorials/{}/tests/{}-e2e.test.ts", slug, slug).cyan());
    println!("{}", format!("  - tutorials/{}/src/", slug).cyan());
}

/// Verifies that all required files were created successfully
pub fn verify_setup(slug: &str) -> Result<()> {
    println!("\n{}", "Step 4/4: Verifying setup...".cyan());

    let tutorial_dir = PathBuf::from("tutorials").join(slug);
    let package_json = tutorial_dir.join("package.json");
    let readme = tutorial_dir.join("README.md");

    if package_json.exists() && readme.exists() {
        println!("{}", "✅ All files created successfully!".green());
    } else {
        eprintln!("{}", "⚠️  Some files may be missing. Please check the tutorial directory.".yellow());
    }

    Ok(())
}

/// Prints the success message with next steps
pub fn print_success_message(slug: &str) {
    println!("\n{}", "============================================================".green());
    println!("{}", "🎉 Tutorial created successfully!".green());
    println!("{}", "============================================================\n".green());

    println!("{}", "📝 Next Steps:".yellow());
    println!();
    println!("{}", "  1. Write your tutorial content:".cyan());
    println!("     tutorials/{}/README.md", slug);
    println!();
    println!("{}", "  2. Add your code implementation:".cyan());
    println!("     tutorials/{}/src/", slug);
    println!();
    println!("{}", "  3. Write comprehensive tests:".cyan());
    println!("     tutorials/{}/tests/", slug);
    println!();
    println!("{}", "  4. Run tests to verify:".cyan());
    println!("     cd tutorials/{} && npm test", slug);
    println!();
    println!("{}", "  5. Update tutorial.config.yml metadata:".cyan());
    println!("     tutorials/{}/tutorial.config.yml", slug);
    println!();
    println!("{}", "  6. When ready, open a Pull Request:".cyan());
    println!("     git add -A");
    println!("     git commit -m \"feat(tutorial): add {}\"", slug);
    println!("     git push origin feat/tutorial-{}", slug);
    println!();

    println!("{}", "📚 Need help? Check CONTRIBUTING.md or open an issue!\n".blue());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_directory_paths() {
        let tutorial_dir = PathBuf::from("tutorials").join("my-tutorial");
        assert_eq!(
            tutorial_dir.join("tests"),
            PathBuf::from("tutorials/my-tutorial/tests")
        );
    }
}
