# List and Fetch Behavior

## List (`GET /v1/records`)

- Returns all persisted records.
- Sort order is lexicographic ascending by `id`.
- Response payload is deterministic for identical storage state.

## Fetch (`GET /v1/records/{id}`)

- Returns exact stored record if present.
- Returns `404` with JSON error if not present.
- Response must include current `revision` and `updated_at`.
