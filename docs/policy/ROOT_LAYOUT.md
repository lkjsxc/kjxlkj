# Root Layout Policy

Back: [/docs/policy/README.md](/docs/policy/README.md)

Allowed top-level layout and derived artifact placement.

## Canonical Root Allowlist

| Path | Purpose |
|---|---|
| `AGENTS.md` | local execution instructions for coding agents |
| `GEMINI.md` | alternate-agent instruction mirror |
| `README.md` | project index |
| `LICENSE` | license |
| `.gitignore` | repository hygiene |
| `.github/` | optional workflow metadata |
| `docs/` | canonical product definition |

Additional root entries require rationale in `/docs/log/proposals/`.

## Derived Runtime Artifacts

Derived runtime artifacts are optional and disposable. When reconstruction is active,
these entries MAY temporarily exist at root:

- `src/`
- `Cargo.toml`
- `Cargo.lock`
- `package.json`
- `package-lock.json`
- `tsconfig.json`
- `Dockerfile`
- `docker-compose.yml`
- `.dockerignore`

Their absence does not reduce product completeness in docs-only mode.

## Repository State Rule

A valid completion for canonical documentation work MAY contain only the allowlist
entries in `Canonical Root Allowlist`.

## Related

- Structure constraints: [/docs/policy/STRUCTURE.md](/docs/policy/STRUCTURE.md)
- Final completion structure: [/docs/spec/architecture/final-file-structure.md](/docs/spec/architecture/final-file-structure.md)
- All in Docs doctrine: [/docs/overview/all-in-docs.md](/docs/overview/all-in-docs.md)
