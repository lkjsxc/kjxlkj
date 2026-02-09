# Documentation

`/docs` is the canonical system definition for `kjxlkj`.

## Contract

- The product MUST be reconstructable from documentation alone.
- Source code and automation artifacts are derived outputs.
- When docs and implementation diverge, docs are corrected first, then drift is recorded in reference ledgers.

## Authority and Precedence

Use this precedence order for all decisions:

1. [/docs/policy/](policy/README.md)
2. [/docs/spec/](spec/README.md)
3. [/docs/reference/](reference/README.md)
4. [/docs/todo/](todo/README.md)
5. [/docs/technical/](technical/README.md), [/docs/design/](design/README.md), [/docs/guides/](guides/README.md), [/docs/overview/](overview/README.md), [/docs/log/](log/README.md)

## Status Model

- `policy` and `spec` are normative.
- `reference` is normative for current verified state only.
- `todo` is normative for reconstruction workflow and completion gates.
- `technical`, `design`, `guides`, and `overview` are explanatory unless explicitly referenced by `policy` or `spec` as normative.
- `log` is historical and non-authoritative.

## Canonical Reading Order

1. [policy/README.md](policy/README.md)
2. [spec/README.md](spec/README.md)
3. [reference/README.md](reference/README.md)
4. [todo/README.md](todo/README.md)
5. [technical/README.md](technical/README.md)
6. [log/README.md](log/README.md)

## Directory Map

| Directory | Role |
|---|---|
| [policy/](policy/README.md) | Repository rules and invariants |
| [spec/](spec/README.md) | Target behavior and architecture |
| [reference/](reference/README.md) | Verified current state and explicit gaps |
| [todo/](todo/README.md) | Reconstruction execution plan and gates |
| [technical/](technical/README.md) | Implementation guidance |
| [design/](design/README.md) | Rationale and decomposition notes |
| [guides/](guides/README.md) | Operator-facing workflows |
| [overview/](overview/README.md) | Concepts and terminology |
| [log/](log/README.md) | Historical audits and proposals |

## Related

- All-in-docs statement: [overview/all-in-docs.md](overview/all-in-docs.md)
- Structure rules: [policy/STRUCTURE.md](policy/STRUCTURE.md)
- Root layout rules: [policy/ROOT_LAYOUT.md](policy/ROOT_LAYOUT.md)
