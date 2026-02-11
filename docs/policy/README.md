# Policy

Back: [/docs/README.md](/docs/README.md)

Operating invariants for documentation and reconstruction.

## Core Invariants

1. Documentation under `/docs/` is authoritative.
2. The shipped application is a single native Rust binary.
3. All user-critical workflows are keyboard-driven.
4. External plugin loading is out of scope.
5. IO and long-running work use supervised async services.
6. Claims of completion require deterministic evidence.

## Normative Policy Set

| Document | Purpose |
|---|---|
| [INSTRUCT.md](INSTRUCT.md) | Session-level operating contract |
| [WORKFLOW.md](WORKFLOW.md) | Reconstruction workflow and completion gates |
| [STRUCTURE.md](STRUCTURE.md) | Documentation and source topology limits |
| [ROOT_LAYOUT.md](ROOT_LAYOUT.md) | Root-level repository layout rules |

## Documentation Content Rule

Documentation under `/docs/` SHOULD prioritize structured prose and tables.

Fenced blocks are allowed when they improve precision:

- `mermaid` diagrams for system shape and flow
- shell command examples for reproducible execution
- short structured snippets where plain prose is ambiguous

Large implementation code dumps are not allowed in canonical docs.

Allowed forms:

- normative prose (`MUST`, `SHOULD`, `MAY`)
- tables
- inline code spans
- links to canonical documents
- mermaid diagrams
- concise fenced examples when needed for deterministic interpretation

## Repository States

| State | Description |
|---|---|
| Docs-only baseline | Canonical docs exist; derived artifacts may be absent |
| Reconstructed implementation | Source and automation artifacts are regenerated from docs |

## Related

- Contract overview: [/docs/README.md](/docs/README.md)
- Canonical spec: [/docs/spec/README.md](/docs/spec/README.md)
- Current-state ledgers: [/docs/reference/README.md](/docs/reference/README.md)
