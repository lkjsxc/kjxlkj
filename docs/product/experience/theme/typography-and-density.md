# Typography and Density Contract

## Type Direction

- Shell, controls, editor chrome, and rendered Markdown use browser-default font families.
- Custom font stacks are not part of the product language.
- IDs are not used as visible typography anchors in normal UI.

## Density Rules

- Homepage blocks, dashboard panels, and search results use dense layouts fit for thousands of notes.
- Rails, history entries, note metadata, and editor chrome stay compact.
- Text-first actions reduce chrome bulk relative to note content.
- Tall empty vertical gaps should be removed before reducing useful content.
- Page title to first-section spacing should stay tight on Home, Search, and Dashboard.
- Desktop browse pages should cap content width before surfaces become visually slack.
- Home and Search lead strips should be denser than the larger content sections below them.
- Dense layouts may slightly increase note-card footprint when that keeps metadata inside the card.

## Readability Rules

- Metadata remains lower contrast than titles.
- Local 24-hour time strings must fit without awkward wrapping.
- Dense layouts still keep title, summary, and timestamps clearly separated.
- Card metadata labels such as `Created` and `Updated` must remain fully inside the card at all supported widths.
- Compact search controls should still align cleanly and never feel undersized relative to nearby section headings.
