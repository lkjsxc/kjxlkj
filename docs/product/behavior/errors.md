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
- `payload_too_large` -> `413`: Upload body or multipart part exceeds the configured limit.
- `not_found` -> `404`: Resource does not exist or is private.
- `storage_error` -> `500`: Object storage or filesystem error.
- `database_error` -> `500`: Database error.
- Application-raised multipart errors use the same JSON shape.
- Browser upload code must also handle plain-text or HTML errors from HTTP middleware and gateways.

## Determinism Rules

- `error` values are stable identifiers.
- `message` values are concise and human-readable.
- No stack traces are emitted in HTTP responses.

## HTML Error Pages

- `404` page: "Resource not found" with link to home.
- `500` page: "Something went wrong" with link to home.
