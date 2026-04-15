# Release Footer Cover — Data Contract

Every value rendered on `cover-chain.svg` is pulled via a single JSON-RPC capture against a Polkadot mainnet node at release-cut time. This cover is a **point-in-time reading of the Polkadot network as it was at the moment the cookbook release was cut** — not live data, not a live dashboard.

This contract defines the exact RPC methods, post-processing, and fallback behavior. If the primary RPC host is unreachable, fallbacks are tried in order. If all three fail within the timeout, the release ships **without** the footer cover (and `RELEASE_NOTES.md` omits the footer embed) — no fabricated values, ever.

## Endpoints (in order)

```
1. https://rpc.polkadot.io                   (primary — Parity)
2. https://polkadot-rpc.dwellir.com           (fallback 1 — Dwellir)
3. https://rpc.ibp.network/polkadot           (fallback 2 — IBP)
```

Timeout: 5s per endpoint. Total budget: 15s. All three use HTTP JSON-RPC (POST), no websocket needed.

## One-shot capture sequence

All reads serialized against a single chosen endpoint. If any step fails, fall through to the next endpoint and restart.

```bash
RPC=https://rpc.polkadot.io
call() { curl -s --max-time 5 -H "Content-Type: application/json" \
  -d "{\"jsonrpc\":\"2.0\",\"method\":\"$1\",\"params\":$2,\"id\":1}" $RPC; }

FH=$(call chain_getFinalizedHead '[]' | jq -r .result)
HEADER=$(call chain_getHeader "[\"$FH\"]" | jq .result)
RT=$(call state_getRuntimeVersion '[]' | jq .result)
HEALTH=$(call system_health '[]' | jq .result)
CHAIN=$(call system_chain '[]' | jq -r .result)
NODE=$(call system_version '[]' | jq -r .result)
PROPS=$(call system_properties '[]' | jq .result)
GENESIS=$(call chain_getBlockHash '[0]' | jq -r .result)
CAPTURE_TS=$(date -u +"%Y-%m-%d %H:%M UTC")
```

## Scalar tokens

| Token | Derived from | Post-processing |
|---|---|---|
| `{{VERSION}}` | cookbook release tag | strip leading `v` |
| `{{CAPTURE_TIMESTAMP}}` | `$CAPTURE_TS` | — |
| `{{CHAIN}}` | `system_chain` result | Display case (e.g. `Polkadot`) |
| `{{CHAIN_NAME_UPPER}}` | `$CHAIN` | uppercase |
| `{{CHAIN_TYPE}}` | `system_properties` or derived: relay if `paras` pallet absent; parachain otherwise | lowercase |
| `{{CHAIN_TYPE_UPPER}}` | `$CHAIN_TYPE` | uppercase |
| `{{PARA_ID}}` | `0` for relay; storage `ParachainInfo.ParachainId` for parachain | integer |
| `{{NETWORK}}` | `MAINNET` for polkadot, `TESTNET` for paseo, etc. | uppercase |
| `{{RPC_ENDPOINT}}` | host portion of chosen endpoint | e.g. `rpc.polkadot.io` |
| `{{SPEC_NAME}}` | `state_getRuntimeVersion.specName` | — |
| `{{SPEC_VERSION}}` | `state_getRuntimeVersion.specVersion` | comma-grouped integer |
| `{{IMPL_NAME}}` | `state_getRuntimeVersion.implName` | — |
| `{{IMPL_VERSION}}` | `state_getRuntimeVersion.implVersion` | integer |
| `{{NODE_VERSION}}` | `system_version` | — |
| `{{AUTHORING_VERSION}}` | `state_getRuntimeVersion.authoringVersion` | integer |
| `{{TX_VERSION}}` | `state_getRuntimeVersion.transactionVersion` | integer |
| `{{STATE_VERSION}}` | `state_getRuntimeVersion.stateVersion` | integer |
| `{{SYSTEM_VERSION}}` | `state_getRuntimeVersion.systemVersion` | integer |
| `{{API_COUNT}}` | `state_getRuntimeVersion.apis.length` | integer |
| `{{AUTHORING}}` | inferred: `BABE` if BabeApi hash present in apis array; else `AURA` | uppercase |
| `{{AUTHORING_LOWER}}` | `$AUTHORING` | lowercase |
| `{{BLOCK_TARGET}}` | `6s` for polkadot/kusama relay; `12s` for asset-hub coretime; `6s` default | string |
| `{{BLOCKS_PER_DAY}}` | `86400 / block_target_seconds` | comma-grouped integer |
| `{{TOKEN_SYMBOL}}` | `system_properties.tokenSymbol` (or `.tokenSymbol[0]` if array) | — |
| `{{TOKEN_DECIMALS}}` | `system_properties.tokenDecimals` (or `[0]`) | integer |
| `{{SS58_FORMAT}}` | `system_properties.ss58Format` | integer |
| `{{FINALIZED_BLOCK}}` | parse `header.number` hex → decimal | comma-grouped |
| `{{HEAD_HASH_SHORT}}` | `$FH` | first 8 chars..last 4 chars (e.g. `0xc2d8e7d3..04e4`) |
| `{{HEAD_HASH_PREFIX}}` | `$FH` | first 8 chars (e.g. `0xc2d8e7d3`) |
| `{{HEAD_HASH_FULL}}` | `$FH` | full 66-char hex string |
| `{{PARENT_HASH_SHORT}}` | `header.parentHash` | first 8..last 4 |
| `{{STATE_ROOT_PREFIX}}` | `header.stateRoot` | first 8 chars |
| `{{STATE_ROOT_SHORT}}` | `header.stateRoot` | first 8..last 4 (note: the last-4 is e.g. `fe5e3`, 5 chars for state root variant) |
| `{{EXTRINSICS_ROOT_SHORT}}` | `header.extrinsicsRoot` | first 8..last 4 |
| `{{GENESIS_PREFIX}}` | `$GENESIS` | first 8 chars |
| `{{GENESIS_SHORT}}` | `$GENESIS` | first 8..last 4 (6 chars suffix for polkadot: `..ce90c3`) |
| `{{GENESIS_DATE}}` | constant per chain: polkadot `2020-05-26`, kusama `2019-11-28`, paseo `2024-08-07` | ISO date |
| `{{NETWORK_AGE}}` | `today - genesis_date` | `~N years, M months` (round down) |
| `{{PEERS}}` | `system_health.peers` | integer |
| `{{SYNC_STATUS}}` | `✓` if `!system_health.isSyncing`; `syncing…` otherwise | — |

### Short-hash formatting rules

For a 66-character `0x`-prefixed hash:

- `_PREFIX` → first 10 chars (`0x` + 8 hex) — example: `0xc2d8e7d3`
- `_SHORT` → `{prefix}..{last-4-hex}` — example: `0xc2d8e7d3..04e4`
- `_FULL` → the entire 66-character string (only used in tickertape)

Exception: hash slots that appear in specific places may use 5-char suffixes for layout balance. The template uses `{{STATE_ROOT_SHORT}}` at two positions (B2 terminal and B3 footer); both render with 4-char suffix. `{{GENESIS_SHORT}}` uses 6-char suffix matching the Polkadot genesis convention. These are hardcoded choices in the template and do not vary per release.

## Variable-count sections

None. The chain readout shape is fixed — same layout for every release. This is the defining difference from the top cover (which has `@@COMMIT_LIST` / `@@BAR_CHART` / `@@CONTRIBUTOR_LIST` sized by commit count).

## Rendering pipeline

The /release skill:

1. Choose endpoint (walk primary → fallbacks until one answers `system_chain` within 5s).
2. Run the capture sequence. On any method failure, abandon this endpoint and try the next.
3. If all endpoints fail: **skip** this cover. Do not write `cover-chain.svg`. Remove the footer embed from `RELEASE_NOTES.md`. Log the failure in the release PR body.
4. If success: compute all scalar tokens per the table above.
5. Read `cover-chain.svg.template`, perform substitutions.
6. Sanitize: `&` → `&amp;`, `<` → `&lt;`, `>` → `&gt;` in any value that could contain them. (In practice only `{{NODE_VERSION}}` could, and rarely.)
7. Write to `.github/releases/v${VERSION}/cover-chain.svg`.
8. `xmllint --noout` — abort release on failure.
9. Append the footer embed to `RELEASE_NOTES.md`:

   ```html
   ---

   <div align="center">
     <img src="https://raw.githubusercontent.com/polkadot-developers/polkadot-cookbook/v{{VERSION}}/.github/releases/v{{VERSION}}/cover-chain.svg" alt="Polkadot network state at v{{VERSION}} release" width="100%" />
   </div>
   ```

10. Commit alongside `cover.svg`, `RELEASE_NOTES.md`, `manifest.yml`.

## Non-negotiables

- **No fabrication.** If an RPC field is missing or malformed, fail the pipeline rather than substitute a default.
- **No caching across releases.** Each release captures its own chain state from a live query at release-cut time. Never reuse the previous release's capture.
- **Capture timestamp is authoritative.** Use UTC. Format `YYYY-MM-DD HH:MM UTC`. The timestamp and `{{FINALIZED_BLOCK}}` pin the reading to a specific chain height; they must be mutually consistent (query the header first, then take the timestamp immediately after).
- **The B1 disclaimer badge is the single source of truth for the point-in-time nature.** Do not add redundant "at snapshot" / "at release-cut" / "historical" qualifiers to other cells.

## Chain-specific defaults

Values that depend on the chain itself (not the RPC response). Hardcoded per target chain in the skill:

| Chain | `{{GENESIS_DATE}}` | `{{BLOCK_TARGET}}` | `{{NETWORK}}` |
|---|---|---|---|
| Polkadot | 2020-05-26 | 6s | MAINNET |
| Kusama | 2019-11-28 | 6s | CANARY |
| Paseo | 2024-08-07 | 6s | TESTNET |

For the cookbook, default target is **Polkadot**. This can be overridden by a skill argument if we ever want to cut a release keyed to Paseo or Kusama.

## Adding new facts

Same rule as the top cover: any new token ships with its RPC method (or derivation) here in `cover-chain.data.md` and its substitution site in `cover-chain.svg.template`. Length-bound every value (hash prefixes, comma-grouped numerals) so the Mondrian cell geometry doesn't break.
