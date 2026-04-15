<!-- =============================================================================
     Polkadot Cookbook — Release PR Body Template
     =============================================================================
     DO NOT hand-edit per release. The /release skill substitutes tokens and fills
     markers, then passes the result to `gh pr create --body`.

     SCALAR TOKENS:
       {{VERSION}}           e.g. 0.14.0
       {{PREV_VERSION}}      e.g. 0.13.0
       {{BUMP_TYPE}}         MAJOR | MINOR | PATCH
       {{HEAD_SHA}}          7-char head commit SHA at PR open time
       {{COMMIT_COUNT}}      integer
       {{CONTRIB_COUNT}}     integer
       {{INSERTIONS}}        comma-grouped
       {{DELETIONS}}         comma-grouped
       {{FILES_COUNT}}       integer

     MARKER SECTIONS:
       <!-- @@SUMMARY -->    Same 2-3 sentence release summary used in
                             RELEASE_NOTES.md (copy, do not reinvent).
       <!-- @@WHATS_NEW -->  Bulleted headline changes for PR reviewers (shorter
                             than RELEASE_NOTES version; ~5-8 top bullets).
       <!-- @@BREAKING -->   Omitted if none. Otherwise: migration notes.

     COVER URL RULE:
       The cover image src in the PR body MUST use the release branch's head
       commit SHA, not the branch name (`release/v{VERSION}`) — the branch is
       deleted on merge and branch-based raw URLs break retroactively.
     ============================================================================= -->
<!-- TEMPLATE_HEADER_END -->

<div align="center">
  <img src="https://raw.githubusercontent.com/polkadot-developers/polkadot-cookbook/{{HEAD_SHA}}/.github/releases/v{{VERSION}}/cover.svg" alt="Release v{{VERSION}}" width="100%" />
</div>

## Release v{{VERSION}} · {{BUMP_TYPE}} bump

{{COMMIT_COUNT}} commits · {{CONTRIB_COUNT}} contributors · +{{INSERTIONS}} / -{{DELETIONS}} lines · {{FILES_COUNT}} files changed · `v{{PREV_VERSION}}` → `v{{VERSION}}`

## Summary

<!-- @@SUMMARY -->

## What's New

<!-- @@WHATS_NEW -->

<!-- @@BREAKING -->

## Test plan

- [ ] `cargo fmt --check --package sdk` passes
- [ ] `cargo clippy --package sdk --locked -- -D warnings` passes
- [ ] `cargo build --workspace --locked` succeeds
- [ ] `cargo test --package sdk --lib --locked -- --test-threads=1` passes
- [ ] `CHANGELOG.md` updated with this release's entries
- [ ] `Cargo.toml` workspace version bumped to `{{VERSION}}`
- [ ] `.github/releases/v{{VERSION}}/RELEASE_NOTES.md` reviewed and looks right
- [ ] `.github/releases/v{{VERSION}}/cover.svg` renders correctly (check in browser — xmllint alone doesn't prove it renders)
- [ ] `.github/releases/v{{VERSION}}/cover-chain.svg` renders correctly (skip check-off if the chain-RPC was unreachable and the footer cover was omitted)

## Next Steps

Merging this PR triggers [`publish-release.yml`](../blob/master/.github/workflows/publish-release.yml), which:
1. Creates the `v{{VERSION}}` git tag
2. Builds the `dot` CLI binaries for Linux, macOS (Intel + ARM), and Windows
3. Publishes the GitHub Release with cover art, manifest, and binaries attached
