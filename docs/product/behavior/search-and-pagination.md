# Search and Pagination Behavior

## Scope

- Search covers current note title plus current note body.
- Search does not index revision history in this pass.
- Public search returns only public notes.
- Admin search returns public and private notes.

## Query Surface

- `/search` is the canonical HTML query surface.
- `q` is a plain text full-text query.
- `cursor` is opaque and search-specific.
- `limit` defaults to `50` and is capped at `100`.

## Ordering

- Search first filters matching notes and then keeps `updated_at DESC, id ASC`.
- Browse pages keep the same ordering without `q`.
- Note-to-note `Prev` and `Next` continue to use `created_at`.

## Cursor Rules

- Cursor pagination is canonical for browse pages and search results.
- Empty cursor means first page.
- Missing next page yields no further cursor.
