# Auth Contract

Back: [/docs/spec/security/README.md](/docs/spec/security/README.md)

## Tenancy Model

- Deployment is single-tenant.
- The tenant MAY contain multiple user accounts.
- Role-based access (`owner`, `admin`, `editor`, `viewer`) is authoritative.

## First-Run Registration

- `POST /setup/register` MUST be enabled only when no owner account exists.
- After first owner creation, setup route MUST be locked.
- Repeated setup attempts MUST return `409 SETUP_ALREADY_COMPLETED`.

## Login/Logout

- Login validates password hash and creates a session.
- Logout revokes the current session token/cookie.
- Disabled users MUST be rejected at login.
- Failed login MUST return `401 INVALID_CREDENTIALS` without revealing account existence.

## Password and Credential Policy

- Password hashes MUST use a memory-hard algorithm (Argon2id recommended).
- Password hash parameters MUST be versioned and upgradeable.
- Plaintext passwords MUST never be logged or persisted.

## Security Controls

- Setup/login endpoints MUST be rate-limited.
- User-management endpoints MUST enforce role checks.
- Session creation MUST bind client metadata needed for security review.

## Deterministic Error Mapping

- missing auth: `401 AUTH_REQUIRED`
- bad credentials: `401 INVALID_CREDENTIALS`
- disabled account: `403 USER_DISABLED`
- role violation: `403 ROLE_FORBIDDEN`
- setup already completed: `409 SETUP_ALREADY_COMPLETED`

## Related

- Sessions: [sessions.md](sessions.md)
- Permissions: [/docs/spec/domain/permissions.md](/docs/spec/domain/permissions.md)
- CSRF: [csrf.md](csrf.md)
- Error model: [/docs/spec/api/errors.md](/docs/spec/api/errors.md)
