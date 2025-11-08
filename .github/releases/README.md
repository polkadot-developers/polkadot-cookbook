# Releases

This directory contains release manifests for the Polkadot Cookbook.

## Structure

```
releases/
├── templates/
│   └── manifest.yml          # Template for release manifests
├── v0.1.0/
│   ├── manifest.yml          # Release manifest
│   └── RELEASE_NOTES.md      # Human-readable release notes
├── v0.2.0/
│   ├── manifest.yml
│   └── RELEASE_NOTES.md
└── README.md                  # This file
```

## Release Process

Releases are created automatically every **Wednesday at 9:00 AM Bangkok time** (02:00 UTC) via GitHub Actions, or can be triggered manually for critical updates.

**Frequency:** Weekly (skips if no changes since last release)

### Release Types

1. **Recipe Releases** (`v0.x.x`) - Versioned collections of tested recipes
2. **CLI Releases** (`cli-v0.x.x`) - User-installable `dot` binary
3. **SDK Releases** (`sdk-v0.x.x`) - SDK library for external tools

### Versioning

Currently in **alpha** (v0.x.x), meaning:
- Breaking changes are expected and normal
- Fast iteration and frequent updates
- APIs may change without notice
- Suitable for testing and development, not production

Will transition to **v1.0.0** when:
- Core recipes are stable
- docs.polkadot.com has integrated several recipes
- Breaking changes become infrequent
- Ready to signal production-ready

### Semantic Versioning

Releases follow [Semantic Versioning](https://semver.org/):

- **Major** (`v1.0.0` → `v2.0.0`): Breaking changes
- **Minor** (`v1.0.0` → `v1.1.0`): New features, backward compatible
- **Patch** (`v1.0.0` → `v1.0.1`): Bug fixes, no API changes

During alpha (v0.x.x):
- Minor bumps (`v0.1.0` → `v0.2.0`): New features or breaking changes
- Patch bumps (`v0.1.0` → `v0.1.1`): Bug fixes only

## Manifest Format

Each release includes a `manifest.yml` file with:

```yaml
release: v0.1.0               # Release version
release_date: 2025-01-01T00:00:00Z  # ISO 8601 timestamp
status: alpha                 # alpha | beta | stable

tooling:                      # Versions used to test recipes
  cli_version: cli-v0.1.0
  sdk_version: sdk-v0.1.0
  rust: "1.86"
  polkadot_sdk: "1.15.0"

recipes:                      # Recipe inventory
  basic-pallet:
    version: "0.1.0"          # Recipe version
    path: "recipes/basic-pallet"
    tested: true              # Passed CI tests?
    commit: "abc123"          # Git commit hash
    pathway: "runtime"        # Recipe pathway
    difficulty: "beginner"    # Difficulty level
    description: "..."        # Brief description
```

## Integration with docs.polkadot.com

Documentation can reference stable recipe code using release tags:

```
https://github.com/polkadot-developers/polkadot-cookbook/tree/v0.1.0/recipes/basic-pallet
```

The manifest provides:
- List of tested recipes
- Commit hashes for immutable references
- Compatibility information (Polkadot SDK, Rust versions)

## Release Triggers

### Automated Triggers

1. **Scheduled Release** - Every Wednesday 9 AM Bangkok time
   - Collects all changes since last release
   - Tests all recipes
   - Generates manifest and release notes
   - Creates GitHub release with tag

2. **Breaking Change Release** - Immediate release after:
   - CLI breaking change (`cli/**` with `semantic:major`)
   - SDK breaking change (`core/**` with `semantic:major`)
   - Tests all recipes with new tooling
   - Creates recipe release if tests pass

### Manual Triggers

Use GitHub Actions `workflow_dispatch` for:
- Critical fixes between scheduled releases
- Coordinated releases with docs.polkadot.com
- Milestone releases (v1.0.0, v2.0.0)

## Contributing

See [CONTRIBUTING.md](../CONTRIBUTING.md) for details on the contribution workflow and how recipe versions are managed.
