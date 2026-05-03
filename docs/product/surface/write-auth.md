# Write Authentication Contract

## Auth Actors

- Browser users authenticate with local credentials and opaque server sessions.
- API clients authenticate with scoped bearer tokens.
- Session and API token secrets are stored only as hashes.
- Authorization is checked against the target personal space on every request.

## Permissions

- `owner` can read, write, manage settings, manage members, broadcast, and mint service tokens.
- `admin` can read, write, manage settings, manage members, broadcast, and mint service tokens.
- `editor` can read space resources, write resources, and broadcast.
- `viewer` can read space-visible resources.
- `service` can only use the scopes attached to its API token.

## Session Auth Rule

- `POST /login` sets a host-only session cookie after valid credentials.
- `POST /logout` revokes the current session and clears session cookies.
- Cookie-authenticated writes require a valid CSRF token.
- `POST`, `PUT`, and `DELETE` under `/{user}/resources/*` require `WriteResource`.
- `POST /{user}/markdown-preview` requires `WriteResource`.
- `POST /{user}/settings*` requires `ManageSettings`.
- `POST /account/password` requires the signed-in user.

## Validation

- Missing, invalid, expired, or revoked credentials return `401`.
- Valid credentials with insufficient permission return `403`.
- Resource reads outside visibility scope return `404`.
- HTML member pages redirect to `/login` when no valid session exists.

## Security Boundaries

- Browser session cookies are `HttpOnly`.
- CSRF cookies are not authorization credentials.
- API tokens use the `Authorization: Bearer` header.
- API tokens are never accepted from query strings.
- Token hashes, not raw tokens, are persisted.
- Password hashes are never exposed in responses.
- Password reset tokens are never stored in plaintext.
- Setup tokens may be emitted to the server console for local first setup.
- Reset links use a side channel in production and console fallback in development.
- Media file routes must respect the same visibility and snapshot-visibility rules as HTML pages.
