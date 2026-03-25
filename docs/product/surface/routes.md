# Route Surface Contract

## HTML Setup + Session Endpoints

- `GET /`:
  - before setup: `302` to `/setup`
  - after setup without session: `200` HTML landing page
  - after setup with valid session: `200` HTML landing page (admin mode)
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

## Admin Dashboard

- `GET /admin`:
  - before setup: `302` to `/setup`
  - without valid session: `302` to `/login`
  - with valid session: `200` HTML admin dashboard (list of all notes)
- `GET /admin/`:
  - same as `GET /admin`

## Note Viewing

- `GET /{slug}`:
  - note not found: `404` HTML not found page
  - note is private and no session: `404` HTML not found page
  - note is private with valid session: `200` HTML note page (editable)
  - note is public: `200` HTML note page (read-only for guests, editable for admin)
- `GET /{slug}/history`:
  - note not found: `404` HTML not found page
  - note is private and no session: `404` HTML not found page
  - accessible note: `200` HTML history index
- `GET /{slug}/history/{revision_number}`:
  - note not found: `404` HTML not found page
  - revision not found: `404` HTML not found page
  - revision is private and no session: `404` HTML not found page
  - accessible revision: `200` HTML history snapshot

## Note Management (Admin Only)

- `POST /records`:
  - without valid session: `401` JSON error
  - valid session: `201` JSON with new note (auto-generated slug)
- `PUT /records/{slug}`:
  - without valid session: `401` JSON error
  - note not found: `404` JSON error
  - valid session and note exists: `200` JSON with updated note
- `DELETE /records/{slug}`:
  - without valid session: `401` JSON error
  - note not found: `404` JSON error
  - valid session: `204` (soft delete)

## Revision History (Admin Only)

- `GET /records/{slug}/history`:
  - without valid session: `401` JSON error
  - note not found: `404` JSON error
  - valid session: `200` JSON array of revisions

## Navigation JSON

- `GET /records/{slug}/prev`:
  - note not found or inaccessible: `404` JSON error
  - returns JSON with previous accessible note's slug by `created_at` order
- `GET /records/{slug}/next`:
  - note not found or inaccessible: `404` JSON error
  - returns JSON with next accessible note's slug by `created_at` order

## Health Check

- `GET /healthz` -> `200` with body `ok`.

## Content Type

- `/healthz` returns plain text.
- Setup/session/admin/note/history routes return HTML.
- `/records/*` endpoints return JSON.
