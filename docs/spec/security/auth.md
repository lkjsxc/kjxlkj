# Auth Contract

Back: [/docs/spec/security/README.md](/docs/spec/security/README.md)

## Tenancy Model

- Deployment is single-tenant.
- The tenant MAY contain multiple user accounts.
- Role-based access (`owner`, `admin`, `editor`, `viewer`) is authoritative.

## First-Run Registration

- `POST /setup/register` MUST be enabled only when no owner account exists.
- After first owner creation, setup route MUST be locked.
- Repeated setup attempts MUST return deterministic rejection.

## Login/Logout

- Login validates password hash and creates a session.
- Logout revokes the current session token/cookie.
- Disabled users MUST be rejected at login.

## Security Controls

- Passwords MUST be hashed with memory-hard algorithm.
- Setup/login endpoints SHOULD be rate-limited.
- User-management endpoints MUST enforce role checks.

## Related

- Sessions: [sessions.md](sessions.md)
- Permissions: [/docs/spec/domain/permissions.md](/docs/spec/domain/permissions.md)
- CSRF: [csrf.md](csrf.md)
