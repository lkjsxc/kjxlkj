# kjxlkj

A documentation-first workspace-suite web platform for collaborative notes,
projects, automation, and knowledge workflows.

## Current Product Contract

- This repository is intentionally in docs-only reconstruction mode.
- Runtime and deployment artifacts are intentionally absent.
- Canonical behavior is defined in `/docs` and MUST be reconstructed from specs and TODO waves.
- Recent UX findings from implementation/user interaction are captured in canonical UI specs.
- Canonical specs include an autonomous Librarian AI agent for advanced
  documentation structuring via OpenRouter or LM Studio compatible models.
- Librarian model interaction uses a strict attribute-less XML-like protocol for
  compatibility with small-parameter LLMs.

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
