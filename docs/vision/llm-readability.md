# LLM Readability Contract

## Format Rules

1. Keep files under 300 lines.
2. Prefer stable headings such as `Goal`, `Rules`, `Links`, `Scope`.
3. Use one requirement per bullet.
4. Use `MUST`, `SHOULD`, and `MAY` consistently.

## Link Rules

- Every directory `README.md` acts as a TOC for its children.
- Links should be relative and explicit.
- Duplicate definitions should be replaced with links to canonical docs.

## Update Rules

- When moving docs, update parent TOCs in the same change.
- After edits, validate topology and line limits via CLI checks.
- Keep contracts explicit; avoid ambiguous narrative text.
