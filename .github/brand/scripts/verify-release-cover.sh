#!/usr/bin/env bash
# Verify release-cover templates only use palette from tokens.yml.
# Called by brand-lint.yml CI.
set -euo pipefail

REPO_ROOT="$(git rev-parse --show-toplevel)"
TEMPLATES=(
  "$REPO_ROOT/.claude/skills/release/covers/cover.svg.template"
  "$REPO_ROOT/.claude/skills/release/covers/cover-chain.svg.template"
)

python3 - "${TEMPLATES[@]}" <<'PY'
import sys, re, yaml
from pathlib import Path

repo = Path(__file__).resolve().parents[3] if False else None
import subprocess
repo = Path(subprocess.check_output(["git","rev-parse","--show-toplevel"]).decode().strip())

with (repo / ".github/brand/tokens.yml").open() as f:
    T = yaml.safe_load(f)

def collect_hex(node, acc):
    if isinstance(node, dict):
        for v in node.values(): collect_hex(v, acc)
    elif isinstance(node, list):
        for v in node: collect_hex(v, acc)
    elif isinstance(node, str) and re.fullmatch(r"#[0-9A-Fa-f]{3,8}", node.strip()):
        acc.add(node.strip().upper())

allowed = set()
collect_hex(T["color"], allowed)
for h in T.get("allowlist", []):
    allowed.add(h.upper())

bad = []
for tpl in sys.argv[1:]:
    p = Path(tpl)
    if not p.exists():
        continue
    for n, line in enumerate(p.read_text().splitlines(), 1):
        for hx in re.findall(r"#[0-9A-Fa-f]{6,8}\b", line):
            if hx.upper() not in allowed:
                bad.append(f"{p.relative_to(repo)}:{n}  {hx}  not in tokens.yml")

if bad:
    print("✗ release cover palette drift:")
    for b in bad: print(f"  {b}")
    print("\nFix: replace with a value from .github/brand/tokens.yml, or add to allowlist.")
    sys.exit(1)
print("✓ release cover templates use only tokens.yml palette")
PY
