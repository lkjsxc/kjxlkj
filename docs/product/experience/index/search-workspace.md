# Search Workspace Contract

## Route Intent

- `GET /search` is the only canonical search page.
- The route is auth-aware:
  - guests search public notes
  - signed-in admins search public and private notes

## Layout Rules

- The global rail stays visible on desktop.
- The rail exposes a link to `/search`, not an inline search form.
- The main pane owns the actual query form and result list.
- The page does not show a top-right `Browse notes` or auth action cluster.
- Explanatory helper cards such as `Search public titles and bodies.` are omitted.

## Search Rules

- Search is server-side and query-param driven.
- Query input uses `q`.
- Pagination uses `cursor` and `limit`.
- Empty query renders the form plus terse guidance, not a default result dump.

## Result Rules

- Results keep the same dense row language as browse pages.
- Guest results never expose private-only metadata.
- Admin results may show visibility state.
