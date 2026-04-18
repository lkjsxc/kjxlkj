# Scalable Listing Contract

## Shared Behavior

- Homepage note blocks, search results, history cards, and live-resource `Prev` / `History` / `Next` cards share the same compact card language.
- Browse/search/history cards are compact, sortable server-side, and safe for thousands of notes.
- The list is cursor-paginated, not infinite scroll.
- Home sections may append one final browse card that shares the same grid rhythm without mimicking a resource card exactly.

## Row Hierarchy

- Title is the strongest text.
- Summary is secondary, clipped to a short preview, and stripped of leading Markdown control markers.
- Badge rows communicate kind first, then optional favorite state, then visibility when visibility is intentionally exposed.
- Created and updated times use browser-local 24-hour formatting.
- Live-note cards on note and history pages use the same Created/Updated metadata language as Home and Search cards.
- Admin rows may show visibility state, but guest rows do not expose private-only metadata.
- Metadata uses a dedicated layout that stays readable even when title or summary text is long.
- Summary preview appends `...` whenever hidden meaningful content remains.
- Title is single-line clamped.
- Summary is two-line clamped so long notes do not enlarge sibling cards.
- Dense layouts should reduce vertical padding before reducing useful metadata.
- Grid cards may gain a small amount of height so timestamp rows remain fully contained.
- Media grid cards may stay thumbnail-only, but thumbnails use one fixed cropped `128px` height.
- Media card badges and metadata must sit below the thumbnail without overlap.
- Live-resource `Prev` / `History` / `Next` cards keep the same one-line title clamp and two-line summary clamp as other note-summary cards.
- Live-resource timeline cards inside note/media pages align to the same max readable width as the main prose/editor surface rather than the full shell column.

## Action Treatment

- Per-page actions use text-first controls instead of filled buttons.
- Pagination actions are text links with disabled states, not large pills.
- Search and filter chrome must not dominate the list.
- `/search` remains authoritative for result browsing.
