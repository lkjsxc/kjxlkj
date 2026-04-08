# Search and Pagination Behavior

## Scope

- Search covers current note alias, title, and body.
- Search does not index saved-snapshot history.
- Public search returns only public notes.
- Admin search returns public and private notes.
- `scope=favorites` limits search to favorite notes.

## Query Surface

- `/search` is the canonical HTML browse and query surface.
- `q` is a plain text full-text query.
- `direction` is `next` or `prev`.
- `sort` selects server-side ordering.
- `scope` selects the browse subset.
- `popular_window` selects the rolling window when popularity ordering is active.
- `cursor` is opaque and search-specific.
- `limit` defaults to the configured search page size and is capped at `100`.
- Homepage and dashboard popularity switching do not reuse the search query string.

## Sort Values

- `relevance`
- `updated_desc`
- `updated_asc`
- `created_desc`
- `created_asc`
- `title_asc`
- `title_desc`
- `popular_desc`
- `views_total_desc`
- `favorite_position_asc`

## Ordering Rules

- Empty `q` defaults to `updated_desc` when `scope=all`.
- Empty `q` defaults to `favorite_position_asc` when `scope=favorites`.
- Non-empty `q` defaults to `relevance`.
- `relevance` orders by search rank, fallback similarity, `updated_at DESC`, and `id ASC`.
- Timestamp sorts use the chosen timestamp plus `id`.
- Title sorts use normalized title plus `id`.
- `popular_desc` orders by the selected rolling-window count, then lifetime views, then `updated_at DESC`, then `id ASC`.
- `views_total_desc` orders by lifetime views, then `updated_at DESC`, then `id ASC`.
- `favorite_position_asc` orders by persistent favorite order and then `id`.
- Note-to-note `Prev` and `Next` remain governed by [../navigation/timeline/semantics-and-order.md](../navigation/timeline/semantics-and-order.md).

## Cursor Rules

- Cursor pagination is canonical for browse and search results.
- Empty cursor means first page.
- Forward and backward page movement remain shareable in the URL.
- Missing next page yields no next cursor.
- Missing previous page yields no previous cursor.
- Cursor payload must encode enough information to reject mismatched `q`, `sort`, `scope`, or `popular_window`.
- `direction=prev` returns the previous page in normal on-screen order rather than reversed order.
- Shared `Prev` / `Next` labels and compact pager layout follow [../navigation/paging/README.md](../navigation/paging/README.md).

## Result Shape

- Empty-query results and searched results use the same compact card language.
- Result cards may show a contextual snippet rather than the plain derived summary.
- Admin results may show favorite and visibility state.
- Results never dump the full note set to the client at once.
- Search chrome uses the shared pager contract instead of one `More notes` action.
- Empty-query `/search` does not echo a `Query` or `All notes` state card.
- Homepage `View more notes` cards deep-link into `/search` by changing `scope`, `sort`, and `popular_window` only.
