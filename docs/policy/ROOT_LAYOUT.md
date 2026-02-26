# Root Layout Policy

Back: [/docs/policy/README.md](/docs/policy/README.md)

Allowed top-level layout and derived artifact placement.

## Root Allowlist

| Path | Purpose |
|---|---|
| `README.md` | project index |
| `LICENSE` | license |
| `.env.example` | secret template |
| `.gitignore` | repository hygiene |
| `data/` | non-secret runtime and prompt config |
| `docs/` | canonical documentation |
| `src/` | derived runtime workspace (optional until rebuilt) |
| `migrations/` | derived schema migrations (optional until rebuilt) |
| `Cargo.toml` | derived workspace manifest (optional until rebuilt) |
| `Cargo.lock` | derived lockfile (optional until rebuilt) |
| `scripts/` | derived operational scripts (optional until rebuilt) |
| `Dockerfile` | derived container helper (optional until rebuilt) |
| `docker-compose.yml` | derived local orchestration helper (optional until rebuilt) |
| `.dockerignore` | optional Docker build-context hygiene |
| `.github/` | CI and automation metadata |

## Docs-Only Baseline

Docs-only baseline MAY include only:

- `docs/`
- `data/config.json`
- `data/agent-prompt.json`
- root hygiene files (`README.md`, `LICENSE`, `.env.example`, `.gitignore`)

## Prohibitions

- `tmp/` MUST NOT exist in canonical state.
- `log/` and `docs/logs/` MUST NOT exist.
- committed secrets are forbidden.

## Reconstruction Rule

- Any path listed as derived MAY be deleted during docs-only reset.
- Re-created runtime paths MUST be produced from `/docs/spec` plus `/docs/todo` only.

## Related

- Structure policy: [/docs/policy/STRUCTURE.md](/docs/policy/STRUCTURE.md)
- Final structure: [/docs/spec/architecture/final-file-structure.md](/docs/spec/architecture/final-file-structure.md)
