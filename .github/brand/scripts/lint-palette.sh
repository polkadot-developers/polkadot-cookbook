#!/usr/bin/env bash
# Scan repo for hex colors outside tokens.yml allowlist.
# CI fails the PR when an unauthorized hex appears in brand-relevant files.
set -euo pipefail

REPO_ROOT="$(git rev-parse --show-toplevel)"

python3 - <<'PY'
import re, subprocess, sys, yaml
from pathlib import Path

repo = Path(subprocess.check_output(["git","rev-parse","--show-toplevel"]).decode().strip())
with (repo / ".github/brand/tokens.yml").open() as f:
    T = yaml.safe_load(f)

allowed = set()
def walk(n):
    if isinstance(n, dict):
        for v in n.values(): walk(v)
    elif isinstance(n, list):
        for v in n: walk(v)
    elif isinstance(n, str) and re.fullmatch(r"#[0-9A-Fa-f]{3,8}", n.strip()):
        allowed.add(n.strip().upper())
walk(T["color"])
for h in T.get("allowlist", []): allowed.add(h.upper())

# Scan brand-relevant files
exts = {".svg", ".css", ".scss", ".rs", ".ts", ".tsx", ".js", ".jsx",
        ".json", ".yml", ".yaml", ".md", ".toml", ".html"}
skip_dirs = {"target", "node_modules", ".git", "dist", "build"}
skip_paths = {
    # pre-existing third-party-origin assets — audit separately
    ".github/media/icons",
    # legacy logos — audit before migrating to tokens
    ".github/media/dot-logo-dark.svg",
    ".github/media/dot-logo-light.svg",
    # tokens + changelog legitimately contain hex codes
    ".github/brand/tokens.yml",
    ".github/brand/CHANGELOG.md",
    # docs site has its own (older) palette — migration tracked in CHANGELOG
    "docs/index.html",
    "docs/404.html",
    "docs/_layouts",
    "docs/assets",
    # historical release artifacts are immutable — do not re-palette past releases
    ".github/releases",
    # generated third-party badges
    ".github/badges",
    # lockfiles
    "Cargo.lock", "package-lock.json",
    # legacy branding doc (superseded; migrating)
    ".github/media/BRANDING.md",
}

hex_re = re.compile(r'#[0-9A-Fa-f]{6}(?:[0-9A-Fa-f]{2})?\b')
bad = []

for path in repo.rglob("*"):
    if not path.is_file(): continue
    rel = path.relative_to(repo)
    if any(part in skip_dirs for part in rel.parts): continue
    if any(str(rel).startswith(s) for s in skip_paths): continue
    if path.suffix.lower() not in exts: continue
    try:
        text = path.read_text(errors="replace")
    except Exception:
        continue
    for n, line in enumerate(text.splitlines(), 1):
        for hx in hex_re.findall(line):
            if hx.upper() in allowed: continue
            # Allow hex inside code/shell snippets that are documenting the palette (heuristic: line contains "tokens" or "allowlist")
            if "tokens.yml" in line or "allowlist" in line: continue
            bad.append(f"{rel}:{n}  {hx}  not in tokens.yml palette or allowlist")

if bad:
    print("✗ palette lint failed:")
    for b in bad[:50]: print(f"  {b}")
    if len(bad) > 50: print(f"  … and {len(bad)-50} more")
    print("\nFix options:")
    print("  1. Use a token from .github/brand/tokens.yml")
    print("  2. If unavoidable, add the hex to tokens.yml `allowlist`")
    sys.exit(1)
print("✓ palette lint passed")
PY
