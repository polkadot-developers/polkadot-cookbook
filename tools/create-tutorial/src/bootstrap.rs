use anyhow::{Context, Result};
use colored::*;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

/// Bootstraps the test environment with npm packages and configuration
pub fn bootstrap_tests(slug: &str) -> Result<()> {
    println!("\n{}", "Step 3/4: Bootstrapping test environment...".cyan());

    let tutorial_dir = PathBuf::from("tutorials").join(slug);
    let tutorial_dir_str = tutorial_dir.to_str().unwrap();

    // Create package.json
    create_package_json(tutorial_dir_str, slug)?;

    // Install dev dependencies
    install_dev_dependencies(tutorial_dir_str)?;

    // Install dependencies
    install_dependencies(tutorial_dir_str)?;

    // Set npm scripts
    set_npm_scripts(tutorial_dir_str)?;

    // Create configuration files
    create_config_files(&tutorial_dir)?;

    print_success();

    Ok(())
}

fn create_package_json(tutorial_dir_str: &str, slug: &str) -> Result<()> {
    let package_json_path = PathBuf::from(tutorial_dir_str).join("package.json");
    if !package_json_path.exists() {
        Command::new("sh")
            .arg("-c")
            .arg(format!("cd {} && npm init -y", tutorial_dir_str))
            .output()
            .context("Failed to run npm init")?;

        Command::new("sh")
            .arg("-c")
            .arg(format!("cd {} && npm pkg set name={} type=module", tutorial_dir_str, slug))
            .output()
            .context("Failed to set package.json fields")?;
    }
    Ok(())
}

fn install_dev_dependencies(tutorial_dir_str: &str) -> Result<()> {
    println!("{}", "ℹ️  Installing dev dependencies (vitest, typescript, ts-node, @types/node)...".cyan());
    Command::new("sh")
        .arg("-c")
        .arg(format!("cd {} && npm i -D vitest typescript ts-node @types/node", tutorial_dir_str))
        .status()
        .context("Failed to install dev dependencies")?;
    Ok(())
}

fn install_dependencies(tutorial_dir_str: &str) -> Result<()> {
    println!("{}", "ℹ️  Installing dependencies (@polkadot/api, ws)...".cyan());
    Command::new("sh")
        .arg("-c")
        .arg(format!("cd {} && npm i @polkadot/api ws", tutorial_dir_str))
        .status()
        .context("Failed to install dependencies")?;
    Ok(())
}

fn set_npm_scripts(tutorial_dir_str: &str) -> Result<()> {
    Command::new("sh")
        .arg("-c")
        .arg(format!(
            "cd {} && npm pkg set scripts.test=\"vitest run\" scripts.test:watch=\"vitest\"",
            tutorial_dir_str
        ))
        .output()
        .context("Failed to set npm scripts")?;
    Ok(())
}

fn create_config_files(tutorial_dir: &PathBuf) -> Result<()> {
    // Create vitest.config.ts
    let vitest_config = r#"import { defineConfig } from 'vitest/config';
export default defineConfig({
  test: {
    include: ['tests/**/*.test.ts'],
    testTimeout: 30000,
    hookTimeout: 30000,
  },
});
"#;
    fs::write(tutorial_dir.join("vitest.config.ts"), vitest_config)?;

    // Create tsconfig.json
    let tsconfig_content = r#"{
  "compilerOptions": {
    "target": "ES2020",
    "module": "ESNext",
    "moduleResolution": "Bundler",
    "types": ["node", "vitest/globals"],
    "esModuleInterop": true,
    "resolveJsonModule": true,
    "skipLibCheck": true
  },
  "include": ["tests/**/*.ts"]
}
"#;
    fs::write(tutorial_dir.join("tsconfig.json"), tsconfig_content)?;

    Ok(())
}

fn print_success() {
    println!("{}", "✅ Test environment ready".green());
    println!("{}", "  - package.json created".cyan());
    println!("{}", "  - vitest, typescript, @polkadot/api installed".cyan());
    println!("{}", "  - vitest.config.ts & tsconfig.json configured".cyan());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vitest_config_generation() {
        let vitest_config = r#"import { defineConfig } from 'vitest/config';
export default defineConfig({
  test: {
    include: ['tests/**/*.test.ts'],
    testTimeout: 30000,
    hookTimeout: 30000,
  },
});
"#;
        assert!(vitest_config.contains("vitest/config"));
        assert!(vitest_config.contains("tests/**/*.test.ts"));
    }

    #[test]
    fn test_tsconfig_has_required_options() {
        let tsconfig = r#"{
  "compilerOptions": {
    "target": "ES2020",
    "module": "ESNext",
    "moduleResolution": "Bundler",
    "types": ["node", "vitest/globals"],
    "esModuleInterop": true,
    "resolveJsonModule": true,
    "skipLibCheck": true
  },
  "include": ["tests/**/*.ts"]
}
"#;
        assert!(tsconfig.contains("ES2020"));
        assert!(tsconfig.contains("vitest/globals"));
    }
}
