# Release Cover Art — Template-Based (Retired Generative Spec)

The previous generative Mondrian spec has been retired. Each release now renders a **single canonical template** filled with facts pulled from git. This guarantees every value on the cover is verifiable and the design can't drift.

Each release ships two covers:

### Top cover — release-focused (commit-activity report)

- **[`cover.svg.template`](cover.svg.template)** — the 1200×630 template. Scalar tokens (`{{VERSION}}`, `{{COMMIT_COUNT}}`, …) and variable-count markers (`<!-- @@COMMIT_LIST -->`, `<!-- @@BAR_CHART -->`, …). **Do not hand-edit per release.**
- **[`cover.data.md`](cover.data.md)** — the data contract. For every token and marker: the exact command that produces it, sanitization rules, and scaling rules for edge cases (1 commit, 200 commits).

### Footer cover — ecosystem-focused (chain-state reading)

- **[`cover-chain.svg.template`](cover-chain.svg.template)** — the 1200×630 template for the point-in-time reading of Polkadot mainnet at release-cut time. Scalar tokens only (no variable-count sections — the chain readout is fixed-shape).
- **[`cover-chain.data.md`](cover-chain.data.md)** — the chain-data contract. For every token: the JSON-RPC method, post-processing, fallback endpoint order, and the "all endpoints fail → skip the footer cover" rule (never fabricate chain data).

## Design anatomy

The cover is a Mondrian composition (Polkadot pink / deep blue / cream / near-black) with a terminal-style data panel anchored in the top-right cell. Each remaining block carries one factual readout:

| Region | Content |
|---|---|
| B1 (pink, top-left) | Giant `v{{VERSION}}`, bump type, date range, days; commit-activity timeline by day; contributor list with commit-count bars |
| B2 (terminal panel, top-right) | Full commit log for the range, aggregate footer (`N commits · N contrib · +ins / -del · N files`), PR numbers, `HEAD · TAG` |
| B3 (deep blue, bottom) | `FILES CHANGED BY AREA` horizontal bar chart, insertions/deletions/Δ-ratio narrative |
| B4 (pink accent) | SEMVER bump callout |
| Empty cell (mid-right) | Commit types tally (feat / fix / release) with colored bars, conventional-commit scopes |
| B5 (blue head, bottom-left) | Repo state snapshot: counts of recipes, docs harnesses, migration tests, CI workflows, skills, Rust crates |

The cover art is **text-with-purpose**, not decorative — every numeral and identifier is real. No version watermark/wordmark is added separately because the cover itself is a structured report.

## Rendering pipeline

See `cover.data.md` § Rendering pipeline. Abbreviated:

1. Compute scalar tokens and fragments from git.
2. Substitute into `cover.svg.template`.
3. Sanitize commit subjects / author names (`&` → `&amp;`, etc.).
4. Write to `.github/releases/v${VERSION}/cover.svg`.
5. `xmllint --noout` — abort release on failure.

## Usage in release notes

Embed with a tag-pinned absolute raw URL (relative paths don't resolve on GitHub Release pages):

```markdown
<div align="center">
  <img src="https://raw.githubusercontent.com/polkadot-developers/polkadot-cookbook/v{{VERSION}}/.github/releases/v{{VERSION}}/cover.svg" alt="Release v{{VERSION}}" width="100%" />
</div>
```

In the release **PR body**, use the branch's head commit SHA (not the branch name — it's deleted on merge, and the tag doesn't exist yet):

```
https://raw.githubusercontent.com/polkadot-developers/polkadot-cookbook/{commit-sha}/.github/releases/v{{VERSION}}/cover.svg
```

## Adding new facts

See `cover.data.md` § Adding new facts. Any new token or marker must ship with its git/filesystem source command and a length-bounded rendering rule.
