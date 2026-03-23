# Upsert and Delete Behavior

## Upsert (`PUT /v1/records/{id}`)

- Request body must include `title`, `body`, and `tags`.
- Path `id` is canonical; body `id` is not accepted.
- Create returns `201` with `revision = 1`.
- Update returns `200` with incremented `revision`.

## Delete (`DELETE /v1/records/{id}`)

- Existing record: delete file and return `204`.
- Missing record: return `404` error payload.

## Write Timestamp

Each successful write updates `updated_at` to current UTC time.
