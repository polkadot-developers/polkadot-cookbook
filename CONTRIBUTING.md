# Contributing to the Polkadot Cookbook

This guide is for external contributors.

 **Visual Guide**: See the [Tutorial Creation Workflow](.github/TUTORIAL_WORKFLOW.md) diagram for a complete overview of the process.

## Quick Start

1. Propose your tutorial via [GitHub issue](https://github.com/polkadot-developers/polkadot-cookbook/issues/new?template=01-tutorial-proposal.md)
2. Wait for approval and a tutorial slug (e.g. `my-tutorial`)
3. Fork and clone the repo and `cd polkadot-cookbook`
4. Run `npm run create-tutorial my-tutorial`
5. Write content, add code, write tests
6. Open a Pull Request

## 1) Propose your tutorial (required)

- Open an issue using the template: `Tutorial Proposal`.
- Include: learning objectives, audience, prerequisites, tools/versions.
- Wait for approval and a tutorial slug (e.g. `my-tutorial`).

## 2) Create your tutorial using the CLI tool

First, fork and clone this repository:

```bash
git clone https://github.com/YOUR_USERNAME/polkadot-cookbook.git
cd polkadot-cookbook
```

Then run the unified tutorial creator from the repository root:

```bash
npm run create-tutorial my-tutorial
```

This single command will:

- ✅ Create a git branch (`feat/tutorial-my-tutorial`)
- ✅ Scaffold the complete folder structure
- ✅ Bootstrap the test environment
- ✅ Install all necessary dependencies
- ✅ Show you clear next steps

The created structure:

```text
tutorials/my-tutorial/
  tutorial.yml           # metadata and configuration
  justfile               # optional just commands
  README.md              # your written tutorial (required)
  my-tutorial-code/      # your project code (contracts or SDK)
  tests/                 # vitest e2e tests
  package.json           # npm dependencies
  vitest.config.ts       # test configuration
  tsconfig.json          # TypeScript configuration
```



## 3) Build the tutorial content

- Write the tutorial in `tutorials/my-tutorial/README.md` (required).
- Add code under `tutorials/my-tutorial/my-tutorial-code/`.
- Add at least one e2e test under `tutorials/my-tutorial/tests/` using `@polkadot/api`.
  - Tests must skip fast when no local node is running.

## 4) Run tests locally

```bash
cd tutorials/my-tutorial
npm run test
```

## 5) Open a Pull Request

```bash
git add -A && git commit -m "feat(tutorial): add my-tutorial"
git push origin feat/tutorial-my-tutorial
```

Open the PR. The PR template will guide your checklist.

## 6) What CI runs on your PR (automatic)

- PR Tutorial Tests: `.github/workflows/ci-test.yml`
  - If your PR ADDS a new tutorial folder under `tutorials/<slug>/`, CI runs tests only for that new tutorial.
  - It installs deps and runs `vitest` for the selected tutorials.
  - Tests that require a node should "skip fast" if no endpoint is available.

## 7) After merge (maintainers do this)

- Generate finalized scripts (with concrete versions) for your tutorial:
  - Workflow: `.github/workflows/generate-scripts.yml` (manual trigger or on `versions.yml` changes)
  - Output: `tutorials/<slug>/scripts/` (committed to the repo for docs consumption)
- Tag and Release: the workflow will create a tutorial-specific tag `tutorial/<slug>/vYYYYMMDD-HHMMSS[-<shortsha>]`. If `create_release` is true on manual runs, it also creates a GitHub Release with resolved versions.

Docs publishing and snippet stability:

- This repo is the code source for `polkadot-developers/polkadot-docs` powering `docs.polkadot.com`.
- The per-tutorial tags let the docs fetch stable snippets without cross-tutorial conflicts. Keep your tutorial self-contained under `tutorials/<slug>/`.
- If your tutorial needs specific anchors/regions for snippet extraction, add clear comment anchors in code (ask maintainers for the current convention) and reference those in your README.

## Notes and tips

- Keep PRs focused: one tutorial per PR.
- If your SDK tutorial needs runtime changes, describe them in `README.md`. We can help you apply an overlay-based approach later to avoid clashes.
- If anything is unclear, open an issue using `Custom Blank Issue`.

Thank you for contributing! 🎉
