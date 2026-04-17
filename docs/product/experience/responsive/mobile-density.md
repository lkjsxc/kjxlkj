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
- Live resource pages keep fixed-size `Prev`, `History`, and `Next` cards even when one or two targets are unavailable.
- Drawer and narrow layouts keep the live-resource top row readable without sending timeline cards back into the rail.
- Narrow search and history pagers stay in one left-right row.
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
- Compact live resource pages keep `Prev`, `History`, and `Next` readable without overflow.
- Compact search and history pagers keep `Prev` and `Next` in one row without overflow.
