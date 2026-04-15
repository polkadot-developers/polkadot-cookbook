#!/usr/bin/env python3
"""Polkadot Cookbook — branding asset generator.

Reads .github/brand/tokens.yml + live repo facts, renders every template
under templates/ into .github/media/. Idempotent.
"""
from __future__ import annotations
import os, re, subprocess, sys, tempfile, yaml, shutil
from pathlib import Path

REPO_ROOT = Path(subprocess.check_output(["git", "rev-parse", "--show-toplevel"]).decode().strip())
BRAND = REPO_ROOT / ".github" / "brand"
TPL = REPO_ROOT / ".claude" / "skills" / "branding" / "templates"
DRY_RUN = "--dry-run" in sys.argv

if DRY_RUN:
    tmp = Path(tempfile.mkdtemp())
    OUT = tmp / "media"
    DOCS = tmp / "docs"
    OUT.mkdir(parents=True); DOCS.mkdir(parents=True)
    print(f"▸ dry-run → {OUT}")
else:
    OUT = REPO_ROOT / ".github" / "media"
    DOCS = REPO_ROOT / "docs"


def count_dirs(path: Path) -> int:
    return sum(1 for p in path.iterdir() if p.is_dir()) if path.exists() else 0


# --- load tokens ----------------------------------------------------------
with (BRAND / "tokens.yml").open() as f:
    T = yaml.safe_load(f)

# --- live facts -----------------------------------------------------------
cargo = (REPO_ROOT / "Cargo.toml").read_text()
m = re.search(r'^version\s*=\s*"([^"]+)"', cargo, re.M)
VERSION = m.group(1) if m else "0.0.0"

recipes = REPO_ROOT / "recipes"
PW = {
    "pallets": count_dirs(recipes / "pallets") + count_dirs(recipes / "parachains"),
    "contracts": count_dirs(recipes / "contracts"),
    "transactions": count_dirs(recipes / "transactions"),
    "xcm": count_dirs(recipes / "cross-chain-transactions"),
    "networks": count_dirs(recipes / "networks"),
}
RECIPE_COUNT = sum(PW.values())
WORKFLOW_COUNT = len(list((REPO_ROOT / ".github" / "workflows").glob("*.yml")))

# polkadot-docs test harnesses (by top-level category under polkadot-docs/)
docs_root = REPO_ROOT / "polkadot-docs"
def count_docs_harnesses(cat: str) -> int:
    p = docs_root / cat
    if not p.exists(): return 0
    return sum(1 for d in p.iterdir() if d.is_dir() and (d / "package.json").exists())
DOCS_BY_CAT = {
    "chain": count_docs_harnesses("chain-interactions"),
    "contracts": count_docs_harnesses("smart-contracts"),
    "parachains": count_docs_harnesses("parachains"),
    "networks": count_docs_harnesses("networks"),
}
DOCS_HARNESS_COUNT = sum(DOCS_BY_CAT.values())

# --- bar chart widths -----------------------------------------------------
max_c = max(max(PW.values()), 1)
def bar_w(c): return 80 + (c * 200 // max_c)
def label_x(c): return 180 + bar_w(c) + 12

BARS = {
    f"BAR_{k.upper()}_W": str(bar_w(v)) for k, v in PW.items()
}
BARS.update({f"BAR_{k.upper()}_LABEL_X": str(label_x(v)) for k, v in PW.items()})


# --- substitution map (base) ----------------------------------------------
def base_subs(canvas: str) -> dict:
    s = {
        "CANVAS": canvas,
        "PINK": T["color"]["primary"]["pink"],
        "BLUE": T["color"]["primary"]["blue"],
        "TERMINAL": T["color"]["surface"]["terminal"],
        "CREAM": T["color"]["surface"]["cream"],
        "WARN": T["color"]["semantic"]["warn"],
        "FIX": T["color"]["semantic"]["fix"],
        "MONO": T["type"]["mono"],
        "REVEAL_DUR": T["motion"]["reveal-dur"],
        "GRADIENT_FLOW_DUR": T["motion"]["gradient-flow-dur"],
        "VERSION": VERSION,
        "RECIPE_COUNT": str(RECIPE_COUNT),
        "WORKFLOW_COUNT": str(WORKFLOW_COUNT),
        "DOCS_HARNESS_COUNT": str(DOCS_HARNESS_COUNT),
        "DOCS_CHAIN": str(DOCS_BY_CAT["chain"]),
        "DOCS_CONTRACTS": str(DOCS_BY_CAT["contracts"]),
        "DOCS_PARACHAINS": str(DOCS_BY_CAT["parachains"]),
        "DOCS_NETWORKS": str(DOCS_BY_CAT["networks"]),
        "PATHWAY_PALLETS": str(PW["pallets"]),
        "PATHWAY_CONTRACTS": str(PW["contracts"]),
        "PATHWAY_TRANSACTIONS": str(PW["transactions"]),
        "PATHWAY_XCM": str(PW["xcm"]),
        "PATHWAY_NETWORKS": str(PW["networks"]),
    }
    s.update(BARS)
    return s


def render(tpl_name: str, out_path: Path, extra: dict | None = None, canvas_key: str = "dark"):
    canvas = T["color"]["surface"]["canvas"] if canvas_key == "dark" else T["color"]["mode"]["light-bg"]
    subs = base_subs(canvas)
    if extra:
        subs.update(extra)
    src = (TPL / tpl_name).read_text()
    for k, v in subs.items():
        src = src.replace("{{" + k + "}}", str(v))
    unresolved = re.findall(r"\{\{[A-Z_]+\}\}", src)
    if unresolved:
        sys.exit(f"ERROR unresolved tokens in {tpl_name}: {set(unresolved)}")
    out_path.parent.mkdir(parents=True, exist_ok=True)
    out_path.write_text(src)
    # validate
    if shutil.which("xmllint"):
        r = subprocess.run(["xmllint", "--noout", str(out_path)], capture_output=True)
        if r.returncode != 0:
            sys.exit(f"xmllint fail {out_path}: {r.stderr.decode()}")


def rasterize(svg: Path, png: Path, width: int):
    """Rasterize SVG → PNG. Tries rsvg-convert, then cairosvg, else warns."""
    png.parent.mkdir(parents=True, exist_ok=True)
    if shutil.which("rsvg-convert"):
        subprocess.run(
            ["rsvg-convert", "-w", str(width), "-o", str(png), str(svg)],
            check=True, capture_output=True,
        )
        return True
    try:
        import cairosvg  # type: ignore
        cairosvg.svg2png(url=str(svg), write_to=str(png), output_width=width)
        return True
    except ImportError:
        pass
    print(f"  ⚠ PNG skipped ({png.name}) — install librsvg: brew install librsvg")
    return False


# --- render everything ----------------------------------------------------
print("▸ hero")
render("hero.svg.template", OUT / "hero-dark.svg", canvas_key="dark")
render("hero.svg.template", OUT / "hero-light.svg", canvas_key="light")

print("▸ divider")
render("divider.svg.template", OUT / "divider-dark.svg", canvas_key="dark")
render("divider.svg.template", OUT / "divider-light.svg", canvas_key="light")

print("▸ social-preview + og-image")
render("social-preview.svg.template", OUT / "social-preview.svg", canvas_key="dark")
shutil.copyfile(OUT / "hero-dark.svg", OUT / "og-image.svg")

# Rasterize PNGs for platforms that don't render SVG OG images (X, Slack, etc.)
rasterize(OUT / "social-preview.svg", OUT / "social-preview.png", 1280)
rasterize(OUT / "og-image.svg", OUT / "og-image.png", 1200)
# Mirror og-image.png into docs/ so GitHub Pages serves it at the URL already
# referenced by docs/index.html meta tags.
if (OUT / "og-image.png").exists():
    shutil.copyfile(OUT / "og-image.png", DOCS / "og-image.png")

print("▸ contributing-hero")
render("contributing-hero.svg.template", OUT / "contributing-hero-dark.svg", canvas_key="dark")
render("contributing-hero.svg.template", OUT / "contributing-hero-light.svg", canvas_key="light")

print("▸ pathway banners")
pathways = [
    ("pallets", "PALLETS", PW["pallets"], "Runtime logic with FRAME pallets"),
    ("contracts", "CONTRACTS", PW["contracts"], "Solidity smart contracts on Polkadot"),
    ("transactions", "TRANSACTIONS", PW["transactions"], "Single-chain tx and state queries"),
    ("xcm", "XCM", PW["xcm"], "Cross-chain messaging between parachains"),
    ("networks", "NETWORKS", PW["networks"], "Zombienet + Chopsticks local networks"),
]
for name, label, count, tagline in pathways:
    render(
        "pathway-banner.svg.template",
        OUT / f"pathway-{name}-dark.svg",
        extra={
            "PATHWAY_NAME": name,
            "PATHWAY_LABEL": label,
            "PATHWAY_COUNT": str(count),
            "PATHWAY_TAGLINE": tagline,
        },
        canvas_key="dark",
    )

print("▸ favicon")
render("favicon.svg.template", DOCS / "favicon.svg", canvas_key="dark")

count = len(list(OUT.glob("*.svg")))
print(f"\n✓ generated {count} SVGs in {OUT}")
if DRY_RUN:
    print("  (dry-run — nothing written to repo)")
print(f"  recipes={RECIPE_COUNT}  pathways=5  docs-harnesses={DOCS_HARNESS_COUNT}  workflows={WORKFLOW_COUNT}  version={VERSION}")
