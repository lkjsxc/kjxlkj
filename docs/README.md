# Documentation

`/docs` is the canonical system definition for `kjxlkj`.

## Contract

- The product MUST be reconstructable from documentation.
- Canonical behavior and policy are defined before implementation.
- Completion claims MUST be backed by deterministic evidence.
- Reference and TODO ledgers MUST stay synchronized with implementation status.

## Authority and Precedence

Use this order for decisions:

1. [/docs/policy/](policy/README.md)
2. [/docs/spec/](spec/README.md)
3. [/docs/reference/](reference/README.md)
4. [/docs/todo/](todo/README.md)
5. [/docs/guides/](guides/README.md), [/docs/overview/](overview/README.md), [/docs/log/](log/README.md)

## Status Model

- `policy` and `spec` are normative.
- `reference` is normative for verified state.
- `todo` is normative for staged execution and closure gates.
- `guides` and `overview` are explanatory unless referenced by policy/spec.
- `log` is historical and non-authoritative.

## Canonical Reading Order

1. [policy/README.md](policy/README.md)
2. [spec/README.md](spec/README.md)
3. [reference/README.md](reference/README.md)
4. [todo/README.md](todo/README.md)
5. [guides/README.md](guides/README.md)
6. [overview/README.md](overview/README.md)
7. [log/README.md](log/README.md)

## Directory Map

| Directory | Role |
|---|---|
| [policy/](policy/README.md) | repository invariants and guardrails |
| [spec/](spec/README.md) | target workspace-suite behavior |
| [reference/](reference/README.md) | verified state and open gaps |
| [todo/](todo/README.md) | recursive migration execution contract |
| [guides/](guides/README.md) | operator workflows |
| [overview/](overview/README.md) | product concepts and vocabulary |
| [log/](log/README.md) | proposals and audits |

## Related

- All-in-docs statement: [overview/all-in-docs.md](overview/all-in-docs.md)
- Structure rules: [policy/STRUCTURE.md](policy/STRUCTURE.md)
- Root layout rules: [policy/ROOT_LAYOUT.md](policy/ROOT_LAYOUT.md)
