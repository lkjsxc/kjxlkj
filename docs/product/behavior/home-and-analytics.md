# Home and Analytics Behavior

## Home Content

- The homepage top section renders the editable global `home_title`.
- The homepage may render optional intro Markdown directly below that title.
- Empty intro copy removes the block entirely rather than rendering placeholder text.
- `Quick search` always renders.
- `Popular notes`, `Recently updated`, and `Favorites` follow the configured visibility and order.
- Initial installs default each visible note section limit to `5`.
- Each visible note section ends with one `View more notes` card that links to the matching `/search` state.

## Popular Windows

- Popular-note ranking is available for `7d`, `30d`, and `90d`.
- The homepage defaults to `30d`.
- The selected window is controlled by the `popular_window` query parameter on `/`.
- The same `popular_window` parameter is reused on `/search` when `sort=popular_desc`.
- Window switching behaves like server-side sorting on `/search`; it does not depend on client-side reordering.
- Guest homepage popularity uses public notes only.
- Admin homepage popularity may include private notes.

## Popular Metrics

- Popular-note cards show the selected rolling-window count.
- Popular-note cards also show the lifetime `all-time` view total.
- Lifetime totals come from the stored `view_count_total` value.
- When rolling-window totals tie, higher lifetime totals sort first.

## View Counting

- A successful canonical `GET /{ref}` note page increments note analytics.
- Counting happens for both guest and admin note views.
- Redirect responses do not increment counters.
- Search pages, home pages, dashboards, settings pages, history indexes, and revision pages do not increment counters.

## Stored Metrics

- Each note stores lifetime view count and last-viewed timestamp.
- Recent popularity is derived from per-note daily view rollups.
- The product exposes `7d`, `30d`, and `90d` rolling totals.
- Rolling totals use UTC day buckets including the current UTC day.

## Visibility

- Dashboard surfaces show popularity and note-view analytics.
- Admin note pages show note-level view metrics.
- Popularity-oriented homepage and search rows may expose rolling and lifetime totals.
