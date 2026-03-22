# Navigation Shell Contract

This contract defines the shared site navigation shell used by public and admin pages.

## Global Shell Rules

- Shell MUST render on all primary pages after setup completion:
  - `/`
  - `/article/{slug}`
  - `/search`
  - `/admin`
  - `/admin/settings` (admin only)
  - `/admin/trash` (admin only)
- Shell MUST remain server-rendered as the baseline.
- JavaScript MAY enhance shell interactions but MUST not be required for basic navigation.

## Responsive Layout Rules

- Wide screens MUST show a persistent left navigation column.
- Narrow screens MUST show a top bar with a left menu-toggle button.
- Narrow-screen menu toggle MUST open and close a navigation drawer.
- Keyboard navigation and focus order MUST remain deterministic.

## Stable Shell IDs

- Root shell wrapper: `#app-shell`.
- Left navigation container: `#app-nav`.
- Top bar container: `#app-topbar`.
- Menu toggle button: `#app-nav-toggle`.
- Main content region: `#app-main`.

## Menu Composition

- Menu MUST include:
  - article list entry point
  - search entry point
  - settings entry point for admins
  - trash entry point for admins
- Menu MUST be visible to non-admin users.
- Menu items unavailable to current role MUST be omitted.

## Privacy Rules in Navigation

- Non-admin users MUST NOT see private articles in menu listings.
- Admin users MAY see both public and private entries with clear private labeling.

## Cross-References

- Page-level IDs and route matrix: [page-contracts.md](page-contracts.md)
- Public visibility and privacy policy: [../policies/privacy.md](../policies/privacy.md)
- Access control and role gates: [../policies/access-control.md](../policies/access-control.md)
