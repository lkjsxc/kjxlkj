# Write Authentication Contract

## Rule

Write endpoints require header `x-admin-token`.

## Validation

- Expected value comes from `ADMIN_TOKEN` environment variable.
- Missing or mismatched token returns `401` JSON error.
- Read endpoints never require token.

## Security Boundaries

- Token is compared exactly as UTF-8 string.
- Token is never echoed in response payloads.
