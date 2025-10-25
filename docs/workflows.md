# GitHub Workflows Documentation

This document describes the automated workflows used in the Polkadot Cookbook repository.

## Overview

The repository uses four main GitHub Actions workflows:

1. **command-generate-scripts.yml** - PR comment command handler
2. **generate-scripts.yml** - Script generation and release workflow
3. **test-recipes.yml** - PR recipe testing
4. **build-kitchensink-parachain.yml** - Parachain build workflow

## Workflow Diagrams

### Script Generation Flow

```mermaid
graph TD
    A[PR Comment: /generate-scripts] --> B{Permission Check}
    B -->|No Write Access| C[❌ Comment: Insufficient permissions]
    B -->|Has Write Access| D[Extract PR Info]

    D --> E{Recipe Slug Provided?}
    E -->|No| F[Analyze PR Changed Files]
    F --> G{Single Recipe Changed?}
    G -->|Yes| H[Use Detected Slug]
    G -->|No| I[❌ Comment: Ambiguous slug]
    E -->|Yes| H

    H --> J[Dispatch generate-scripts.yml]
    J --> K[✅ Comment: Workflow dispatched]

    K --> L[Check Version Changes in versions.yml]
    L --> M{Versions Changed?}
    M -->|No & Not Manual| N[⏭️ Skip - No changes detected]
    M -->|Yes or Manual| O[Build Parachain]

    O --> P[Generate Recipe Scripts]
    P --> Q[Create Versioned Scripts]
    Q --> R[setup-rust.sh<br/>install-chain-spec-builder.sh<br/>install-omni-node.sh<br/>generate-chain-spec.sh<br/>start-node.sh]

    R --> S[Commit Scripts to Repo]
    S --> T{Scripts Changed?}
    T -->|No| U[No Commit Needed]
    T -->|Yes| V[Create Git Commit]

    V --> W[Create Recipe Tag]
    W --> X[Tag Format:<br/>recipe/SLUG/vYYYYMMDD-HHMMSS]

    X --> Y{create_release=true?}
    Y -->|Yes| Z[Create GitHub Release]
    Y -->|No| AA[Tag Only - No Release]

    style A fill:#e1f5ff
    style K fill:#d4edda
    style Q fill:#fff3cd
    style V fill:#d4edda
    style X fill:#d1ecf1
    style Z fill:#d4edda
```

### PR Testing Flow

```mermaid
graph TD
    A[Pull Request Created/Updated] --> B{Changed Files in recipes/?}
    B -->|No| C[⏭️ Skip - No recipe changes]
    B -->|Yes| D[Detect New Recipes]

    D --> E{New Recipe Folder Added?}
    E -->|No| F[⏭️ Skip - Only modifications]
    E -->|Yes| G[Extract Recipe Slug]

    G --> H[Read recipe.config.yml]
    H --> I{needs_node: true?}
    I -->|No| J[Install npm deps only]
    I -->|Yes| K[Setup Rust Toolchain]

    K --> L[Install chain-spec-builder]
    L --> M[Install polkadot-omni-node]
    M --> N[Build kitchensink-parachain]
    N --> O[Generate Chain Spec]
    O --> P[Start polkadot-omni-node]

    J --> Q[Install Recipe Dependencies]
    P --> Q

    Q --> R[Run npm test]
    R --> S{Tests Pass?}
    S -->|Yes| T[✅ PR Check Success]
    S -->|No - Fast Skip| U[✅ PR Check Success<br/>Tests skipped - no node]
    S -->|No - Real Failure| V[❌ PR Check Failed]

    style A fill:#e1f5ff
    style T fill:#d4edda
    style U fill:#fff3cd
    style V fill:#f8d7da
```

## Workflow Details

### 1. command-generate-scripts.yml

**Trigger**: PR comment containing `/generate-scripts` or `/generate-release`

**Purpose**: Allows maintainers to trigger script generation for a recipe via PR comments

**How it works**:

1. **Permission Check**: Verifies the comment author has write access to the repository
2. **Command Parsing**: Extracts command and optional parameters:
   - `slug=<recipe-slug>` - Specify which recipe
   - `key=<recipe-key>` - versions.yml key (default: zero_to_hero)
   - `force=1` - Force generation even if no version changes
3. **Slug Detection**: If no slug provided, analyzes PR changed files to detect the recipe
4. **Workflow Dispatch**: Triggers `generate-scripts.yml` with the resolved parameters
5. **Acknowledgment**: Posts a comment confirming the dispatch

**Usage Examples**:

```bash
# Auto-detect recipe from PR changes
/generate-scripts

# Specify recipe explicitly
/generate-scripts slug=my-recipe

# Force generation regardless of version changes
/generate-scripts slug=my-recipe force=1

# Generate and create a GitHub release
/generate-release slug=my-recipe

# Override versions.yml key
/generate-scripts slug=my-recipe key=my_custom_key
```

**Key Differences**:

- `/generate-scripts` - Creates tag only
- `/generate-release` - Creates tag AND GitHub release

### 2. generate-scripts.yml

**Triggers**:
- Workflow dispatch (manual or from command-generate-scripts.yml)
- Push to master/dev with changes to `versions.yml`

**Purpose**: Generates recipe-specific scripts with pinned versions and optionally creates releases

**Inputs**:
- `recipe_key` (default: zero_to_hero) - Key in versions.yml
- `recipe_slug` (default: zero-to-hero) - Recipe folder name
- `create_release` (default: false) - Whether to create GitHub release
- `force_generation` (default: false) - Force generation without version check

**How it works**:

1. **Version Change Detection**:
   - Reads `versions.yml` for the specified recipe_key
   - Compares with previous commit to detect changes in:
     - `rust` version
     - `chain_spec_builder` version
     - `polkadot_omni_node` version
   - Skips if no changes (unless forced or manual trigger)

2. **Parachain Build** (if versions changed):
   - Uses `build-kitchensink-parachain.yml` workflow
   - Builds with resolved Rust version
   - Caches build artifacts

3. **Script Generation**:
   - Creates `recipes/<slug>/scripts/` directory
   - Generates pinned setup scripts:
     ```bash
     setup-rust.sh                       # Rust version setup
     install-chain-spec-builder.sh       # Tool installation
     install-omni-node.sh                # Tool installation
     generate-chain-spec.sh              # Chain spec creation
     start-node.sh                       # Node startup
     ```
   - All scripts use exact versions from `versions.yml`

4. **Commit & Push**:
   - Commits generated scripts to the repository
   - Only commits if scripts changed
   - Uses `github-actions[bot]` as committer

5. **Tagging**:
   - Creates recipe-specific tag: `recipe/<slug>/vYYYYMMDD-HHMMSS`
   - If tag exists, appends short commit SHA: `recipe/<slug>/vYYYYMMDD-HHMMSS-abc1234`
   - Tag message includes all resolved versions

6. **Release** (optional):
   - If `create_release=true`, creates GitHub release with tag
   - Release body includes commit SHA and all tool versions

**Outputs**:
- `rust-version` - Resolved Rust version
- `chain-spec-builder-version` - Resolved chain-spec-builder version
- `omni-node-version` - Resolved omni-node version
- `scripts-committed` - Whether scripts were committed
- `commit-sha` - Commit SHA of the script commit
- `recipe-slug` - Recipe slug processed

### 3. test-recipes.yml

**Trigger**: Pull request with changes to `recipes/**` (excluding `scripts/`)

**Purpose**: Runs tests for newly added recipes

**How it works**:

1. **Find Changed Recipes**:
   - Compares PR base and head commits
   - Identifies NEWLY ADDED recipe folders (not modifications)
   - Returns list of recipe slugs to test

2. **Read Recipe Requirements**:
   - Reads `recipe.config.yml` for each recipe
   - Checks `needs_node` field to determine if node setup is required

3. **Setup Environment** (if `needs_node: true`):
   - Installs Rust toolchain (from `versions.yml`)
   - Installs chain-spec-builder (from `versions.yml`)
   - Installs polkadot-omni-node (from `versions.yml`)
   - Builds kitchensink-parachain runtime
   - Generates chain specification
   - Starts polkadot-omni-node in background

4. **Run Tests**:
   - Installs recipe npm dependencies
   - Runs `npm run test --silent`
   - Tests should implement fast-skip pattern if no node available

5. **Pass Criteria**:
   - Tests pass
   - OR tests skip gracefully (fast-skip pattern)

**Strategy**: Matrix strategy - runs tests for each detected recipe in parallel

### 4. build-kitchensink-parachain.yml

**Trigger**: Called by `generate-scripts.yml` as a reusable workflow

**Purpose**: Build the kitchensink parachain runtime with specific Rust version

**Inputs**:
- `recipe_key` - versions.yml key to read Rust version from

**How it works**:

1. Reads Rust version from `versions.yml` for the specified recipe
2. Sets up Rust toolchain with that version
3. Adds wasm32-unknown-unknown target
4. Builds kitchensink-parachain with `cargo build --release`
5. Uses Swatinem/rust-cache for faster subsequent builds

**Outputs**: Compiled WASM runtime in `target/release/wbuild/`

## Workflow Relationships

```mermaid
graph LR
    A[PR Comment] --> B[command-generate-scripts.yml]
    B --> C[generate-scripts.yml]
    C --> D[build-kitchensink-parachain.yml]

    E[PR with recipe changes] --> F[test-recipes.yml]
    F --> G[Uses kitchensink build steps]

    H[Push to master<br/>versions.yml changes] --> C

    style A fill:#e1f5ff
    style E fill:#e1f5ff
    style H fill:#e1f5ff
```

## Version Management

The `versions.yml` file at the repository root defines tool versions:

```yaml
versions:
  rust: "1.86.0"
  chain_spec_builder: "0.20.0"
  polkadot_omni_node: "0.4.1"

# Recipe-specific overrides (optional)
zero_to_hero:
  rust: "1.86.0"
  chain_spec_builder: "0.20.0"
  polkadot_omni_node: "0.4.1"
```

Workflows resolve versions with this priority:
1. Recipe-specific version (`<recipe_key>.<tool>`)
2. Global version (`versions.<tool>`)

## Best Practices

### For Contributors

1. **Don't manually run workflows** - They're triggered automatically or via PR comments
2. **Use fast-skip pattern in tests** - Always check for node availability
3. **Keep recipe.config.yml accurate** - CI relies on this metadata
4. **Test locally first** - Use `npm test` before pushing

### For Maintainers

1. **Use /generate-scripts after merge** - Generates production scripts with pinned versions
2. **Update versions.yml carefully** - Changes trigger script regeneration
3. **Create releases for major updates** - Use `/generate-release` for significant changes
4. **Monitor workflow runs** - Check Actions tab for failures
