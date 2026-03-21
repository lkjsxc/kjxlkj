# Docs Authoring Contract

## Authoring Rules

- Keep docs declarative and contract-oriented.
- Keep headings stable for retrieval.
- Keep each file below 300 lines.
- Use deterministic checklists for workflows that gate destructive changes.

## Topology Rules

- Each docs directory must have one `README.md` TOC.
- Each docs directory should include multiple child entries.
- Parent TOCs must be updated whenever child docs change.

## Validation Rules

- Run `kjxlkj docs validate-topology` after topology edits.
- Run `kjxlkj quality check-lines` after content edits.

## Executable-Intent Preservation Rules

- Documentation MUST preserve executable intent even when runtime files are removed.
- Each intent statement SHOULD answer:
  - what behavior is required
  - where canonical definition lives
  - how an agent can verify compliance
- Implementation-specific details MAY exist only as non-authoritative examples; normative requirements MUST live in contracts.
- Before deleting implementation artifacts, ensure equivalent intent exists in:
  - `docs/product/` for user-visible behavior
  - `docs/architecture/` for system/module/data boundaries
  - `docs/operations/` for command and validation expectations
  - `docs/repository/` for layout and governance rules
