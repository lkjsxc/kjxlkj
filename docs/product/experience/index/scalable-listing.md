# Scalable Listing Contract

## Shared Behavior

- Homepage note blocks, admin library, search results, and history cards share the same compact card language.
- Browse/search/history cards are compact, sortable by recency, and safe for thousands of notes.
- The list is cursor-paginated, not infinite scroll.

## Row Hierarchy

- Title is the strongest text.
- Summary is secondary, clipped to a short preview, and stripped of leading Markdown control markers.
- Created and updated times use browser-local 24-hour formatting.
- Admin rows may show visibility state, but guest rows do not expose private-only metadata.
- Metadata uses a dedicated layout that stays readable even when title or summary text is long.
- Summary preview appends `...` whenever hidden meaningful content remains.
- Title and summary are line-clamped so long notes do not enlarge sibling cards.

## Action Treatment

- Per-page actions use text-first controls instead of filled buttons.
- Pagination actions are text links with disabled states, not large pills.
- Search and filter chrome must not dominate the list.
- The main list remains authoritative for result browsing.
