# Documentation Structure Policy

Back: [/docs/policy/README.md](/docs/policy/README.md)

Mandatory structural constraints for documentation and rebuild artifacts.

## Directory Constraints

| Constraint | Value |
|---|---|
| Max items per directory | 12 |
| README.md per docs directory | required |
| Max lines per docs file | 200 |
| Source file line target | 200 |

## Navigation Requirements

- Every doc file MUST be reachable from `docs/README.md`.
- Parent READMEs MUST link to direct children.
- TODO checklists MUST include direct links to governing docs.

## TODO Link Policy

All TODO files under `docs/todo/` MUST:

1. include `## Relevant Documents`
2. link directly to required docs
3. include checklist items with at least one direct doc link

## Evidence and Traceability Policy

Every implementation checkbox under `docs/todo/` and `docs/todo/waves/` MUST satisfy all rules below before being marked complete:

1. bind to at least one acceptance ID or required suite category from [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)
2. map to a row in [/docs/reference/TODO_TRACE_MATRIX.md](/docs/reference/TODO_TRACE_MATRIX.md)
3. map to a row in [/docs/reference/TEST_MATRIX.md](/docs/reference/TEST_MATRIX.md) when acceptance IDs are used
4. include evidence pointer(s) in [/docs/reference/EVIDENCE_INDEX.md](/docs/reference/EVIDENCE_INDEX.md)
5. update corresponding ledger status in the same change

Wave-level build/test commands are necessary but not sufficient; checkbox closure requires explicit acceptance/suite evidence.

## Source Length Audit Rule

During runtime rebuild, if any source file exceeds 200 lines:

1. record file path and line count in reference docs
2. add refactor task to improvement backlog
3. add TODO item for module split

## Compliance Checklist

- [x] docs directories satisfy max-item rule
- [x] every docs directory has README
- [x] no docs file exceeds 200 lines
- [x] TODO checklists link to governing docs
- [x] source >200 line exceptions are recorded when present
- [x] TODO checkboxes require acceptance/suite binding and evidence linkage
