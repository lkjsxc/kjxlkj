# Route Surface Contract

## HTML Setup + Session Endpoints

- `GET /`:
  - before setup: `302` to `/setup`
  - after setup without session: `200` HTML (public mode)
  - after setup with valid session: `200` HTML (admin mode)
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
- `GET /admin`:
  - before setup: `302` to `/setup`
  - after setup without valid session: `302` to `/login`
  - with valid session: `200` HTML admin page

## Read Endpoints

- `GET /healthz` -> `200` with body `ok`.
- `GET /v1/records` -> `200` with JSON array sorted by `id`.
- `GET /v1/records/{id}` -> `200` with record JSON or `404`.

## Write Endpoints

- `PUT /v1/records/{id}` -> `200` on update, `201` on create.
- `DELETE /v1/records/{id}` -> `204` on delete, `404` when absent.

## Content Type

- `/healthz` returns plain text.
- Setup/session/admin routes return HTML.
- Record API success and error payloads return JSON.
