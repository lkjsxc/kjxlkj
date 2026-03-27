# Scalable Listing Contract

## Shared Behavior

- Public root, admin dashboard, and search results share the same dense row language.
- Rows are compact, sortable by recency, and safe for thousands of notes.
- The list is cursor-paginated, not infinite scroll.

## Row Hierarchy

- Title is the strongest text.
- Summary is secondary, clipped to a short preview, and stripped of leading Markdown control markers.
- Created and updated times use browser-local 24-hour formatting.
- Admin rows may show visibility state, but guest rows do not expose private-only metadata.

## Action Treatment

- Per-page actions use text-first controls instead of filled buttons.
- Pagination actions are text links with disabled states, not large pills.
- Search and filter chrome must not dominate the list.
- The main list remains authoritative for result browsing.
