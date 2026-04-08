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
- Live note context.
- Timeline navigation.
- One history card labeled `History` that opens the dedicated history page.
- One GitHub section that links to `https://github.com/lkjsxc/kjxlkj`.
- Mode-specific actions.
- On note and history pages, the rail places `History` above GitHub and GitHub above the trailing action block.
- On guest note and history pages, the trailing action block contains `Admin sign in`.
- On admin note and history pages, the trailing action block contains note management actions such as `Delete note` and `Logout`.
- `Prev` and `Next` timeline slots follow [../../navigation/timeline/README.md](../../navigation/timeline/README.md).
- Visible section titles such as `CREATE`, `NAVIGATE`, `ACTIONS`, and `SCOPE` are omitted.
- Section boundaries use spacing first and minimal separators only when needed.

## Metadata Rules

- Created and updated values render as browser-local 24-hour time.
- Live-note cards keep Created and Updated inside the shared card metadata used across Home, Search, and History surfaces.
- Timestamp layout must stack or grid safely.
- Raw note IDs are not shown in normal rail metadata.
- Long note titles or previews may not cause timestamp collisions.
- Rail cards keep consistent heights even when text is long.
- Rail note titles clamp to one visible line with end truncation.
- Rail summary previews clamp to two visible lines with end truncation.

## Action Treatment

- Home, Search, Dashboard, New note, Logout, and Delete use restrained controls from the same size family.
- `New note` is the first admin action in the rail.
- Guest sign-in may sit lower in the rail rather than directly under navigation.
- Narrow screens add one quiet top-right menu toggle plus a backdrop.
- Brand spacing must clearly separate `kjxlkj` from the first navigation card.

## Timeline Rules

- The timeline always renders exactly two cards.
- `Prev` and `Next` keep the shared fixed positions even when one target does not exist.
- Timeline cards reuse the same note-summary card language as Home and Search.
- Timeline labels remain outside the cards instead of becoming part of the card body.
- Timeline cards show title, short summary preview, and created time when a target exists.
- Timeline preview text follows the same compact summary rules as Home, Search, and History cards.
- Missing targets render as muted non-interactive cards with explanatory copy.
- Compact note/history rails keep the timeline vertical; see [../../navigation/timeline/compact-rail-layout.md](../../navigation/timeline/compact-rail-layout.md).
