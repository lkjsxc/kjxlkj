# Scalable Listing Contract

## Shared Behavior

- Homepage note blocks, search results, history cards, and rail timeline cards share the same compact card language.
- Browse/search/history cards are compact, sortable server-side, and safe for thousands of notes.
- The list is cursor-paginated, not infinite scroll.
- Home sections may append one final `View more notes` card that shares the same grid rhythm without mimicking a note card exactly.

## Row Hierarchy

- Title is the strongest text.
- Summary is secondary, clipped to a short preview, and stripped of leading Markdown control markers.
- Created and updated times use browser-local 24-hour formatting.
- Live-note cards on note and history pages use the same Created/Updated metadata language as Home and Search cards.
- Admin rows may show visibility state, but guest rows do not expose private-only metadata.
- Metadata uses a dedicated layout that stays readable even when title or summary text is long.
- Summary preview appends `...` whenever hidden meaningful content remains.
- Title is single-line clamped.
- Summary is two-line clamped so long notes do not enlarge sibling cards.
- Dense layouts should reduce vertical padding before reducing useful metadata.
- Grid cards may gain a small amount of height so timestamp rows remain fully contained.
- Rail timeline cards keep the same one-line title clamp and two-line summary clamp as other note-summary cards.

## Action Treatment

- Per-page actions use text-first controls instead of filled buttons.
- Pagination actions are text links with disabled states, not large pills.
- Search and filter chrome must not dominate the list.
- `/search` remains authoritative for result browsing.
