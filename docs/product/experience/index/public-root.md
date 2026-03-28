# Public Root Contract

## Route Intent

- `GET /` is the public note browse page after setup completes.
- Unauthenticated users land on the global note shell, not a bare list page.
- Authenticated admins may still open `/`, but `/admin` remains the full admin index.

## Layout Rules

- The persistent side menu is visible on the public root.
- The rail contains navigation and session actions.
- Wide screens arrange public notes in a responsive multi-column grid.
- Narrow screens collapse back to a single-column stack.
- The rail supplements the list; it does not replace the main result pane.

## Content Rules

- Public rows show title, summary, created time, and updated time.
- Opaque note IDs are not shown in normal UI.
- When a note has no heading-derived title, the row title is `Untitled note`.
- The visible summary is the opening meaningful body excerpt.
- If more meaningful content exists after the displayed excerpt, the summary ends with `...`.

## Search Link Rule

- The rail links to `/search`.
- The public root itself does not carry the primary search input.
- The page does not carry top-right `Search` or `Admin sign in` actions.
