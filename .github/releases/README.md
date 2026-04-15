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

Releases are created using the `/release` Claude Code skill, which can be run on-demand or scheduled via Claude Code triggers.

### How It Works

1. **`/release` skill** — Analyzes changes since the last tag, determines the semver bump, generates a manifest + release notes, updates `Cargo.toml`, and opens a draft PR
2. **Review & merge** — A maintainer reviews and merges the release PR
3. **`publish-release.yml`** — Automatically triggers on merge, builds CLI binaries for 5 platforms, creates a git tag, and publishes the GitHub Release

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
previous_release: v0.0.9      # Prior release tag (for diff links)
release_date: 2025-01-01T00:00:00Z  # ISO 8601 timestamp
status: alpha                 # alpha | beta | stable

tooling:                      # Versions used to test recipes
  rust: "1.91"
  node: "v20.20.1"
```

## CHANGELOG.md

A cumulative changelog is maintained at the repository root following [Keep a Changelog](https://keepachangelog.com/) format. The `/release` skill appends to it automatically with each release. Each entry includes Added/Changed/Fixed/Breaking sections and a compare link.

## Integration with docs.polkadot.com

Documentation can reference stable recipe code using release tags:

```
https://github.com/polkadot-developers/polkadot-cookbook/tree/v0.1.0/recipes/basic-pallet
```

## Contributing

See [CONTRIBUTING.md](../CONTRIBUTING.md) for details on the contribution workflow and how recipe versions are managed.
