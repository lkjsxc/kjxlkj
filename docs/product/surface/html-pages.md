# HTML Pages Contract

## Canonical Pages

- `/`: auth-aware homepage.
- `/admin`: admin dashboard.
- `/admin/settings`: admin settings page.
- `/search`: auth-aware browse and search page.
- `/{ref}`: live note page, live media page, or saved-snapshot page.
- `/{ref}/history`: shared history index for the live resource at `/{ref}`.

## Shared Rules

- Every page renders inside the global shell.
- Root, admin, admin-settings, and search pages remain shell-first inside that shell.
- Resource page shell rules are owned by [../experience/shell/README.md](../experience/shell/README.md).
- Editor rules are owned by [../experience/editor/README.md](../experience/editor/README.md).
- Shared title, description, canonical, and robots-meta rules are owned by [../discoverability/head-metadata.md](../discoverability/head-metadata.md).
