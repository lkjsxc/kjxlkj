# kjxlkj

All in Docs knowledge-workspace platform.

## Current Contract (2026-02-13)

- Only documentation is canonical product value.
- Implementation artifacts are disposable derivatives of `/docs`.
- A repository state with no runtime source code is valid by design.
- Rebuild claims are valid only with deterministic evidence and synchronized ledgers.
- Frontend and backend implementation targets are statically typed only:
  - frontend: TypeScript (`strict`)
  - backend: Rust + PostgreSQL
  - direct JavaScript application source is forbidden

## Documentation Access

Read directly from `docs/` in the repository.

Docker launch artifacts (`Dockerfile`, `docker-compose.yml`, `.dockerignore`)
are required repository-root derivatives and must stay synchronized with
`docs/guides/DOCKER.md`.

## Canonical Reading Order

1. `docs/policy/README.md`
2. `docs/spec/README.md`
3. `docs/reference/README.md`
4. `docs/todo/README.md`
5. `docs/guides/README.md`

## Repository Layout

| Path | Purpose |
|---|---|
| `docs/` | canonical product definition |
| `README.md` | top-level entry point |
| `LICENSE` | license text |
