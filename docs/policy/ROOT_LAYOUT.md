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
| `src/` | derived runtime workspace (optional until rebuilt) |
| `Cargo.toml` | derived workspace manifest (optional until rebuilt) |
| `Cargo.lock` | derived lockfile (optional until rebuilt) |
| `scripts/` | derived operational scripts (optional until rebuilt) |
| `.gitignore` | repository hygiene |
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
- Docker artifacts (`Dockerfile`, `docker-compose.yml`, `.dockerignore`) MUST NOT exist.
- committed secrets are forbidden.

## Related

- Structure policy: [/docs/policy/STRUCTURE.md](/docs/policy/STRUCTURE.md)
- Final structure: [/docs/spec/architecture/final-file-structure.md](/docs/spec/architecture/final-file-structure.md)
