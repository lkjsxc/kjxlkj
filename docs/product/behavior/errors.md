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
- `not_found` -> `404`: Resource does not exist or is private.
- `storage_error` -> `500`: Database error.
- Multipart write endpoints always return JSON errors for invalid media or unsupported extensions.

## Determinism Rules

- `error` values are stable identifiers.
- `message` values are concise and human-readable.
- No stack traces are emitted in HTTP responses.

## HTML Error Pages

- `404` page: "Resource not found" with link to home.
- `500` page: "Something went wrong" with link to home.
