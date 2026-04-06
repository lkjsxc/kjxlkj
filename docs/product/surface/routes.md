# Route Surface Contract

## HTML Setup + Session Endpoints

- `GET /`:
  - before setup: `302` to `/setup`
  - after setup: `200` HTML homepage
- `GET /setup`:
  - before setup: `200` HTML setup page
  - after setup: `302` to `/login`
- `POST /setup`:
  - invalid payload before setup: `400` HTML validation page
  - valid payload before setup: `303` to `/login`
  - after setup: `302` to `/login`
- `GET /login`:
  - before setup: `302` to `/setup`
  - after setup without session: `200` HTML login page
  - with valid session: `303` to `/admin`
- `POST /login`:
  - before setup: `302` to `/setup`
  - invalid credentials: `401` HTML error page
  - valid credentials: `303` to `/admin` and sets `session_id` cookie
- `POST /logout`:
  - after setup: `303` to `/` and clears `session_id` cookie

## Home, Admin, and Search Pages

- `GET /`:
  - returns auth-aware homepage shell
- `GET /admin` and `GET /admin/`:
  - before setup: `302` to `/setup`
  - without valid session: `302` to `/login`
  - with valid session: `200` HTML admin dashboard
- `GET /admin/settings`:
  - before setup: `302` to `/setup`
  - without valid session: `302` to `/login`
  - with valid session: `200` HTML admin settings page
- `GET /search`:
  - before setup: `302` to `/setup`
  - without valid session: `200` HTML public browse/search page using `q`, `direction`, `sort`, `scope`, `popular_window`, `cursor`, `limit`
  - with valid session: `200` HTML admin-capable browse/search page using `q`, `direction`, `sort`, `scope`, `popular_window`, `cursor`, `limit`

## HTML Fragment Endpoints

- `GET /_/popular-notes/home/{window}`:
  - `{window}` is `7d`, `30d`, or `90d`
  - returns the homepage popular-notes section as HTML
  - uses current session state to decide guest vs admin note details
- `GET /_/popular-notes/admin/{window}`:
  - `{window}` is `7d`, `30d`, or `90d`
  - without valid session: `401`
  - with valid session: returns the admin dashboard popular-notes section as HTML
- invalid popularity fragment route parts return `404`

## Asset Delivery

- `GET /favicon.ico`:
  - returns the canonical production favicon
- `GET /assets/icon.svg`:
  - returns the authored vector icon source
- `POST /admin/markdown-preview`:
  - without valid session: `401` JSON error
  - valid session: `200` JSON containing rendered preview HTML

## Note Viewing

- `GET /{ref}`:
  - target not found: `404`
  - target is private and no session: `404`
  - canonical redirect responses do not count as note views
  - accessible current note: `200` HTML note page with first-party Markdown editor for admins
  - accessible saved snapshot: `200` HTML history snapshot page
- `GET /{ref}/history`:
  - note not found: `404`
  - note is private and no session: `404`
  - accessible note: `200` HTML history index using `cursor`, `direction`, and `limit`

## Note Management (Admin Only)

- `POST /records`:
  - without valid session: `401` JSON error
  - invalid payload or missing `body`: `400` JSON error
  - valid session: `201` JSON with new note editor state and generated `id`
- `PUT /records/favorites/order`:
  - without valid session: `401` JSON error
  - invalid favorite set or invalid IDs: `400` JSON error
  - valid session: `204`
- `PUT /records/{id}`:
  - without valid session: `401` JSON error
  - note not found: `404` JSON error
  - valid session and note exists: `200` JSON with updated note editor state
- `DELETE /records/{id}`:
  - without valid session: `401` JSON error
  - note not found: `404` JSON error
  - valid session: `204`
- `POST /admin/settings`:
  - without valid session: `302` to `/login`
  - invalid payload: `400` HTML validation page
  - valid session: `303` to `/admin/settings`

## Revision History and Navigation JSON

- `GET /records/{id}/history`:
  - without valid session: `401`
  - note not found: `404`
  - valid session: `200` JSON paginated saved-snapshot payload using `cursor`, `direction`, and `limit`
- `GET /records/{id}/prev`:
  - note not found or inaccessible: `404`
  - returns previous accessible note `id` by `created_at`
- `GET /records/{id}/next`:
  - note not found or inaccessible: `404`
  - returns next accessible note `id` by `created_at`

## Health Check

- `GET /healthz` -> `200` with body `ok`.
