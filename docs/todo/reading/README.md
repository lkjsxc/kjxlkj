# Reading Discipline

Back: [/docs/todo/README.md](/docs/todo/README.md)

## Purpose

Maintain deterministic read-before-implement behavior without accumulating
unnecessary historical noise.

## Required Priority

- [ ] `/docs/policy/` first
- [ ] `/docs/spec/` second
- [ ] `/docs/reference/` third
- [ ] `/docs/todo/current/` fourth

## Rules

- [ ] read canonical docs before any behavior-changing implementation work
- [ ] resolve contradictions in canonical docs first, not in temporary notes
- [ ] if a gap is intentionally deferred, record it in limitations and TODO
- [ ] avoid creating long-lived historical notes when canonical docs can be updated directly

## Minimal Recording

When recording is required, store concise evidence only in canonical ledgers:

- [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md)
- [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
