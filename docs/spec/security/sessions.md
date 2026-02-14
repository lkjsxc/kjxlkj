# Session Contract

Back: [/docs/spec/security/README.md](/docs/spec/security/README.md)

## Session Model

- Auth uses secure cookie sessions.
- Session TTL default is 7 days with rolling renewal.

## Cookie Rules

| Attribute | Requirement |
|---|---|
| `HttpOnly` | MUST be enabled |
| `Secure` | SHOULD be enabled in production |
| `SameSite` | MUST be `Lax` or stricter |
| Path | MUST scope to `/` |

## Session Storage

- Session state MUST be persisted in PostgreSQL.
- Session revocation MUST take effect immediately for new requests.
- Session records MUST bind to user identity and role-evaluation context.

## Related

- Auth: [auth.md](auth.md)
- Transport: [transport.md](transport.md)
