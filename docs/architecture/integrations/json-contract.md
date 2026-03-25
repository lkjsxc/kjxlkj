# JSON Integration Contract

## Note Management Endpoints

All `/records/*` endpoints return JSON.

## Create Success

- `POST /records` returns JSON object with created note.

## Update Success

- `PUT /records/{slug}` returns JSON object with updated note.

## Delete Success

- `DELETE /records/{slug}` returns no body with status `204`.

## History Success

- `GET /records/{slug}/history` returns JSON array of revisions.

## Navigation Success

- `GET /records/{slug}/prev` returns `{ "slug": "..." }` or `{ "slug": null }`.
- `GET /records/{slug}/next` returns `{ "slug": "..." }` or `{ "slug": null }`.

## Error Payloads

Error payload shape is defined in [product/behavior/errors.md](../../product/behavior/errors.md).
