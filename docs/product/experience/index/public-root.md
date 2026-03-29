# Home Page Contract

## Route Intent

- `GET /` is the homepage after setup completes.
- Guests land on a public-facing home surface rather than a raw browse grid.
- Signed-in admins still get the homepage, but `/admin` remains the heavy admin workspace.

## Layout Rules

- The persistent side menu is visible on the homepage.
- The main pane uses a stacked dashboard composition rather than one giant list.
- Each note block uses compact cards in a responsive grid.
- Narrow screens collapse to one column without stretched card heights.

## Content Rules

- The homepage contains recent notes, favorite notes, and current month/year statistics.
- Guest homepage data is public-only.
- Signed-in admins may see private-capable recent/favorite blocks and quick admin actions.
- Opaque IDs are not shown in normal homepage cards.
- A quick search form may appear on the homepage, but canonical results stay on `/search`.
