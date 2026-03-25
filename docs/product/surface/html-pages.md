# HTML Pages Contract

## Canonical Ownership

- Route and status behavior is defined in [routes.md](routes.md).
- Theme and contrast rules are defined in [../experience/theme/README.md](../experience/theme/README.md).
- Note and dashboard shell rules are defined in [../experience/shell/README.md](../experience/shell/README.md).
- Responsive behavior is defined in [../experience/responsive/README.md](../experience/responsive/README.md).

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
- Dark mode is the default and only documented theme.
