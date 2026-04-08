# Mobile Density Contract

## Layout Rules

- Compact mode reduces page padding and card chrome.
- The title, metadata, and action surfaces remain readable without horizontal scrolling.
- The top bar, drawer, and main pane must coexist without clipping.
- Admin note pages must keep the editor inside the viewport width.
- Compact layouts keep the same explicit system font stack as wide layouts.
- Compact layouts keep the same tight rectangular radius family as wide layouts.

## Content Rules

- Editor and reader surfaces use the full remaining width.
- Dashboard and history rows keep compact padding and clear contrast.
- Sticky or floating controls must not obscure note content.
- The note rail keeps fixed-size `Prev` and `Next` cards even when one side is unavailable.
- Drawer and narrow layouts show the two timeline cards in one horizontal row.
- Overflow is acceptable only inside tightly scoped editor internals, never at the page level.
- The note page remains the primary vertical scroll container while editing.
- Markdown links, long URLs, code spans, and metadata wrap or clip safely without page-level horizontal scrolling.

## Verification Targets

- The drawer is closed on first paint.
- The drawer opens without clipping the main pane.
- Button labels and note metadata stay readable on real mobile widths.
- Compact admin note pages show no horizontal page overflow.
- Compact layouts do not reintroduce visible rail section headings or helper copy.
- Compact iPhone-width layouts keep the same UI font family and control sizing as other compact layouts.
- Compact note/history rails keep `Prev` and `Next` readable in one row without overflow.
