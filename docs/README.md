# Documentation

`/docs` is the canonical system definition for `kjxlkj`.

## Contract

- The full product MUST be reconstructable from documentation alone.
- Implementation files under `src/` are derived artifacts.
- If implementation and docs diverge, documentation MUST be corrected first, then tracked in
  `/docs/reference/CONFORMANCE.md` and `/docs/reference/LIMITATIONS.md`.
- Documentation is written for AI-first reconstruction and verification.

## Canonical Reading Order

1. [policy/README.md](policy/README.md)
2. [spec/README.md](spec/README.md)
3. [reference/README.md](reference/README.md)
4. [todo/README.md](todo/README.md)
5. [log/README.md](log/README.md)

## Directory Map

| Directory | Role |
|---|---|
| [policy/](policy/README.md) | Normative rules for repository operation and document quality |
| [spec/](spec/README.md) | Target product specification (what must exist) |
| [reference/](reference/README.md) | Verified current implementation surface and known gaps |
| [todo/](todo/README.md) | Reconstruction control plane with checklists |
| [technical/](technical/README.md) | Engineering guidance and implementation notes |
| [design/](design/README.md) | Rationale and decomposition guidance |
| [guides/](guides/README.md) | Usage guides and operator workflows |
| [overview/](overview/README.md) | Concepts and glossary |
| [log/](log/README.md) | Non-canonical wave logs, audits, and proposals |

## Reconstruction Discipline

- Use `/docs/spec/` for target behavior.
- Use `/docs/reference/` for what is currently proven in-repo.
- Use `/docs/todo/current/` to plan and gate each reconstruction wave.
- Do not mark TODO items complete without deterministic evidence.

## Related

- All-in-docs statement: [overview/all-in-docs.md](overview/all-in-docs.md)
- Structure policy: [policy/STRUCTURE.md](policy/STRUCTURE.md)
- Root layout policy: [policy/ROOT_LAYOUT.md](policy/ROOT_LAYOUT.md)
