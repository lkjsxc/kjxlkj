# Home and Analytics Behavior

## Home Content

- The homepage hero uses only the editable global `home_intro_markdown`.
- Empty hero Markdown removes the hero block entirely rather than rendering placeholder text.
- `Quick search` always renders.
- `Popular notes`, `Recently updated`, and `Favorites` follow the configured visibility and order.
- Initial installs default each visible note section limit to `5`.
- Each visible note section ends with one `View more notes` card that links to the matching `/search` state.

## Popular Windows

- Popular-note ranking is available for `7d`, `30d`, and `90d`.
- The homepage defaults to `30d`.
- The admin dashboard also defaults to `30d`.
- Homepage and dashboard window switching are client-driven and replace the visible section in place.
- Homepage and dashboard switching must not reload the page.
- Homepage and dashboard switching must not add a query string, hash, or alternate path to the visible URL.
- `/search` remains the canonical query-param surface for popularity browsing and uses `popular_window` when `sort=popular_desc`.
- Homepage and dashboard switching use server-rendered HTML fragments rather than client-side reordering.
- Guest homepage popularity uses public notes only.
- Admin homepage popularity may include private notes.

## Popular Metrics

- Admin-facing popular-note cards show the selected rolling-window count.
- Admin-facing popular-note cards also show the lifetime `all-time` view total.
- Guest-facing popular-note cards do not reveal any view counts.
- Lifetime totals come from the stored `view_count_total` value.
- When rolling-window totals tie, higher lifetime totals sort first.

## View Counting

- A successful canonical `GET /{ref}` note page increments note analytics.
- Counting happens for both guest and admin note views.
- Redirect responses do not increment counters.
- Search pages, home pages, dashboards, settings pages, history indexes, and saved-snapshot pages do not increment counters.

## Stored Metrics

- Each note stores lifetime view count and last-viewed timestamp.
- Recent popularity is derived from per-note daily view rollups.
- The product exposes `7d`, `30d`, and `90d` rolling totals.
- Rolling totals use UTC day buckets including the current UTC day.
- Failed homepage or dashboard window refresh keeps the previous visible list and reports a compact inline error.

## Visibility

- Dashboard surfaces show popularity and note-view analytics.
- Admin note pages show note-level view metrics.
- Non-admin surfaces do not expose rolling or lifetime totals.
