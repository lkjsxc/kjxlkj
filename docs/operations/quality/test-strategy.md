# Test Strategy Contract

## Unit Tests

- ID validation and tag normalization.
- Revision and timestamp update logic.
- Storage adapter file read/write/delete behavior.
- CLI command parsing and gate orchestration.

## Integration Tests

- Health endpoint behavior.
- Public browse/fetch behavior.
- Dedicated search page behavior.
- Token-protected write behavior.
- End-to-end create, update, delete lifecycle.
- Template-level flat-shell, drawer, and editor markup behavior.

## Compose Verification

Compose verify profile must execute the full quality gate bundle.

## Browser Verification

- Browser-rendered screenshots verify desktop and compact layouts.
- Visual checks assert flat dark surfaces, hidden rail search, hidden `RECENT`, quiet drawer controls, and the single-mode editor surface.
