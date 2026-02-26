# CSRF Contract

Back: [/docs/spec/security/README.md](/docs/spec/security/README.md)

## Enforcement Rule

State-changing HTTP operations MUST enforce CSRF validation for browser sessions.

## Token Contract

- CSRF token is bound to session.
- Token MAY be delivered via dedicated endpoint or authenticated session payload.
- Mutating requests MUST include matching token header.
- Token validation MUST be constant-time compare.
- Token scope MUST be bound to session identity.

## Exception Rules

- Purely read-only endpoints (`GET`) do not require CSRF token.
- WebSocket handshake MUST verify authenticated session before upgrade.
- Non-browser service-to-service auth MAY bypass CSRF when explicitly configured.

## Endpoint Coverage Rule

Every mutating route in [/docs/spec/api/http.md](/docs/spec/api/http.md) MUST declare CSRF requirement and test coverage.

## Error Semantics

- missing token: `403 CSRF_MISSING`
- invalid token: `403 CSRF_INVALID`
- mismatched token/session: `403 CSRF_INVALID`

## Related

- Sessions: [sessions.md](sessions.md)
- API errors: [/docs/spec/api/errors.md](/docs/spec/api/errors.md)
- HTTP contract: [/docs/spec/api/http.md](/docs/spec/api/http.md)
