# Search and Pagination Behavior

## Scope

- Search covers current note title plus current note body.
- Search does not index revision history in this pass.
- Public search returns only public notes.
- Admin search returns public and private notes.

## Query Surface

- HTML index routes accept `q`, `cursor`, and `limit`.
- `q` is a plain text full-text query.
- `cursor` is opaque and route-specific.
- `limit` defaults to `50` and is capped at `100`.

## Ordering

- Without `q`, lists sort by `updated_at DESC, id ASC`.
- With `q`, search first filters matching notes and then keeps `updated_at DESC, id ASC`.
- Note-to-note `Prev` and `Next` continue to use `created_at`.

## Cursor Rules

- Cursor pagination is canonical for both public and admin indexes.
- Empty cursor means first page.
- Missing next page yields no further cursor.
