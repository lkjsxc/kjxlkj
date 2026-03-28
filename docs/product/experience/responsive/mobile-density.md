# Mobile Density Contract

## Layout Rules

- Compact mode reduces page padding and card chrome.
- The title, metadata, and action surfaces remain readable without horizontal scrolling.
- The top bar, drawer, and main pane must coexist without clipping.
- Admin note pages must keep the editor inside the viewport width.

## Content Rules

- Editor and reader surfaces use the full remaining width.
- Dashboard and history rows keep compact padding and clear contrast.
- Sticky or floating controls must not obscure note content.
- The editor toolbar may compact or reduce tool count on narrow screens.
- Narrow screens wrap toolbar groups instead of exposing a detached horizontal toolbar scrollbar.
- The note rail keeps fixed-size `Prev` and `Next` cards even when one side is unavailable.
- Overflow is acceptable only inside tightly scoped editor internals, never at the page level.
- The note page remains the primary vertical scroll container while editing.

## Verification Targets

- The drawer is closed on first paint.
- The drawer opens without clipping the main pane.
- Button labels and note metadata stay readable on real mobile widths.
- Compact admin note pages show no horizontal page overflow.
- Compact admin note pages do not show a misplaced toolbar scrollbar.
- Compact layouts do not reintroduce visible rail section headings or helper copy.
