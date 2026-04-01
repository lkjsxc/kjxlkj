# Home Page Contract

## Route Intent

- `GET /` is the homepage after setup completes.
- Guests land on a public-facing home surface rather than a raw browse grid.
- Signed-in admins still get the homepage, but `/search` remains the full browse surface.

## Layout Rules

- The persistent side menu is visible on the homepage.
- The homepage uses the same restrained main-column width and vertical rhythm on rail-visible and drawer layouts.
- The main pane uses a short stacked composition with slightly larger gaps around `Quick search`, `Popular notes`, and `Recently updated`.
- `Quick search`, `Popular notes`, `Recently updated`, and `Favorites` use the same lightweight section language.
- Each note block uses compact but not cramped cards in a responsive grid.
- Narrow screens collapse to one column without stretched card heights.
- The homepage avoids tall summary panels and tall statistics blocks.
- The quick-search section should not visually dominate the recent and favorite sections.
- Section wrappers avoid redundant borders when they do not contain note cards.
- Grid gaps are slightly wider than the older tight dashboard/home spacing.

## Content Rules

- The homepage may render optional Markdown intro copy directly under `Home`.
- Signed-in admins may edit that intro copy directly from the opening `Home` area.
- The homepage contains a quick search section, popular notes, recent notes, and favorite notes.
- The homepage does not contain a statistics block.
- The popular-notes section exposes one server-driven window selector for `7d`, `30d`, and `90d`.
- The default homepage popularity window is `30d`.
- `Popular notes`, `Recently updated`, and `Favorites` each end with one clear browse-action card that links into `/search`.
- Guest homepage data is public-only.
- Signed-in admins may see private-capable popular, recent, and favorite blocks plus quick admin actions.
- Opaque IDs are not shown in normal homepage cards.
- Favorites follow the persistent admin-defined favorite order.
- The order and visibility of `Popular notes`, `Recently updated`, and `Favorites` are configurable only for the homepage.
