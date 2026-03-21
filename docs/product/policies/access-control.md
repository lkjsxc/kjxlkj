# Access Control Contract

## Session Requirements

- Admin-only routes require a valid session.
- Session invalidation must occur on logout.

## Route-Level Expectations

- Setup routes are available only before first admin user exists.
  - See [Setup-First Contract](../../vision/setup-first.md) for ordering requirements.
- Login routes are available after setup completion.
- Admin mutation routes are inaccessible to logged-out users.

## Security Baseline

- Session cookies are HTTP-only.
- Auth checks happen before admin handler logic executes.
