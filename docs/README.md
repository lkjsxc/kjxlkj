# Documentation

`/docs` is the product for `kjxlkj`.

## Contract

- All in Docs is mandatory: docs are the value, code is derivative.
- Policy and spec define requirements before any implementation.
- Reference and TODO ledgers define truth of execution state.
- Completion claims require deterministic evidence.
- Typed implementation contract is normative:
  - frontend MUST be TypeScript only
  - backend MUST be Rust only
  - direct JavaScript app source MUST NOT be committed

## All in Docs vs Docs-Only

- `All in Docs`: governance model. Docs remain canonical even when code exists.
- `docs-only`: one possible repository state where derivatives are absent.

`All in Docs` is permanent policy. `docs-only` is optional state.

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
| [log/](log/README.md) | audit/proposal/improvement history |

## Related

- All in Docs statement: [overview/all-in-docs.md](overview/all-in-docs.md)
- Type safety contract: [spec/technical/type-safety.md](spec/technical/type-safety.md)
- Structure rules: [policy/STRUCTURE.md](policy/STRUCTURE.md)
