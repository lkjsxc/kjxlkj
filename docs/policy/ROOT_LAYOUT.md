# Root Layout Policy

Back: [/docs/policy/README.md](/docs/policy/README.md)

Allowed top-level layout and derived artifact placement.

## Root Allowlist

| Path | Purpose |
|---|---|
| `README.md` | project index |
| `LICENSE` | license |
| `docs/` | canonical product definition |
| `.gitignore` | repository hygiene |
| `.github/` | optional workflow/instructions metadata |

Additional root entries require rationale in `/docs/log/proposals/`.

## Derived Runtime Artifacts

Derived runtime artifacts are optional and disposable.

Allowed derived runtime roots when reconstruction is active:

- `src/`
- `Cargo.toml`
- `Cargo.lock`
- frontend package manifests (`package.json`, `pnpm-lock.yaml`, `tsconfig.json`)
- Docker artifacts (`Dockerfile`, `docker-compose.yml`, `.dockerignore`)

Their absence does not reduce canonical product value.

## Repository State Rule

A valid baseline MAY contain only documentation and repository hygiene files.

## Related

- Structure constraints: [/docs/policy/STRUCTURE.md](/docs/policy/STRUCTURE.md)
- All in Docs doctrine: [/docs/overview/all-in-docs.md](/docs/overview/all-in-docs.md)
