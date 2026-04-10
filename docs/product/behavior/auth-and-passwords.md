# Auth and Password Behavior

## Initial Setup

- First setup requires username, password, confirmation, and a one-time setup code.
- The setup code is generated at server startup when no admin exists.
- The setup code is written to the server console only.
- Verification may set a deterministic setup code through `SETUP_CODE`.
- A consumed setup code cannot be reused.

## Login Return Path

- `GET /login` accepts optional `return_to`.
- `return_to` must be a same-origin relative path beginning with `/`.
- Setup, login, logout, reset-password, write API, and health routes are invalid return targets.
- Successful login redirects to valid `return_to`.
- Missing or invalid `return_to` redirects to `/admin`.

## Signed-In Password Change

- Signed-in admins may change the password from `/admin/settings`.
- The form requires current password, new password, and confirmation.
- Current password uses the same no-user-leak verification behavior as login.
- Successful password change invalidates existing sessions.

## Forgotten Password Reset

- `POST /reset-password/request` creates a one-time reset token and writes it to the server console.
- Only a hash of the token is stored.
- Tokens expire after a short fixed lifetime.
- `GET /reset-password` renders the reset form.
- `POST /reset-password` sets a new password only for a valid unused token.
- Successful reset consumes the token and invalidates existing sessions.
