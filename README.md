# kjxlkj

Documentation-first workspace-suite platform for collaborative notes, automation,
and knowledge workflows.

## Current Contract (2026-02-13)

- Repository is intentionally in a docs-only reconstruction baseline.
- Runtime and deployment implementation artifacts are intentionally absent.
- Canonical behavior is defined in `/docs` and MUST be rebuilt from specs/TODO.
- Implementation/user findings (`IMP-*`, `USR-*`) are canonical and mapped in
  [`/docs/spec/ui/findings-traceability.md`](docs/spec/ui/findings-traceability.md).
- UX reconstruction requirements are normative in
  [`/docs/spec/ui/reconstruction-ux-requirements.md`](docs/spec/ui/reconstruction-ux-requirements.md).

## Launch Documentation (Single Container)

```bash
docker compose up --build
```

Then open `http://127.0.0.1:8080`.

## Rebuild Start

1. Read [`docs/todo/README.md`](docs/todo/README.md).
2. Execute waves in [`docs/todo/waves/README.md`](docs/todo/waves/README.md).
3. Rebuild runtime artifacts from canonical docs.
4. Keep ledgers synchronized in [`docs/reference/`](docs/reference/README.md).

## Repository Layout

| Path | Purpose |
|---|---|
| `docs/` | canonical policy, spec, reference, and execution docs |
| `Dockerfile` | docs container image |
| `docker-compose.yml` | one-container docs launch |
| `README.md` | top-level index |
| `LICENSE` | license text |
