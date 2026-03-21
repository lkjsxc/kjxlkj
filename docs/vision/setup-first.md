# Setup-First Contract

## Condition

- No admin user exists.

## Required Behavior

- `/setup` must be the first actionable admin-facing flow.
- `/login` must not be the first flow before setup completion.
- `/admin` must not be accessible before setup completion.

## Exit Condition

- Initial admin setup has completed successfully.

## Post-Exit Behavior

- Login and admin-editor flows proceed according to [product contracts](../product/README.md).
- Public content rendering proceeds independently of admin session state.
