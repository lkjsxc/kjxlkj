# Test Strategy Contract

## Unit Tests

- ID validation and tag normalization.
- Revision and timestamp update logic.
- Storage adapter file read/write/delete behavior.
- CLI command parsing and gate orchestration.

## Integration Tests

- Health endpoint behavior.
- Public list/fetch behavior.
- Token-protected write behavior.
- End-to-end create, update, delete lifecycle.
- Template-level dark-shell and compact-nav markup behavior.

## Compose Verification

Compose verify profile must execute the full quality gate bundle.

## Browser Verification

- Browser-rendered screenshots verify desktop and compact layouts.
- Visual checks assert dark surfaces, readable control contrast, and subdued compact navigation.
