# JSON Contract

## Resource Success

- `POST /resources/notes` returns the created resource object with `id` and `kind = "note"`.
- `POST /resources/media` returns the created resource object with `id` and `kind = "media"`.
- `PUT /resources/{id}` returns the updated resource object with `id`.
- `DELETE /resources/{id}` returns `204` with no body.

## History Success

- `GET /resources/{id}/history` returns newest-first saved snapshots plus `previous_cursor` and `next_cursor`.
- History JSON remains admin-only.

## Navigation Success

- `GET /resources/{id}/prev` returns `{ "id": "..." }` or `{ "id": null }`.
- `GET /resources/{id}/next` returns `{ "id": "..." }` or `{ "id": null }`.

## Error Shape

- Error responses stay machine-readable and non-HTML on `/resources/*`.
