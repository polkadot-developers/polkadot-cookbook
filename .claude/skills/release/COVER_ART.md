# Release Cover Art Specification

Each release gets a unique, generative **Mondrian-inspired** SVG artwork — like a textbook cover for that version. The art uses the Polkadot brand palette and is seeded by the version number so each release has a distinct composition.

## Design Rules

- **Format:** SVG, 1200x630px (GitHub social preview / Open Graph size)
- **Style:** Abstract geometric — Mondrian grid with Polkadot network overlay
- **Palette:** See `.github/media/BRANDING.md` for authoritative colors:
  - `#E6007A` (Polkadot Pink) — primary blocks and dots
  - `#11116B` (Deep Blue) — secondary blocks
  - `#0D0D0D` (near-black) — background
  - `#FFFFFF` at low opacity — subtle accent blocks
- **Output:** `.github/releases/vX.Y.Z/cover.svg`

## Composition Structure

1. **Background:** Dark (#0D0D0D)
2. **Mondrian grid:** 4-5 vertical lines and 3-4 horizontal lines, creating an asymmetric grid. Vary the line positions per release — use the version number components (major, minor, patch) to shift positions. Lines should be bold (#1A1A1A, 6px stroke) for the characteristic Mondrian look.
3. **Color blocks:** Fill 4-6 grid cells with brand colors at varying opacities (0.5–0.9). Balance pink and blue blocks. Leave some cells empty for breathing room. Add subtle `fade-block` CSS animation for a living feel.
4. **Polkadot network overlay:** Scatter 4-6 small circles (Polkadot dots) across the composition. Connect them with thin lines (1px, low opacity) to suggest network connectivity — the defining visual metaphor of Polkadot.
5. **Version watermark:** Bottom-right corner, monospace font, very low opacity (0.15). Shows `vX.Y.Z`.
6. **Wordmark:** Bottom-left corner, "POLKADOT COOKBOOK" in small caps, very low opacity (0.25).

## Variation Strategy

Each release must look **distinct but recognizably part of the same series.** Vary:
- Grid line positions (shift by version components)
- Which grid cells get filled and with which colors
- Dot positions and connection topology
- Block opacity levels
- Number and placement of accent blocks

**Do NOT** vary: the overall aesthetic (Mondrian + Polkadot dots), the color palette, the dimensions, or the watermark placement.

## Reference

Study the existing cover at `.github/releases/v0.13.0/cover.svg` for the established pattern. Each subsequent release should follow the same structure but with a fresh composition.

## Usage in Release Notes

The cover appears at the very top of `RELEASE_NOTES.md`:

```markdown
<div align="center">
  <img src="cover.svg" alt="Release vX.Y.Z" width="100%" />
</div>

# Release vX.Y.Z
...
```

