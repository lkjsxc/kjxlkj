# Session Contract

Back: [/docs/spec/security/README.md](/docs/spec/security/README.md)

## Session Model

- Auth uses secure cookie sessions.
- Session TTL default is 7 days with rolling renewal.
- Session IDs MUST be cryptographically random and non-guessable.
- Session renewal MUST rotate token identity on privilege-sensitive actions.

## Cookie Rules

| Attribute | Requirement |
|---|---|
| `HttpOnly` | MUST be enabled |
| `Secure` | SHOULD be enabled in production |
| `SameSite` | MUST be `Lax` or stricter |
| Path | MUST scope to `/` |

Cookie name and max-age MUST be documented in runtime config.

## Session Storage

- Session state MUST be persisted in PostgreSQL.
- Session revocation MUST take effect immediately for new requests.
- Session records MUST bind to user identity and role-evaluation context.

## Revocation and Broadcast

- Revocation events MUST invalidate concurrent connections (HTTP + WebSocket).
- Credential reset MUST revoke all active sessions for the user.
- Revocation propagation delay MUST be bounded and measured.

## Deterministic Outcomes

- expired session: `401 SESSION_EXPIRED`
- revoked session: `401 SESSION_REVOKED`
- missing session: `401 AUTH_REQUIRED`

## Related

- Auth: [auth.md](auth.md)
- Transport: [transport.md](transport.md)
- WebSocket contract: [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md)
