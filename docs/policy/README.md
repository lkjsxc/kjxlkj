# Policy

Back: [/docs/README.md](/docs/README.md)

Operating invariants for documentation and reconstruction.

## Core Invariants

1. `/docs/` is authoritative over all other artifacts.
2. All in Docs governance is permanent.
3. Implementation artifacts are disposable derivatives.
4. Runtime target is Rust backend + PostgreSQL + TypeScript frontend.
5. Direct JavaScript runtime source is forbidden.
6. Completion claims require deterministic evidence and synchronized ledgers.

## Normative Policy Set

| Document | Purpose |
|---|---|
| [INSTRUCT.md](INSTRUCT.md) | session-level operating contract |
| [WORKFLOW.md](WORKFLOW.md) | execution gates and drift handling |
| [STRUCTURE.md](STRUCTURE.md) | topology and document hygiene constraints |
| [ROOT_LAYOUT.md](ROOT_LAYOUT.md) | root-level repository layout policy |

## Repository States

| State | Description |
|---|---|
| All in Docs baseline | docs are canonical; derived artifacts may be absent |
| Derived runtime snapshot | typed implementation artifacts exist as generated projections |

## Related

- Contract overview: [/docs/README.md](/docs/README.md)
- Canonical spec: [/docs/spec/README.md](/docs/spec/README.md)
- Current-state ledgers: [/docs/reference/README.md](/docs/reference/README.md)
