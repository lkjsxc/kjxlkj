# Reading Discipline

Back: [/docs/todo/README.md](/docs/todo/README.md)

## Purpose

Maintain a deterministic “read before implement” discipline without keeping long-lived historical logs that can become noise.

The canonical traversal mechanism is the doc coverage set:

- [/docs/todo/doc-coverage/README.md](/docs/todo/doc-coverage/README.md)

## Normative rules

- Reading MUST be performed before implementation work that changes observable behavior.
- Reading MUST prioritize:
  - `/docs/policy/` (constraints)
  - `/docs/spec/` (target behavior)
  - `/docs/reference/` (current surface and known gaps)
  - `/docs/todo/current/` (execution plan)
- Contradictions MUST be resolved by:
  - updating the canonical spec/policy document, and/or
  - recording user-visible drift in `/docs/reference/LIMITATIONS.md`, and/or
  - creating a proposal under `/docs/log/proposals/` when design work is required.

## Minimal recording (only when useful)

If an iteration requires recording what was read, keep it minimal and non-historical:

- add a short note to an audit under `/docs/log/audits/` describing:
  - which doc subtree was reviewed
  - what contradictions were found
  - what canonical edits were made
