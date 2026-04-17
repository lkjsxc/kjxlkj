# Search and Pagination Behavior

## Search Inputs

- `/search` accepts `q`, `kind`, `scope`, `sort`, `cursor`, `direction`, and `limit`.
- `kind=all|note|media`.
- `scope=all|favorites`.
- Search popularity sorts are `popular_1d_desc`, `popular_7d_desc`, `popular_30d_desc`, `popular_90d_desc`, and `popular_all_desc`.
- Search does not expose a standalone popularity-window control.

## Search Matching

- Search matches aliases, titles, bodies, and kind-aware metadata such as uploaded filenames.
- Search snippets may come from the Markdown body or other indexed descriptive text.
- Empty-query browse remains the canonical all-resources card view.

## Pagination

- `Prev` and `Next` remain the only canonical pager labels.
- Pagers preserve `kind`, `scope`, `sort`, `limit`, and `q`.
- History pagination and search pagination keep the same directional cursor semantics.
