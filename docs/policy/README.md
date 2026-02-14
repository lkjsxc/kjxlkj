# Policy

Back: [/docs/README.md](/docs/README.md)

Operating invariants for documentation and reconstruction.

## Core Invariants

1. Documentation under `/docs/` is authoritative.
2. The shipped runtime is a Rust Actix/Tokio server with PostgreSQL persistence.
3. API and WebSocket contracts are explicitly documented in canonical specs.
4. Event consistency and conflict behavior are deterministic.
5. Claims of completion require deterministic evidence.
6. Legacy terminal-editor behavior is out of scope.

## Normative Policy Set

| Document | Purpose |
|---|---|
| [INSTRUCT.md](INSTRUCT.md) | session-level operating contract |
| [WORKFLOW.md](WORKFLOW.md) | reconstruction workflow and completion gates |
| [STRUCTURE.md](STRUCTURE.md) | documentation and source topology limits |
| [ROOT_LAYOUT.md](ROOT_LAYOUT.md) | root-level repository layout rules |

## Repository States

| State | Description |
|---|---|
| Docs-only baseline | canonical docs exist; derived source artifacts may be absent |
| Reconstructed implementation | source, compose, and automation regenerated from docs |

## Related

- Contract overview: [/docs/README.md](/docs/README.md)
- Canonical spec: [/docs/spec/README.md](/docs/spec/README.md)
- Current-state ledgers: [/docs/reference/README.md](/docs/reference/README.md)
