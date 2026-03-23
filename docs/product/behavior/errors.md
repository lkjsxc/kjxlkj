# Error Behavior Contract

## Error JSON Shape

```json
{
  "error": "unauthorized",
  "message": "x-admin-token is missing or invalid"
}
```

## Error Codes

- `unauthorized` -> `401`
- `invalid_request` -> `400`
- `not_found` -> `404`
- `storage_error` -> `500`

## Determinism Rules

- `error` values are stable identifiers.
- `message` values are concise and human-readable.
- No stack traces are emitted in HTTP responses.
