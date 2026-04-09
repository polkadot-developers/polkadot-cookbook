---
layout: doc
title: "Maintainers Guide"
permalink: /maintainers/
---

# Maintainers Guide

This section contains documentation for repository maintainers managing releases, workflows, and repository operations.

## Documentation

### Core Processes

- **[Release Process](release-process.md)** - Semantic versioning and automated releases
- **[Workflows](workflows.md)** - GitHub Actions workflows and automation

## Quick Reference

### Release Management

The Polkadot Cookbook uses the `/release` Claude Code skill with three release streams:

1. **Recipe Releases** (`v0.x.x`) - Created via `/release` skill (on-demand or scheduled)
2. **CLI Releases** (`cli-v0.x.x`) - Via `release-cli.yml` workflow
3. **SDK Releases** (`sdk-v0.x.x`) - Via manual workflow dispatch

[→ Full Release Process Guide](release-process.md)

### Workflow Management

CI/CD is handled through GitHub Actions and Claude Code skills:

- **Testing Workflows** - Recipe and SDK tests on every PR
- **Release Publishing** - `publish-release.yml` builds binaries and publishes on merge
- **Release Creation** - `/release` skill analyzes changes and creates release PRs

[→ Complete Workflows Documentation](workflows.md)

## Maintainer Responsibilities

As a maintainer, you're responsible for:

✅ **PR Review** - Reviewing and merging pull requests
✅ **Release Oversight** - Monitoring automated releases
✅ **Quality Control** - Ensuring recipes meet standards
✅ **Workflow Management** - Maintaining CI/CD pipelines
✅ **Community Support** - Helping contributors succeed

## Best Practices

### For Pull Request Review

- Verify semantic labels are correct
- Check that tests pass in CI
- Ensure recipe follows guidelines
- Review for security issues
- Provide constructive feedback

### For Release Management

- Run `/release` skill when ready to cut a release
- Review draft release PRs before merging
- Monitor `publish-release.yml` after merging
- Verify release artifacts are correct

### For Workflow Maintenance

- Keep workflows up to date
- Monitor for CI failures
- Update dependencies regularly
- Document workflow changes

---

[← Back to Documentation Hub](../)
