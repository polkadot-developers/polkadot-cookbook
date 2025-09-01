## Contributing

Thank you for helping validate and improve the Polkadot Docs tutorials. This repo hosts runnable examples and tests to ensure the tutorials in `docs.polkadot.com` keep working.

### Prerequisites

- Rust toolchain (see `versions.yml` for the pinned version). In CI we use Rust 1.86.
- `just` command runner (optional but recommended).
- Zombienet for local relay scenarios (optional).

### Add a new tutorial (plug‑and‑play)

1) Create a branch

```bash
git checkout -b feat/tutorial-<slug>
```

2) Create the tutorial structure

```
tutorials/<slug>/
  tutorial.yml            # manifest: build/runtime, chain spec, network, tests
  justfile                # tutorial-local Just recipes
  zombienet/
    zombienet-omni-node.toml
    zombienet.toml        # optional alternative using template node
  tests/
    <your-tests>.test.ts  # polkadot.js based e2e tests (recommended)
  scripts/                # optional; prefer using repo-level Just recipes
```

3) Fill out `tutorial.yml`

Minimal example:

```yaml
name: kitchensink-parachain
rust_toolchain: "1.86.0"
relay_chain: "paseo-local"
para_id: 1000
runtime_wasm: ./kitchensink-parachain/target/release/wbuild/parachain-template-runtime/parachain_template_runtime.wasm
chain_spec:
  output: ./kitchensink-parachain/chain_spec.json
  builder: chain-spec-builder
  preset: development
zombienet:
  provider: native
  network_toml: ./tutorials/<slug>/zombienet/zombienet-omni-node.toml
tests:
  type: nodejs
  command: pnpm test --filter @tests/<slug>
```

4) Build and generate chain spec

If you use the kitchensink example runtime in this repo:

```bash
just build
just chain-spec
```

This produces `kitchensink-parachain/chain_spec.json` and the runtime WASM in `kitchensink-parachain/target/release/wbuild/...`.

5) Write a minimal e2e test (optional)

Example: assert that the node produces blocks over WS at `ws://localhost:9988`.

```ts
import { ApiPromise, WsProvider } from '@polkadot/api';

test('produces blocks', async () => {
  const api = await ApiPromise.create({ provider: new WsProvider('ws://localhost:9988') });
  const h1 = await api.rpc.chain.getFinalizedHead();
  await new Promise(r => setTimeout(r, 3000));
  const h2 = await api.rpc.chain.getFinalizedHead();
  expect(h1.toHex()).not.toBe(h2.toHex());
  await api.disconnect();
});
```

6) Run locally

- Fast dev mode (no relay):

```bash
cd tutorials/<slug> && just start-dev
```

- Local relay via Zombienet (realistic):

```bash
cd tutorials/<slug> && just spawn-omni
```

Then run your tests using the command from `tutorial.yml`.

7) CI integration

- This repository provides a reusable composite action `.github/actions/verify-command` to run commands and assert on:
  - stdout/stderr substrings
  - file existence
  - JSON fields via `jq`
- Workflows should call `just` recipes (e.g., `just build`, `just chain-spec`) and verify outputs.
- The long‑term goal is a matrix that finds all `tutorials/**/tutorial.yml` and runs each tutorial end‑to‑end.

8) Open a PR

- Include a short description and the tutorial slug.
- Ensure CI passes.

### Style and Hygiene

- Keep versions in `versions.yml` only; do not hardcode versions in scripts.
- Prefer `rust-toolchain.toml` to pin toolchains over changing global rustup defaults.
- Align relay chain names and filenames consistently (e.g., `paseo-local`, `chain_spec.json`).
- Run `just fmt` and `just clippy` locally before pushing.


