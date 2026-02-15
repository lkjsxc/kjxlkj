# Source Layout Blueprint

Back: [/docs/spec/architecture/README.md](/docs/spec/architecture/README.md)

Blueprint for runtime topology after reconstruction.

## Canonical Tree

| Path | Purpose |
|---|---|
| `src/crates/search/kjxlkj-search/` | lexical + vector retrieval services |
| `src/crates/automation/kjxlkj-automation/` | `kjxlkj-agent` loop, parser, prompt loader |
| `src/frontend/app/` | Obsidian-like markdown UI |

## Layout Constraints

| Trigger | Required Action |
|---|---|
| directory has >12 children | split by subdomain |
| source file >200 lines | extract cohesive module |
| mixed IO + domain logic | split repository/service layers |

## Related

- Structure policy: [/docs/policy/STRUCTURE.md](/docs/policy/STRUCTURE.md)
- Crates: [crates.md](crates.md)
