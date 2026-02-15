# kjxlkj

All-in-Docs workspace-suite platform for collaborative notes, automation,
and knowledge workflows.

## Current Contract (2026-02-15)

- Canonical behavior is defined in `/docs`.
- Runtime source code has been intentionally removed for full rebuild.
- Implementation/user findings (`IMP-*`, `USR-*`) are canonically mapped in
  [`/docs/spec/ui/findings-traceability.md`](docs/spec/ui/findings-traceability.md).
- All non-secret runtime configuration is defined in `data/config.json`.
- Secrets are provided via `.env` (see `.env.example`).
- Improvement backlog is canonicalized in
  [`/docs/reference/IMPROVEMENT_BACKLOG.md`](docs/reference/IMPROVEMENT_BACKLOG.md).

## Reconstruction Start

1. Read [`docs/todo/README.md`](docs/todo/README.md).
2. Execute waves in [`docs/todo/waves/README.md`](docs/todo/waves/README.md).
3. Configure `.env` from `.env.example`.
4. Rebuild runtime artifacts from docs contracts.
5. Start with `docker compose up --build`.
6. Keep ledgers synchronized in [`docs/reference/`](docs/reference/README.md).

## Repository Layout

| Path | Purpose |
|---|---|
| `docs/` | canonical policy, spec, reference, and execution docs |
| `data/config.json` | non-secret runtime configuration |
| `.env.example` | local secret template |
| `src/` | generated runtime source tree after reconstruction |
| `README.md` | top-level index |
| `LICENSE` | license text |
