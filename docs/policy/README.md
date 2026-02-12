# Policy

Back: [/docs/README.md](/docs/README.md)

Operating invariants for documentation and reconstruction.

## Core Invariants

1. Documentation under `/docs/` is authoritative.
2. The shipped runtime is a Rust Actix/Tokio server with PostgreSQL persistence.
3. API contracts are versioned and explicitly documented.
4. Event consistency and conflict behavior are deterministic.
5. Claims of completion require deterministic evidence.
6. Legacy terminal-editor behavior is out of scope.

## Normative Policy Set

| Document | Purpose |
|---|---|
| [INSTRUCT.md](INSTRUCT.md) | Session-level operating contract |
| [WORKFLOW.md](WORKFLOW.md) | Reconstruction workflow and completion gates |
| [STRUCTURE.md](STRUCTURE.md) | Documentation and source topology limits |
| [ROOT_LAYOUT.md](ROOT_LAYOUT.md) | Root-level repository layout rules |

## Repository States

| State | Description |
|---|---|
| Docs-only baseline | Canonical docs exist; derived source artifacts may be absent |
| Reconstructed implementation | Source, compose, and automation regenerated from docs |

## Related

- Contract overview: [/docs/README.md](/docs/README.md)
- Canonical spec: [/docs/spec/README.md](/docs/spec/README.md)
- Current-state ledgers: [/docs/reference/README.md](/docs/reference/README.md)
