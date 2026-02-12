# CSRF Contract

Back: [/docs/spec/security/README.md](/docs/spec/security/README.md)

## Enforcement Rule

State-changing HTTP operations MUST enforce CSRF validation for browser sessions.

## Token Contract

- CSRF token is bound to session.
- Token MAY be delivered via dedicated endpoint or authenticated session payload.
- Mutating requests MUST include matching token header.

## Exception Rules

- Purely read-only endpoints (`GET`) do not require CSRF token.
- WebSocket handshake MUST verify authenticated session before upgrade.

## Related

- Sessions: [sessions.md](sessions.md)
- API errors: [/docs/spec/api/errors.md](/docs/spec/api/errors.md)
