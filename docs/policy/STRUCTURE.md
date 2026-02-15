# Documentation Structure Policy

Back: [/docs/policy/README.md](/docs/policy/README.md)

Mandatory structural constraints for documentation.

## Directory Constraints

| Constraint | Value | Rationale |
|---|---|---|
| Max items per directory | 12 | enables fast scanning and deterministic navigation |
| README.md per directory | Required | every directory has an index entry point |
| Max lines per file | 200 | keeps documents focused |
| Soft max columns per line | 100 | prevents horizontal scrolling |

## Directory Hierarchy Requirements

Every directory under `docs/` MUST contain exactly one `README.md`.

The README.md MUST:

- introduce directory scope
- link to all direct children
- provide navigation back to parent

## Navigation Requirements

| Requirement | Description |
|---|---|
| Reachability | every document MUST be reachable from `docs/README.md` |
| Bidirectional links | parent README links to children; children link to parent |
| No orphan documents | no unlinked document is allowed |
| Link validation | internal links MUST be verified on change |

## TODO Link Policy

All TODO markdown files under `docs/todo/` MUST include a section titled
`## Relevant Documents` with direct Markdown links to all documents required to
complete that TODO.

Every checklist item (`- [ ]` / `- [x]`) in `docs/todo/` MUST include at least
one direct Markdown link to its governing documentation target.

Required minimum links in each TODO file:

- canonical spec index
- testing contract
- conformance ledger
- limitations ledger
- TODO index and wave program

## Compliance Checklist

For any documentation change:

- [ ] no directory exceeds 12 direct children
- [ ] all directories contain exactly one `README.md`
- [ ] no file exceeds 200 lines
- [ ] all docs are reachable from `docs/README.md`
- [ ] no orphan docs exist
- [ ] every TODO file includes `## Relevant Documents` with Markdown links
- [ ] every TODO checklist item links directly to governing docs
