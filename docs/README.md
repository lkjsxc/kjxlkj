# Documentation

`/docs` is the canonical system definition for `kjxlkj`.

## Contract

- The product MUST be reconstructable from documentation.
- Specs define target behavior; references define verified present state.
- TODO checklists define executable rebuild order.

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
| [reference/](reference/README.md) | current-state evidence and drift |
| [todo/](todo/README.md) | staged rebuild execution plan |
| [guides/](guides/README.md) | operator workflows |
| [overview/](overview/README.md) | concepts and vocabulary |

## Notes

Historical execution logs are kept in git history and reference ledgers.

## Related

- All-in-docs doctrine: [overview/all-in-docs.md](overview/all-in-docs.md)
- Structure policy: [policy/STRUCTURE.md](policy/STRUCTURE.md)
