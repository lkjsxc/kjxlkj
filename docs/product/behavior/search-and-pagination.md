# Search and Pagination Behavior

## Scope

- Search covers current note alias, title, and body.
- Search does not index revision history in this pass.
- Public search returns only public notes.
- Admin search returns public and private notes.

## Query Surface

- `/search` is the canonical HTML query surface.
- `q` is a plain text full-text query.
- `cursor` is opaque and search-specific.
- `limit` defaults to the configured search page size and is capped at `100`.

## Ordering

- Search ranks matching notes before applying `updated_at DESC, id ASC`.
- Search may use PostgreSQL full-text ranking plus trigram-assisted fallback matching.
- Library browse pages keep `updated_at DESC, id ASC`.
- Note-to-note `Prev` and `Next` continue to use `created_at`.

## Cursor Rules

- Cursor pagination is canonical for browse pages and search results.
- Empty cursor means first page.
- Missing next page yields no further cursor.

## Result Shape

- Search results use the same compact card language as other note lists.
- Search result cards may show a contextual snippet rather than the plain derived summary.
- Admin results may show favorite and visibility state.
