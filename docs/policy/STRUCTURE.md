# Documentation Structure Policy

Back: [/docs/policy/README.md](/docs/policy/README.md)

Mandatory structural constraints for documentation.

## Directory Constraints

| Constraint | Value | Rationale |
|---|---|---|
| Max items per directory | 12 | Enables fast scanning and deterministic navigation |
| README.md per directory | Required | Every directory has an index entry point |
| Max lines per file | 200 | Keeps documents focused |
| Soft max columns per line | 100 | Prevents horizontal scrolling |

## Directory Hierarchy Requirements

Every directory under `docs/` MUST contain exactly one `README.md`.

The README.md MUST:

- introduce directory scope
- link to all direct children
- provide navigation back to parent

## Navigation Requirements

| Requirement | Description |
|---|---|
| Reachability | Every document MUST be reachable from `docs/README.md` |
| Bidirectional links | Parent README links to children; children link to parent |
| No orphan documents | No unlinked document is allowed |
| Link validation | Internal links MUST be verified on change |

## Compliance Checklist

For any documentation change:

- [ ] no directory exceeds 12 direct children
- [ ] all directories contain exactly one `README.md`
- [ ] no file exceeds 200 lines
- [ ] all docs are reachable from `docs/README.md`
- [ ] no orphan docs exist
