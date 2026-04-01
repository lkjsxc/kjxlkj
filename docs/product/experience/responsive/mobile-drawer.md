# Mobile Drawer Contract

## Narrow Layout

- The side menu is closed by default on narrow screens.
- A toggle in the top-right of the screen opens and closes the menu.
- The menu opens as an overlay drawer with a backdrop.

## Interaction Rules

- Backdrop click closes the drawer.
- `Escape` closes the drawer.
- Focus returns to the toggle after close.
- Desktop keeps the rail persistently visible and ignores drawer-only chrome.
- Compact preview overlay uses the same close affordances.
- Compact preview overlay must still render correctly when the drawer is unavailable or closed.

## Anti-Regression Rules

- The narrow layout does not stack the rail above the main pane.
- The drawer control stays visually quiet relative to note content.
- Hidden-rail layouts may not break editor preview positioning or preview repaint behavior.
