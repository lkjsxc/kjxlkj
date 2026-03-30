# Home Page Contract

## Route Intent

- `GET /` is the homepage after setup completes.
- Guests land on a public-facing home surface rather than a raw browse grid.
- Signed-in admins still get the homepage, but `/search` remains the full browse surface.

## Layout Rules

- The persistent side menu is visible on the homepage.
- The main pane uses a capped content width on desktop so homepage sections do not stretch into overly wide slabs.
- The page title and first control block stay visually close together.
- `Quick search`, `Recently updated`, and `Favorites` use the same dark-panel language, but quick search is rendered as the compact lead strip of the page.
- Each note block uses compact but not cramped cards in a responsive grid.
- Narrow screens collapse to one column without stretched card heights.
- The homepage avoids tall summary panels and tall statistics blocks.
- The quick-search strip should not visually dominate the recent and favorite sections.
- On desktop, the quick-search input should stay noticeably narrower than the recent and favorite section width.

## Content Rules

- The homepage contains a quick search section, recent notes, and favorite notes.
- The homepage does not contain a statistics block.
- `Recently updated` ends with one clear browse-action card that links to `/search`.
- Guest homepage data is public-only.
- Signed-in admins may see private-capable recent and favorite blocks plus quick admin actions.
- Opaque IDs are not shown in normal homepage cards.
- Quick search may include only the heading, input, and submit action without helper prose.
