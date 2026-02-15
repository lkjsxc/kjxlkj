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

## Source Length Audit Rule

During runtime rebuild, if any source file exceeds 200 lines:

1. record file path and line count in reference docs
2. add refactor task to improvement backlog
3. add TODO item for module split

## Compliance Checklist

- [ ] docs directories satisfy max-item rule
- [ ] every docs directory has README
- [ ] no docs file exceeds 200 lines
- [ ] TODO checklists link to governing docs
- [ ] source >200 line exceptions are recorded when present
