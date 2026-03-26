# HTML Pages Contract

## Canonical Pages

- `/`: public searchable note index.
- `/admin`: admin searchable note index.
- `/{id}`: current note page.
- `/{id}/history`: note history index.
- `/{id}/history/{revision_number}`: historical snapshot page.

## Shared Rules

- Every page renders inside the global shell.
- Root and admin pages remain list-first inside that shell.
- Note and history shell rules are defined in [../experience/shell/README.md](../experience/shell/README.md).
- Public root list rules are defined in [../experience/index/README.md](../experience/index/README.md).
- Rich editor rules are defined in [../experience/editor/README.md](../experience/editor/README.md).
