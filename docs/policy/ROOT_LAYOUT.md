# Root Layout Policy

Back: [/docs/policy/README.md](/docs/policy/README.md)

Allowed top-level layout and derived artifact placement.

## Root Allowlist

| Path | Purpose |
|---|---|
| `README.md` | project index |
| `LICENSE` | license |
| `.env.example` | secret template |
| `data/` | non-secret runtime and prompt config |
| `docs/` | canonical documentation |
| `src/` | derived runtime workspace |
| `Cargo.toml` | derived workspace manifest |
| `Cargo.lock` | derived lockfile |
| `Dockerfile` | derived container build |
| `docker-compose.yml` | derived orchestration |
| `.gitignore` | repository hygiene |

## Docs-Only Baseline

Docs-only baseline MAY temporarily include only:

- `docs/`
- `data/config.json`
- `data/agent-prompt.json`
- root hygiene files (`README.md`, `LICENSE`, `.env.example`, `.gitignore`)

## Forbidden Top-Level Paths

- `tmp/`
- `log/`
- committed secrets

## Related

- Structure policy: [/docs/policy/STRUCTURE.md](/docs/policy/STRUCTURE.md)
- Final structure: [/docs/spec/architecture/final-file-structure.md](/docs/spec/architecture/final-file-structure.md)
