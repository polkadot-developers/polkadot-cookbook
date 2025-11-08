# Testing GitHub Actions Workflows

This guide explains how to test the GitHub Actions workflows, particularly the version management integration.

## Understanding Version Management in CI

The Polkadot Cookbook uses a flexible version management system that ensures each recipe is tested with its specified dependency versions.

### How Versions Work in CI

1. **Global Defaults**: Repository root `versions.yml` defines default versions for all recipes
2. **Recipe Overrides**: Each recipe can override specific versions in `recipes/<slug>/versions.yml`
3. **Version Resolution**: The CLI merges global and recipe versions (recipe takes precedence)
4. **CI Integration**: Workflows use `dot versions <slug> --ci` to get resolved versions
5. **Environment Variables**: Resolved versions are exported as env vars for CI steps

### Example Version Resolution

**Global versions.yml:**
```yaml
versions:
  rust: "1.86"
  polkadot_omni_node: "0.5.0"
  chain_spec_builder: "10.0.0"
```

**Recipe versions.yml:**
```yaml
versions:
  polkadot_omni_node: "0.6.0"  # Override
```

**Resolved for recipe:**
- `rust`: `1.86` (from global)
- `polkadot_omni_node`: `0.6.0` (from recipe override)
- `chain_spec_builder`: `10.0.0` (from global)

See [Release Process](RELEASE_PROCESS.md#dependency-version-management) for complete version management documentation.

## Testing the Version Resolution Workflow

The `test-recipes.yml` workflow uses the polkadot-cookbook CLI to resolve versions. Here's how to test it:

### Option 1: Manual Workflow Trigger (Recommended)

The workflow includes a `workflow_dispatch` trigger that allows manual execution:

1. **Navigate to Actions tab** on GitHub
2. **Select "PR Recipe Tests"** workflow
3. **Click "Run workflow"** button
4. **Enter recipe slug** to test (e.g., `test-version-workflow`)
5. **Click "Run workflow"** to execute

This will run the full workflow including:
- Building the CLI
- Resolving versions using the SDK
- Setting up dependencies
- Running recipe tests

### Option 2: Create a Test Pull Request

The workflow automatically triggers on:
- Changes to any file in `recipes/**`
- Changes to global `versions.yml`
- Changes to any `**/versions.yml` file

1. **Create a test branch:**
   ```bash
   git checkout -b test/version-workflow
   ```

2. **Create or modify a recipe** in the `recipes/` directory:
   ```bash
   dot recipe create --title "Test Version Workflow" --skip-install --no-git --non-interactive
   ```

3. **Customize versions.yml** (optional):
   ```yaml
   # recipes/test-version-workflow/versions.yml
   versions:
     polkadot_omni_node: "0.6.0"  # Override global version
     chain_spec_builder: "11.0.0"  # Override global version

   metadata:
     schema_version: "1.0"
   ```

4. **Commit and push:**
   ```bash
   git add recipes/test-version-workflow
   git commit -m "test: add version workflow test recipe"
   git push origin test/version-workflow
   ```

5. **Create a Pull Request** - the workflow will automatically run

**Note:** The workflow will also trigger if you only modify `versions.yml` files:
```bash
# Modifying global versions will trigger workflow for all recipes
git add versions.yml
git commit -m "chore: update global rust version"

# Modifying a recipe's versions will trigger workflow for that recipe
git add recipes/my-recipe/versions.yml
git commit -m "chore: update my-recipe dependency versions"
```

### Option 3: Local Simulation

Simulate the workflow steps locally:

```bash
# Navigate to repository root
cd /path/to/polkadot-cookbook

# Build the CLI (same as workflow)
cargo build --release

# Test version resolution for a recipe
TUTORIAL_SLUG="test-version-workflow"

# Resolve versions (same as workflow)
eval $(./target/release/dot versions $TUTORIAL_SLUG --ci)

# Verify environment variables are set
echo "RUST=$RUST"
echo "CHAIN_SPEC_BUILDER=$CHAIN_SPEC_BUILDER"
echo "POLKADOT_OMNI_NODE=$POLKADOT_OMNI_NODE"
echo "FRAME_OMNI_BENCHER=$FRAME_OMNI_BENCHER"

# These would be used for dependency installation in the workflow
```

### Option 4: Using `act` (Local GitHub Actions Runner)

[act](https://github.com/nektos/act) allows you to run GitHub Actions locally:

1. **Install act:**
   ```bash
   # macOS
   brew install act

   # Linux
   curl https://raw.githubusercontent.com/nektos/act/master/install.sh | sudo bash
   ```

2. **Run the workflow:**
   ```bash
   # Run with workflow_dispatch event
   act workflow_dispatch \
     -W .github/workflows/test-recipes.yml \
     --input recipe_slug=test-version-workflow
   ```

   **Note:** This requires Docker and may have limitations with some actions.

## Verification Checklist

When testing the workflow, verify:

- ✅ **CLI builds successfully** in the workflow
- ✅ **Version resolution completes** without errors
- ✅ **Recipe-specific versions override global versions** correctly
- ✅ **Global versions are used** when not overridden
- ✅ **Environment variables are set** with correct values
- ✅ **Dependencies install** with resolved versions
- ✅ **Tests run** successfully with resolved versions

## Test Recipe: test-version-workflow

A test recipe is included in `recipes/test-version-workflow` with custom versions:

```yaml
versions:
  rust: "1.86"
  polkadot_omni_node: "0.6.0"        # Overridden (global: 0.5.0)
  chain_spec_builder: "11.0.0"       # Overridden (global: 10.0.0)
  # frame_omni_bencher inherited from global: 0.13.0
```

**Expected resolution:**
- `polkadot_omni_node`: 0.6.0 (from recipe)
- `chain_spec_builder`: 11.0.0 (from recipe)
- `rust`: 1.86 (from recipe)
- `frame_omni_bencher`: 0.13.0 (from global)

**Verify locally:**
```bash
dot versions test-version-workflow --show-source
```

## Troubleshooting

### CLI Build Fails

**Symptom:** Cargo build fails in workflow

**Solution:**
- Verify `Cargo.toml` is valid
- Check Rust toolchain version
- Review build logs for specific errors

### Version Resolution Fails

**Symptom:** `dot versions` command fails

**Solution:**
- Verify `versions.yml` syntax is valid YAML
- Check that global `versions.yml` exists at repository root
- Ensure recipe directory exists in `recipes/`

### Wrong Versions Resolved

**Symptom:** Expected versions don't match resolved versions

**Solution:**
- Check recipe's `versions.yml` for typos
- Verify key names match global `versions.yml` exactly
- Use `--show-source` flag to debug which versions are from where:
  ```bash
  dot versions <slug> --show-source
  ```

### Workflow Doesn't Trigger

**Symptom:** PR created but workflow doesn't run

**Solution:**
- Verify PR includes changes in `recipes/**` path
- Check workflow file syntax is valid
- Ensure workflow is enabled in repository settings

## Testing New Version Keys

When adding new version-managed dependencies:

1. **Add to global `versions.yml`:**
   ```yaml
   versions:
     rust: "1.86"
     new_tool: "1.0.0"  # New dependency
   ```

2. **Update template** in `core/src/templates/versions_yml.rs`:
   ```rust
   # new_tool: "1.0.0"
   ```

3. **Test resolution:**
   ```bash
   dot versions --show-source
   ```

4. **Create test recipe with override:**
   ```yaml
   versions:
     new_tool: "2.0.0"
   ```

5. **Verify override works:**
   ```bash
   dot versions test-recipe --show-source
   ```

## Continuous Validation

Run these checks regularly:

```bash
# Test CLI builds
cargo build --release

# Test SDK tests pass
cargo test --package core

# Test global version resolution
dot versions

# Validate global versions.yml
dot versions --validate

# Test recipe override resolution
dot versions test-version-workflow --show-source

# Validate recipe versions.yml
dot versions test-version-workflow --validate

# Test CI format
dot versions test-version-workflow --ci
```

## Further Reading

- [GitHub Actions Documentation](https://docs.github.com/en/actions)
- [Version Management Documentation](RELEASE_PROCESS.md#dependency-version-management)
- [Workflow Documentation](workflows.md)
