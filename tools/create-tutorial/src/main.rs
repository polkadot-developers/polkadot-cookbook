use anyhow::{Context, Result};
use clap::Parser;
use colored::*;
use regex::Regex;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

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
    validate_working_directory()?;

    // Validate slug format
    if !is_valid_slug(&cli.slug) {
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
    create_git_branch(&cli.slug)?;

    // Step 2: Scaffold structure
    scaffold_structure(&cli.slug)?;

    // Step 3: Bootstrap tests
    bootstrap_tests(&cli.slug)?;

    // Step 4: Verify setup
    verify_setup(&cli.slug)?;

    // Success message
    print_success_message(&cli.slug);

    Ok(())
}

fn validate_working_directory() -> Result<()> {
    if !Path::new("tutorials").exists() {
        anyhow::bail!("This script must be run from the repository root!\nExpected directory structure: ./tutorials/, ./utils/, etc.");
    }

    if !Path::new("versions.yml").exists() {
        anyhow::bail!("versions.yml not found. Are you in the correct repository?");
    }

    Ok(())
}

fn is_valid_slug(slug: &str) -> bool {
    let slug_regex = Regex::new(r"^[a-z0-9]+(-[a-z0-9]+)*$").unwrap();
    slug_regex.is_match(slug)
}

fn create_git_branch(slug: &str) -> Result<()> {
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

fn scaffold_structure(slug: &str) -> Result<()> {
    println!("\n{}", "Step 2/4: Scaffolding tutorial structure...".cyan());

    let tutorial_dir = PathBuf::from("tutorials").join(slug);

    // Create directories
    fs::create_dir_all(tutorial_dir.join("tests"))?;
    fs::create_dir_all(tutorial_dir.join("scripts"))?;
    fs::create_dir_all(tutorial_dir.join(format!("{}-code", slug)))?;

    // Create justfile
    let justfile_content = r#"default:
  @just --list

say-hello:
  echo "Hello, world!"
"#;
    fs::write(tutorial_dir.join("justfile"), justfile_content)?;

    // Create example test
    let test_content = format!(r#"import {{ describe, it, expect }} from 'vitest';
import {{ ApiPromise, WsProvider }} from '@polkadot/api';
import net from 'node:net';

async function isPortReachable(host: string, port: number, timeoutMs: number): Promise<boolean> {{
  return new Promise((resolve) => {{
    const socket = new net.Socket();
    const done = (ok: boolean) => {{ try {{ socket.destroy(); }} catch {{}} ; resolve(ok); }};
    socket.setTimeout(timeoutMs);
    socket.once('error', () => done(false));
    socket.once('timeout', () => done(false));
    socket.connect(port, host, () => done(true));
  }});
}}

describe('{} e2e', () => {{
  it('connects and reads chain info', async () => {{
    const endpoint = process.env.POLKADOT_WS || 'ws://127.0.0.1:9944';
    const {{ hostname, port }} = new URL(endpoint.replace('ws://', 'http://'));
    if (!(await isPortReachable(hostname, Number(port || 9944), 1000))) {{
      console.log('⏭️  Skipping test - node not available');
      return;
    }}

    const api = await ApiPromise.create({{ provider: new WsProvider(endpoint, 1) }});
    const header = await api.rpc.chain.getHeader();
    expect(header.number.toNumber()).toBeGreaterThanOrEqual(0);
    await api.disconnect();
  }});
}});
"#, slug);

    fs::write(tutorial_dir.join("tests").join(format!("{}-e2e.test.ts", slug)), test_content)?;

    // Create tutorial.yml
    let title = slug.split('-')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ");

    let tutorial_yml_content = format!(r#"name: {}
slug: {}
category: polkadot-sdk-cookbook
needs_node: true
description: Replace with a short description.
type: sdk # or contracts
"#, title, slug);

    fs::write(tutorial_dir.join("tutorial.yml"), tutorial_yml_content)?;

    // Create README.md
    let readme_content = format!(r#"# {}

Describe the goal, prerequisites, and step-by-step instructions for this tutorial.

## Prerequisites

- Rust `1.86+` (check with `rustc --version`)
- Node.js `20+` (check with `node --version`)
- Basic knowledge of Polkadot SDK

## Steps

1. **Setup environment**
   ```bash
   cd tutorials/{}
   npm install
   ```

2. **Build the project**
   ```bash
   # Add your build commands here
   ```

3. **Run tests**
   ```bash
   npm run test
   ```

## Testing

To run the end-to-end tests:

```bash
cd tutorials/{}
npm run test
```

## Next Steps

- Add your implementation code to `{}-code/`
- Write comprehensive tests in `tests/`
- Update this README with detailed instructions
"#, slug, slug, slug, slug);

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

    println!("{}", "✅ Scaffolded folder structure".green());
    println!("{}", format!("  - tutorials/{}/README.md", slug).cyan());
    println!("{}", format!("  - tutorials/{}/tutorial.yml", slug).cyan());
    println!("{}", format!("  - tutorials/{}/tests/{}-e2e.test.ts", slug, slug).cyan());
    println!("{}", format!("  - tutorials/{}/{}-code/", slug, slug).cyan());

    Ok(())
}

fn bootstrap_tests(slug: &str) -> Result<()> {
    println!("\n{}", "Step 3/4: Bootstrapping test environment...".cyan());

    let tutorial_dir = PathBuf::from("tutorials").join(slug);
    let tutorial_dir_str = tutorial_dir.to_str().unwrap();

    // Create package.json
    let package_json_path = tutorial_dir.join("package.json");
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

    // Install dev dependencies
    println!("{}", "ℹ️  Installing dev dependencies (vitest, typescript, ts-node, @types/node)...".cyan());
    Command::new("sh")
        .arg("-c")
        .arg(format!("cd {} && npm i -D vitest typescript ts-node @types/node", tutorial_dir_str))
        .status()
        .context("Failed to install dev dependencies")?;

    // Install dependencies
    println!("{}", "ℹ️  Installing dependencies (@polkadot/api, ws)...".cyan());
    Command::new("sh")
        .arg("-c")
        .arg(format!("cd {} && npm i @polkadot/api ws", tutorial_dir_str))
        .status()
        .context("Failed to install dependencies")?;

    // Set npm scripts
    Command::new("sh")
        .arg("-c")
        .arg(format!(
            "cd {} && npm pkg set scripts.test=\"vitest run\" scripts.test:watch=\"vitest\"",
            tutorial_dir_str
        ))
        .output()
        .context("Failed to set npm scripts")?;

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

    println!("{}", "✅ Test environment ready".green());
    println!("{}", "  - package.json created".cyan());
    println!("{}", "  - vitest, typescript, @polkadot/api installed".cyan());
    println!("{}", "  - vitest.config.ts & tsconfig.json configured".cyan());

    Ok(())
}

fn verify_setup(slug: &str) -> Result<()> {
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

fn print_success_message(slug: &str) {
    println!("\n{}", "============================================================".green());
    println!("{}", "🎉 Tutorial created successfully!".green());
    println!("{}", "============================================================\n".green());

    println!("{}", "📝 Next Steps:".yellow());
    println!();
    println!("{}", "  1. Write your tutorial content:".cyan());
    println!("     tutorials/{}/README.md", slug);
    println!();
    println!("{}", "  2. Add your code implementation:".cyan());
    println!("     tutorials/{}/{}-code/", slug, slug);
    println!();
    println!("{}", "  3. Write comprehensive tests:".cyan());
    println!("     tutorials/{}/tests/", slug);
    println!();
    println!("{}", "  4. Run tests to verify:".cyan());
    println!("     cd tutorials/{} && npm test", slug);
    println!();
    println!("{}", "  5. Update tutorial.yml metadata:".cyan());
    println!("     tutorials/{}/tutorial.yml", slug);
    println!();
    println!("{}", "  6. When ready, open a Pull Request:".cyan());
    println!("     git add -A");
    println!("     git commit -m \"feat(tutorial): add {}\"", slug);
    println!("     git push origin feat/tutorial-{}", slug);
    println!();

    println!("{}", "📚 Need help? Check CONTRIBUTING.md or open an issue!\n".blue());
}
