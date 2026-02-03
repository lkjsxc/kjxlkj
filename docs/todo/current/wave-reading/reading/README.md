# Reading (Iteration 33)

Back: [/docs/todo/current/wave-reading/README.md](/docs/todo/current/wave-reading/README.md)

## Purpose

Make the TODO system “doc-complete”: every documentation file is explicitly linked, and is read in-depth before implementing or changing behavior.

## Requirements (normative)

- The reading checklist MUST link to every documentation file under `/docs/` excluding `/docs/todo/`.
- The reading log MUST record what was fully read in this iteration.
- Notes MUST focus on requirements, invariants, and acceptance criteria (not implementation sketches).

## Work Items

### A. Complete the doc-coverage checklist

- Follow the recursive checklist:
  - [/docs/todo/doc-coverage/README.md](/docs/todo/doc-coverage/README.md)
- For each directory page:
  - [ ] Read every document linked under `### Files`.
  - [ ] Continue recursively into every entry under `### Subdirectories`.

### B. Record reading completion and contradictions

- Add a dated entry under:
  - [/docs/todo/reading/README.md](/docs/todo/reading/README.md)
- In the iteration entry, record:
  - which top-level directories are fully read
  - contradictions found (links)
  - follow-up TODO leaves created under `/docs/todo/current/`

### C. Gate behavior changes on reading + conformance + tests

- For any behavior change, re-read the relevant spec subtree in `/docs/todo/doc-coverage/`.
- If the change touches `/docs/spec/`, update:
  - [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
  - [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md) (when user-visible)
- Add or update unit/integration/E2E tests to prevent regressions.
