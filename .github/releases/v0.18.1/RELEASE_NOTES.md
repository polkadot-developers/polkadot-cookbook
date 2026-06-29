
<div align="center">
  <img src="https://raw.githubusercontent.com/polkadot-developers/polkadot-cookbook/v0.18.1/.github/releases/v0.18.1/cover.svg" alt="Release v0.18.1" width="100%" />
</div>

# Release v0.18.1

Released: 2026-06-29

## Summary

Syncs the cookbook's tracked dependency versions with polkadot-docs — `polkadot-stable2603-1` plus the latest JS/Rust package set — and repairs the pre-existing CI failures that sync surfaced, so the full test-harness suite is green again. Also prepares the repository for handoff to Parity maintainers.

## What's New

### Infrastructure

- **Synced all tracked dependencies to polkadot-docs** — `polkadot-sdk` to `polkadot-stable2603-1`, plus subxt/subxt-cli `0.50.1`, polkadot-omni-node `0.15.0`, polkadot-api `2.1.0`, chopsticks `1.3.1`, resolc `1.2.0`, hardhat-polkadot `0.3.0`, paraspell `13.4.0`, hdkd-helpers `0.0.30`, keyring/util-crypto `14.0.3`, and solc `0.8.35` across `versions.yml` and the per-harness manifests, keeping the test harnesses aligned with the published tutorials (#298).
- **Repaired the CI suite** that the bump surfaced — `cargo install --locked` for `revive-dev-node` (the transitive `core2` crate was yanked from crates.io), `--authoring slot-based` for the Asset Hub zombienet collator (stable2603 requires relay-parent descendants or the parachain stalls at block #0), a `mkdir -p metadata` fix plus a PID-scoped chopsticks teardown for the Pay Fees harness, and moved flaky Paseo RPC endpoints to reliable providers (`rpc.polkadot.io` / Zondax) so the chain-interaction harnesses stop hanging on `papi add` (#298).

### Tooling

- **Maintainer tasks now run quarterly** instead of weekly — the auto-created checklist issue opens every 3 months, matching a lighter-touch maintenance cadence (#300).
- **Handoff preparation** — removed personal code owners and maintainers, leaving placeholders for the incoming Parity team so no stale `@`-mentions fire (#300).
- **Naming convention** — renamed the Uniswap V2 Core harness test to the `docs.test.ts` convention used across the docs pathway (#296).

## Commits

- chore: sync dependency versions from polkadot-docs (stable2603-1) + CI fixes (#298)
- chore: handoff housekeeping — quarterly maintainer tasks, drop personal owners (#300)
- chore: update name to match convention (#296)

## Stats

**3 commits, +4,247 / -6,851 lines**

**Full Changelog:** https://github.com/polkadot-developers/polkadot-cookbook/compare/v0.18.0...v0.18.1

## Compatibility

Tested with:
- Rust: 1.91.1
- Node.js: v24.7.0

## Next Steps

Merging the release PR triggers [`publish-release.yml`](../../../.github/workflows/publish-release.yml), which:
1. Creates the `v0.18.1` git tag
2. Builds the `dot` CLI binaries for Linux, macOS (Intel + ARM), and Windows
3. Publishes the GitHub Release with cover art, manifest, and binaries attached

---

**Status:** Alpha (v0.x.x)

---

<div align="center">
  <img src="https://raw.githubusercontent.com/polkadot-developers/polkadot-cookbook/v0.18.1/.github/releases/v0.18.1/cover-chain.svg" alt="Polkadot network state at v0.18.1 release" width="100%" />
</div>
