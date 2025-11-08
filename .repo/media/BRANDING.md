# Polkadot Cookbook Branding Guide

This document contains all branding assets, colors, and styling guidelines for the Polkadot Cookbook project.

## Color Palette

Based on the official Polkadot brand colors:

- **Primary Pink**: `#E6007A` - Main accent color
- **Deep Blue**: `#11116B` - Secondary color for contrast
- **White**: `#FFFFFF` - Light theme backgrounds

## Logos

### Main Logo (Dot CLI)
- **Location**: `.media/dot-logo-dark.svg` (dark mode) | `.media/dot-logo-light.svg` (light mode)
- **Sizes**:
  - Header: 80x80px
  - Footer: 40x40px
- **Features**: Animated (pulsing, glowing, orbital dots)

### Polkadot Attribution Logo
- **External**: GitHub raw URLs to official Polkadot SDK logos
- **Size**: 24px height
- **Animation**: Fades from 70% to 100% opacity over 60 seconds (5s delay)

## Icons

All custom icons in `.media/icons/` with dark/light variants:

| Icon | Filename | Usage |
|------|----------|-------|
| Recipes | `recipes-dark/light.svg` | Main recipes section |
| Runtime | `runtime-dark/light.svg` | Runtime development |
| Contracts | `contracts-dark/light.svg` | Smart contracts |
| Interactions | `interactions-dark/light.svg` | Basic interactions |
| XCM | `xcm-dark/light.svg` | Cross-chain messaging |
| Testing | `testing-dark/light.svg` | Testing infrastructure |
| Beginner | `beginner-dark/light.svg` | Difficulty indicator |
| Idea | `idea-dark/light.svg` | Tips and suggestions |
| Rocket | `rocket-dark/light.svg` | Quick start |
| Target | `target-dark/light.svg` | Goals/targets |
| Chart | `chart-dark/light.svg` | Metrics/levels |
| Docs | `docs-dark/light.svg` | Documentation |
| Refresh | `refresh-dark/light.svg` | Updates/automation |
| Package | `package-dark/light.svg` | Dependencies |
| Contributing | `contributing-dark/light.svg` | Contributions |
| Book | `book-dark/light.svg` | Recipes |
| Bug | `bug-dark/light.svg` | Bug reports |
| Memo | `memo-dark/light.svg` | Documentation |

**Icon Sizes:**
- Section headers (##): 20x20px
- Subsection headers (###): 20x20px
- List items: 16x16px
- Table cells: 14x14px
- Callouts: 18x18px

## Badges

### Style
- **Primary color**: `#E6007A` (pink)
- **Secondary color**: `#11116B` (dark blue)
- **Pattern**: Alternate colors for visual variety

### Badge Examples
```markdown
[![License](https://img.shields.io/badge/License-MIT%20%2F%20Apache%202.0-11116B.svg)](LICENSE)
[![SDK Status](https://img.shields.io/github/actions/workflow/status/polkadot-developers/polkadot-cookbook/test-sdk.yml?label=Polkadot%20Cookbook%20SDK&color=E6007A)](workflows)
[![CLI](https://img.shields.io/badge/CLI-dot%20v0.1.0-E6007A?logo=rust&logoColor=white)](cli/)
[![Rust](https://img.shields.io/badge/dynamic/yaml?url=raw.githubusercontent.com/polkadot-developers/polkadot-cookbook/master/versions.yml&query=$.versions.rust&prefix=v&label=rust&color=11116B)](https://www.rust-lang.org/)
```

## Dividers

### Animated Gradient Dividers
```html
<hr class="polkadot-divider" />
```

**Style:**
```css
.polkadot-divider {
  height: 3px;
  background: linear-gradient(90deg, #E6007A 0%, #11116B 25%, #E6007A 50%, #11116B 75%, #E6007A 100%);
  background-size: 200% 100%;
  border: none;
  margin: 40px 0;
  opacity: 0.6;
  animation: gradientFlow 8s ease-in-out infinite;
}

@keyframes gradientFlow {
  0% { background-position: 0% 50%; }
  50% { background-position: 100% 50%; }
  100% { background-position: 0% 50%; }
}
```

## Animations

### Logo Fade-In (Polkadot Attribution)
- Start: 70% opacity
- End: 100% opacity
- Duration: 60 seconds
- Delay: 5 seconds
- Easing: ease-in

### Main Logo/Title Fade-In
- Start: 90% opacity
- End: 100% opacity
- Duration: 30 seconds
- Easing: ease-in

### Section Heading Float
- Movement: 0px → -2px → 0px
- Duration: 6 seconds
- Easing: ease-in-out
- Infinite loop

### Gradient Divider Flow
- Duration: 8 seconds
- Movement: Left to right and back
- Infinite loop

## Typography

### Headings
- Apply `.heading-shine` class for subtle float animation
- Wrap heading text in `<span class="heading-shine">` tags
- Always include icon before heading text

**Example:**
```markdown
## <img src=".media/icons/recipes-dark.svg#gh-dark-mode-only" width="20" height="20" alt="" /> <img src=".media/icons/recipes-light.svg#gh-light-mode-only" width="20" height="20" alt="" /> <span class="heading-shine">Recipes</span>
```

## Layout Structure

### Header
1. Small Polkadot attribution logo (left-aligned, fading in)
2. Center-aligned main section with:
   - Animated dot logo (80x80px)
   - "Polkadot Cookbook" title
   - Tagline
   - Navigation links
   - Badges

### Footer
- Animated dot logo (40x40px)
- "Built by" credit
- Navigation links

### Sections
- Separated by animated gradient dividers
- Icons before all headings
- Consistent spacing (40px between major sections)

## CSS Classes

All styling is defined inline in the document `<style>` block at the top:

- `.polkadot-logo-fade` - Polkadot attribution logo fade-in
- `.cookbook-fade-in` - Main branding fade-in
- `.polkadot-divider` - Animated gradient dividers
- `.heading-shine` - Section heading float animation

## Theme Application Checklist

When applying branding to a new document:

- [ ] Add CSS style block at the top
- [ ] Add Polkadot attribution logo (if appropriate)
- [ ] Add main dot logo to header
- [ ] Replace plain `---` dividers with `<hr class="polkadot-divider" />`
- [ ] Add icons to all section headings
- [ ] Wrap heading text in `<span class="heading-shine">` tags
- [ ] Add badges (if appropriate)
- [ ] Add footer with dot logo
- [ ] Update badge colors to match palette
- [ ] Test dark/light mode switching

## File Organization

```
.media/
├── dot-logo-dark.svg          # Main logo (dark mode)
├── dot-logo-light.svg         # Main logo (light mode)
├── icons/                     # All icon assets
│   ├── recipes-dark.svg
│   ├── recipes-light.svg
│   └── ... (38 total icons)
└── BRANDING.md               # This file
```

## Brand Voice

- **Professional yet approachable**
- **Technical but not intimidating**
- **Community-focused**
- **Action-oriented** (Quick Start, Contribute, Build)

## Design Principles

1. **Minimalism** - Clean, uncluttered design
2. **Consistency** - Same patterns throughout
3. **Animation** - Subtle, elegant, never distracting
4. **Accessibility** - Dark/light mode support
5. **Polkadot Identity** - Clear connection to Polkadot ecosystem
6. **Professionalism** - Enterprise-ready appearance
