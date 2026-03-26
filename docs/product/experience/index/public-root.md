# Public Root Contract

## Route Intent

- `GET /` is the public note index after setup completes.
- Unauthenticated users land on a searchable list, not a rail-first shell.
- Authenticated admins may still open `/`, but `/admin` remains the full admin index.

## Layout Rules

- No persistent left rail on the public root.
- A compact top bar contains product name, search, pagination context, and sign-in/admin text actions.
- The main area is a dense note list optimized for thousands of rows.

## Content Rules

- Public rows show title, summary, created time, updated time, and search context when a query is active.
- Opaque note IDs are not shown in normal UI.
- When a note has no heading-derived title, the row title is `Untitled note`.

## Search Rules

- Search is server-side and query-param driven.
- The query input is first-class on the public root.
- Empty-state and no-result copy remain terse and non-marketing.
