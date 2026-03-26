# Public Root Contract

## Route Intent

- `GET /` is the public note index after setup completes.
- Unauthenticated users land on the global note shell, not a bare list page.
- Authenticated admins may still open `/`, but `/admin` remains the full admin index.

## Layout Rules

- The persistent side menu is visible on the public root.
- The rail contains search, scope context, recent public notes, and session actions.
- The main pane remains a dense public result list optimized for thousands of rows.
- The rail supplements the list; it does not replace the main result pane.

## Content Rules

- Public rows show title, summary, created time, updated time, and search context when a query is active.
- Opaque note IDs are not shown in normal UI.
- When a note has no heading-derived title, the row title is `Untitled note`.

## Search Rules

- Search is server-side and query-param driven.
- The primary query input lives in the rail.
- Empty-state and no-result copy remain terse and non-marketing.
