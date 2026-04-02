# Test Strategy Contract

## Unit Tests

- ID validation and tag normalization.
- Revision and timestamp update logic.
- Summary and title derivation, including hidden-content ellipsis behavior.
- CLI command parsing and gate orchestration.

## Integration Tests

- Health endpoint behavior.
- Public browse/fetch behavior.
- Dedicated search page behavior.
- Token-protected write behavior.
- End-to-end create, update, delete lifecycle.
- Template-level shell, drawer, grid/list, and editor markup behavior.

## Compose Verification

Compose verify profile must execute the full quality gate bundle.

## Browser Verification

- Browser-rendered screenshots verify desktop and compact layouts.
- Visual checks assert hidden rail headings and helper copy, absent top-right browse/search/login actions, preview-toggle flow, and the wide-screen public grid.
- Compact admin note checks must fail on horizontal overflow.
- Admin note checks must fail if idle editing triggers repeated no-op saves.
- Guest-facing checks must fail if view counts leak onto non-admin surfaces.
- Settings checks must fail if draggable home-section ordering does not persist after save.
