# Home Page Contract

## Route Intent

- `GET /` is the homepage after setup completes.
- Guests land on a public-facing home surface rather than a raw browse grid.
- Signed-in admins still get the homepage, but `/search` remains the full browse surface.

## Layout Rules

- The persistent side menu is visible on the homepage.
- The homepage uses the same restrained main-column width and vertical rhythm on rail-visible and drawer layouts.
- The main pane uses a short stacked composition with restrained gaps between the page title and first section.
- `Quick search`, `Popular`, `Recently updated`, and `Favorites` use the same lightweight section language.
- `Quick search`, `Popular`, and `Recently updated` use slightly more generous internal spacing than the older tight layout.
- Each note block uses compact but not cramped cards in a responsive grid.
- Narrow screens collapse to one column without stretched card heights.
- The homepage avoids tall summary panels and tall statistics blocks.
- The quick-search section should not visually dominate the note sections.
- Section wrappers avoid redundant borders when they do not contain note cards.
- Grid gaps are slightly wider than the older tight dashboard/home spacing.

## Content Rules

- The homepage hero renders only admin-authored `home_intro_markdown`.
- Empty hero Markdown removes the hero block entirely.
- The homepage always contains `Quick search`.
- The homepage may contain `Popular`, `Recently updated`, and `Favorites` in the configured order.
- The homepage does not contain a statistics block.
- The Popular section exposes one in-place window switch for `7d`, `30d`, `90d`, and `All time`.
- The default homepage popularity window is `30d`.
- Popular switching leaves the visible URL at `/`.
- Popular switching replaces the full section from server-rendered HTML instead of re-sorting the existing DOM.
- Popular switching preserves browser-local timestamp rendering after fragment replacement.
- Without JavaScript, the homepage stays on the default `30d` render.
- `Popular`, `Recently updated`, and `Favorites` each end with one `View more notes` card that links into `/search`.
- Guest Popular cards do not expose popularity totals.
- Admin Popular cards may expose the active popularity total only.
- When the active window is `All time`, the visible card metric label is `Views`.
- Guest homepage data is public-only.
- Signed-in admins may see private-capable popular, recent, and favorite blocks plus quick admin actions.
- The visible section label is `Popular`, while deep links still use `sort=popular_desc&popular_window=...`.
- Opaque IDs are not shown in normal homepage cards.
- Favorites follow the persistent admin-defined favorite order.
