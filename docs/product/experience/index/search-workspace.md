# Search Workspace Contract

## Route Intent

- `GET /search` is the only canonical browse and search page.
- The route is auth-aware:
  - guests search public resources
  - signed-in admins search public and private resources

## Layout Rules

- The global rail stays visible on desktop.
- The rail exposes a link to `/search`, not an inline search form.
- The main pane owns the actual query form and result list.
- The search controls are one aligned row on wide screens: input, optional query display, kind, sort, popularity window, and submit action.
- The sort control, popularity-window control, and submit button share one visual control height and vertical alignment.
- The active query is echoed in a compact read-only card near sort only when `q` is non-empty.
- The sort control keeps an accessible label, but the visible `Sort` text is omitted.
- The page does not show a top-right `Browse notes` or auth action cluster.
- Explanatory helper cards such as `Search public titles and bodies.` are omitted.
- Empty-query state should still feel like a full resources page rather than an empty helper page.

## Search Rules

- Search is server-side and query-param driven.
- Search remains query-param driven even though homepage and dashboard popularity switching are in-place.
- Query input uses `q`.
- Query direction uses `direction`.
- Sort input uses `sort`.
- Scope input uses `scope`.
- Popular window input uses `popular_window`.
- Pagination uses `cursor` and `limit`.
- Returning to `/search` from a shell rail link restores the last remembered in-tab search URL and main-pane scroll position.
- Empty query returns the first paginated page of all viewable notes inside the active scope.
- Result snippets may differ from derived summaries when search context is more useful.
- Non-empty query defaults to relevance ordering.
- Empty-query all-notes scope defaults to updated-newest ordering.
- Empty-query favorites scope defaults to favorite-order browsing.

## Result Rules

- Results keep the same dense row language as browse pages.
- Pagination actions should read like movement across result pages rather than endless loading.
- Wide result cards are large enough to keep created and updated metadata inside the card bounds.
- Guest results never expose private-only metadata.
- Admin results may show visibility and favorite state.
- Search result sorting and pagination remain server-side only.
- Empty-query results do not render a `Query` or `All notes` state card.
- Home-section browse links preserve their server-side sort or scope on first load.
