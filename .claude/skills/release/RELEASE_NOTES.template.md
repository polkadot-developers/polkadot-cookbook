<!-- =============================================================================
     Polkadot Cookbook — Release Notes Template
     =============================================================================
     DO NOT hand-edit per release. The /release skill substitutes scalars and fills
     marker sections, then writes to .github/releases/v{VERSION}/RELEASE_NOTES.md.

     SCALAR TOKENS:
       {{VERSION}}           e.g. 0.14.0
       {{PREV_VERSION}}      e.g. 0.13.0
       {{RELEASE_DATE}}      YYYY-MM-DD (release commit date, UTC)
       {{RUST_VERSION}}      e.g. 1.91.0
       {{NODE_VERSION}}      e.g. v24.7.0
       {{COMMIT_COUNT}}      integer
       {{INSERTIONS}}        comma-grouped
       {{DELETIONS}}         comma-grouped

     MARKER SECTIONS (LLM-filled narrative inside locked scaffolding):
       <!-- @@SUMMARY -->       2-3 sentences: what this release delivers and why.
                                Lead with the most impactful change.
       <!-- @@BREAKING -->      Omitted entirely if no breaking changes. If present,
                                emit `## Breaking Changes` with a bullet list of what
                                broke and migration steps.
       <!-- @@WHATS_NEW -->     ## What's New body — category subheadings (### ...)
                                and bulleted entries with PR links (#N).
       <!-- @@COMMITS -->       Bulleted list of commit subjects with PR links,
                                one line per commit, newest first.
     ============================================================================= -->
<!-- TEMPLATE_HEADER_END -->

<div align="center">
  <img src="https://raw.githubusercontent.com/polkadot-developers/polkadot-cookbook/v{{VERSION}}/.github/releases/v{{VERSION}}/cover.svg" alt="Release v{{VERSION}}" width="100%" />
</div>

# Release v{{VERSION}}

Released: {{RELEASE_DATE}}

## Summary

<!-- @@SUMMARY -->

<!-- @@BREAKING -->

## What's New

<!-- @@WHATS_NEW -->

## Commits

<!-- @@COMMITS -->

## Stats

**{{COMMIT_COUNT}} commits, +{{INSERTIONS}} / -{{DELETIONS}} lines**

**Full Changelog:** https://github.com/polkadot-developers/polkadot-cookbook/compare/v{{PREV_VERSION}}...v{{VERSION}}

## Compatibility

Tested with:
- Rust: {{RUST_VERSION}}
- Node.js: {{NODE_VERSION}}

## Next Steps

Merging the release PR triggers [`publish-release.yml`](../../../.github/workflows/publish-release.yml), which:
1. Creates the `v{{VERSION}}` git tag
2. Builds the `dot` CLI binaries for Linux, macOS (Intel + ARM), and Windows
3. Publishes the GitHub Release with cover art, manifest, and binaries attached

---

**Status:** Alpha (v0.x.x)

---

<div align="center">
  <img src="https://raw.githubusercontent.com/polkadot-developers/polkadot-cookbook/v{{VERSION}}/.github/releases/v{{VERSION}}/cover-chain.svg" alt="Polkadot network state at v{{VERSION}} release" width="100%" />
</div>
