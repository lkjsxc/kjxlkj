# Scalable Listing Contract

## Shared Behavior

- Public root and admin dashboard share the same dense listing language.
- Rows are compact, sortable by recency, and safe for thousands of notes.
- The list is cursor-paginated, not infinite scroll.

## Row Hierarchy

- Title is the strongest text.
- Summary is secondary and clipped to a short preview.
- Created and updated times use browser-local 24-hour formatting.
- Admin rows may show visibility state, but guest rows do not expose private-only metadata.

## Action Treatment

- Per-page actions use text-only controls instead of filled buttons.
- Pagination actions are text links with disabled states, not large pills.
- Search and filter chrome must not dominate the list.
