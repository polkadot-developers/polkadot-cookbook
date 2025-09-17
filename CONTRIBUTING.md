## Contributing to the Polkadot Cookbook

This guide is for external contributors.

### 1) Propose your tutorial (required)

- Open an issue using the template: `Tutorial Proposal`.
- Include: learning objectives, audience, prerequisites, tools/versions.
- Wait for approval and a tutorial slug (e.g. `my-tutorial`).

### 2) Create a branch

```bash
git checkout -b feat/tutorial-my-tutorial
```

### 3) Scaffold your tutorial folder

Use the helpers to create the structure and testing setup.

```bash
chmod +x ./utils/scaffold-tutorial.sh && ./utils/scaffold-tutorial.sh my-tutorial
chmod +x ./utils/bootstrap-tests.sh && ./utils/bootstrap-tests.sh my-tutorial
```

This creates:

```text
tutorials/my-tutorial/
  tutorial.yml
  justfile
  README.md              # your written tutorial (required)
  my-tutorial-code/      # your project code (contracts or SDK)
  tests/                 # vitest tests
```

### 4) Build the tutorial content

- Write the tutorial in `tutorials/my-tutorial/README.md` (required).
- Add code under `tutorials/my-tutorial/my-tutorial-code/`.
- Add at least one e2e test under `tutorials/my-tutorial/tests/` using `@polkadot/api`.
  - Tests must skip fast when no local node is running.

### 5) Run tests locally

```bash
cd tutorials/my-tutorial
npm run test
```

### 6) Open a Pull Request

```bash
git add -A && git commit -m "feat(tutorial): add my-tutorial"
git push origin feat/tutorial-my-tutorial
```

Open the PR. The PR template will guide your checklist.

### 7) What CI runs on your PR (automatic)

- PR Tutorial Tests: `.github/workflows/ci-test.yml`
  - If your PR ADDS a new tutorial folder under `tutorials/<slug>/`, CI runs tests only for that new tutorial.
  - Otherwise, CI runs tests for any tutorials that changed.
  - It ignores generated files under `tutorials/**/scripts/**`.
  - It installs deps and runs `vitest` for the selected tutorials.
  - Tests that require a node should "skip fast" if no endpoint is available.

### 8) After merge (maintainers do this)

- Generate finalized scripts (with concrete versions) for your tutorial:
  - Workflow: `.github/workflows/generate-scripts.yml` (manual trigger or on `versions.yml` changes)
  - Output: `tutorials/<slug>/scripts/` (committed to the repo for docs consumption)
- Optional release: same workflow may tag and create a release when scripts changed on `master`/`dev`.

### Notes and tips

- Keep PRs focused: one tutorial per PR.
- If your SDK tutorial needs runtime changes, describe them in `README.md`. We can help you apply an overlay-based approach later to avoid clashes.
- If anything is unclear, open an issue using `Custom Blank Issue`.

Thank you for contributing! 🎉


