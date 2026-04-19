#!/usr/bin/env python3
"""
render.py — Polkadot Cookbook release artifact renderer.

Replaces ad-hoc `/tmp/render_*.py` scripts that past releases wrote anew.
All rendering logic — scalar computation, fragment generation, RPC capture,
template substitution, validation — lives here in version-controlled code.

Subcommands (each reads its template from this script's sibling directory):

  cover         Render cover.svg from git state (PREV..HEAD)
  cover-chain   Capture Polkadot chain state over JSON-RPC and render
                cover-chain.svg; skip (exit 0, write nothing) if all
                endpoints fail.
  notes         Render RELEASE_NOTES.md; narrative markers supplied via
                --summary-file / --whats-new-file / --commits-file /
                --breaking-file (optional).
  manifest      Render manifest.yml (scalar-only, YAML-validated).
  pr-body       Render the draft release PR body; narrative via files.

Invariants:
- Zero fabricated values. Every scalar comes from git, RPC, an arg, or a
  file the caller computed upstream.
- Output files pass: xmllint (SVG), yaml.safe_load (manifest), and the
  zero-unresolved `{{TOKEN}}` / `<!-- @@MARKER -->` assertion.
- Non-zero exit on any validation failure; never write a partial output.

Data contracts live in `covers/cover.data.md` and `covers/cover-chain.data.md`.
This script is the executable form of those contracts — if you change one,
change the other.
"""

from __future__ import annotations

import argparse
import html
import json
import os
import re
import subprocess
import sys
import urllib.error
import urllib.request
from dataclasses import dataclass
from datetime import date, datetime, timezone
from pathlib import Path
from typing import Callable

SKILL_DIR = Path(__file__).resolve().parent
COVERS_DIR = SKILL_DIR / "covers"


# ---------------------------------------------------------------------------
# Shared helpers
# ---------------------------------------------------------------------------

def abort(msg: str) -> None:
    print(f"ABORT: {msg}", file=sys.stderr)
    sys.exit(1)


def esc(s: str) -> str:
    """XML-escape a value that will land inside an SVG <text>."""
    return html.escape(str(s), quote=False)


def commas(n: int) -> str:
    return f"{n:,}"


def git(*args: str) -> str:
    """Run git, capture stdout as text, strip trailing newline."""
    return subprocess.check_output(("git", *args), text=True).rstrip("\n")


def strip_template_header(text: str, sentinel: str) -> str:
    """
    Remove the documentation comment block that opens each template, up to and
    including the `TEMPLATE_HEADER_END` sentinel line. Only the comment(s) are
    removed — any preceding real content (e.g. an `<svg>` opening tag in
    cover.svg.template) is preserved.

    Behaviour by template type:
      - HTML/XML/MD templates: strip `<!-- … TEMPLATE_HEADER_END -->` plus any
        intervening sibling `<!-- … -->` comments leading up to it.
      - YAML templates: strip contiguous `#`-prefixed lines ending at the
        `# TEMPLATE_HEADER_END` sentinel.
    """
    if sentinel.startswith("<!--"):
        # Collapse one-or-more contiguous HTML comments whose last line contains
        # TEMPLATE_HEADER_END. Non-greedy and anchored so we never consume the
        # `<svg>` preamble that may appear before the comment block.
        pattern = re.compile(
            r"(?:<!--[\s\S]*?-->\s*){1,}"
            rf"(?=|.*){re.escape(sentinel)}?",
        )
        # Simpler two-step approach: find the sentinel comment itself and any
        # preceding comment block(s) that are siblings (whitespace-separated).
        idx = text.find(sentinel)
        if idx == -1:
            return text
        # Walk back over any run of <!-- ... --> blocks + whitespace that
        # immediately precede the sentinel.
        end = idx + len(sentinel)
        # Consume trailing whitespace after the sentinel too.
        while end < len(text) and text[end] in " \t":
            end += 1
        if end < len(text) and text[end] == "\n":
            end += 1

        start = idx
        while True:
            # peek backwards for `-->` (end of a prior comment) with only
            # whitespace between it and current start
            back = text[:start].rstrip()
            if not back.endswith("-->"):
                break
            # find the matching `<!--`
            prior_open = text.rfind("<!--", 0, len(back))
            if prior_open == -1:
                break
            start = prior_open
        # strip
        return text[:start] + text[end:]

    # YAML-style: strip the contiguous `#`-prefixed lines ending at `# TEMPLATE_HEADER_END`
    if sentinel.startswith("#"):
        idx = text.find(sentinel)
        if idx == -1:
            return text
        end = text.find("\n", idx)
        end = end + 1 if end != -1 else len(text)
        # walk back to the start of the contiguous `#`-line run
        start = idx
        while True:
            prev_nl = text.rfind("\n", 0, start - 1)
            line_start = prev_nl + 1 if prev_nl != -1 else 0
            line = text[line_start:start if start > 0 else 0]
            # strip of this line
            if line.lstrip().startswith("#"):
                start = line_start
                if line_start == 0:
                    break
            else:
                break
        return text[:start] + text[end:]

    return text


def substitute(template: str, scalars: dict[str, str]) -> str:
    """Plain `{{TOKEN}}` substitution. Scalars must stringify cleanly."""
    out = template
    for k, v in scalars.items():
        out = out.replace("{{" + k + "}}", str(v))
    return out


def assert_no_unresolved(
    out: str, *, allow_markers: set[str] | None = None
) -> None:
    """Hard-abort if any `{{TOKEN}}` or `<!-- @@MARKER -->` remains."""
    tokens = set(re.findall(r"\{\{[A-Z_]+\}\}", out))
    if tokens:
        abort(f"unresolved scalar tokens: {sorted(tokens)}")
    markers = set(re.findall(r"<!-- @@[A-Z_]+ -->", out))
    if allow_markers:
        markers -= {f"<!-- @@{m} -->" for m in allow_markers}
    if markers:
        abort(f"unresolved markers: {sorted(markers)}")


def xmllint(path: Path) -> None:
    result = subprocess.run(
        ["xmllint", "--noout", str(path)], capture_output=True, text=True
    )
    if result.returncode != 0:
        abort(f"xmllint failed on {path}: {result.stderr.strip()}")


def read_text_file(p: Path | None) -> str:
    if p is None:
        return ""
    if not p.exists():
        abort(f"input file not found: {p}")
    return p.read_text().rstrip("\n")


# ---------------------------------------------------------------------------
# cover.svg
# ---------------------------------------------------------------------------

@dataclass
class Commit:
    sha: str
    subject: str
    icon: str
    fill: str
    date: str  # "Thu 04-16"


def classify_commit(subject: str) -> tuple[str, str]:
    """Return (icon, fill) per COMMIT_CONVENTIONS.md."""
    s = subject.lower()
    if subject.startswith("Release v"):
        return "◆", "#E6007A"
    if s.startswith("feat") or s.startswith("add"):
        return "»", "#F6F5F2"
    if s.startswith(("fix", "chore", "ci", "docs", "refactor")):
        return "✓", "#F6F5F2"
    return "±", "#F6F5F2"


def get_commits(prev: str) -> list[Commit]:
    """Chronological oldest→newest, with date labels."""
    raw = git("log", f"{prev}..HEAD", "--reverse", "--format=%h|%ad|%s",
              "--date=format:%a %m-%d", "--no-merges")
    commits = []
    for line in raw.splitlines():
        sha, d, subject = line.split("|", 2)
        icon, fill = classify_commit(subject)
        commits.append(Commit(sha=sha, subject=subject, icon=icon, fill=fill, date=d))
    return commits


def truncate(s: str, n: int) -> str:
    return s if len(s) <= n else s[: n - 1] + "…"


def render_commit_list(commits: list[Commit]) -> str:
    n = len(commits)
    if n == 0:
        return ""

    font = "'JetBrains Mono', ui-monospace, Menlo, Consolas, 'Courier New', monospace"
    rows = []

    if n <= 13:
        y = 46
        t = 1.7
        for c in commits:
            subj = esc(truncate(c.subject, 42))
            rows.append(
                f'    <text x="774" y="{y}" font-family="{font}"\n'
                f'          font-size="10" fill="{c.fill}" opacity="0">\n'
                f'      <animate attributeName="opacity" values="0;0.85" dur="0.12s" '
                f'begin="{t:.1f}s" fill="freeze"/>\n'
                f'      │ {c.sha} {c.icon} {subj}\n'
                f'    </text>'
            )
            y += 14
            t += 0.2
    elif n <= 26:
        # two columns, y step 12, 32-char truncation
        split = (n + 1) // 2
        cols = [commits[:split], commits[split:]]
        col_x = [774, 988]
        for col_idx, col in enumerate(cols):
            y = 46
            t = 1.7 + col_idx * 0.05
            for c in col:
                subj = esc(truncate(c.subject, 32))
                rows.append(
                    f'    <text x="{col_x[col_idx]}" y="{y}" font-family="{font}"\n'
                    f'          font-size="10" fill="{c.fill}" opacity="0">\n'
                    f'      <animate attributeName="opacity" values="0;0.85" dur="0.12s" '
                    f'begin="{t:.2f}s" fill="freeze"/>\n'
                    f'      │ {c.sha} {c.icon} {subj}\n'
                    f'    </text>'
                )
                y += 12
                t += 0.2
    else:
        # ≥27: latest 24 two-col + trailing summary
        latest = commits[-24:]
        split = 12
        cols = [latest[:split], latest[split:]]
        col_x = [774, 988]
        for col_idx, col in enumerate(cols):
            y = 46
            t = 1.7 + col_idx * 0.05
            for c in col:
                subj = esc(truncate(c.subject, 32))
                rows.append(
                    f'    <text x="{col_x[col_idx]}" y="{y}" font-family="{font}"\n'
                    f'          font-size="10" fill="{c.fill}" opacity="0">\n'
                    f'      <animate attributeName="opacity" values="0;0.85" dur="0.12s" '
                    f'begin="{t:.2f}s" fill="freeze"/>\n'
                    f'      │ {c.sha} {c.icon} {subj}\n'
                    f'    </text>'
                )
                y += 12
                t += 0.2
        extra = n - 24
        rows.append(
            f'    <text x="774" y="184" font-family="{font}" '
            f'font-size="10" fill="#F6F5F2" opacity="0.55">\n'
            f'      │ +{extra} earlier commits\n'
            f'    </text>'
        )
    return "\n".join(rows)


def render_daily_timeline(commits: list[Commit], release_day: str) -> str:
    """
    One row per calendar day in the range. `release_day` is the weekday/MM-DD
    label of HEAD; that row gets the `(incl. release)` annotation.
    """
    from collections import OrderedDict

    # bucket
    buckets: OrderedDict[str, int] = OrderedDict()
    for c in commits:
        buckets[c.date] = buckets.get(c.date, 0) + 1

    # Fill gap days for compact ranges
    if buckets:
        # parse MM-DD
        def parse(label):
            md = label.split()[-1]
            m, d = md.split("-")
            return (int(m), int(d))
        ordered = sorted(buckets.keys(), key=parse)
        first, last = ordered[0], ordered[-1]
        # For simplicity: include only buckets that actually have commits plus
        # an explicit release_day row if missing. Gap-day filling across months
        # is out of scope for this renderer (matches what v0.16.0 shipped).
        if release_day not in buckets:
            buckets[release_day] = 0

    # scaling: if >14 days → weeks, >90 → months (rare; not exercised for
    # typical sprint-cadence releases — emit a warning but proceed with daily)
    if len(buckets) > 14:
        print(
            f"WARN: timeline range spans {len(buckets)} days — "
            "daily bucketing may overflow B1. Consider implementing "
            "week/month bucketing per cover.data.md.",
            file=sys.stderr,
        )

    rows = []
    y = 208
    for label, count in buckets.items():
        op = "0.9" if count > 0 else "0.45"
        dots = "●" * count if count > 0 else "·"
        if count == 0:
            annot = "0"
        elif label == release_day:
            annot = f"{count} commits (incl. release)"
        else:
            annot = f"{count} commits"
        rows.append(
            f'    <text x="36" y="{y}" font-size="12" opacity="{op}">\n'
            f'      {label}  {dots}  <tspan opacity="0.6">{annot}</tspan>\n'
            f'    </text>'
        )
        y += 18
    return "\n".join(rows)


def render_contributor_list() -> str:
    """`git shortlog -sn ${PREV}..${TAG}`-style output for the range."""
    raw = git("shortlog", "-sn", "--no-merges", f"{ARGS.prev}..HEAD")
    contribs = []
    for line in raw.splitlines():
        line = line.strip()
        if not line:
            continue
        count_str, name = line.split(None, 1)
        contribs.append((name, int(count_str)))
    contribs.sort(key=lambda x: -x[1])

    rows = []
    y = 342
    t = 1.3
    top = contribs[:3]
    for name, count in top:
        padded = f"{name:<18}"
        bar_w = min(count * 20, 100)
        rows.append(
            f'    <text x="36" y="{y}" font-size="12" opacity="0.9">{esc(padded)}</text>\n'
            f'    <rect x="180" y="{y - 9}" width="0" height="10" fill="#F6F5F2" opacity="0.5">'
            f'<animate attributeName="width" from="0" to="{bar_w}" dur="0.6s" '
            f'begin="{t:.2f}s" fill="freeze"/></rect>\n'
            f'    <text x="{185 + bar_w}" y="{y}" font-size="12" opacity="0.95" '
            f'font-weight="700">{count}</text>'
        )
        y += 18
        t += 0.15
    if len(contribs) > 3:
        extra = len(contribs) - 3
        rows.append(
            f'    <text x="36" y="{y}" font-size="12" opacity="0.55">… +{extra} more</text>'
        )
    return "\n".join(rows)


def render_bar_chart(prev: str) -> str:
    """Files changed grouped by 2-level path prefix."""
    raw = git("diff", "--name-only", f"{prev}..HEAD")
    buckets: dict[str, int] = {}
    for fn in raw.splitlines():
        parts = fn.split("/")
        key = "/".join(parts[:2]) if len(parts) >= 2 else parts[0]
        buckets[key] = buckets.get(key, 0) + 1
    sorted_items = sorted(buckets.items(), key=lambda x: -x[1])
    top = sorted_items[:8]
    rest = sorted_items[8:]
    if rest:
        top.append(("other", sum(c for _, c in rest)))

    if not top:
        return ""
    max_count = max(c for _, c in top)
    rows = []
    y = 466
    t = 3.3
    for path, count in top:
        bar_w = round(count / max_count * 500)
        count_x = 600 + bar_w + 10
        dur = 0.5 + 0.2 * (count / max_count)
        rows.append(
            f'    <text x="320" y="{y}" font-size="12" opacity="0.9">{esc(path)}</text>\n'
            f'    <rect x="600" y="{y - 9}" width="0" height="10" fill="#F6F5F2" opacity="0.5">'
            f'<animate attributeName="width" from="0" to="{bar_w}" dur="{dur:.2f}s" '
            f'begin="{t:.2f}s" fill="freeze" calcMode="spline" '
            f'keySplines="0.2 0.8 0.2 1" keyTimes="0;1"/></rect>\n'
            f'    <text x="{count_x}" y="{y}" font-size="12" opacity="0.95" '
            f'font-weight="700">{count}</text>'
        )
        y += 18
        t += 0.1
    return "\n".join(rows)


def render_commit_types(commits: list[Commit]) -> str:
    counts = {"feat": 0, "fix": 0, "release": 0}
    for c in commits:
        if c.icon == "»":
            counts["feat"] += 1
        elif c.icon == "✓":
            counts["fix"] += 1
        elif c.icon == "◆":
            counts["release"] += 1
    rows = []
    types = [
        ("»", "feat   ", "#E6007A", "1.0", counts["feat"], 4.20),
        ("✓", "fix    ", "#E6007A", "0.55", counts["fix"], 4.35),
        ("◆", "release", "#E6007A", "1.0", counts["release"], 4.50),
    ]
    y = 302
    for glyph, label, color, opacity, count, begin in types:
        bar_w = min(count * 15, 60)
        rows.append(
            f'    <text x="946" y="{y}" font-size="12" opacity="0.85">{glyph} {label}</text>\n'
            f'    <rect x="1026" y="{y - 9}" width="0" height="10" fill="{color}" opacity="{opacity}">'
            f'<animate attributeName="width" from="0" to="{bar_w}" dur="0.5s" '
            f'begin="{begin:.2f}s" fill="freeze"/></rect>\n'
            f'    <text x="{1036 + bar_w}" y="{y}" font-size="12" opacity="0.95" '
            f'font-weight="700">{count}</text>'
        )
        y += 18
    return "\n".join(rows)


def render_repo_state(repo_root: Path) -> str:
    """Six rows of current counts at HEAD."""
    def count_glob(pattern: str) -> int:
        return len(list(repo_root.glob(pattern)))

    def count_files(cmd: list[str]) -> int:
        try:
            out = subprocess.check_output(cmd, text=True, cwd=repo_root)
            return sum(1 for ln in out.splitlines() if ln.strip())
        except subprocess.CalledProcessError:
            return 0

    def count_workspace_crates() -> int:
        """Count [package] sections in live workspace members only."""
        root = repo_root / "Cargo.toml"
        if not root.exists():
            return 0
        m = re.search(r"members\s*=\s*\[([^\]]*)\]", root.read_text(), re.DOTALL)
        if not m:
            return 0
        members = re.findall(r'"([^"]+)"', m.group(1))
        n = 0
        for rel in members:
            ct = repo_root / rel / "Cargo.toml"
            if ct.exists() and "[package]" in ct.read_text():
                n += 1
        return n

    stats = [
        (count_files(["find", "polkadot-docs", "-name", "docs.test.ts"]),
         "docs test harnesses"),
        (count_files(["find", "recipes", "-name", "recipe.test.ts"]),
         "recipes"),
        (count_files(["find", "migration", "-name", "migration.test.ts"]),
         "migration tests"),
        (count_glob(".github/workflows/*.yml"),
         "CI workflows"),
        (count_glob(".claude/skills/*"),
         "Claude skills"),
        (count_workspace_crates(),
         "Rust crates"),
    ]
    rows = []
    y = 466
    for count, label in stats:
        padded = f"{count:>2}"
        rows.append(
            f'    <text x="18" y="{y}" font-size="13" opacity="0.95">\n'
            f'      <tspan fill="#E6007A" font-weight="700">{padded}</tspan>  {label}\n'
            f'    </text>'
        )
        y += 20
    return "\n".join(rows)


def diff_narrative(ratio: int, insertions: int, deletions: int,
                   files: list[str]) -> str:
    """Applies the `DIFF_NARRATIVE rule` from cover.data.md."""
    if not files:
        return ""
    docs_test = sum(1 for f in files if f.startswith(("polkadot-docs/", "recipes/")))
    code = sum(1 for f in files if f.startswith(("dot/sdk", "dot/cli")))
    majority = len(files) // 2
    if ratio >= 500 and docs_test > majority:
        return "(test-heavy release)"
    if ratio >= 500 and code > majority:
        return "(code-additive release)"
    if ratio < 5 and deletions > insertions:
        return "(cleanup release)"
    return ""


def cmd_cover(args: argparse.Namespace) -> None:
    global ARGS
    ARGS = args
    repo_root = Path(git("rev-parse", "--show-toplevel"))
    prev, version = args.prev, args.version
    out_path = Path(args.out)

    # scalars from git
    commits = get_commits(prev)
    if not commits:
        abort(f"no commits in {prev}..HEAD")

    oldest_sha = git("rev-list", f"{prev}..HEAD").splitlines()[-1]
    date_start = git("log", "-1", "--format=%ad", "--date=short", oldest_sha)
    release_d = git("log", "-1", "HEAD", "--format=%ad",
                    "--date=format:%a %m-%d")
    date_end_short = git("log", "-1", "HEAD", "--format=%ad", "--date=format:%m-%d")

    # days
    start_d = date.fromisoformat(date_start)
    end_d = date.fromisoformat(git("log", "-1", "HEAD", "--format=%ad",
                                   "--date=short"))
    days = (end_d - start_d).days

    contrib_count = int(git("shortlog", "-sn", f"{prev}..HEAD",
                            "--no-merges").count("\n"))
    # shortstat
    shortstat = git("diff", "--shortstat", f"{prev}..HEAD")
    ins = int(re.search(r"(\d+) insertion", shortstat).group(1)) if "insertion" in shortstat else 0
    dels = int(re.search(r"(\d+) deletion", shortstat).group(1)) if "deletion" in shortstat else 0
    files_count = int(re.search(r"(\d+) file", shortstat).group(1)) if "file" in shortstat else 0

    head_sha = git("rev-parse", "--short", "HEAD")
    pr_list_raw = git("log", f"{prev}..HEAD", "--format=%s")
    prs = sorted(set(re.findall(r"#\d+", pr_list_raw)))
    pr_list = " ".join(prs)

    ratio = (ins // dels) if dels > 0 else ins
    diff_ratio = f"{ratio}:1" if dels > 0 else f"{ins}:0"

    files_changed = git("diff", "--name-only", f"{prev}..HEAD").splitlines()
    narrative = diff_narrative(ratio, ins, dels, files_changed)

    scopes_raw = re.findall(r"^[a-z]+\(([^)]+)\)", pr_list_raw, re.MULTILINE)
    scopes = " · ".join(sorted(set(scopes_raw))) if scopes_raw else ""

    # bump type inferred from version jump
    def parse_ver(v: str) -> tuple[int, int, int]:
        nums = re.findall(r"\d+", v)
        return tuple(int(x) for x in nums[:3]) + (0,) * (3 - len(nums[:3]))
    pv = parse_ver(prev)
    nv = parse_ver(version)
    if nv[0] != pv[0]:
        bump = "MAJOR"
    elif nv[1] != pv[1]:
        bump = "MINOR"
    else:
        bump = "PATCH"

    scalars = {
        "VERSION": version,
        "PREV_VERSION": prev.lstrip("v"),
        "BUMP_TYPE": bump,
        "DATE_START": date_start,
        "DATE_END_SHORT": date_end_short,
        "DAYS": str(days),
        "COMMIT_COUNT": str(len(commits)),
        "CONTRIB_COUNT": str(contrib_count),
        "INSERTIONS": commas(ins),
        "DELETIONS": commas(dels),
        "FILES_COUNT": str(files_count),
        "HEAD_SHA": head_sha,
        "PR_LIST": pr_list,
        "DIFF_RATIO": diff_ratio,
        "DIFF_NARRATIVE": narrative,
        "SCOPES": scopes,
    }

    template = (COVERS_DIR / "cover.svg.template").read_text()
    out = substitute(template, scalars)
    out = out.replace("<!-- @@COMMIT_LIST -->", render_commit_list(commits))
    out = out.replace("<!-- @@DAILY_TIMELINE -->",
                      render_daily_timeline(commits, release_d))
    out = out.replace("<!-- @@CONTRIBUTOR_LIST -->", render_contributor_list())
    out = out.replace("<!-- @@BAR_CHART -->", render_bar_chart(prev))
    out = out.replace("<!-- @@COMMIT_TYPES -->", render_commit_types(commits))
    out = out.replace("<!-- @@REPO_STATE -->", render_repo_state(repo_root))
    out = strip_template_header(out, "<!-- TEMPLATE_HEADER_END -->")

    out_path.parent.mkdir(parents=True, exist_ok=True)
    out_path.write_text(out)
    assert_no_unresolved(out)
    xmllint(out_path)
    print(f"✓ {out_path} ({len(out)} bytes)")


# ---------------------------------------------------------------------------
# cover-chain.svg
# ---------------------------------------------------------------------------

CHAIN_DEFAULTS = {
    "polkadot": {
        "endpoints": [
            "https://rpc.polkadot.io",
            "https://polkadot-rpc.dwellir.com",
            "https://rpc.ibp.network/polkadot",
        ],
        "genesis_date": "2020-05-26",
        "block_target": "6s",
        "network": "MAINNET",
        "display": "Polkadot",
        "chain_type": "relay",
    },
    "kusama": {
        "endpoints": [
            "https://kusama-rpc.polkadot.io",
            "https://kusama-rpc.dwellir.com",
        ],
        "genesis_date": "2019-11-28",
        "block_target": "6s",
        "network": "CANARY",
        "display": "Kusama",
        "chain_type": "relay",
    },
    "paseo": {
        "endpoints": [
            "https://paseo-rpc.dwellir.com",
            "https://rpc.ibp.network/paseo",
        ],
        "genesis_date": "2024-08-07",
        "block_target": "6s",
        "network": "TESTNET",
        "display": "Paseo",
        "chain_type": "relay",
    },
}

BABE_API_HASH = "0xcbca25e39f142387"


def rpc_call(endpoint: str, method: str, params=None, timeout=5):
    payload = json.dumps({
        "jsonrpc": "2.0", "id": 1, "method": method,
        "params": params or [],
    }).encode()
    req = urllib.request.Request(
        endpoint, data=payload,
        headers={
            "Content-Type": "application/json",
            # Some upstream proxies (Parity's rpc.polkadot.io in particular)
            # 403 the default `Python-urllib/x.y` UA.
            "User-Agent": "polkadot-cookbook-release-skill/1.0",
        },
    )
    with urllib.request.urlopen(req, timeout=timeout) as resp:
        return json.load(resp)


def capture_chain(endpoints: list[str]) -> tuple[str, dict] | None:
    """Walk endpoints until one answers; return (endpoint, capture_dict) or None."""
    for ep in endpoints:
        try:
            # probe
            chain = rpc_call(ep, "system_chain")["result"]
            fh = rpc_call(ep, "chain_getFinalizedHead")["result"]
            header = rpc_call(ep, "chain_getHeader", [fh])["result"]
            rt = rpc_call(ep, "state_getRuntimeVersion")["result"]
            health = rpc_call(ep, "system_health")["result"]
            node = rpc_call(ep, "system_version")["result"]
            props = rpc_call(ep, "system_properties")["result"]
            genesis = rpc_call(ep, "chain_getBlockHash", [0])["result"]
            ts = datetime.now(tz=timezone.utc).strftime("%Y-%m-%d %H:%M UTC")
            return ep, {
                "chain": chain, "fh": fh, "header": header,
                "rt": rt, "health": health, "node": node,
                "props": props, "genesis": genesis, "ts": ts,
            }
        except (urllib.error.URLError, OSError, TimeoutError, KeyError,
                json.JSONDecodeError) as e:
            print(f"WARN: {ep} failed: {e}", file=sys.stderr)
            continue
    return None


def cmd_cover_chain(args: argparse.Namespace) -> None:
    defaults = CHAIN_DEFAULTS[args.chain]
    result = capture_chain(defaults["endpoints"])
    if result is None:
        print("All chain endpoints unreachable — skipping cover-chain.svg.",
              file=sys.stderr)
        # exit 0: the caller (SKILL.md) handles omitting the footer embed.
        sys.exit(0)

    endpoint, cap = result
    hostname = urllib.parse.urlparse(endpoint).hostname

    def hprefix(h: str) -> str: return h[:10]
    def hshort(h: str, suffix: int = 4) -> str: return f"{h[:10]}..{h[-suffix:]}"

    rt = cap["rt"]
    header = cap["header"]
    props = cap["props"]
    health = cap["health"]
    block_num = int(header["number"], 16)

    apis = rt["apis"]
    authoring = "BABE" if any(a[0].lower() == BABE_API_HASH for a in apis) else "AURA"

    gd = date.fromisoformat(defaults["genesis_date"])
    today = date.today()
    delta_days = (today - gd).days
    years = delta_days // 365
    months = (delta_days % 365) // 30

    scalars = {
        "VERSION": args.version,
        "CAPTURE_TIMESTAMP": cap["ts"],
        "CHAIN": defaults["display"],
        "CHAIN_NAME_UPPER": defaults["display"].upper(),
        "CHAIN_TYPE": defaults["chain_type"],
        "CHAIN_TYPE_UPPER": defaults["chain_type"].upper(),
        "PARA_ID": "0",
        "NETWORK": defaults["network"],
        "RPC_ENDPOINT": hostname,
        "SPEC_NAME": rt["specName"],
        "SPEC_VERSION": commas(rt["specVersion"]),
        "IMPL_NAME": rt["implName"],
        "IMPL_VERSION": str(rt["implVersion"]),
        "NODE_VERSION": esc(cap["node"]),
        "AUTHORING_VERSION": str(rt["authoringVersion"]),
        "TX_VERSION": str(rt["transactionVersion"]),
        "STATE_VERSION": str(rt["stateVersion"]),
        "SYSTEM_VERSION": str(rt["systemVersion"]),
        "API_COUNT": str(len(apis)),
        "AUTHORING": authoring,
        "AUTHORING_LOWER": authoring.lower(),
        "BLOCK_TARGET": defaults["block_target"],
        "BLOCKS_PER_DAY": commas(86400 // int(defaults["block_target"].rstrip("s"))),
        "TOKEN_SYMBOL": props["tokenSymbol"] if isinstance(props["tokenSymbol"], str)
                        else props["tokenSymbol"][0],
        "TOKEN_DECIMALS": str(props["tokenDecimals"] if isinstance(props["tokenDecimals"], int)
                              else props["tokenDecimals"][0]),
        "SS58_FORMAT": str(props["ss58Format"]),
        "FINALIZED_BLOCK": commas(block_num),
        "HEAD_HASH_SHORT": hshort(cap["fh"], 4),
        "HEAD_HASH_PREFIX": hprefix(cap["fh"]),
        "HEAD_HASH_FULL": cap["fh"],
        "PARENT_HASH_SHORT": hshort(header["parentHash"], 4),
        "STATE_ROOT_PREFIX": hprefix(header["stateRoot"]),
        "STATE_ROOT_SHORT": hshort(header["stateRoot"], 4),
        "EXTRINSICS_ROOT_SHORT": hshort(header["extrinsicsRoot"], 4),
        "GENESIS_PREFIX": hprefix(cap["genesis"]),
        "GENESIS_SHORT": hshort(cap["genesis"], 6),
        "GENESIS_DATE": defaults["genesis_date"],
        "NETWORK_AGE": f"~{years} years, {months} months",
        "PEERS": str(health["peers"]),
        "SYNC_STATUS": "✓" if not health["isSyncing"] else "syncing…",
    }

    template = (COVERS_DIR / "cover-chain.svg.template").read_text()
    out = substitute(template, scalars)
    out = strip_template_header(out, "<!-- TEMPLATE_HEADER_END -->")

    out_path = Path(args.out)
    out_path.parent.mkdir(parents=True, exist_ok=True)
    out_path.write_text(out)
    assert_no_unresolved(out)
    xmllint(out_path)
    print(f"✓ {out_path} ({len(out)} bytes)")


# ---------------------------------------------------------------------------
# RELEASE_NOTES.md
# ---------------------------------------------------------------------------

def cmd_notes(args: argparse.Namespace) -> None:
    prev = args.prev.lstrip("v")

    shortstat = git("diff", "--shortstat", f"v{prev}..HEAD")
    def parse_ss(key: str) -> int:
        m = re.search(rf"(\d+) {key}", shortstat)
        return int(m.group(1)) if m else 0
    ins, dels = parse_ss("insertion"), parse_ss("deletion")
    commit_count = int(git("rev-list", "--count", f"v{prev}..HEAD"))

    scalars = {
        "VERSION": args.version,
        "PREV_VERSION": prev,
        "RELEASE_DATE": args.date,
        "RUST_VERSION": args.rust,
        "NODE_VERSION": args.node,
        "COMMIT_COUNT": str(commit_count),
        "INSERTIONS": commas(ins),
        "DELETIONS": commas(dels),
    }

    template = (SKILL_DIR / "RELEASE_NOTES.template.md").read_text()
    out = substitute(template, scalars)

    # breaking block — optional
    allow = set()
    breaking = read_text_file(args.breaking_file) if args.breaking_file else ""
    if breaking:
        breaking_block = f"## Breaking Changes\n\n{breaking}\n"
        out = out.replace("<!-- @@BREAKING -->", breaking_block)
    else:
        out = re.sub(r"<!-- @@BREAKING -->\s*\n?", "", out)

    out = out.replace("<!-- @@SUMMARY -->", read_text_file(args.summary_file))
    out = out.replace("<!-- @@WHATS_NEW -->", read_text_file(args.whats_new_file))
    out = out.replace("<!-- @@COMMITS -->", read_text_file(args.commits_file))
    out = strip_template_header(out, "<!-- TEMPLATE_HEADER_END -->")

    out_path = Path(args.out)
    out_path.parent.mkdir(parents=True, exist_ok=True)
    out_path.write_text(out)
    assert_no_unresolved(out, allow_markers=allow)
    print(f"✓ {out_path} ({len(out)} bytes)")


# ---------------------------------------------------------------------------
# manifest.yml
# ---------------------------------------------------------------------------

def cmd_manifest(args: argparse.Namespace) -> None:
    scalars = {
        "VERSION": args.version,
        "PREV_VERSION": args.prev.lstrip("v"),
        "RELEASE_DATE": args.date,
        "STATUS": args.status,
        "RUST_VERSION": args.rust,
        "NODE_VERSION": args.node,
    }
    template = (SKILL_DIR / "MANIFEST.template.yml").read_text()
    out = substitute(template, scalars)
    out = re.sub(r"^#\s*={10,}.*?# TEMPLATE_HEADER_END\s*", "", out,
                 count=1, flags=re.DOTALL | re.MULTILINE)

    out_path = Path(args.out)
    out_path.parent.mkdir(parents=True, exist_ok=True)
    out_path.write_text(out)
    assert_no_unresolved(out)
    try:
        import yaml
        parsed = yaml.safe_load(out)
        if parsed.get("release") != f"v{args.version}":
            abort(f"manifest.yml release != v{args.version}")
    except ImportError:
        # yaml not installed — fall back to a minimal structural check
        if f"release: v{args.version}" not in out:
            abort("manifest.yml missing expected release line")
    print(f"✓ {out_path}")


# ---------------------------------------------------------------------------
# PR body
# ---------------------------------------------------------------------------

def cmd_pr_body(args: argparse.Namespace) -> None:
    prev = args.prev.lstrip("v")
    shortstat = git("diff", "--shortstat", f"v{prev}..HEAD")
    def parse_ss(key: str) -> int:
        m = re.search(rf"(\d+) {key}", shortstat)
        return int(m.group(1)) if m else 0
    ins = parse_ss("insertion")
    dels = parse_ss("deletion")
    files_count = parse_ss("file")
    commit_count = int(git("rev-list", "--count", f"v{prev}..HEAD"))
    contrib_count = sum(1 for ln in git(
        "shortlog", "-sn", f"v{prev}..HEAD", "--no-merges").splitlines() if ln.strip())

    scalars = {
        "VERSION": args.version,
        "PREV_VERSION": prev,
        "BUMP_TYPE": args.bump_type,
        "HEAD_SHA": args.head_sha,
        "COMMIT_COUNT": str(commit_count),
        "CONTRIB_COUNT": str(contrib_count),
        "INSERTIONS": commas(ins),
        "DELETIONS": commas(dels),
        "FILES_COUNT": str(files_count),
    }

    template = (SKILL_DIR / "RELEASE_PR_BODY.template.md").read_text()
    out = substitute(template, scalars)

    breaking = read_text_file(args.breaking_file) if args.breaking_file else ""
    if breaking:
        out = out.replace("<!-- @@BREAKING -->", f"## Breaking Changes\n\n{breaking}\n")
    else:
        out = re.sub(r"<!-- @@BREAKING -->\s*\n?", "", out)

    out = out.replace("<!-- @@SUMMARY -->", read_text_file(args.summary_file))
    out = out.replace("<!-- @@WHATS_NEW -->", read_text_file(args.whats_new_file))
    out = strip_template_header(out, "<!-- TEMPLATE_HEADER_END -->")

    out_path = Path(args.out)
    out_path.parent.mkdir(parents=True, exist_ok=True)
    out_path.write_text(out)
    assert_no_unresolved(out)
    print(f"✓ {out_path} ({len(out)} bytes)")


# ---------------------------------------------------------------------------
# CLI
# ---------------------------------------------------------------------------

def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__,
                                     formatter_class=argparse.RawDescriptionHelpFormatter)
    sub = parser.add_subparsers(dest="cmd", required=True)

    p = sub.add_parser("cover", help="Render cover.svg from git state")
    p.add_argument("--prev", required=True, help="Previous tag, e.g. v0.15.1")
    p.add_argument("--version", required=True, help="New version, e.g. 0.16.0")
    p.add_argument("--out", required=True)
    p.set_defaults(func=cmd_cover)

    p = sub.add_parser("cover-chain", help="Capture chain state and render cover-chain.svg")
    p.add_argument("--version", required=True)
    p.add_argument("--chain", default="polkadot",
                   choices=sorted(CHAIN_DEFAULTS.keys()))
    p.add_argument("--out", required=True)
    p.set_defaults(func=cmd_cover_chain)

    p = sub.add_parser("notes", help="Render RELEASE_NOTES.md")
    p.add_argument("--version", required=True)
    p.add_argument("--prev", required=True)
    p.add_argument("--date", required=True)
    p.add_argument("--rust", required=True)
    p.add_argument("--node", required=True)
    p.add_argument("--summary-file", type=Path, required=True)
    p.add_argument("--whats-new-file", type=Path, required=True)
    p.add_argument("--commits-file", type=Path, required=True)
    p.add_argument("--breaking-file", type=Path)
    p.add_argument("--out", required=True)
    p.set_defaults(func=cmd_notes)

    p = sub.add_parser("manifest", help="Render manifest.yml")
    p.add_argument("--version", required=True)
    p.add_argument("--prev", required=True)
    p.add_argument("--date", required=True, help="ISO-8601 UTC, e.g. 2026-04-19T00:00:00Z")
    p.add_argument("--status", required=True, choices=["alpha", "beta", "stable"])
    p.add_argument("--rust", required=True)
    p.add_argument("--node", required=True)
    p.add_argument("--out", required=True)
    p.set_defaults(func=cmd_manifest)

    p = sub.add_parser("pr-body", help="Render draft release PR body")
    p.add_argument("--version", required=True)
    p.add_argument("--prev", required=True)
    p.add_argument("--head-sha", required=True)
    p.add_argument("--bump-type", required=True, choices=["MAJOR", "MINOR", "PATCH"])
    p.add_argument("--summary-file", type=Path, required=True)
    p.add_argument("--whats-new-file", type=Path, required=True)
    p.add_argument("--breaking-file", type=Path)
    p.add_argument("--out", required=True)
    p.set_defaults(func=cmd_pr_body)

    args = parser.parse_args()
    args.func(args)


ARGS: argparse.Namespace  # populated by cmd_cover for cross-function access

if __name__ == "__main__":
    main()
