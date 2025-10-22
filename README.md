# Polkadot Cookbook
A place for Polkadot tutorials and guides.

[![Kitchensink Parachain](https://github.com/polkadot-developers/polkadot-docs-tests/actions/workflows/build-kitchensink-parachain.yml/badge.svg)](https://github.com/polkadot-developers/polkadot-docs-tests/actions/workflows/build-kitchensink-parachain.yml)
[![semantic-release](https://img.shields.io/badge/semantic--release-automated-blue?logo=semantic-release)](https://github.com/semantic-release/semantic-release)
[![Release](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/release.yml/badge.svg)](https://github.com/polkadot-developers/polkadot-cookbook/actions/workflows/release.yml)

Includes a scalable, reusable, and flexible test suite that ensures all Polkadot Cookbook tutorials work. 

## Releases and Versioning

This repository follows Semantic Versioning (X.Y.Z):

- X (MAJOR): breaking changes
- Y (MINOR): enhancements without breaking changes
- Z (PATCH): fixes

Releases are automated with semantic-release. On every push to `master`, the workflow:

- infers the next version from Conventional Commits
- updates `CHANGELOG.md`
- creates a GitHub Release

See `CONTRIBUTING.md` for commit guidelines.
