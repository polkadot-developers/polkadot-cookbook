#!/usr/bin/env python3
"""Polkadot Cookbook — branding asset generator.

v2 product palette: near-black canvas, warm paper, grey ramp, Index Mark.
Every template is rendered in both dark and light modes via the
color.mode.{dark,light} token set.
"""
from __future__ import annotations
import os, re, shutil, subprocess, sys, tempfile, yaml
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
PATHWAY_COUNT = sum(1 for v in PW.values() if v >= 0)  # always 5
WORKFLOW_COUNT = len(list((REPO_ROOT / ".github" / "workflows").glob("*.yml")))

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


# --- pathway glyphs (80×80 SVG snippets, centered at 1070,100) -----------
PATHWAY_GLYPHS = {
    "pallets": '''<g transform="translate(1070, 100)">
    <rect x="-30" y="-24" width="60" height="12" fill="{{PINK}}" rx="2"/>
    <rect x="-30" y="-6" width="60" height="12" fill="{{PINK}}" opacity=".6" rx="2"/>
    <rect x="-30" y="12" width="60" height="12" fill="{{PINK}}" opacity=".3" rx="2"/>
  </g>''',
    "contracts": '''<g transform="translate(1070, 100)">
    <rect x="-24" y="-28" width="48" height="56" fill="none" stroke="{{FG_DIM}}" stroke-width="1.5" rx="2"/>
    <text x="0" y="6" text-anchor="middle" font-family="{{MONO}}" font-size="20" font-weight="700" fill="{{PINK}}">&lt;/&gt;</text>
  </g>''',
    "transactions": '''<g transform="translate(1070, 100)">
    <line x1="-28" y1="0" x2="28" y2="0" stroke="{{FG_DIM}}" stroke-width="1.5"/>
    <polygon points="-12,-18 12,-18 0,-30" fill="{{PINK}}" opacity=".7"/>
    <polygon points="-12,18 12,18 0,30" fill="{{PINK}}"/>
    <circle cx="0" cy="0" r="5" fill="{{PINK}}"/>
  </g>''',
    "xcm": '''<g transform="translate(1070, 100)">
    <line x1="-24" y1="-24" x2="24" y2="24" stroke="{{PINK}}" stroke-width="2"/>
    <line x1="24" y1="-24" x2="-24" y2="24" stroke="{{PINK}}" stroke-width="2" opacity=".6"/>
    <circle cx="-24" cy="-24" r="6" fill="{{PINK}}"/>
    <circle cx="24" cy="24" r="6" fill="{{PINK}}"/>
    <circle cx="24" cy="-24" r="5" fill="{{PINK}}" opacity=".6"/>
    <circle cx="-24" cy="24" r="5" fill="{{PINK}}" opacity=".6"/>
  </g>''',
    "networks": '''<g transform="translate(1070, 100)">
    <circle cx="0" cy="-20" r="6" fill="{{PINK}}"/>
    <circle cx="-20" cy="12" r="6" fill="{{PINK}}" opacity=".7"/>
    <circle cx="20" cy="12" r="6" fill="{{PINK}}" opacity=".7"/>
    <line x1="0" y1="-14" x2="-17" y2="7" stroke="{{FG_DIM}}" stroke-width="1.5"/>
    <line x1="0" y1="-14" x2="17" y2="7" stroke="{{FG_DIM}}" stroke-width="1.5"/>
    <line x1="-14" y1="12" x2="14" y2="12" stroke="{{FG_DIM}}" stroke-width="1.5"/>
  </g>''',
}


def resolve_glyph(glyph_svg: str, mode: str) -> str:
    """Pre-resolve {{TOKEN}} refs inside pathway glyphs before injection."""
    subs = mode_subs(mode)
    for k, v in subs.items():
        glyph_svg = glyph_svg.replace("{{" + k + "}}", str(v))
    return glyph_svg


# --- substitution map -----------------------------------------------------
def mode_subs(mode: str) -> dict:
    m = T["color"]["mode"][mode]
    s = {
        # Core palette
        "PINK": T["color"]["primary"]["pink"],
        "BLACK": T["color"]["base"]["black"],
        "WHITE": T["color"]["base"]["white"],
        # Mode-dependent surfaces
        "CANVAS": m["canvas"],
        "SURFACE": m["surface"],
        "SURFACE_2": m["surface-2"],
        "LINE": m["line"],
        "FG": m["fg"],
        "FG_MUTED": m["fg-muted"],
        "FG_DIM": m["fg-dim"],
        # Semantic (mode-independent)
        "INK": T["color"]["semantic"]["ink"],
        "PAPER": T["color"]["semantic"]["paper"],
        # Type
        "MONO": T["type"]["mono"],
        # Motion
        "REVEAL_DUR": T["motion"]["reveal-dur"],
        "CASCADE_STAGGER": T["motion"]["cascade-stagger"],
        "GRADIENT_FLOW_DUR": T["motion"]["gradient-flow-dur"],
        "EASE_OUT": T["motion"]["ease-out"],
        # Facts
        "VERSION": VERSION,
    }
    return s


def render(tpl_name: str, out_path: Path, mode: str = "dark", extra: dict | None = None):
    subs = mode_subs(mode)
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
    if shutil.which("xmllint"):
        r = subprocess.run(["xmllint", "--noout", str(out_path)], capture_output=True)
        if r.returncode != 0:
            sys.exit(f"xmllint fail {out_path}: {r.stderr.decode()}")


def rasterize(svg: Path, png: Path, width: int):
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


# --- render ---------------------------------------------------------------
print("▸ hero")
render("hero.svg.template", OUT / "hero-dark.svg",  "dark")
render("hero.svg.template", OUT / "hero-light.svg", "light")

print("▸ divider")
render("divider.svg.template", OUT / "divider-dark.svg",  "dark")
render("divider.svg.template", OUT / "divider-light.svg", "light")

print("▸ social-preview + og-image")
render("social-preview.svg.template", OUT / "social-preview.svg", "dark")
# OG: use social-preview as source (hero is now 1200×400, doesn't match OG 1200×630)
shutil.copyfile(OUT / "social-preview.svg", OUT / "og-image.svg")
rasterize(OUT / "social-preview.svg", OUT / "social-preview.png", 1280)
rasterize(OUT / "og-image.svg", OUT / "og-image.png", 1200)
if (OUT / "og-image.png").exists():
    shutil.copyfile(OUT / "og-image.png", DOCS / "og-image.png")

print("▸ contributing-hero")
render("contributing-hero.svg.template", OUT / "contributing-hero-dark.svg",  "dark")
render("contributing-hero.svg.template", OUT / "contributing-hero-light.svg", "light")

print("▸ pathway banners")
pathways = [
    ("pallets",      "Pallets",      "Runtime logic with FRAME pallets",          "01"),
    ("contracts",    "Contracts",    "Solidity smart contracts on Polkadot",      "02"),
    ("transactions", "Transactions", "Single-chain tx and state queries",         "03"),
    ("xcm",          "XCM",          "Cross-chain messaging between parachains",  "04"),
    ("networks",     "Networks",     "Zombienet + Chopsticks local networks",     "05"),
]
for name, label, tagline, number in pathways:
    render(
        "pathway-banner.svg.template",
        OUT / f"pathway-{name}-dark.svg",
        "dark",
        extra={
            "PATHWAY_NAME": name,
            "PATHWAY_LABEL": label,
            "PATHWAY_TAGLINE": tagline,
            "PATHWAY_NUMBER": number,
            "PATHWAY_GLYPH": resolve_glyph(PATHWAY_GLYPHS[name], "dark"),
        },
    )

print("▸ wordmark")
render("wordmark.svg.template", OUT / "wordmark-dark.svg", "dark")
render("wordmark.svg.template", OUT / "wordmark-light.svg", "light")

print("▸ favicon")
render("favicon.svg.template", DOCS / "favicon.svg", "dark")

count = len(list(OUT.glob("*.svg")))
print(f"\n✓ generated {count} SVGs in {OUT}")
if DRY_RUN:
    print("  (dry-run — nothing written to repo)")
print(f"  recipes={RECIPE_COUNT}  pathways={PATHWAY_COUNT}  docs-harnesses={DOCS_HARNESS_COUNT}  workflows={WORKFLOW_COUNT}  version={VERSION}")
