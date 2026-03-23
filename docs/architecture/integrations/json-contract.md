# JSON Integration Contract

## Read Success

- `GET /v1/records` returns JSON array.
- `GET /v1/records/{id}` returns JSON object.

## Write Success

- `PUT` returns JSON object with persisted values.
- `DELETE` returns no body with status `204`.

## Error Payloads

Error payload shape is defined in [product/behavior/errors.md](../../product/behavior/errors.md).
