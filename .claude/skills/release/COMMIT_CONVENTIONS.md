# Release Commit & Tag Conventions

Locked format for the release commit, tag, and branch. The /release skill must match these exactly — no variation.

## Branch

```
release/v{VERSION}
```

Branched from `master`. Deleted on merge.

## Release commit (on the release branch)

**Subject on the branch:** `chore(release): v{VERSION}` — required to pass the repo's `commit-msg` hook (`.cargo-husky/hooks/commit-msg`), which enforces conventional-commit prefixes and explicitly exempts `chore(release)`. `Release v{VERSION}` is not a conventional-commit type and will be rejected.

**PR title (and therefore squash-merge subject on `master`):** `Release v{VERSION}`

**Body (optional):** Empty; all narrative lives in `RELEASE_NOTES.md`.

GitHub derives the squashed commit on `master` from the **PR title**, not the branch commit. So setting the PR title to `Release v{VERSION}` produces the final commit subject `Release v{VERSION} (#N)` on `master` — which is what downstream tooling matches with the regex `^Release v[0-9]+\.[0-9]+\.[0-9]+ \(#[0-9]+\)$`. The `chore(release): v{VERSION}` branch commit is only visible in the PR diff view and is discarded on merge.

Do **not** bypass the hook with `--no-verify`. The two-subject pattern (branch vs. PR title) is intentional — it keeps both the local hook and the downstream tag regex happy without contorting either.

## Tag

- **Format:** `v{VERSION}` (e.g. `v0.14.0`), lightweight tag (not annotated — the GitHub Release carries the narrative).
- **Created by:** `.github/workflows/publish-release.yml` after the release commit lands on `master`.
- **Points at:** the squashed release commit on `master` (so `git log v0.13.0..v0.14.0` returns exactly the range of commits that the release incorporated).
- **Never moved.** Tags are immutable. If v0.14.0 ships with a bug, fix it in v0.14.1 — do not re-tag.

## Conventional-commit scopes used by the skill

Release notes and the cover art classify commits by subject prefix:

| Prefix | Icon | Role |
|---|---|---|
| `feat:` or `add:` or `feat(...)` | `»` | New feature / capability |
| `fix:` or `fix(...)` | `✓` | Bug fix |
| `chore:` or `chore(...)` | `✓` | Non-functional / tooling |
| `ci:` | `✓` | CI workflow change |
| `docs:` | `✓` | Documentation change |
| `refactor:` | `±` | Structural refactor |
| `Release v*` | `◆` | The release commit itself |
| *anything else* | `±` | Falls through to the generic icon |

These classifications feed `@@COMMIT_LIST` (cover.svg), `@@COMMIT_TYPES` (cover.svg), the `## Commits` section in `RELEASE_NOTES.md`, and the `What's New` grouping logic. A subject that doesn't match any known prefix gets the `±` icon and no semantic category — this is intentional: the skill should never guess a category for an ambiguous commit.

## What the skill must **not** do

- Edit an existing release commit (no `--amend`, no force-push to an already-pushed release branch).
- Create a tag locally — that's `publish-release.yml`'s job.
- Move or recreate a tag that already exists on the remote (would break immutability guarantees for anyone who already pulled it).
- Use `Release v{VERSION}` as the **branch** commit subject (the `commit-msg` hook rejects it). The string `Release v{VERSION}` belongs on the PR **title**, where GitHub turns it into the squash-merge subject on `master`.
- Bypass the `commit-msg` hook with `--no-verify` to force `Release v{VERSION}` on the branch.
