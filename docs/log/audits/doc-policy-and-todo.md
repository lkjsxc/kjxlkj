# Audit: Docs Policy and TODO System Integrity

Back: [/docs/log/audits/README.md](/docs/log/audits/README.md)

## Scope

This audit validates and/or repairs:

- documentation policy compliance under `/docs/`
- correctness of the “Mermaid-only fenced blocks” rule
- TODO system integrity (checkmark-verifiable tasks; no pre-checked “future” work)
- derived-artifact clarity (CI/Docker/toolchain may be absent in docs-only baselines)

## Checks performed (policy-driven)

| Check | Policy reference | Result |
|---|---|---|
| No non-Mermaid fenced blocks under `/docs/` | [/docs/policy/README.md](/docs/policy/README.md) | Pass (one non-compliant fence removed from a conformance doc). |
| Docs ≤200 lines per file | [/docs/policy/STRUCTURE.md](/docs/policy/STRUCTURE.md) | Pass |
| Direct children ≤12 per directory | [/docs/policy/STRUCTURE.md](/docs/policy/STRUCTURE.md) | Pass |
| Exactly one `README.md` per directory | [/docs/policy/STRUCTURE.md](/docs/policy/STRUCTURE.md) | Pass |
| No `../` links in docs | Project constraint + TODO policy | Pass |

## Repairs made

### A. Documentation fence compliance

- Removed a non-Mermaid fenced block in a docs/reference conformance file and converted it to schema tables and prose.

### B. Derived automation artifacts clarity

- Updated guides/reference/policy docs to treat CI/Docker/toolchain as derived artifacts that may be absent, while preserving canonical expected locations for reconstruction.

### C. TODO system integrity

- Began Iteration 34 TODO reset: converted “Tasks” sections to checkboxes and introduced explicit unchecked leaves for known high-priority gaps (long lines, interactive newline, perf harness, file explorer MVP).

## Follow-ups (recorded as TODO leaves)

- Execute the leaf TODOs under `/docs/todo/current/` for the reported high-priority gaps.
- Expand conformance/limitations to reflect any verified drift discovered during implementation.

