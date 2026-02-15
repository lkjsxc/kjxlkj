# kjxlkj

All-in-docs workspace-suite platform for collaborative notes, search, and
`kjxlkj-agent` automation.

## Current Contract (2026-02-15)

- Canonical behavior is defined in `/docs`.
- Runtime source code is intentionally removed (docs-only baseline).
- Rebuild execution order is defined in [`/docs/todo/README.md`](docs/todo/README.md).
- Non-secret runtime config is defined in `data/config.json`.
- Agent prompt is fully defined in `data/agent-prompt.json`.

## Reconstruction Start

1. Read [`docs/todo/README.md`](docs/todo/README.md).
2. Execute waves in [`docs/todo/waves/README.md`](docs/todo/waves/README.md).
3. Rebuild runtime artifacts from specs.
4. Keep reference ledgers synchronized during execution.

## Repository Layout

| Path | Purpose |
|---|---|
| `docs/` | canonical policy, spec, reference, and TODO contracts |
| `data/config.json` | non-secret runtime configuration |
| `data/agent-prompt.json` | `kjxlkj-agent` prompt definition |
| `.env.example` | local secret template |
| `src/` | regenerated runtime source tree after reconstruction |
