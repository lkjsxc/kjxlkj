# JSON Contract

## Resource Success

- `POST /{user}/resources/notes` returns the created resource object with `id` and `kind = "note"`.
- `POST /{user}/resources/media` returns the created resource object with `id` and `kind = "media"`.
- `PUT /{user}/resources/{id}` returns the updated resource object with `id`.
- `DELETE /{user}/resources/{id}` returns `204` with no body.

## History Success

- `GET /{user}/resources/{id}/history` returns newest-first saved snapshots plus `previous_cursor` and `next_cursor`.
- History JSON requires resource write permission.

## Navigation Success

- `GET /{user}/resources/{id}/prev` returns `{ "id": "..." }` or `{ "id": null }`.
- `GET /{user}/resources/{id}/next` returns `{ "id": "..." }` or `{ "id": null }`.

## Error Shape

- Error responses stay machine-readable and non-HTML on `/{user}/resources/*`.
