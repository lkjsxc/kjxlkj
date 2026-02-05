# Verification: Documentation Policy (Iteration 36)

Back: [/docs/todo/current/wave-verification/README.md](/docs/todo/current/wave-verification/README.md)

## Purpose

Make documentation compliance provable and prevent policy drift.

This checklist is normative for any change that touches `/docs/`.

## Checklist (normative)

### A. Fenced blocks (Mermaid-only)

- [ ] Verify there are no fenced blocks under `/docs/` except Mermaid fences.
- [ ] If a non-Mermaid fence exists, replace it with:
  - prose using MUST/SHOULD language, and/or
  - a table, and/or
  - a Mermaid diagram.

### B. Navigation and reachability

- [ ] Ensure every directory under `/docs/` has exactly one `README.md`.
- [ ] Ensure every document under `/docs/` is reachable from [/docs/README.md](/docs/README.md) by navigation links.
- [ ] Ensure no directory under `/docs/` exceeds the max child limit from policy.

### C. Link hygiene

- [ ] Ensure no internal doc link uses `../`.
- [ ] Ensure all internal links resolve.
- [ ] Define (or update) a deterministic link validation workflow and integrate it into the verification gate.

### D. File size compliance

- [ ] Ensure documentation files under `/docs/` do not exceed the policy line limit, or record an explicit exception in policy.
- [ ] Ensure source files do not exceed the policy line limit, or record refactor tasks and limitations.

## Related

- Policy: [/docs/policy/README.md](/docs/policy/README.md)
- Structure rules: [/docs/policy/STRUCTURE.md](/docs/policy/STRUCTURE.md)
- Verification gate: [/docs/reference/CI.md](/docs/reference/CI.md)
