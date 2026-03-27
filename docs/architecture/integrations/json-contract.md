# JSON Contract

## Note Success

- `POST /records` returns the created note object with `id`.
- `PUT /records/{id}` returns the updated note object with `id`.
- `DELETE /records/{id}` returns `204` with no body.
- JSON note payloads continue to store canonical raw Markdown in `body`.

## History Success

- `GET /records/{id}/history` returns JSON array of revisions.
- History JSON remains admin-only.

## Navigation Success

- `GET /records/{id}/prev` returns `{ "id": "..." }` or `{ "id": null }`.
- `GET /records/{id}/next` returns `{ "id": "..." }` or `{ "id": null }`.
- HTML timeline placeholders are derived from `null` responses rather than a separate API.

## Error Shape

- Error responses stay machine-readable and non-HTML on `/records/*`.
