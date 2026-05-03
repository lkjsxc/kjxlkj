# Auth and Password Behavior

## Initial Setup

- First setup requires email, username, display name, password, confirmation, and a one-time setup code.
- The setup code is generated at server startup when no user exists.
- The setup code is written to the server console only.
- Verification may set a deterministic setup code through `SETUP_CODE`.
- A consumed setup code cannot be reused.
- First setup creates one active user and one personal space using the username slug.
- The first user is `owner` of that personal space.

## Login Return Path

- `GET /login` accepts optional `return_to`.
- Guest-shell `Admin sign in` controls preserve the exact current same-origin path and query in `return_to`.
- Member-only HTML routes redirect to `/login` with the attempted same-origin path and query as `return_to`.
- `return_to` must be a same-origin relative path beginning with `/`.
- Setup, login, logout, reset-password, write API, and health routes are invalid return targets.
- Successful login redirects to valid `return_to`.
- Missing or invalid `return_to` redirects to `/`.

## Signed-In Password Change

- Signed-in users may change the password from `/account/password`.
- The form requires current password, new password, and confirmation.
- Current password uses the same no-user-leak verification behavior as login.
- Successful password change invalidates existing sessions.

## Forgotten Password Reset

- `POST /reset-password/request` accepts an email or username and returns a generic response.
- Existing accounts receive a one-time reset token by configured side channel.
- Local development may write reset links to the server console.
- Only a hash of the token is stored.
- Tokens expire after a short fixed lifetime.
- `GET /reset-password` renders the reset form.
- `POST /reset-password` sets a new password only for a valid unused token.
- Successful reset consumes the token and invalidates existing sessions.
