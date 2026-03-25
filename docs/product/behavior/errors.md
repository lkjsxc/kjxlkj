# Error Behavior Contract

## Error JSON Shape

```json
{
  "error": "unauthorized",
  "message": "Session required"
}
```

## Error Codes

- `unauthorized` -> `401`: No valid session for write operation.
- `invalid_request` -> `400`: Malformed request body.
- `not_found` -> `404`: Note does not exist or is private.
- `storage_error` -> `500`: Database error.

## Determinism Rules

- `error` values are stable identifiers.
- `message` values are concise and human-readable.
- No stack traces are emitted in HTTP responses.

## HTML Error Pages

- `404` page: "Note not found" with link to home.
- `500` page: "Something went wrong" with link to home.
