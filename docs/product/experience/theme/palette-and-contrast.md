# Palette and Contrast Contract

## Theme Policy

- The product is dark-only.
- No light theme is documented or implemented in this phase.
- Every surface, border, and text token is authored for dark presentation first.

## Surface Hierarchy

- Global background uses near-black graphite.
- Primary panels use a raised charcoal tone.
- Secondary panels use a slightly lighter layer for grouping.
- Borders stay subtle but always visible against adjacent dark surfaces.

## Accent Policy

- Accent is restrained and never floods the interface.
- Accent appears on active note state, selected history items, links, and focused controls.
- Destructive actions use a dedicated danger hue and never share the primary accent.

## Contrast Rules

- Button text must always have explicit foreground color tokens.
- Ghost and secondary buttons must remain readable on dark panels and overlays.
- Muted metadata may be softer than body text, but never dim enough to disappear.
- Compact menu and close controls must be visually quiet without sacrificing contrast.

## Reference Translation

- `tmp/image01.png` implies layered dark surfaces, not flat black.
- `tmp/image02.png` is rejected because bright content panels and weak button contrast break the shell.
- `tmp/image03.png` is rejected because compact-nav controls visually dominate the drawer.
