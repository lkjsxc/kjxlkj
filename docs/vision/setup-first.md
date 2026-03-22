# Setup-First Contract

## Condition

- No admin user exists.

## Required Behavior

- Before first admin setup completes, `GET /` must redirect to `/setup`.
- `GET /setup` must render a complete setup page for first-admin creation and must not return placeholder-only content.
- `/setup` must be the first actionable admin-facing flow.
- `/login` must not be the first flow before setup completion.
- `/admin` must not be accessible before setup completion.

## Exit Condition

- Initial admin setup has completed successfully.

## Post-Exit Behavior

- Login and admin-editor flows proceed according to [product contracts](../product/README.md).
- Login semantics remain password-only for fixed identity `admin`.
- Public content rendering proceeds according to [Public Site Flow](../product/flows/public-site.md).
