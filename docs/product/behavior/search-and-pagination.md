# Search and Pagination Behavior

## Search Inputs

- `/search` accepts `q`, `kind`, `scope`, `sort`, `popular_window`, `cursor`, `direction`, and `limit`.
- `kind=all|note|media`.
- `scope=all|favorites`.
- `popular_window=1d|7d|30d|90d|all` when popularity sorting is active.
- Search UI exposes a visible popularity-window control so `popular_window` is not a hidden-only state.

## Search Matching

- Search matches aliases, titles, bodies, and kind-aware metadata such as uploaded filenames.
- Search snippets may come from the Markdown body or other indexed descriptive text.
- Empty-query browse remains the canonical all-resources card view.

## Pagination

- `Prev` and `Next` remain the only canonical pager labels.
- Pagers preserve `kind`, `scope`, `sort`, `popular_window`, `limit`, and `q`.
- History pagination and search pagination keep the same directional cursor semantics.
