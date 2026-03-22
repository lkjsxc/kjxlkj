# Admin Trash Flow

This contract defines recoverable deletion and trash management.

## Access Rules

- `GET /admin/trash` requires authenticated admin session.
- Trash actions are admin-only.

## Deletion Model

- Delete action from editor/list is soft-delete by default.
- Soft-deleted articles move to trash state and are removed from active lists.
- Soft-deleted articles MUST not appear in public surfaces.

## Trash Page Contract

- Root container: `<main id="admin-trash-page">`.
- Trash list container: `#admin-trash-list`.
- Status banner: `#admin-trash-status`.

## Trash Mutations

- `POST /admin/trash/restore/{slug}` restores a soft-deleted article.
- `POST /admin/trash/delete-permanent/{slug}` permanently deletes article data.

## UX Rules

- Admin menu MUST include entry to trash page.
- Restore and permanent-delete outcomes MUST be deterministic and visible.
- Permanent delete SHOULD require explicit confirmation.

## Cross-References

- Admin editor base flow: [admin-editor.md](admin-editor.md)
- Shared shell: [navigation-shell.md](navigation-shell.md)
- Access control: [../policies/access-control.md](../policies/access-control.md)
