# hero.data.md — Data contract for hero.svg.template

The generator substitutes scalar tokens via exact-string replacement. Every token below must resolve from `.github/brand/tokens.yml` or live repo facts.

## Token → source (v2)

| Token                      | Source                                                      | Mode-dep? |
| -------------------------- | ----------------------------------------------------------- | --------- |
| `{{CANVAS}}`               | `color.mode.{mode}.canvas`                                  | **yes**   |
| `{{SURFACE}}`              | `color.mode.{mode}.surface`                                 | **yes**   |
| `{{SURFACE_2}}`            | `color.mode.{mode}.surface-2`                               | **yes**   |
| `{{LINE}}`                 | `color.mode.{mode}.line`                                    | **yes**   |
| `{{FG}}`                   | `color.mode.{mode}.fg`                                      | **yes**   |
| `{{FG_MUTED}}`             | `color.mode.{mode}.fg-muted`                                | **yes**   |
| `{{FG_DIM}}`               | `color.mode.{mode}.fg-dim`                                  | **yes**   |
| `{{INK}}`                  | `color.base.canvas` (near-black `#0A0A0B`)                  | no        |
| `{{PAPER}}`                | `color.base.paper` (warm paper `#F6F5F2`)                   | no        |
| `{{PINK}}`                 | `color.primary.pink`                                        | no        |
| `{{MONO}}`                 | `type.mono` (JetBrains Mono)                                | no        |
| `{{VERSION}}`              | `Cargo.toml` `[workspace.package].version`                  | no        |
| `{{REVEAL_DUR}}`           | `motion.reveal-dur`                                         | no        |
| `{{CASCADE_STAGGER}}`      | `motion.cascade-stagger`                                    | no        |
| `{{GRADIENT_FLOW_DUR}}`    | `motion.gradient-flow-dur`                                  | no        |
| `{{EASE_OUT}}`             | `motion.ease-out`                                           | no        |

## Bar widths (B3 pathway chart)

Bars scale proportionally. Max bar = 280px, min = 80px. Map:

```
width = 80 + (count / max_count) * 200
```

Expressed as `{{BAR_PALLETS_W}}` etc. — generator computes.

## Accessibility

- Every animated element has `begin` ≥ 0s and `fill="freeze"` so the static end-state is valid without JS.
- The SVG ships a `<style>` block with `@media (prefers-reduced-motion: reduce) { * { animation: none !important; } }`. Honored by modern renderers; static end-state still looks correct.

## Validation

- `xmllint --noout` must pass.
- No hex codes outside the token substitution map (CI palette lint).
- Contrast: cream on terminal ≥ 4.5:1, cream on pink ≥ 4.5:1 (spot-checked).
