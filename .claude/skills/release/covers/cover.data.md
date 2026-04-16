# Release Cover Data Contract

Every value rendered on the cover comes from a git/filesystem query defined here. **No hand-entered values.** If a command fails, abort the release rather than ship fabricated data.

All queries assume `${PREV}` = previous tag (e.g. `v0.13.0`) and `${TAG}` = the new tag (e.g. `v0.14.0`).

---

## Scalar tokens

Substitute `{{TOKEN}}` in `cover.svg.template` with the command output.

| Token | Command | Notes |
|---|---|---|
| `{{VERSION}}` | `${TAG#v}` | Strip leading `v` |
| `{{PREV_VERSION}}` | `${PREV#v}` | |
| `{{BUMP_TYPE}}` | derive by comparing semver | `MAJOR` / `MINOR` / `PATCH` (uppercase) |
| `{{DATE_START}}` | `git log -1 --format='%ad' --date=short $(git rev-list ${PREV}..${TAG} | tail -1)` | ISO date of oldest commit in range |
| `{{DATE_END_SHORT}}` | `git log -1 ${TAG} --format='%ad' --date=format:'%m-%d'` | `MM-DD` of release commit |
| `{{DAYS}}` | integer days between DATE_START and release commit date (floor) | |
| `{{COMMIT_COUNT}}` | `git rev-list --count ${PREV}..${TAG}` | |
| `{{CONTRIB_COUNT}}` | `git shortlog -sn ${PREV}..${TAG} \| wc -l \| tr -d ' '` | |
| `{{INSERTIONS}}` | parse from `git diff --shortstat ${PREV}..${TAG}`, format with thousands separator | e.g. `16,756` |
| `{{DELETIONS}}` | same, deletions | |
| `{{FILES_COUNT}}` | parse from `git diff --shortstat ${PREV}..${TAG}` | |
| `{{HEAD_SHA}}` | `git rev-parse --short ${TAG}` | 7 chars |
| `{{PR_LIST}}` | `git log ${PREV}..${TAG} --format='%s' \| grep -oE '#[0-9]+' \| sort -u \| tr '\n' ' '` | space-separated `#N` |
| `{{DIFF_RATIO}}` | `$(( ${INSERTIONS} / ${DELETIONS} )):1` if deletions > 0 else `${INSERTIONS}:0` | integer divide |
| `{{DIFF_NARRATIVE}}` | see rule below | parenthetical |
| `{{SCOPES}}` | unique conventional-commit scopes from `git log ${PREV}..${TAG} --format='%s'`, joined by ` · ` | e.g. `docs · ci · skill` |

### `DIFF_NARRATIVE` rule

Compute once, use one of these fixed strings based on ratio and file breakdown:

- ratio ≥ 500:1 AND majority files under `polkadot-docs/` or `recipes/` → `(test-heavy release)`
- ratio ≥ 500:1 AND majority files under `dot/sdk` or `dot/cli` → `(code-additive release)`
- ratio < 5:1 AND deletions > insertions → `(cleanup release)`
- otherwise → `` (empty string)

---

## Variable-count fragments

Each marker comment `<!-- @@NAME -->` is replaced by a generated SVG fragment. Format strictly — every row is one `<text>` element with an `<animate>` child.

### `@@COMMIT_LIST` (in B2 terminal panel)

One `<text>` per commit, chronological oldest→newest. Staggered `begin` times.

**Source:** `git log ${PREV}..${TAG} --reverse --format='%h|%s'`

**Icon rules:**
- Subject starts with `feat` or `add` → `»` (feat)
- Subject starts with `fix`, `chore`, `ci`, `docs` (non-feat) → `✓`
- Subject starts with `Release v` (the release commit) → `◆`, fill `#E6007A`, opacity 0.95
- Otherwise → `±`

**Row template** (position y increments by 14 per row, starting at 46):
```xml
<text x="774" y="{Y}" font-family="ui-monospace, Menlo, Consolas, 'Courier New', monospace"
      font-size="10" fill="{FILL}" opacity="0">
  <animate attributeName="opacity" values="0;0.85" dur="0.12s" begin="{T}s" fill="freeze"/>
  │ {SHORT_SHA} {ICON} {SUBJECT}
</text>
```

`{T}` starts at 1.7s and increments by 0.2s per row.

**Scaling rule:**

| Commits | Layout |
|---|---|
| ≤13 | Single column, all rows shown, y step = 14px |
| 14-26 | Two columns (split at N/2), y step = 12px, subject truncated to 32 chars with `…` |
| ≥27 | Two columns, show latest 24, final row is `│ +{N-24} earlier commits` at low opacity |

**Sanitization:** escape `&` → `&amp;`, `<` → `&lt;`, `>` → `&gt;` in subjects. Strip characters outside ASCII + common box-drawing.

### `@@DAILY_TIMELINE` (in B1)

One `<text>` per calendar day in `DATE_START..DATE_END`. Gap days shown at low opacity. Starts at y=208, increments 18px.

**Source:** bucket commits by `git log --format='%ad' --date=format:'%a %m-%d'`, count per day.

**Row template:**
```xml
<text x="36" y="{Y}" font-size="12" opacity="{OP}">
  {DAY_LABEL}  {DOTS}{PADDING}<tspan opacity="0.6">{COUNT} {ANNOTATION}</tspan>
</text>
```

Where:
- `{DAY_LABEL}` = `Thu 04-09` style (3-letter weekday + MM-DD)
- `{DOTS}` = `●` × count, or `·` if count = 0
- `{OP}` = 0.9 if count > 0 else 0.45
- `{ANNOTATION}` = `commits`, `(incl. release)` on release day, `0` on gap days

**Scaling rule:** if range > 14 days, bucket by week instead of day (label: `wk 04-06`). If range > 90 days, bucket by month.

### `@@CONTRIBUTOR_LIST` (in B1)

One `<text>` per contributor, sorted by commit count descending. Starts at y=342, increments 18px.

**Source:** `git shortlog -sn ${PREV}..${TAG}`

**Row template:**
```xml
<text x="36" y="{Y}" font-size="12" opacity="0.9">
  {NAME_PADDED}<tspan opacity="0.6">{BAR}  {COUNT}</tspan>
</text>
```

- `{NAME_PADDED}` = name padded to 18 chars
- `{BAR}` = `█` × count (1 char per commit, cap at 30)

**Scaling rule:** show top 3 contributors, trailing row `… +N more` if more exist.

### `@@BAR_CHART` (in B3)

Three elements per row: label text (left), **animated rect bar** (middle, staggered fill-in), count text (right). Starts at y=466, increments 18px.

**Source:**
```bash
git diff --name-only ${PREV}..${TAG} | awk -F'/' 'NF>=2{print $1"/"$2} NF==1{print $0}' | sort | uniq -c | sort -rn
```

**Row template (per path):**
```xml
<text x="320" y="{Y}" font-size="12" opacity="0.9">{PATH}</text>
<rect x="600" y="{Y-9}" width="0" height="10" fill="#FFFFFF" opacity="0.5">
  <animate attributeName="width" from="0" to="{BAR_W}" dur="{DUR}s" begin="{T}s" fill="freeze"
           calcMode="spline" keySplines="0.2 0.8 0.2 1" keyTimes="0;1"/>
</rect>
<text x="{COUNT_X}" y="{Y}" font-size="12" opacity="0.95" font-weight="700">{COUNT}</text>
```

Computed values:
- `{BAR_W}` = `round(count / max_count * 500)`; max bar width = 500px (fits within B3)
- `{COUNT_X}` = `600 + {BAR_W} + 10` (count text sits 10px right of the bar's end)
- `{DUR}` = `0.5 + 0.2 * (count / max_count)` — top bar animates longest, trailing bars shorter
- `{T}` starts at 3.3s, increments by 0.1s per row

**Scaling rule:** show top 8 paths; group remainder into final row `other` with summed count if >8. Max bar width is fixed at 500px regardless of commit count — this prevents any overflow even for a single dominant path.

### Animated contributor bars (`@@CONTRIBUTOR_LIST` — updated)

Matches the bar-chart pattern: label + animated rect + count, instead of the previous text-based `█` bars. Bar width = `min(count * 20, 100)` (1 commit = 20px, cap at 100). Stagger begin at 1.3s, +0.15s per row.

### Animated commit-types bars (`@@COMMIT_TYPES` — updated)

Same pattern. Bar fill is `#E6007A`; fix commits use `opacity="0.55"` to differentiate from feat/release (which use full opacity). This keeps the strict 3-color palette (pink/black/white) — status differentiation is achieved with opacity, not a secondary hue. Bar width = `min(count * 15, 60)`. Stagger begin at 4.2s, +0.15s per row.

### Why animated rects instead of text `█` characters

- **Precise width control.** Text bars use character count; character widths vary subtly across platforms and can overflow a fixed cell.
- **Progressive reveal.** `<rect>` width animates smoothly via SMIL `<animate>`, reinforcing the "data being plotted" feel — a short cosmetic win without cost.
- **Length-scaled to the max.** Rect bars always fit the cell regardless of commit count (1 commit or 200), because scaling is relative to the row with the highest count.

### `@@COMMIT_TYPES` (in empty cell)

Three fixed rows (feat / fix / release), counts from `@@COMMIT_LIST` icon assignment. Starts at y=302, increments 18px.

**Row template:**
```xml
<text x="946" y="{Y}" font-size="12" opacity="0.85">
  {GLYPH} {LABEL_PADDED}<tspan fill="{COLOR}">{BARS}</tspan>  {COUNT}
</text>
```

| Glyph | Label | Color | Counted as |
|---|---|---|---|
| `»` | `feat` | `#E6007A` | feat / add commits |
| `✓` | `fix` | `#E6007A" opacity="0.55` | fix / chore / ci / docs / refactor |
| `◆` | `release` | `#E6007A` | `Release v*` commit |

`{BARS}` = `█` × count, capped at 8.

### `@@REPO_STATE` (in B5)

Six fixed rows of current counts at the release tag. Starts at y=466, increments 20px.

**Sources:**

| Row label | Command |
|---|---|
| `docs test harnesses` | `find polkadot-docs -name 'docs.test.ts' \| wc -l` |
| `recipes` | `find recipes -name 'recipe.test.ts' \| wc -l` |
| `migration tests` | `find migration -name 'migration.test.ts' 2>/dev/null \| wc -l` |
| `CI workflows` | `ls .github/workflows/*.yml \| wc -l` |
| `Claude skills` | `ls .claude/skills \| wc -l` |
| `Rust crates` | count `[package]` sections across workspace Cargo.toml files |

**Row template:**
```xml
<text x="18" y="{Y}" font-size="13" opacity="0.95">
  <tspan fill="#E6007A" font-weight="700">{COUNT_PADDED}</tspan>  {LABEL}
</text>
```

`{COUNT_PADDED}` = right-aligned to 2 chars.

---

## Rendering pipeline

The /release skill performs:

1. Compute all scalar tokens from the command table above.
2. Generate all six variable fragments.
3. Read `cover.svg.template`, perform substitutions (scalars first, then markers).
4. Sanitize all injected commit subjects and author names.
5. Write to `.github/releases/v${VERSION}/cover.svg`.
6. Run `xmllint --noout .github/releases/v${VERSION}/cover.svg` — if it fails, abort.
7. Commit the generated cover alongside `RELEASE_NOTES.md` and `manifest.yml`.

Any command failure in steps 1–2 aborts the release. No fallbacks, no placeholders, no fabricated values.

## Adding new facts

If a future release wants to surface something new on the cover:

1. Add the computed token or marker here in `cover.data.md`.
2. Add the substitution/marker to `cover.svg.template`.
3. Keep the rendered value length bounded so it fits the Mondrian cell geometry (test with edge cases: 1 commit, 200 commits).
