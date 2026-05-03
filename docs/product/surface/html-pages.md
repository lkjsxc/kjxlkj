# HTML Pages Contract

## Canonical Pages

- `/`: global public feed.
- `/login`: user login page.
- `/setup`: first-user setup page.
- `/reset-password`: password reset page.
- `/{user}`: personal-space public feed.
- `/{user}/admin`: personal-space dashboard.
- `/{user}/settings`: personal-space settings page.
- `/{user}/search`: personal-space browse and search page.
- `/{user}/live`: public live broadcast page for one personal space.
- `/{user}/{ref}`: live note page, live media page, or saved-snapshot page.
- `/{user}/{ref}/history`: member-only history index for the live resource at `/{user}/{ref}`.

## Shared Rules

- Every page renders inside the shared shell.
- Root, dashboard, settings, and search pages remain shell-first inside that shell.
- Resource page shell rules are owned by [../experience/shell/README.md](../experience/shell/README.md).
- Editor rules are owned by [../experience/editor/README.md](../experience/editor/README.md).
- Shared title, description, canonical, and robots-meta rules are owned by [../discoverability/head-metadata.md](../discoverability/head-metadata.md).
