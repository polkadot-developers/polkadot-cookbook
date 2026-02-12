# Reference Documentation

This section contains reference material, schemas, and specifications.

## Available References

### Schemas & Specifications

- **[Manifest Schema](manifest-schema.md)** - Release manifest format

### Security

- **[Security Guidelines](security.md)** - Security best practices

## Quick Reference

### Manifest Format

Release manifests track tested recipes:

```yaml
release: v0.2.0
release_date: 2025-11-06T02:00:00Z
status: alpha

tooling:
  cli_version: cli-v0.2.0
  rust: "1.88"

recipes:
  basic-pallet:
    version: "0.1.0"
    tested: true
```

[→ Complete Manifest Schema](manifest-schema.md)


## Security Guidelines

Before contributing recipes, review:

- Code review requirements
- Security best practices
- Vulnerability reporting process
- Safe coding patterns

[→ Security Guidelines](security.md)

---

[← Back to Documentation Hub](../README.md)
