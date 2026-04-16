# hero.data.md — Data contract for hero.svg.template

The generator substitutes scalar tokens via exact-string replacement. Every token below must resolve from `.github/brand/tokens.yml` or live repo facts.

## Token → source

| Token                      | Source                                                      |
| -------------------------- | ----------------------------------------------------------- |
| `{{CANVAS}}`               | `tokens.color.surface.canvas` (dark) / `mode.light-bg` (light) |
| `{{PINK}}`                 | `tokens.color.primary.pink`                                 |
| `{{BLUE}}`                 | `tokens.color.primary.blue`                                 |
| `{{TERMINAL}}`             | `tokens.color.surface.terminal`                             |
| `{{CREAM}}`                | `tokens.color.surface.cream`                                |
| `{{MONO}}`                 | `tokens.type.mono`                                          |
| `{{VERSION}}`              | `grep version Cargo.toml` workspace.package.version         |
| `{{RECIPE_COUNT}}`         | `find recipes -mindepth 2 -maxdepth 2 -type d \| wc -l`     |
| `{{WORKFLOW_COUNT}}`       | `ls .github/workflows/*.yml \| wc -l`                       |
| `{{PATHWAY_PALLETS}}`      | count under `recipes/pallets/` + `recipes/parachains/`      |
| `{{PATHWAY_CONTRACTS}}`    | count under `recipes/contracts/`                            |
| `{{PATHWAY_TRANSACTIONS}}` | count under `recipes/transactions/`                         |
| `{{PATHWAY_XCM}}`          | count under `recipes/cross-chain-transactions/`             |
| `{{PATHWAY_NETWORKS}}`     | count under `recipes/networks/`                             |
| `{{REVEAL_DUR}}`           | `tokens.motion.reveal-dur`                                  |

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
