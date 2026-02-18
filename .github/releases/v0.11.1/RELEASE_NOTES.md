# Release v0.11.1

Released: 2026-02-18

## Changes Since v0.11.0

**Version Bump:** PATCH

### Changes

- ci: remove @nhussein11 from CODEOWNERS (#144) (783d75f)
- ci: batch update last_tested dates (#142) (12fc8ac)
- ci: fix branch checkout in batch last_tested workflow (#143) (be6f78f)
- test: backdate contracts-example last_tested for CI verification (#141) (1dee3f3)
- ci: batch last_tested updates into a single PR (#140) (96730d6)
- ci: filter badge SVGs to show push-event status only (#139) (ed3a58f)
- add uniswap v2 periphery migration (#110) (08e1527)
- ci: improve PR recipe guidance bot for first-time contributors (#138) (9096443)
- ci: update last_tested for fork-a-parachain (#137) (4351053)
- Fix fork-a-parachain test timeouts in CI (#136) (c3c25e4)
- ci: add PR guidance bot for recipe and docs contributions (#135) (b934eae)
- ci: detect upstream tutorial drift in sync workflow (#134) (d78f27c)
- ci: update last_tested for cross-chain-transaction-example (#117) (3c1899a)
- ci: update last_tested for contracts-example (#118) (20a3a06)
- ci: update last_tested for transaction-example (#119) (b621842)
- ci: update last_tested for install-polkadot-sdk (#120) (e924fb5)
- ci: update last_tested for pallet-testing (#121) (dd9c63a)
- ci: update last_tested for benchmark-pallet (#122) (f44f6c2)
- ci: update last_tested for mock-runtime (#123) (dbf2e9e)
- ci: update last_tested for pallet-example (#124) (ad83ef2)
- ci: update last_tested for create-a-pallet (#125) (99355d2)
- ci: update last_tested for add-existing-pallets (#126) (7282a6d)
- ci: update last_tested for set-up-parachain-template (#127) (36d92d3)
- ci: update last_tested for add-pallet-instances (#128) (413f168)
- ci: update last_tested for run-a-parachain-network (#130) (583e02d)
- ci: update last_tested for parachain-example (#133) (81ab80b)
- chore: add CODEOWNERS for automatic PR review routing (#132) (185a861)
- ci: add stale issue and PR automation (#131) (9a65e7d)
- ci: add fork protection guards to all workflows (#129) (fad4d8a)
- ci: update last_tested for network-example (#116) (f929773)
- fix(ci): use explicit git fetch --tags for reliable tag detection (#108) (f13722e)


## Compatibility

This release was tested with:
- Rust: 1.91.1
- Node.js: v20.20.0

## Testing

All recipes have passed CI tests.

Full manifest: [manifest.yml](./manifest.yml)

---

**Status:** Alpha (v0.x.x)
