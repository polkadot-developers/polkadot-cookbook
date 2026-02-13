---
layout: doc
title: "Maintainers Guide"
---

# Maintainers Guide

This section contains documentation for repository maintainers managing releases, workflows, and repository operations.

## Documentation

### Core Processes

- **[Release Process](release-process.md)** - Semantic versioning and automated releases
- **[Workflows](workflows.md)** - GitHub Actions workflows and automation

### Governance

- **[Governance](governance.md)** - Decision-making and review process

## Quick Reference

### Release Management

The Polkadot Cookbook uses automated semantic versioning with three release streams:

1. **Recipe Releases** (`v0.x.x`) - Weekly automated releases
2. **CLI Releases** (`cli-v0.x.x`) - Breaking change triggered
3. **SDK Releases** (`sdk-v0.x.x`) - Breaking change triggered

[→ Full Release Process Guide](release-process.md)

### Workflow Management

All CI/CD is handled through GitHub Actions:

- **Testing Workflows** - Recipe and SDK tests on every PR
- **Release Workflows** - Automated version detection and releases
- **Automation Workflows** - Semantic labeling and quality checks

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

- Monitor weekly release workflow
- Review breaking change releases carefully
- Verify release artifacts are correct
- Update documentation as needed

### For Workflow Maintenance

- Keep workflows up to date
- Monitor for CI failures
- Update dependencies regularly
- Document workflow changes

---

[← Back to Documentation Hub](../)
