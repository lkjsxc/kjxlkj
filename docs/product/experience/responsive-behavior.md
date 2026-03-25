# Responsive Behavior Contract

## Breakpoint Model

- Desktop behavior applies at widths above `900px`.
- Compact behavior applies at widths of `900px` and below.

## Compact Navigation

- The navigation rail is hidden by default.
- A top-right menu button opens the rail as an overlay drawer.
- Opening the drawer adds a backdrop and traps attention visually on navigation.
- Closing is available from the close button, backdrop click, and `Esc`.

## Compact Content Rules

- Header controls remain visible without horizontal scrolling.
- Editor and rendered content use full available width.
- Long titles wrap instead of overflowing.

## Desktop Rules

- The rail remains pinned on note, dashboard, and home pages.
- Content width stays readable and does not span the full viewport on large screens.

## Accessibility Rules

- The menu button has an explicit accessible name.
- Drawer open and closed states are expressed through ARIA attributes.
- Hidden navigation is removed from pointer interaction when closed.
