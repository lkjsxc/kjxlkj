# Admin Settings Flow

This contract defines the admin settings page and mutations.

## Access Rules

- `GET /admin/settings` requires authenticated admin session.
- Non-admin or unauthenticated access follows admin guard contract.

## Page Contract

- Root container: `<main id="admin-settings-page">`.
- Settings form ID: `#admin-settings-form`.
- Status banner ID: `#admin-settings-status`.
- Validation banner ID: `#admin-settings-errors`.

## Initial Settings Scope (v1)

- Site title (`site_title`).
- Session timeout in minutes (`session_timeout_minutes`).
- Search reindex trigger action (`reindex_now`).

## Mutation Surface

- `POST /admin/settings/save` persists `site_title` and `session_timeout_minutes`.
- `POST /admin/settings/reindex` triggers deterministic reindex operation.

## Validation Rules

- `site_title` is required and trimmed.
- `session_timeout_minutes` MUST be a bounded positive integer.
- Validation failures MUST re-render deterministic error output.

## UX Rules

- Settings page MUST be reachable from shared menu for admins.
- Save and reindex outcomes MUST update status banner deterministically.

## Cross-References

- Access policy: [../policies/access-control.md](../policies/access-control.md)
- Shared shell: [navigation-shell.md](navigation-shell.md)
