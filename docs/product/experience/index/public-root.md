# Home Page Contract

## Route Intent

- `GET /` is the homepage after setup completes.
- Guests land on a public-facing home surface rather than a raw browse grid.
- Signed-in admins still get the homepage, but `/search` remains the full browse surface.

## Layout Rules

- The persistent side menu is visible on the homepage.
- The main pane uses a short stacked composition with restrained gaps between the page title and first section.
- `Quick search`, `Recently updated`, and `Favorites` use the same lightweight section language.
- Each note block uses compact but not cramped cards in a responsive grid.
- Narrow screens collapse to one column without stretched card heights.
- The homepage avoids tall summary panels and tall statistics blocks.
- The quick-search section should not visually dominate the recent and favorite sections.
- Section wrappers avoid redundant borders when they do not contain note cards.
- Grid gaps are slightly wider than the older tight dashboard/home spacing.

## Content Rules

- The homepage contains a quick search section, recent notes, and favorite notes.
- The homepage does not contain a statistics block.
- `Recently updated` ends with one clear browse-action card that links to `/search`.
- Guest homepage data is public-only.
- Signed-in admins may see private-capable recent and favorite blocks plus quick admin actions.
- Opaque IDs are not shown in normal homepage cards.
- Favorites follow the persistent admin-defined favorite order.
