# Search and Pagination Behavior

## Scope

- Search covers current note alias, title, and body.
- Search does not index revision history.
- Public search returns only public notes.
- Admin search returns public and private notes.

## Query Surface

- `/search` is the canonical HTML browse and query surface.
- `q` is a plain text full-text query.
- `sort` selects server-side ordering.
- `cursor` is opaque and search-specific.
- `limit` defaults to the configured search page size and is capped at `100`.

## Sort Values

- `relevance`
- `updated_desc`
- `updated_asc`
- `created_desc`
- `created_asc`
- `title_asc`
- `title_desc`

## Ordering Rules

- Empty `q` defaults to `updated_desc`.
- Non-empty `q` defaults to `relevance`.
- `relevance` orders by search rank, fallback similarity, `updated_at DESC`, and `id ASC`.
- Timestamp sorts use the chosen timestamp plus `id`.
- Title sorts use normalized title plus `id`.
- Note-to-note `Prev` and `Next` continue to use `created_at`.

## Cursor Rules

- Cursor pagination is canonical for browse and search results.
- Empty cursor means first page.
- Missing next page yields no further cursor.
- Cursor payload must encode enough information to reject mismatched `q` or `sort`.

## Result Shape

- Empty-query results and searched results use the same compact card language.
- Result cards may show a contextual snippet rather than the plain derived summary.
- Admin results may show favorite and visibility state.
- Results never dump the full note set to the client at once.
