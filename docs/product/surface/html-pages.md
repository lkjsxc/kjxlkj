# HTML Pages Contract

## Canonical Ownership

- Route and status behavior is defined in [routes.md](routes.md).
- Shared shell rules are defined in [../experience/shell-layout.md](../experience/shell-layout.md).
- Note-page behavior is defined in [../experience/note-page.md](../experience/note-page.md).
- Admin behavior is defined in [../experience/admin-workspace.md](../experience/admin-workspace.md).
- History UX is defined in [../experience/history-flow.md](../experience/history-flow.md).
- Responsive behavior is defined in [../experience/responsive-behavior.md](../experience/responsive-behavior.md).

## Page Inventory

- `/`: landing page with guest or admin-aware shell.
- `/admin`: admin dashboard.
- `/{slug}`: current note page.
- `/{slug}/history`: note history index.
- `/{slug}/history/{revision_number}`: historical snapshot page.
- `/setup`: initial admin setup.
- `/login`: session login.

## HTML Rendering Rules

- All HTML pages render server-side.
- Note and history pages remain readable without JavaScript.
- JavaScript enhances autosave, drawer toggling, and destructive actions.
