# Auth Contract

Back: [/docs/spec/security/README.md](/docs/spec/security/README.md)

## Tenancy Model

- Deployment is single-tenant.
- One account scope is authoritative for system access.

## First-Run Registration

- `POST /setup/register` MUST be enabled only when no account exists.
- After first account creation, setup route MUST be locked.
- Repeated setup attempts MUST return deterministic rejection.

## Login/Logout

- Login validates password hash and creates a session.
- Logout revokes the current session token/cookie.

## Security Controls

- Passwords MUST be hashed with memory-hard algorithm.
- Setup/login endpoints SHOULD be rate-limited.

## Related

- Sessions: [sessions.md](sessions.md)
- CSRF: [csrf.md](csrf.md)
