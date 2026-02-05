# Reading (Iteration 35)

Back: [/docs/todo/current/wave-reading/README.md](/docs/todo/current/wave-reading/README.md)

## Purpose

Make the TODO system “doc-complete”: every documentation file is explicitly linked, and is read in-depth before implementing or changing behavior.

## Requirements (normative)

- The reading checklist MUST link to every documentation file under `/docs/` excluding `/docs/todo/`.
- Notes (when recorded) MUST focus on requirements, invariants, and acceptance criteria (not implementation sketches).
- Avoid long-lived historical reading logs; use minimal, targeted audit notes when the record is actually useful.

## Work Items

### A. Complete the doc-coverage checklist

- Follow the recursive checklist:
  - [/docs/todo/doc-coverage/README.md](/docs/todo/doc-coverage/README.md)
- For each directory page:
  - [ ] Read every document linked under `### Files`.
  - [ ] Continue recursively into every entry under `### Subdirectories`.

### B. Record reading completion and contradictions

- If recording is useful for a specific iteration, add a short audit note under:
  - [/docs/log/audits/README.md](/docs/log/audits/README.md)
- In the audit note, record only:
  - which doc subtree was reviewed
  - contradictions found (links)
  - follow-up TODO leaves created under `/docs/todo/current/`

### C. Gate behavior changes on reading + conformance + tests

- For any behavior change, re-read the relevant spec subtree in `/docs/todo/doc-coverage/`.
- If the change touches `/docs/spec/`, update:
  - [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
  - [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md) (when user-visible)
- Add or update unit/integration/E2E tests to prevent regressions.
