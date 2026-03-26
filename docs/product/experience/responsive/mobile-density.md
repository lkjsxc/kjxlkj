# Mobile Density Contract

## Layout Rules

- Compact mode reduces page padding and card chrome.
- The title, metadata, and action surfaces remain readable without horizontal scrolling.
- The top bar, drawer, and main pane must coexist without clipping.

## Content Rules

- Editor and reader surfaces use the full remaining width.
- Dashboard and history rows keep compact padding and clear contrast.
- Sticky or floating controls must not obscure note content.

## Verification Targets

- The drawer is closed on first paint.
- The drawer opens without clipping the main pane.
- Button labels and note metadata stay readable on real mobile widths.
