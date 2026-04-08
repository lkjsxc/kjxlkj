# Pager Cursor and Labels

## Shared Pager Rules

- `/search` and `/{ref}/history` use explicit `Prev` and `Next` actions.
- The main-pane pager is separate from the note/history rail timeline.
- Empty cursor means the first page.
- Missing previous page disables `Prev`.
- Missing next page disables `Next`.
- URL-shareable paging uses `cursor`, `direction`, and route-specific extra fields such as `limit`, `q`, `sort`, `scope`, or `popular_window`.
- `direction=prev` still renders results in normal on-screen order.

## Label Rules

- `Prev` always means backward page movement.
- `Next` always means forward page movement.
- Search keeps explicit `Prev` / `Next` labels instead of `More notes`.
- History index uses the same label pair as search.
