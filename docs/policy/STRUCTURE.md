# Documentation Structure Policy

Back: [/docs/policy/README.md](/docs/policy/README.md)

Mandatory structural constraints for docs and derived runtime topology.

## Documentation Constraints

| Constraint | Value | Rationale |
|---|---|---|
| Max items per directory | 12 (archive exception) | predictable navigation |
| README per directory | Required | deterministic entry point |
| Max lines per file | 200 | focused docs |
| Soft max columns per line | 100 | readable diffs |

Archive exception:

- `docs/log/audits/` and `docs/log/proposals/` MAY exceed 12 children.
- archive directories MUST keep date-prefixed filenames and indexed README tables.

## Navigation Requirements

| Requirement | Description |
|---|---|
| Reachability | every doc is reachable from `docs/README.md` |
| Bidirectional links | parent README links to children; children link to parent |
| No orphan docs | unlinked docs are not allowed |
| Link validation | internal links are validated on change |

## TODO Link Policy

All TODO files under `docs/todo/` MUST include `## Relevant Documents` with direct links to:

- spec index
- testing contract
- conformance ledger
- limitations ledger
- TODO index and wave program

## Typed Runtime Structure Rule

When runtime artifacts exist:

- frontend runtime source MUST be `.ts` or `.tsx`
- backend runtime source MUST be `.rs`
- handwritten `.js` runtime source files are forbidden
- generated `src/frontend/app/dist/*.js` files are allowed

## Compliance Checklist

- [ ] no non-archive directory exceeds 12 direct children
- [ ] every directory includes one `README.md`
- [ ] no file exceeds 200 lines
- [ ] no orphan docs exist
- [ ] every TODO file includes `## Relevant Documents`
- [ ] typed runtime structure rule is satisfied
