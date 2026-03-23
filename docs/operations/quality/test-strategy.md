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

## Compose Verification

Compose verify profile must execute the full quality gate bundle.
