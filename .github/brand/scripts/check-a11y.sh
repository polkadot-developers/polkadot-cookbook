#!/usr/bin/env bash
# Accessibility checks on generated SVG assets.
# - Every SVG with <animate> must include a prefers-reduced-motion style block.
# - Every SVG must have a role/aria-label or role="presentation" (aria-hidden).
set -euo pipefail

REPO_ROOT="$(git rev-parse --show-toplevel)"

python3 - <<'PY'
import sys, subprocess, re
from pathlib import Path

repo = Path(subprocess.check_output(["git","rev-parse","--show-toplevel"]).decode().strip())
media = repo / ".github/media"
docs = repo / "docs"

bad = []
for svg in list(media.glob("*.svg")) + [docs / "favicon.svg"]:
    if not svg.exists(): continue
    # Skip third-party icons and pre-existing logos
    if "icons" in str(svg) or svg.name.startswith("dot-logo"): continue
    if svg.name in {"coverage.svg"}: continue
    s = svg.read_text()
    if "<animate" in s and "prefers-reduced-motion" not in s:
        bad.append(f"{svg.relative_to(repo)}  has <animate> but no prefers-reduced-motion guard")
    if 'role=' not in s:
        bad.append(f"{svg.relative_to(repo)}  missing role attribute (use role=\"img\"+aria-label or role=\"presentation\")")

if bad:
    print("✗ a11y check failed:")
    for b in bad: print(f"  {b}")
    sys.exit(1)
print("✓ a11y check passed")
PY
