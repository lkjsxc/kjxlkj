# HTML Pages Contract

## Canonical Pages

- `/`: auth-aware homepage.
- `/admin`: hybrid admin dashboard plus admin library.
- `/search`: auth-aware search page.
- `/{ref}`: current note page.
- `/{ref}/history`: note history index.
- `/{ref}/history/{revision_number}`: historical snapshot page.

## Shared Rules

- Every page renders inside the global shell.
- Root, admin, and search pages remain shell-first inside that shell.
- Note and history shell rules are defined in [../experience/shell/README.md](../experience/shell/README.md).
- Public root and search rules are defined in [../experience/index/README.md](../experience/index/README.md).
- Editor rules are defined in [../experience/editor/README.md](../experience/editor/README.md).
