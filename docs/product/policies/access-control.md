# Access Control Contract

## Session Requirements

- Admin-only routes require a valid session.
- Session invalidation must occur on logout.
- System has exactly one admin identity with fixed username `admin`.
- Post-setup login authenticates the fixed `admin` identity using password-only input.
- Both `/setup` and `/login` are password-only inputs; identity is fixed to `admin` in runtime behavior.

## Route-Level Expectations

- Setup routes are available only before first admin user exists.
  - See [Setup-First Contract](../../vision/setup-first.md) for ordering requirements.
- Login routes are available after setup completion.
- Admin mutation routes are inaccessible to logged-out users.
- Admin settings and trash routes are admin-only.
- Search route is available to non-admin and admin users after setup completion.
- Non-admin users have read-only access and MUST NOT receive editing or mutation surfaces.

## Security Baseline

- Session cookies are HTTP-only.
- Auth checks happen before admin handler logic executes.
