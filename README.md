# kjxlkj

A documentation-first workspace-suite web platform for collaborative notes,
projects, automation, and knowledge workflows.

## Current Product Contract

- This repository now includes reconstructed runtime and frontend artifacts.
- Canonical behavior remains defined in `/docs`.
- Remaining release-gate closure depends on full acceptance/performance/ops evidence.

## Documentation

All canonical system definitions live in [`docs/`](docs/README.md).

Implementation artifacts are derived from canonical documentation.
When documentation and implementation diverge, documentation is updated first
and drift is recorded in reference ledgers.

## Reconstruction Start

1. Read [`docs/todo/README.md`](docs/todo/README.md).
2. Execute waves in [`docs/todo/waves/README.md`](docs/todo/waves/README.md).
3. Keep ledgers synchronized in [`docs/reference/`](docs/reference/README.md).

## Project Structure

| Path | Purpose |
|---|---|
| `docs/` | Canonical policy, spec, reference, and execution docs |
| `README.md` | Top-level project entry |
