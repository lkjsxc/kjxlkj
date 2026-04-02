# Palette and Contrast Contract

## Theme Policy

- The shell is dark-first.
- No alternate full light theme is documented or implemented.
- Preview content stays in the same dark-mode family as guest note rendering.

## Surface Hierarchy

- Global background uses near-black graphite.
- Primary note cards use solid charcoal tone.
- Secondary grouping surfaces use a slightly lighter solid layer only when spacing alone is insufficient.
- Borders stay subtle but always visible against adjacent dark surfaces.
- Depth comes from spacing and borders, not from gradients or shadows.

## Accent Policy

- Accent is restrained and never floods the interface.
- Accent appears on active note state, selected history items, Markdown links, and focused controls.
- Destructive actions use a dedicated danger hue and never share the primary accent.

## Contrast Rules

- Button text must always have explicit foreground color tokens.
- Secondary buttons must remain readable on dark panels and overlays.
- Muted metadata may be softer than body text, but never dim enough to disappear.
- Compact menu controls must be visually quiet without sacrificing contrast.

## Reference Translation

- `tmp/image01.png` implies flat dark surfaces with strong sectional separation.
- `tmp/image02.png` is rejected because bright content panels and weak button contrast break the shell.
- `tmp/image03.png` is rejected because compact-nav controls visually dominate the drawer.
- Gradients, glass blur, and soft shadow elevation are not canonical.
