# Release v0.7.0

## Highlights

This release focuses on improving the developer experience with better progress feedback, PAPI integration fixes, and enhanced submit workflow.

## New Features

### Draft Pull Requests
- The `dot submit` command now creates **draft PRs** instead of regular PRs
- Users are prompted to review and edit the PR description before marking it ready for review
- Clear next steps are displayed after submission

### Verbose Progress Output
- Project creation now shows step-by-step progress:
  - Creating project directory
  - Initializing git repository
  - Copying template files
  - Installing dependencies
- All pathways now include a "Change to project directory" step in Next Steps

### Improved PAPI Integration
- Auto-generate PAPI types before running tests
- Use `papi add` for reliable metadata fetching
- Fixed transactions template to use proper PAPI descriptors

## Bug Fixes

- **Fixed .gitignore copying**: Templates now correctly include .gitignore files in new projects
- **Fixed submit command**: Handle symlinks and skip node_modules during project copy
- **Fixed repository references**: Updated all references from paritytech to polkadot-developers
- **Fixed PAPI configuration**: Let `papi add` create its own config instead of pre-creating it
- **Fixed node connectivity**: Added retry logic for node readiness checks

## Improvements

- Simplified parachain dev node startup to align with upstream template
- Removed redundant zombienet-xcm.toml configuration
- Updated getting-started documentation for auto-generated types
- Better error handling for npm install during project creation

## Breaking Changes

None

## Contributors

Thanks to all contributors who made this release possible!
