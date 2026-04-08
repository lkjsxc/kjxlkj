# Typography and Density Contract

## Type Direction

- Shell, controls, editor chrome, rendered Markdown, and preview use one explicit system UI stack.
- The stack must resolve consistently across desktop, Android, iPhone, and iPad.
- Narrow-screen or mobile-specific font substitutions are not allowed.
- IDs are not used as visible typography anchors in normal UI.

## Density Rules

- Homepage blocks, dashboard sections, search results, and history pages use dense layouts fit for thousands of notes.
- Rails, history entries, note metadata, and editor chrome stay compact.
- Text-first actions reduce chrome bulk relative to note content.
- Tall empty vertical gaps should be removed before reducing useful content.
- Wide layouts may not introduce extra top or side whitespace that compact layouts do not need.
- Soft oversized corner radii count as unnecessary chrome bulk and should be removed.
- Page title to first-section spacing stays restrained on Home, Search, Dashboard, and History.
- `Quick search`, `Popular`, and `Recently updated` may use a slightly looser internal gap than the older build when that improves scan speed.
- Page titles may scale slightly, but wide screens may not inflate them into billboard-sized banners.
- Dense layouts may slightly increase note-card footprint when that keeps metadata inside the card.

## Readability Rules

- Metadata remains lower contrast than titles.
- Local 24-hour time strings must fit without awkward wrapping.
- Dense layouts still keep title, summary, and timestamps clearly separated.
- Card metadata labels such as `Created` and `Updated` must remain fully inside the card at all supported widths.
- Note-card titles clamp to one visible line.
- Long prose, URLs, aliases, and inline code wrap inside their container instead of forcing page overflow.
