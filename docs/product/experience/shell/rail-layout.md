# Rail Layout Contract

## Global Rail

- The rail is present on every HTML page.
- Desktop keeps the rail as a fixed left column.
- Rail width stays compact and must not be used as the primary full result list.
- Scrollable rails must keep metadata readable without line wrapping collisions.

## Rail Sections

- Brand and session mode.
- Admin-only `New note` action near the top of the rail, but not flush against the brand block.
- Primary navigation.
- Current note context.
- Created and updated metadata.
- `Prev` and `Next` timeline cards with explicit relationship labels.
- One history card that opens the dedicated history page.
- Mode-specific actions.
- Visible section titles such as `CREATE`, `NAVIGATE`, `ACTIONS`, and `SCOPE` are omitted.
- Section boundaries use spacing first and minimal separators only when needed.

## Metadata Rules

- Created and updated values render as browser-local 24-hour time.
- Timestamp layout must stack or grid safely.
- Raw note IDs are not shown in normal rail metadata.
- Long note titles or previews may not cause timestamp collisions.
- Rail cards keep consistent heights even when text is long.

## Action Treatment

- Home, Search, Dashboard, New note, Logout, and Delete use restrained controls from the same size family.
- `New note` is the first admin action in the rail.
- Guest sign-in may sit lower in the rail rather than directly under navigation.
- Narrow screens add one quiet top-right menu toggle plus a backdrop.
- Brand spacing must clearly separate `kjxlkj` from the first navigation card.

## Timeline Rules

- The timeline always renders exactly two cards.
- `Prev` and `Next` keep fixed positions even when one target does not exist.
- Missing targets render as muted non-interactive cards with explanatory copy.
