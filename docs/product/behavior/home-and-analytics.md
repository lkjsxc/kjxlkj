# Home and Analytics Behavior

## Home Content

- The homepage may render optional explanatory Markdown directly under the `Home` page title.
- The intro copy is global app state edited from the admin dashboard.
- Empty intro copy removes the block entirely rather than rendering placeholder text.
- Homepage note blocks include `Popular notes`, `Recently updated`, and `Favorites`.

## Popular Windows

- Popular-note ranking is available for `7d`, `30d`, and `90d`.
- The homepage defaults to `30d`.
- The selected window is controlled by the `popular_window` query parameter on `/`.
- Window switching behaves like server-side sorting on `/search`; it does not depend on client-side reordering.
- Guest homepage popularity uses public notes only.
- Admin homepage popularity may include private notes.

## View Counting

- A successful canonical `GET /{ref}` note page increments note analytics.
- Counting happens for both guest and admin note views.
- Redirect responses do not increment counters.
- Search pages, home pages, dashboards, history indexes, and revision pages do not increment counters.

## Stored Metrics

- Each note stores lifetime view count and last-viewed timestamp.
- Recent popularity is derived from per-note daily view rollups.
- The product exposes `7d`, `30d`, and `90d` rolling totals.
- Rolling totals use UTC day buckets including the current UTC day.

## Admin Visibility

- Admin dashboard shows popularity and note-view analytics.
- Admin note pages show note-level view metrics.
- Guest UI may use popularity ordering without exposing internal-only numeric analytics.
