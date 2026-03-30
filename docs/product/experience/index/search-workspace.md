# Search Workspace Contract

## Route Intent

- `GET /search` is the only canonical browse and search page.
- The route is auth-aware:
  - guests search public notes
  - signed-in admins search public and private notes

## Layout Rules

- The global rail stays visible on desktop.
- The rail exposes a link to `/search`, not an inline search form.
- The main pane owns the actual query form and result list.
- The search controls are one aligned row on wide screens: input, query display, sort, and submit action.
- The active query is echoed in a compact read-only `Query` card near `Sort`.
- The page does not show a top-right `Browse notes` or auth action cluster.
- Explanatory helper cards such as `Search public titles and bodies.` are omitted.
- Empty-query state should still feel like a full notes page rather than an empty helper page.

## Search Rules

- Search is server-side and query-param driven.
- Query input uses `q`.
- Query direction uses `direction`.
- Sort input uses `sort`.
- Pagination uses `cursor` and `limit`.
- Empty query returns the first paginated page of all viewable notes.
- Result snippets may differ from derived summaries when search context is more useful.
- Non-empty query defaults to relevance ordering.
- Empty query defaults to updated-newest ordering.

## Result Rules

- Results keep the same dense row language as browse pages.
- Pagination actions should read like movement across result pages rather than endless loading.
- Wide result cards are large enough to keep created and updated metadata inside the card bounds.
- Guest results never expose private-only metadata.
- Admin results may show visibility and favorite state.
- Search result sorting and pagination remain server-side only.
