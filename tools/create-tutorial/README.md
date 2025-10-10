# create-tutorial 🦀

Rust CLI tool for creating new Polkadot Cookbook tutorials.

## Features

- **One-command setup** – Generates the entire tutorial structure automatically
- **Creates a new Git branch** – Starts a clean branch for your tutorial (e.g. `feat/tutorial-my-guide`)
- **Scaffolds folder layout** – Adds `tutorial.yml`, `README.md`, `tests/`, `scripts/`, and sample code folders
- **Bootstraps test environment** – Installs and configures Vitest, TypeScript, and @polkadot/api
- **Validates tutorial slug** – Prevents naming conflicts or invalid slugs
- **Guides you through next steps** – Prints what to do next once setup is done

## Building

From the repository root:

```bash
cd tools/create-tutorial
cargo build --release
```

The binary will be at `target/release/create-tutorial`.

## Usage

From the repository root:

```bash
# Via npm script (recommended)
npm run create-tutorial my-tutorial

# Or directly
./tools/create-tutorial/target/release/create-tutorial my-tutorial
```

## What it does

1. Creates git branch: `feat/tutorial-{slug}`
2. Scaffolds folder structure with all required files
3. Bootstraps test environment (vitest, TypeScript, @polkadot/api)
4. Installs npm dependencies
5. Shows clear next steps

## Dependencies

- **clap** - Command-line argument parsing
- **colored** - Terminal colors
- **regex** - Slug validation
- **anyhow** - Error handling
- **serde_yaml** - (future) YAML parsing for versions

## License

MIT
