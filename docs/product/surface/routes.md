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
  - returns auth-aware homepage shell using optional `popular_window`
- `GET /admin` and `GET /admin/`:
  - before setup: `302` to `/setup`
  - without valid session: `302` to `/login`
  - with valid session: `200` HTML admin dashboard
- `GET /search`:
  - before setup: `302` to `/setup`
  - without valid session: `200` HTML public browse/search page using `q`, `direction`, `sort`, `cursor`, `limit`
  - with valid session: `200` HTML admin-capable browse/search page using `q`, `direction`, `sort`, `cursor`, `limit`

## Asset Delivery

- `GET /assets/icon.svg`:
  - returns the canonical site icon SVG
- `GET /assets/vendor/toastui/3.2.2/toastui-editor.min.css`:
  - returns vendored editor CSS
- `GET /assets/vendor/toastui/3.2.2/toastui-editor-dark.min.css`:
  - returns vendored dark-theme CSS
- `GET /assets/vendor/toastui/3.2.2/toastui-editor-all.min.js`:
  - returns vendored editor JS bundle

## Note Viewing

- `GET /{ref}`:
  - note not found: `404`
  - note is private and no session: `404`
  - canonical redirect responses do not count as note views
  - accessible note: `200` HTML note page with Markdown editor for admins
- `GET /{ref}/history`:
  - note not found: `404`
  - note is private and no session: `404`
  - accessible note: `200` HTML history index using `cursor`, `direction`, and `limit`
- `GET /{ref}/history/{revision_number}`:
  - note not found: `404`
  - revision not found: `404`
  - revision is private and no session: `404`
  - accessible revision: `200` HTML history snapshot

## Note Management (Admin Only)

- `POST /records`:
  - without valid session: `401` JSON error
  - invalid payload or missing `body`: `400` JSON error
  - valid session: `201` JSON with new note and generated `id`
- `PUT /records/favorites/order`:
  - without valid session: `401` JSON error
  - invalid favorite set or invalid IDs: `400` JSON error
  - valid session: `204`
- `PUT /records/{id}`:
  - without valid session: `401` JSON error
  - note not found: `404` JSON error
  - valid session and note exists: `200` JSON with updated note
- `DELETE /records/{id}`:
  - without valid session: `401` JSON error
  - note not found: `404` JSON error
  - valid session: `204`
- `POST /admin/settings`:
  - without valid session: `302` to `/login`
  - invalid payload: `400` HTML validation page
  - valid session: `303` to `/admin`

## Revision History and Navigation JSON

- `GET /records/{id}/history`:
  - without valid session: `401`
  - note not found: `404`
  - valid session: `200` JSON paginated revision payload using `cursor`, `direction`, and `limit`
- `GET /records/{id}/prev`:
  - note not found or inaccessible: `404`
  - returns previous accessible note `id` by `created_at`
- `GET /records/{id}/next`:
  - note not found or inaccessible: `404`
  - returns next accessible note `id` by `created_at`

## Health Check

- `GET /healthz` -> `200` with body `ok`.
