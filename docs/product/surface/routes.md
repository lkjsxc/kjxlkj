# Route Surface Contract

## HTML Setup + Session Endpoints

- `GET /`:
  - before setup: `302` to `/setup`
  - after setup: `200` HTML public note index
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
  - after setup: `204` and clears `session_id` cookie

## Searchable Index Pages

- `GET /`:
  - returns public note list with query params `q`, `cursor`, `limit`
- `GET /admin` and `GET /admin/`:
  - before setup: `302` to `/setup`
  - without valid session: `302` to `/login`
  - with valid session: `200` HTML admin note list with `q`, `cursor`, `limit`

## Note Viewing

- `GET /{id}`:
  - note not found: `404`
  - note is private and no session: `404`
  - accessible note: `200` HTML note page
- `GET /{id}/history`:
  - note not found: `404`
  - note is private and no session: `404`
  - accessible note: `200` HTML history index
- `GET /{id}/history/{revision_number}`:
  - note not found: `404`
  - revision not found: `404`
  - revision is private and no session: `404`
  - accessible revision: `200` HTML history snapshot

## Note Management (Admin Only)

- `POST /records`:
  - without valid session: `401` JSON error
  - valid session: `201` JSON with new note and generated `id`
- `PUT /records/{id}`:
  - without valid session: `401` JSON error
  - note not found: `404` JSON error
  - valid session and note exists: `200` JSON with updated note
- `DELETE /records/{id}`:
  - without valid session: `401` JSON error
  - note not found: `404` JSON error
  - valid session: `204`

## Revision History and Navigation JSON

- `GET /records/{id}/history`:
  - without valid session: `401`
  - note not found: `404`
  - valid session: `200` JSON array of revisions
- `GET /records/{id}/prev`:
  - note not found or inaccessible: `404`
  - returns previous accessible note `id` by `created_at`
- `GET /records/{id}/next`:
  - note not found or inaccessible: `404`
  - returns next accessible note `id` by `created_at`

## Health Check

- `GET /healthz` -> `200` with body `ok`.
