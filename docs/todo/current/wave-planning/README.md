# Wave: Planning and Specification Closure (Iteration 34)

Back: [/docs/todo/current/README.md](/docs/todo/current/README.md)

## Purpose

Convert “what the documents say” into a complete, implementation-ordered plan with explicit acceptance criteria and test requirements.

## Entry points

| Checklist | What it covers |
|---|---|
| [cursor-viewport-input/README.md](cursor-viewport-input/README.md) | Cursor rendering/movement, viewport follow, input ordering |
| [terminal-spec/README.md](terminal-spec/README.md) | Integrated terminal specification closure |
| [testing-spec/README.md](testing-spec/README.md) | Normative test plan (unit/integration/E2E) |

## Checklist (normative)

### A. Plan completeness and ordering

- [x] Every spec area has an implementation checklist under:
  - [/docs/todo/current/wave-implementation/README.md](/docs/todo/current/wave-implementation/README.md)
- [x] The plan is ordered so later items depend only on earlier items.
- [x] Each leaf item defines:
  - observable behavior
  - acceptance criteria (Given/When/Then)
  - test strategy (unit/integration/E2E)
  - required conformance updates

### B. Doc-to-code mapping (explicit)

- [x] For each major spec subtree, define:
  - which crate(s) own it
  - which module(s) are source-of-truth
  - which tests prove the behavior
- [x] Keep the mapping synchronized in:
  - [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
  - [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)

### C. Resolve repository-scope contradictions (if any remain)

- [x] Ensure docs do not contradict the repository's actual artifacts:
  - [/docs/policy/ROOT_LAYOUT.md](/docs/policy/ROOT_LAYOUT.md)
- [x] If the implementation topology differs from the spec topology, record:
  - the decision and rationale in `/docs/reference/IMPLEMENTATION_HISTORY.md`
  - the resulting limitations in `/docs/reference/LIMITATIONS.md`
