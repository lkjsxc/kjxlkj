# Reading Discipline

Back: [/docs/todo/README.md](/docs/todo/README.md)

## Purpose

Maintain deterministic read-before-implement behavior without accumulating
unnecessary historical noise.

## Required Priority

- [x] `/docs/policy/` first
- [x] `/docs/spec/` second
- [x] `/docs/reference/` third
- [x] `/docs/todo/current/` fourth

## Rules

- [x] read canonical docs before any behavior-changing implementation work
- [x] resolve contradictions in canonical docs first, not in temporary notes
- [x] if a gap is intentionally deferred, record it in limitations and TODO
- [x] avoid creating long-lived historical notes when canonical docs can be updated directly

## Minimal Recording

When recording is required, store concise evidence only in canonical ledgers:

- [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md)
- [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
