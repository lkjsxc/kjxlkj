# Policy

Back: [/docs/README.md](/docs/README.md)

Operating invariants for documentation and reconstruction.

## Core Invariants

1. `/docs/` is authoritative over all other artifacts.
2. All in Docs governance is permanent.
3. Runtime artifacts are disposable derivatives with no canonical value.
4. Docs-only baseline is always valid, even with zero runtime source files.
5. Runtime reconstruction target is Rust backend + TypeScript frontend.
6. Handwritten JavaScript runtime source is forbidden.
7. Completion claims require deterministic evidence and synchronized ledgers.

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
| Canonical docs-only state | only docs and hygiene files exist |
| Reconstruction state | typed runtime artifacts are generated from docs |

## Related

- Contract overview: [/docs/README.md](/docs/README.md)
- Canonical spec: [/docs/spec/README.md](/docs/spec/README.md)
- Current-state ledgers: [/docs/reference/README.md](/docs/reference/README.md)
