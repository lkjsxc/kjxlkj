# Root Layout Policy

Back: [/docs/policy/README.md](/docs/policy/README.md)

Allowed top-level layout and derived artifact placement.

## Root Allowlist

The repository root SHOULD contain only:

| Path | Purpose |
|---|---|
| `README.md` | project index |
| `LICENSE` | license |
| `.env.example` | local secret template (no real secrets) |
| `data/` | non-secret runtime configuration |
| `docs/` | canonical documentation |
| `src/` | Rust/TypeScript runtime workspace (derived) |
| `Cargo.toml` | workspace manifest (derived) |
| `Cargo.lock` | dependency lockfile (derived) |
| `package.json` | frontend package manifest (derived) |
| `package-lock.json` | frontend lockfile (derived) |
| `tsconfig.json` | frontend type-check policy (derived) |
| `.gitignore` | ignore rules |
| `docker-compose.yml` | single-service deployment file (derived) |
| `Dockerfile` | container image definition (derived) |
| `.dockerignore` | docker context filter (derived) |

Additional root entries require explicit rationale in:

- [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
- [/docs/todo/README.md](/docs/todo/README.md)

## Docs-Only Baseline

A docs-only baseline MAY temporarily contain only:

- `docs/`
- `README.md`
- `LICENSE`
- minimal repository hygiene files

Derived artifacts (`src/`, compose files, and manifests) are regenerated during
ordered TODO waves.

## Deployment Layout Rule

Deployment MUST use a single compose service container that runs:

- PostgreSQL process
- `kjxlkj` application server process

This non-standard shape is intentional and mandatory for this product.

## Related

- Structure constraints: [/docs/policy/STRUCTURE.md](/docs/policy/STRUCTURE.md)
- Workspace members: [/docs/spec/architecture/workspace-manifest.md](/docs/spec/architecture/workspace-manifest.md)
