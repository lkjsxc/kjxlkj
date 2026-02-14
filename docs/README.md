# Documentation

`/docs` is the product for `kjxlkj`.

## Contract

- All in Docs is mandatory: docs are the value, runtime artifacts are disposable derivatives.
- A docs-only repository is a fully valid canonical state.
- Spec defines target behavior for future reconstruction.
- Reference defines truthful current verification state.
- TODO defines ordered execution for rebuilding derivatives.
- Completion claims require deterministic evidence.

## Authority and Precedence

1. [/docs/policy/](policy/README.md)
2. [/docs/spec/](spec/README.md)
3. [/docs/reference/](reference/README.md)
4. [/docs/todo/](todo/README.md)
5. [/docs/guides/](guides/README.md), [/docs/overview/](overview/README.md), [/docs/log/](log/README.md)

## Status Model

- `policy` and `spec` are normative requirements.
- `reference` is normative for verified current state.
- `todo` is normative for execution order and closure.
- `guides` and `overview` are explanatory unless explicitly referenced by policy/spec.
- `log` is historical evidence.

## Directory Map

| Directory | Role |
|---|---|
| [policy/](policy/README.md) | invariants and guardrails |
| [spec/](spec/README.md) | target behavior and implementation contracts |
| [reference/](reference/README.md) | verified state and open gaps |
| [todo/](todo/README.md) | staged execution contract |
| [guides/](guides/README.md) | operational walkthroughs |
| [overview/](overview/README.md) | concepts and terminology |
| [log/](log/README.md) | audit/proposal history |

## Related

- All in Docs statement: [overview/all-in-docs.md](overview/all-in-docs.md)
- Type safety contract: [spec/technical/type-safety.md](spec/technical/type-safety.md)
- Structure rules: [policy/STRUCTURE.md](policy/STRUCTURE.md)
