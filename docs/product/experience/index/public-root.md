# Home Page Contract

## Route Intent

- `GET /` is the homepage after setup completes.
- Guests land on a public-facing home surface rather than a raw browse grid.
- Signed-in admins still get the homepage, but `/search` remains the full browse surface.

## Layout Rules

- The persistent side menu is visible on the homepage.
- The main pane uses a short stacked composition.
- Each note block uses compact cards in a responsive grid.
- Narrow screens collapse to one column without stretched card heights.
- The homepage avoids tall summary panels and tall statistics blocks.

## Content Rules

- The homepage contains a quick search form, recent notes, and favorite notes.
- The homepage does not contain a statistics block.
- `Recently updated` ends with one clear browse-action card that links to `/search`.
- Guest homepage data is public-only.
- Signed-in admins may see private-capable recent and favorite blocks plus quick admin actions.
- Opaque IDs are not shown in normal homepage cards.
