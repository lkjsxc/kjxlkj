# Wave: Placeholder Scaffolding (Iteration 34)

Back: [/docs/todo/current/README.md](/docs/todo/current/README.md)

## Purpose

Create a deliberately incomplete but executable skeleton so later waves can fill it in without redesigning the whole project repeatedly.

This wave is expected to contain many placeholders.

## Checklist (normative)

### A. Establish an explicit “source of truth” loop

- [ ] Confirm the canonical spec set is under:
  - [/docs/spec/README.md](/docs/spec/README.md)
- [ ] Confirm project policies and constraints are under:
  - [/docs/policy/README.md](/docs/policy/README.md)
- [ ] Confirm current surface tracking exists and is treated as authoritative for "what is implemented now":
  - [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
  - [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)

### B. Create placeholder plan structure for implementation work

- [ ] Create child checklists under:
  - [/docs/todo/current/wave-implementation/README.md](/docs/todo/current/wave-implementation/README.md)
- [ ] In each child checklist, include:
  - scope statement
  - direct links to the defining spec documents
  - placeholder acceptance criteria (Given/When/Then) to be refined later
  - placeholder test list (unit/integration/E2E) to be implemented later

### C. Create placeholder plan structure for verification work

- [ ] Create child checklists under:
  - [/docs/todo/current/wave-verification/README.md](/docs/todo/current/wave-verification/README.md)
- [ ] Include explicit placeholders for:
  - doc link validation workflow
  - documentation fence compliance (Mermaid-only)
  - conformance updates required for each implemented slice
  - performance/latency baselines

### D. Guardrails for later waves

- [ ] Record all known missing/placeholder areas as explicit TODO leaves (no implicit assumptions).
- [ ] Do not implement new behavior in code during this wave unless it is required to unblock later waves' test harness and verification pipeline.
