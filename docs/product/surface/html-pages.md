# HTML Pages Contract

## Canonical Pages

- `/`: public browse page.
- `/admin`: admin browse page.
- `/search`: auth-aware search page.
- `/{id}`: current note page.
- `/{id}/history`: note history index.
- `/{id}/history/{revision_number}`: historical snapshot page.

## Shared Rules

- Every page renders inside the global shell.
- Root, admin, and search pages remain list-first inside that shell.
- Note and history shell rules are defined in [../experience/shell/README.md](../experience/shell/README.md).
- Public root and search rules are defined in [../experience/index/README.md](../experience/index/README.md).
- Editor rules are defined in [../experience/editor/README.md](../experience/editor/README.md).
