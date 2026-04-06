# HTML Pages Contract

## Canonical Pages

- `/`: auth-aware homepage.
- `/admin`: admin dashboard.
- `/admin/settings`: admin settings page.
- `/search`: auth-aware browse and search page.
- `/{ref}`: live note page or saved-snapshot page.
- `/{ref}/history`: note history index.

## Shared Rules

- Every page renders inside the global shell.
- Root, admin, admin-settings, and search pages remain shell-first inside that shell.
- Note and history shell rules are defined in [../experience/shell/README.md](../experience/shell/README.md).
- Public root and search rules are defined in [../experience/index/README.md](../experience/index/README.md).
- Editor rules are defined in [../experience/editor/README.md](../experience/editor/README.md).
- Shared title, description, canonical, and robots-meta rules are defined in [../discoverability/head-metadata.md](../discoverability/head-metadata.md).
