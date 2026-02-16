# Documentation

`/docs` is the canonical system definition for `kjxlkj`.

## Contract

- The system MUST be reconstructable from documentation alone.
- Specs define target behavior; references define verified present state.
- TODO waves define the only allowed implementation order.

## Authority and Precedence

1. [/docs/policy/](policy/README.md)
2. [/docs/spec/](spec/README.md)
3. [/docs/reference/](reference/README.md)
4. [/docs/todo/](todo/README.md)
5. [/docs/guides/](guides/README.md), [/docs/overview/](overview/README.md)

## Directory Map

| Directory | Role |
|---|---|
| [policy/](policy/README.md) | invariants and guardrails |
| [spec/](spec/README.md) | target product behavior |
| [reference/](reference/README.md) | verified current state and drift |
| [todo/](todo/README.md) | staged reconstruction workflow |
| [guides/](guides/README.md) | operator playbooks |
| [overview/](overview/README.md) | concepts and glossary |

## Baseline (2026-02-16)

- Repository supports docs-only baseline and reconstructed-runtime states.
- Runtime source and Docker tooling may be present after reconstruction.
- All behavior claims must remain anchored to policy/spec/reference/todo precedence.

## Related

- All-in-docs doctrine: [overview/all-in-docs.md](overview/all-in-docs.md)
- Structure policy: [policy/STRUCTURE.md](policy/STRUCTURE.md)
