# kjxlkj

Documentation-first workspace-suite platform for collaborative notes, automation,
and knowledge workflows.

## Current Contract (2026-02-13)

- Repository is intentionally in docs-only rebuild baseline.
- Runtime and deployment artifacts are intentionally absent.
- Canonical behavior is defined in `/docs` and MUST be rebuilt from specs/TODO.
- Implementation/user findings (`IMP-*`, `USR-*`) are canonically mapped in
  [`/docs/spec/ui/findings-traceability.md`](docs/spec/ui/findings-traceability.md).
- Deployment target is a single-container Docker Compose service defined by
  [`/docs/spec/architecture/deployment.md`](docs/spec/architecture/deployment.md).

## Rebuild Start

1. Read [`docs/todo/README.md`](docs/todo/README.md).
2. Execute waves in [`docs/todo/waves/README.md`](docs/todo/waves/README.md).
3. Rebuild runtime artifacts, then start with `docker compose up --build`.
4. Keep ledgers synchronized in [`docs/reference/`](docs/reference/README.md).

## Repository Layout

| Path | Purpose |
|---|---|
| `docs/` | canonical policy, spec, reference, and execution docs |
| `README.md` | top-level index |
| `LICENSE` | license text |
