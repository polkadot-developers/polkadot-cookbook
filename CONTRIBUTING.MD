## Contributing

Steps for contributing to the Polkadot Cookbook. The Polkadot Cookbook contains two main sections:

- **Polkadot Smart Contract Cookbook**: Tutorials focused on smart contract development
- **Polkadot SDK Cookbook**: Tutorials for Polkadot SDK development and runtime building

## Contribution Workflow

1. Propose a new tutorial by opening a GitHub Issue with the idea
2. Wait for approval
3. Once approved, create a branch for the new tutorial
4. Create a draft PR with the markdown content of the tutorial, once approved, go ahead and push the changes to the PR and mark it as ready for review
5. After your PR is merged, you’re done 🎉.

  - (Optional) If you would also like your tutorial to be considered for the official Polkadot Docs, create a PR in the polkadot-docs repo that links to your merged PR in this repo. Otherwise, the team may handle this step for you.

- If you are proposing a new tutorial for the Polkadot Smart Contract Cookbook, please follow the instructions in the [Polkadot Smart Contract Cookbook](#polkadot-smart-contract-cookbook) section.

- If you are proposing a new tutorial for the Polkadot SDK Cookbook, please follow the instructions in the [Polkadot SDK Cookbook](#polkadot-sdk-cookbook) section.

## Polkadot Smart Contract Cookbook

To contribute to the Polkadot Smart Contract Cookbook, please follow these instructions:

1. Create an issue with the idea. The issue should include:
    - Clear learning objectives
    - Target audience (beginner/intermediate/advanced)
    - Prerequisites needed
    - List of tools, versions, and technologies used (eg. Polkadot-API, ParaSpell, etc.)

2. Wait for approval
3. Once approved, fork the polkadot-cookbook and create a new branch for your new tutorial:

```bash
git checkout -b feat/tutorial-<slug>
```

4. Create the tutorial structure

```bash
tutorials/<slug>/
  tutorial.yml            # manifest: build/runtime, chain spec, network, tests
  justfile                # tutorial-local Just recipes
  <slug>-code/             # project directory with the smart contract project code. For example, nft-code, erc20-code, etc.
  tests/
    <your-tests>.test.ts  # polkadot.js based e2e tests (recommended)
  scripts/                # optional; prefer using repo-level Just recipes
```

5. Push the changes to the branch and create a PR:

```bash
git push origin feat/tutorial-<slug>
```

6. After the PR is approved and merged, you will have to decide the following:

- Do you want to include the tutorial in the official documentation?
  - If no, you can still use the tutorial as a standalone tutorial in the cookbook
  - If yes, do you want to do it by yourself or do you want the team to do it for you?
    - If you want to do it by yourself, you will have to create a new PR in the polkadot-docs repo to include the tutorial in the official documentation
    - If you want the team to do it for you, you will have to wait for the team to do it for you


## Polkadot SDK Cookbook

To contribute to the Polkadot SDK Cookbook, please follow these instructions:

1. Create an issue with the idea. The issue should include:
    - Clear learning objectives
    - Target audience (beginner/intermediate/advanced)
    - Prerequisites needed
    - List of tools, versions, and technologies used (eg. Polkadot-API@1.110.0, ParaSpell@0.4.0, etc.)

2. Wait for approval
3. Once approved, fork the polkadot-cookbook and create a new branch for your new tutorial:

```bash
git checkout -b feat/tutorial-<slug>
```

4. Create the tutorial structure

```bash
tutorials/<slug>/
  tutorial.yml            # manifest: build/runtime, chain spec, network, tests
  justfile                # tutorial-local Just recipes
  <slug>-code/             # project directory with the project code. For example, add-nft-pallet, customize-your-pallet, etc.
  zombienet/
    zombienet-omni-node.toml
    zombienet.toml        # optional alternative using template node
  tests/
    <your-tests>.test.ts  # polkadot.js based e2e tests (recommended)
  scripts/                # optional; prefer using repo-level Just recipes
```

When possible, this tutorials should take advantage of the existing `parachain-kitchensink` project.

- Note: The `zombienet` folder is optional, you can use it if you want to test the tutorial locally using zombienet.

5. Push the changes to the branch and create a PR:

```bash
git push origin feat/tutorial-<slug>
```

6. After the PR is approved and merged, you will have to decide the following:

- Do you want to include the tutorial in the official documentation?
  - If no, you can still use the tutorial as a standalone tutorial in the cookbook
  - If yes, do you want to do it by yourself or do you want the team to do it for you?
    - If you want to do it by yourself, you will have to create a new PR in the polkadot-docs repo to include the tutorial in the official documentation
    - If you want the team to do it for you, you will have to wait for the team to do it for you