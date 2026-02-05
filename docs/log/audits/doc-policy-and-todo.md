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
| No non-Mermaid fenced blocks under `/docs/` | [/docs/policy/README.md](/docs/policy/README.md) | Pass |
| Docs ≤200 lines per file | [/docs/policy/STRUCTURE.md](/docs/policy/STRUCTURE.md) | Pass |
| Direct children ≤12 per directory | [/docs/policy/STRUCTURE.md](/docs/policy/STRUCTURE.md) | Pass |
| Exactly one `README.md` per directory | [/docs/policy/STRUCTURE.md](/docs/policy/STRUCTURE.md) | Pass |
| No `../` links in docs | Project constraint + TODO policy | Pass |
| TODO wave list uses checkboxes | [/docs/todo/current/README.md](/docs/todo/current/README.md) | Pass |
| Current iteration is not pre-completed | [/docs/todo/current/README.md](/docs/todo/current/README.md) | Pass |
| Completion handshake invokes `Ask` | [/docs/policy/WORKFLOW.md](/docs/policy/WORKFLOW.md) | Pass |
| Doc coverage is a topology index | [/docs/todo/doc-coverage/README.md](/docs/todo/doc-coverage/README.md) | Pass |

## Notes

- Root-level `LICENSE` is 201 lines (legal text) and is treated as an allowed exception to the 200-line guideline, which applies to documentation under `/docs/` and source files under `src/` when reconstructed.

## Repairs made

This audit repaired drift between the intended workflow and what the TODO system
made machine-actionable:

- Converted the current TODO wave list into real task checkboxes and fixed
  checkbox semantics.
- Rolled `/docs/todo/current/` forward to a fresh iteration with all tasks
  unchecked so a docs-only baseline remains executable.
- Converted `/docs/todo/doc-coverage/` from a progress checklist into a pure link
  index (no long-lived checkmark history).
- Added an explicit completion handshake: when the iteration is complete and the
  verification gate is green, invoke `Ask` to request the next objective.

## Follow-ups (recorded as TODO leaves)

- None. Any newly discovered gaps MUST become TODO leaves under
  `/docs/todo/current/`.
