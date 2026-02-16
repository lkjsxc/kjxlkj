# Transport Security

Back: [/docs/spec/security/README.md](/docs/spec/security/README.md)

## TLS Boundary

- App server MAY run plain HTTP only on trusted internal interfaces.
- TLS termination MUST be handled by a trusted reverse proxy or direct TLS endpoint.

## Header Expectations

When behind a trusted reverse proxy:

- forwarded proto/header mapping MUST be configured explicitly
- secure cookie behavior MUST respect forwarded HTTPS context

## Deployment Guidance

- Do not expose plain HTTP publicly.
- Restrict direct app and DB network access to trusted origins.
- Validate host and origin boundaries for browser-facing routes.

## Related

- Deployment: [/docs/spec/architecture/deployment.md](/docs/spec/architecture/deployment.md)
- Auth: [/docs/spec/security/auth.md](/docs/spec/security/auth.md)
