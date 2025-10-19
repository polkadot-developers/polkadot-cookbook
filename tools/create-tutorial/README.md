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

## Project Structure

The codebase is modular and fully tested:

```text
src/
├── main.rs          # CLI entry point
├── lib.rs           # Library exports
├── validator.rs     # Slug and directory validation
├── git.rs          # Git branch operations
├── scaffold.rs     # Directory and file scaffolding
├── bootstrap.rs    # Test environment setup
└── templates/      # All template generators
    ├── mod.rs
    ├── justfile.rs
    ├── test.rs
    ├── readme.rs
    └── tutorial_yml.rs

tests/
└── e2e_test.rs     # End-to-end integration tests
```

## Testing

The project includes comprehensive tests:

- **Unit tests** - Test individual functions in each module
- **Template tests** - Verify template generation
- **E2E tests** - Full workflow tests using temporary directories

Run tests:

```bash
# Run all tests
cargo test

# Run only library unit tests
cargo test --lib

# Run only integration tests
cargo test --test e2e_test
```

## Dependencies

- **clap** - Command-line argument parsing
- **colored** - Terminal colors
- **regex** - Slug validation
- **anyhow** - Error handling
- **serde_yaml** - (future) YAML parsing for versions

**Dev Dependencies:**

- **tempfile** - Temporary directories for E2E tests

## Contributing

When adding new features:

1. Add the functionality in the appropriate module
2. Write unit tests in the same file
3. Update E2E tests if needed
4. Run `cargo test` to ensure all tests pass
5. Run `cargo clippy` for linting

## License

MIT
